use std::{collections::HashMap, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // The component library lives in the root-level `tela-ui/` folder and is
    // registered under the `tela-ui` import name, so consumers write:
    //   import { Button } from "tela-ui/button.slint";
    let library_paths = HashMap::from([("tela-ui".to_string(), manifest_dir.join("tela-ui"))]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths)
        .with_style("fluent".into());

    slint_build::compile_with_config("ui/MainWindow.slint", config).unwrap();
}
