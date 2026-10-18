//! Startup failure reporting.
//!
//! Release builds use the `windows` subsystem and `panic = "abort"`, so a
//! failed start has no console and no unwind report to reach the user; a
//! native message box is then the only channel left. Debug builds keep the
//! console, so they only write to stderr, which also keeps automated runs
//! (and the Rust event probes) from blocking on a modal dialog.

use std::panic::PanicHookInfo;

/// Reports `message` and exits with a non-zero code.
pub fn exit_with_error(message: &str) -> ! {
    eprintln!("tela: {message}");
    dialog(&format!("tela could not start:\n\n{message}"));
    std::process::exit(1);
}

/// Reports panics, which `panic = "abort"` would otherwise swallow silently.
pub fn install_panic_reporter() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        previous(info);
        dialog(&format!("tela panicked:\n\n{}", message_of(info)));
    }));
}

fn message_of(info: &PanicHookInfo<'_>) -> String {
    let payload = info
        .payload()
        .downcast_ref::<&str>()
        .map(|text| (*text).to_string())
        .or_else(|| info.payload().downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "no panic message".to_string());
    match info.location() {
        Some(location) => format!("{payload}\n\nat {location}"),
        None => payload,
    }
}

/// Shows a native message box, only where it is the only channel left:
/// Windows without a console (i.e. release builds).
#[cfg(all(windows, not(debug_assertions)))]
fn dialog(message: &str) {
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

    const MB_OK: u32 = 0x0000_0000;
    const MB_ICONERROR: u32 = 0x0000_0010;
    const MB_TASKMODAL: u32 = 0x0000_2000;

    // Declared by hand: this is the crate's only Win32 call, so it does not
    // justify a `windows-sys` dependency.
    #[link(name = "user32")]
    unsafe extern "system" {
        fn MessageBoxW(
            window: *mut core::ffi::c_void,
            text: *const u16,
            caption: *const u16,
            kind: u32,
        ) -> i32;
    }

    fn wide(text: &str) -> Vec<u16> {
        OsStr::new(text).encode_wide().chain([0]).collect()
    }

    let text = wide(message);
    let caption = wide("Tela UI");
    // Safety: both buffers are NUL-terminated and outlive the call.
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            MB_OK | MB_ICONERROR | MB_TASKMODAL,
        );
    }
}

#[cfg(not(all(windows, not(debug_assertions))))]
fn dialog(_message: &str) {}
