//! Compile runtime layout fixtures against the public Atlas library paths.

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(atlas_ui::slint_library_paths());
    slint_build::compile_with_config("ui/layout-runtime.slint", config)
        .expect("compile Atlas runtime layout fixture");
}
