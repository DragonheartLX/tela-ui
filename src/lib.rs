slint::include_modules!();

fn ui() -> MainWindow {
    MainWindow::new().unwrap()
}

/// Apply the theme preference from the `TELA_THEME` environment variable
/// ("light" / "dark"); anything else follows the OS color scheme.
fn apply_theme_env(ui: &MainWindow) {
    match std::env::var("TELA_THEME").as_deref() {
        Ok("dark") => {
            TelaTheme::get(ui).set_preference(TelaThemePreference::Dark);
        }
        Ok("light") => {
            TelaTheme::get(ui).set_preference(TelaThemePreference::Light);
        }
        _ => {}
    }
}

pub fn main() {
    let ui = ui();
    apply_theme_env(&ui);
    ui.run().unwrap();
}
