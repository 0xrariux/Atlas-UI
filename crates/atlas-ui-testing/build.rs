//! Compile runtime layout fixtures against the public Atlas library paths.

use std::collections::HashMap;

fn main() {
    let [tokens, core, icons] = atlas_ui_components::dependency_ui_paths();
    let library_paths = HashMap::from([
        ("atlas-ui".to_owned(), atlas_ui_components::ui_path()),
        ("atlas-ui-core".to_owned(), core),
        ("atlas-ui-icons".to_owned(), icons),
        ("atlas-ui-tokens".to_owned(), tokens),
    ]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config("ui/layout-runtime.slint", config)
        .expect("compile Atlas runtime layout fixture");
}
