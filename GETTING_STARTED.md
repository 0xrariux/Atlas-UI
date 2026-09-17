# Getting started with Atlas UI

Atlas `0.2.2` requires Rust 1.92 or newer and Slint 1.18.0. Add `atlas-ui` as both a runtime and build
dependency so `build.rs` can configure the named Slint imports.

## Published crate

```toml
[dependencies]
atlas-ui = "=0.2.2"
slint = "=1.18.0"

[build-dependencies]
atlas-ui = "=0.2.2"
slint-build = "=1.18.0"
```

## Local source checkout

Place your application beside an Atlas `v0.2.2` checkout and adjust the path
if your directories differ:

```toml
[dependencies]
atlas-ui = { path = "../Atlas/crates/atlas-ui", version = "=0.2.2" }
slint = "=1.18.0"

[build-dependencies]
atlas-ui = { path = "../Atlas/crates/atlas-ui", version = "=0.2.2" }
slint-build = "=1.18.0"
```

Neither form needs an experimental Slint flag.

## Compile a first view

Configure Slint's named libraries in your application's `build.rs`:

```rust
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(atlas_ui::slint_library_paths());
    slint_build::compile_with_config("ui/app.slint", config)
        .expect("compile Atlas UI consumer");
}
```

Import components in `ui/app.slint`:

```slint
import { AtlasButton, AtlasTextField } from "@atlas-ui/stable.slint";

export component App inherits Window {
    callback create-requested();
    VerticalLayout {
        AtlasTextField { label: "Project"; placeholder: "Name"; }
        AtlasButton { text: "Create"; clicked => { root.create-requested(); } }
    }
}
```

Use `slint::include_modules!()` in Rust to access the generated component and
handle `create-requested` in the application. Atlas callbacks express
intentions; the host owns data, navigation, persistence, and external effects.

The compiled workspace example lives in `examples/getting-started`; check it
with `cargo check -p atlas-ui-getting-started` from the Atlas root. Continue
with the [component catalog](docs/COMPONENTS.md) and the
[compatibility matrix](docs/COMPATIBILITY.md).
