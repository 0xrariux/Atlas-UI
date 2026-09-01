# Integration guide for AI agents

This guide provides the minimum technical information needed to generate code
that consumes Atlas UI without relying on private context.

For complete screen generation, also follow
`docs/AGENT_VISUAL_WORKFLOW.md`. This file describes API integration; it does
not replace a product visual brief or screenshot review.

## Choose a facade

For new code, import guaranteed foundations:

```slint
import {
    AtlasButton, AtlasTextField, AtlasHeading, AtlasTheme
} from "@atlas-ui/stable.slint";
```

A non-responsive feature unavailable in the stable facade may use preview
without enabling experimental Slint features:

```slint
import { AtlasDataTable, AtlasModal } from "@atlas-ui/preview-nonresponsive.slint";
```

Use `@atlas-ui/preview.slint` only for responsive preview contracts. That
compatibility facade and `@atlas-ui/components.slint` eagerly load
`FlexboxLayout`, so they require `SLINT_ENABLE_EXPERIMENTAL_FEATURES=1` even
when an import names only a non-responsive symbol. Do not mix maturity levels
unless the consumer explicitly acknowledges its preview dependency.

## Screen model

```slint
import {
    AtlasTheme, AtlasGrid, AtlasHeading, HeadingLevel,
    AtlasTextField, AtlasButton
} from "@atlas-ui/stable.slint";

export component ExampleScreen inherits Rectangle {
    callback submit-requested(string);
    in-out property <string> value;

    background: AtlasTheme.canvas;

    VerticalLayout {
        padding: AtlasGrid.space-6;
        spacing: AtlasGrid.space-4;

        AtlasHeading { text: "Create item"; level: HeadingLevel.h1; }
        AtlasTextField {
            label: "Name";
            value <=> root.value;
        }
        AtlasButton {
            text: "Create";
            clicked => { root.submit-requested(root.value); }
        }
    }
}
```

The callback creates nothing by itself. The Rust host receives the intention,
runs application logic, and then supplies the new state.

## Current Rust/Slint configuration

The compiled example lives in `examples/getting-started`. Until the crates are
published, an external application references the Atlas workspace by path or
Git dependency. Use `atlas_ui::slint_library_paths()` with
`slint_build::CompilerConfiguration::with_library_paths()` so imports remain
independent of Cargo's package extraction layout.

## Authoritative references

- `docs/atlas-ui-agent-manifest.json`: generated versions, facades, component
  signatures, inherited properties, callbacks, types, globals, defaults, and
  source locations;
- `docs/AGENT_VISUAL_WORKFLOW.md`: external-repository context, visual inputs,
  rendering loop, review checklist, and reusable task prompt;
- `docs/AGENT_MANIFEST.md`: manifest schema and query examples;
- `docs/AGENT_COMPONENT_INDEX.md`: lookup by need and component family;
- `crates/atlas-ui-components/ui/stable.slint`: stable properties and types;
- `crates/atlas-ui-components/ui/preview-nonresponsive.slint`: preview
  contracts without experimental layout dependencies;
- `crates/atlas-ui-components/ui/preview.slint`: complete preview compatibility
  aggregate, including experimental responsive contracts;
- `examples/getting-started`: compiled integration;
- `screenshots/scenarios.json`: addressable visual evidence.

Public `.slint` declarations are the final authority for property, callback,
enum, and struct names. An agent must not extrapolate an API from a component name.

## Recommended agent workflow

1. Establish a product reference, target viewports, theme, density, and required
   states.
2. Use `AGENT_COMPONENT_INDEX.md` to select a component by user need.
3. Query `atlas-ui-agent-manifest.json` for its maturity and complete effective
   signature, including inherited Atlas properties.
4. Prefer stable components. If preview is necessary, make that dependency
   explicit in the generated code and handoff.
5. Follow the manifest's `source` and `source_line` fields when implementation
   details or final confirmation are required.
6. Start from `minimal_example`, then provide host-owned data and callbacks.
7. Compile the consumer, capture the requested viewports, inspect the rendered
   screenshots, and refine the composition against the product reference.
8. When changing Atlas itself, run `sh scripts/quality-gate.sh`.

The manifest is generated from public Slint declarations. Regenerate it after
an API change with:

```bash
cargo run -p atlas-ui-tooling -- generate-agent-manifest
```

The quality gate runs the generator in check mode and rejects stale metadata.
