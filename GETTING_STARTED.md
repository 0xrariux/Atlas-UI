# Getting started with Atlas UI

The reference example lives in `examples/getting-started` and is compiled by
every quality-gate run. Check it directly with:

```bash
cargo check -p atlas-ui-getting-started
```

Add the published Atlas facade as a normal and build dependency. This guide
targets the crates.io `0.1.1` packages and their matching `v0.1.1` source tag.

```toml
[dependencies]
atlas-ui = "=0.1.1"
slint = "=1.17.1"

[build-dependencies]
atlas-ui = "=0.1.1"
slint-build = "=1.17.1"
```

The stable facade requires no experimental Slint configuration. If the
application imports responsive contracts from `@atlas-ui/preview.slint`, enable
experimental compilation in the consumer repository:

```toml
# .cargo/config.toml
[env]
SLINT_ENABLE_EXPERIMENTAL_FEATURES = "1"
```

Rust `1.92` or newer is required by `slint-build 1.17.1`.

Configure the named Slint libraries and compile your entry point:

```rust
// build.rs
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(atlas_ui::slint_library_paths());
    slint_build::compile_with_config("ui/app.slint", config)
        .expect("compile Atlas UI consumer");
}
```

```slint
import { AtlasButton, AtlasTextField, AtlasSettings, ThemeMode }
    from "@atlas-ui/stable.slint";

export component App inherits Window {
    AtlasSettings.theme-mode: ThemeMode.dark;
    VerticalLayout {
        AtlasTextField { label: "Project"; placeholder: "atlas-ui"; }
        AtlasButton { text: "Create"; clicked => { root.create-requested(); } }
    }
    callback create-requested();
}
```

With `slint::include_modules!()`, public properties and callbacks become Rust
bindings. Keep data, networking, and persistence in Rust; Atlas components emit
intentions and never trigger silent remote actions.

See `docs/COMPONENTS.md` for public component families and
`docs/SLINT_INTEGRATION.md` for maturity levels and Slint-related constraints.
