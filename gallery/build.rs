use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // The component library is bundled in the workspace-root `tela-ui` crate
    // and registered under the `tela-ui` import name, so consumers write:
    //   import { Button } from "tela-ui/button.slint";
    let library_paths = tela_ui::import_paths();

    // `ui/` compiles with the library, so manifest_dir changes on rebuilds.
    println!("cargo:rerun-if-changed=ui");
    println!("cargo:rerun-if-changed={}", manifest_dir.join("tela-ui").display());

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths)
        .with_style("fluent".into())
        .with_edition((1, 19));

    slint_build::compile_with_config("ui/MainWindow.slint", config).unwrap();
}
