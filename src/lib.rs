slint::include_modules!();

mod fatal;

/// The `TELA_THEME` override: exactly `light` or `dark`, matched literally.
/// Unset follows the OS; any other value is a configuration error rather
/// than a silent fallback.
fn theme_from_env() -> Result<Option<TThemePreference>, String> {
    match std::env::var("TELA_THEME") {
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(value)) => Err(format!(
            "TELA_THEME is not valid Unicode: {}",
            value.to_string_lossy()
        )),
        Ok(value) => match value.as_str() {
            "light" => Ok(Some(TThemePreference::Light)),
            "dark" => Ok(Some(TThemePreference::Dark)),
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
    if let Some(preference) = preference {
        TTheme::get(&ui).set_preference(preference);
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
