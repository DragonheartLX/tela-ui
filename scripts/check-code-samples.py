# Gallery code-sample gate: extract every `code:` string from ui/pages/*.slint,
# wrap each into a standalone compilable file, and run slint-viewer --check on it.
# A sample is only valid if it compiles standalone — the embedded strings never
# ride through the normal compile, so API drift rots them invisibly.
#
# Usage: python scripts/check-code-samples.py
# Exit code 1 if any sample reports diagnostics (errors or warnings).

import os
import re
import shutil
import subprocess
import sys
import tempfile

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PAGES = os.path.join(REPO, "ui", "pages")
CODE_RE = re.compile(r'(?m)^[ \t]*code\s*:\s*"')


def parse_string(src, i):
    """i points at the opening quote; return (value, index_after_closing_quote)."""
    out = []
    i += 1
    while i < len(src):
        c = src[i]
        if c == "\\":
            nxt = src[i + 1]
            if nxt == "n":
                out.append("\n")
            elif nxt == "t":
                out.append("\t")
            elif nxt == "\\":
                out.append("\\")
            elif nxt == '"':
                out.append('"')
            elif nxt == "'":
                out.append("'")
            else:
                raise ValueError("unknown escape \\" + nxt)
            i += 2
        elif c == '"':
            return "".join(out), i + 1
        else:
            out.append(c)
            i += 1
    raise ValueError("unterminated string literal")


def extract_samples():
    samples = []
    for fname in sorted(os.listdir(PAGES)):
        if not fname.endswith(".slint"):
            continue
        src = open(os.path.join(PAGES, fname), encoding="utf-8").read()
        for m in CODE_RE.finditer(src):
            val, _ = parse_string(src, m.end() - 1)
            samples.append((fname, src.count("\n", 0, m.start()) + 1, val))
    return samples


def wrap(text):
    lines = text.split("\n")
    imports = [l for l in lines if l.startswith("import ")]
    body = "\n".join(l for l in lines if not l.startswith("import ")).strip("\n")
    first = next((l for l in body.split("\n") if l.strip()), "")
    header = "".join(imp + "\n" for imp in imports) + ("\n" if imports else "")
    # A sample that declares a whole component (e.g. a window root via
    # `export component X inherits TBorderlessWindow`) compiles as-is;
    # element fragments get wrapped in a plain Rectangle host.
    if first.startswith(("export component", "component ")):
        return header + body + "\n"
    return header + "export component Sample inherits Rectangle {\n" + body + "\n}\n"


def main():
    samples = extract_samples()
    scratch = tempfile.mkdtemp(prefix="tela-samples-")
    fail = 0
    try:
        for idx, (fname, line, text) in enumerate(samples):
            path = os.path.join(scratch, f"{idx:02d}-{fname}.slint")
            with open(path, "w", encoding="utf-8", newline="\n") as f:
                f.write(wrap(text))
            r = subprocess.run(
                ["slint-viewer", "-L", f"tela-ui={REPO}\\tela-ui", "--check",
                 path, "--style", "fluent"],
                capture_output=True, text=True,
            )
            out = (r.stdout + r.stderr).strip()
            if out:
                fail += 1
                clean = re.sub(r"\x1b\[[0-9;]*m", "", out)
                msgs = [l for l in clean.splitlines()
                        if l.startswith(("error", "warning"))]
                print(f"FAIL {fname}:{line}")
                for mline in msgs:
                    print(f"   {mline}")
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    print(f"{len(samples)} samples, {fail} with diagnostics")
    return 1 if fail else 0


if __name__ == "__main__":
    sys.exit(main())
