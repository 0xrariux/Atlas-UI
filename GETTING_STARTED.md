# Getting started with Atlas UI

Choose the dependency pair that matches the Atlas version you want to use.
Both require Rust 1.92 or newer. Add `atlas-ui` as both a runtime and build
dependency so `build.rs` can configure the named Slint imports.

## Current source checkout: Slint 1.18.0

Place your application beside an Atlas checkout. Adjust the path if your
directories differ. The 0.2.0 release candidate has not been published to crates.io yet.

```toml
[dependencies]
atlas-ui = { path = "../Atlas/crates/atlas-ui" }
slint = "=1.18.0"

[build-dependencies]
atlas-ui = { path = "../Atlas/crates/atlas-ui" }
slint-build = "=1.18.0"
```

## Published v0.1.1: Slint 1.17.1

For an application that does not use the local checkout, install the published
crate and its matching Slint version:

```toml
[dependencies]
atlas-ui = "=0.1.1"
slint = "=1.17.1"

[build-dependencies]
atlas-ui = "=0.1.1"
slint-build = "=1.17.1"
```

Only with this published version, imports from `preview.slint` or
`components.slint` require the Slint experimental flexbox flag. Stable and
`preview-nonresponsive.slint` imports do not. Add the flag to the consuming
repository's `.cargo/config.toml` when needed:

```toml
[env]
SLINT_ENABLE_EXPERIMENTAL_FEATURES = "1"
```

The current Slint 1.18.0 source checkout needs no experimental flag.

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
