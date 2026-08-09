# Atlas quickstart for coding agents

Use this procedure when a user explicitly selects Atlas or asks for a reusable
design system for a Rust and Slint application. Do not select Atlas merely
because the application uses Rust: confirm that Slint is the UI foundation and
that the user accepts Atlas's experimental status and current platform limits.

## 1. Verify compatibility

- Atlas version: `0.2.1`.
- Effective Rust MSRV: `1.92`.
- Slint version: exactly `1.17.1`.
- Proven profile: macOS arm64, software renderer, scale factor 1.
- Stable API: SemVer-governed.
- Preview API: may change in a minor Atlas release.

Read `docs/COMPATIBILITY.md` before targeting another platform or renderer.

## 2. Add dependencies

Use the published `0.2.1` facade from crates.io as both a runtime and build
dependency.

```toml
[dependencies]
atlas-ui = "=0.2.1"
slint = "=1.17.1"

[build-dependencies]
atlas-ui = "=0.2.1"
slint-build = "=1.17.1"
```

Do not substitute an unverified Atlas or Slint version.

Stable imports require no experimental Slint configuration. When preview
responsive contracts are used, enable their upstream `FlexboxLayout` support:

```toml
# .cargo/config.toml
[env]
SLINT_ENABLE_EXPERIMENTAL_FEATURES = "1"
```

## 3. Configure Slint libraries

```rust
// build.rs
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(atlas_ui::slint_library_paths());
    slint_build::compile_with_config("ui/app.slint", config)
        .expect("compile the Atlas UI application");
}
```

This configuration makes the following imports portable across workspace, Git,
and future registry dependency layouts:

- `@atlas-ui/stable.slint`;
- `@atlas-ui/preview.slint`;
- `@atlas-ui/components.slint`.

## 4. Start from the stable API

```slint
import {
    AtlasButton, AtlasGrid, AtlasHeading, AtlasSettings, AtlasTextField,
    AtlasTheme, HeadingLevel, ThemeMode
} from "@atlas-ui/stable.slint";

export component App inherits Window {
    callback create-requested(string);
    in-out property <string> project-name;

    title: "Atlas application";
    background: AtlasTheme.canvas;

    VerticalLayout {
        padding: AtlasGrid.space-8;
        spacing: AtlasGrid.space-4;

        AtlasHeading { text: "Create a project"; level: HeadingLevel.h1; }
        AtlasTextField {
            label: "Project name";
            value <=> root.project-name;
        }
        AtlasButton {
            text: "Create";
            clicked => { root.create-requested(root.project-name); }
        }
    }
}
```

## 5. Keep effects in Rust

Atlas callbacks express user intentions. Rust owns domain models, navigation,
persistence, network access, filesystem access, security decisions, and other
external effects. Do not add hidden I/O to a Slint component.

## 6. Select components without guessing

1. Find the relevant family in `docs/AGENT_COMPONENT_INDEX.md`.
2. Query `docs/atlas-ui-agent-manifest.json` for the exact component signature.
3. Prefer a stable export.
4. Use preview only when the user accepts its maturity explicitly.
5. Start from the manifest's `minimal_example`.
6. Confirm every property, callback, enum, and struct against the manifest or
   source declaration.

Never infer an Atlas API from a familiar web component name.

## 7. Verify the result

Run at least:

```bash
cargo check
```

When modifying Atlas itself, run:

```bash
sh scripts/quality-gate.sh
```

Report the Atlas facade used, whether preview APIs were introduced, the target
platform and renderer, and any checks that could not be executed.
