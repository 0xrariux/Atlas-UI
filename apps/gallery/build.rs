//! Compiles the native Atlas UI gallery.

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(atlas_ui::slint_library_paths());
    slint_build::compile_with_config("ui/gallery.slint", config)
        .expect("failed to compile the Atlas UI gallery");
}
