use std::env;
use std::path::PathBuf;

use std::collections::HashMap;

const LIB_NAME: &str = "tela";

pub fn import_paths() -> HashMap<String, PathBuf> {
    let mut import_paths = HashMap::new();

    let ui_lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui");

    import_paths.insert(LIB_NAME.to_string(), ui_lib_path);

    import_paths
}