slint::include_modules!();

mod fatal;

// The mode preference is the std-widgets Palette's in-out `color-scheme`;
// the enum is only reachable from user code through the same re-export
// path the generated code itself uses.
use slint::private_unstable_api::re_exports::ColorScheme;

/// The `TELA_THEME` override: exactly `light` or `dark`, matched literally.
/// Unset follows the OS; any other value is a configuration error rather
/// than a silent fallback.
fn theme_from_env() -> Result<Option<ColorScheme>, String> {
    match std::env::var("TELA_THEME") {
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(value)) => Err(format!(
            "TELA_THEME is not valid Unicode: {}",
            value.to_string_lossy()
        )),
        Ok(value) => match value.as_str() {
            "light" => Ok(Some(ColorScheme::Light)),
            "dark" => Ok(Some(ColorScheme::Dark)),
            _ => Err(format!(
                "TELA_THEME must be exactly \"light\" or \"dark\", got \"{value}\""
            )),
        },
    }
}

fn run() -> Result<(), String> {
    // Validated before the render backend comes up, so a typo in the
    // environment cannot be masked by a window-creation failure.
    let preference = theme_from_env()?;
    let ui = MainWindow::new().map_err(|error| format!("cannot create the window: {error}"))?;
    if let Some(color_scheme) = preference {
        Palette::get(&ui).set_color_scheme(color_scheme);
    }
    ui.run()
        .map_err(|error| format!("cannot run the event loop: {error}"))
}

pub fn main() {
    fatal::install_panic_reporter();
    if let Err(message) = run() {
        fatal::exit_with_error(&message);
    }
}
