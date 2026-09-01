# Atlas quickstart for coding agents

Use this procedure when a user explicitly selects Atlas or asks for a reusable
design system for a Rust and Slint application. Do not select Atlas merely
because the application uses Rust: confirm that Slint is the UI foundation and
that the user accepts Atlas's experimental status and current platform limits.

## 0. Load visual context

When the application is outside the Atlas repository, a Cargo path does not
automatically expose Atlas instructions to the agent. Read
`docs/AGENT_VISUAL_WORKFLOW.md` and add its Atlas context block to the consumer
repository's `AGENTS.md` or task prompt.

Obtain a product reference and explicit target viewport before composing a
screen. Atlas defines shared visual contracts, not the application's hierarchy
or art direction. A generated interface is not complete until the agent has
captured and inspected its rendered output.

## 1. Verify compatibility

- Atlas version: `0.2.1`.
- Effective Rust MSRV: `1.92`.
- Slint version: exactly `1.17.1`.
- CI-verified systems: Linux, Windows, and macOS with Rust `1.92`.
- Visually verified profile: macOS arm64, software renderer, scale factor 1.
- Stable API: SemVer-governed.
- Preview API: may change in a minor Atlas release.

Read `docs/COMPATIBILITY.md` before selecting a renderer or making a production
platform-support claim. Cross-platform CI validates the code and contracts; it
does not guarantee pixel-identical rendering on every deployment profile.

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

Stable and non-responsive preview imports require no experimental Slint
configuration. The compatibility `preview.slint` and `components.slint`
facades eagerly load the responsive module, so either one requires upstream
`FlexboxLayout` support even when the selected symbol is non-responsive:

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
- `@atlas-ui/preview-nonresponsive.slint`;
- `@atlas-ui/preview.slint`;
- `@atlas-ui/components.slint`.

Use `preview-nonresponsive.slint` for evolving controls such as `AtlasTab`,
`AtlasSpinner`, and `AtlasProgressBar` without enabling experimental Slint
features. Use `preview.slint` only when the responsive preview contracts are
also required; `components.slint` remains an experimental compatibility
aggregate.

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

Then capture the consumer at its agreed desktop and narrow viewports. Inspect
the images for clipping, overlap, unintended stretching, hierarchy, alignment,
typography, whitespace, contrast, and realistic content density. Compare them
with the product reference and iterate. Compilation alone is not visual
validation.

When modifying Atlas itself, run:

```bash
sh scripts/quality-gate.sh
```

Report the Atlas facade used, whether preview APIs were introduced, the target
platform and renderer, screenshot viewport and scale factor, remaining visual
differences, and any checks that could not be executed.
