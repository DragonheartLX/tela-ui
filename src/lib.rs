//! Library sources ship inside this crate; consumers register them:
//!
//! ```rust,ignore
//! fn main() {
//!     let config = slint_build::CompilerConfiguration::new()
//!         .with_library_paths(tela_ui::import_paths());
//! }
//! ```

use std::{collections::HashMap, path::PathBuf};

/// Import name → absolute path of the `.slint` files of this crate.
///
/// `CARGO_MANIFEST_DIR` resolves when this crate is compiled as a
/// dependency, so the path stays valid for the registry checkout
/// (`~/.cargo/registry/src/.../tela-ui-<vers>/tela-ui`).
pub fn import_paths() -> HashMap<String, PathBuf> {
    HashMap::from([(
        "tela-ui".to_string(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tela-ui"),
    )])
}
