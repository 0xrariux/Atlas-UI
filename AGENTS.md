# Atlas UI — public agent context

Atlas UI is a Rust and Slint component library. This file contains only public
technical context for coding agents.

## Public entry points

- Rust facade and Slint path helper: `crates/atlas-ui`
- Stable Slint API: `crates/atlas-ui-components/ui/stable.slint`
- Preview Slint API: `crates/atlas-ui-components/ui/preview.slint`
- Compatibility aggregate: `crates/atlas-ui-components/ui/components.slint`
- Machine-readable API catalog and signatures: `docs/atlas-ui-agent-manifest.json`
- Manifest schema and query guide: `docs/AGENT_MANIFEST.md`
- Deterministic integration procedure: `docs/AGENT_QUICKSTART.md`
- Human component index: `docs/AGENT_COMPONENT_INDEX.md`
- Compiled consumer: `examples/getting-started`
- Native gallery: `apps/gallery`

## Technical boundaries

- Slint owns presentation, local interaction state and rendering.
- Rust owns domain data, navigation, persistence, network, filesystem and other
  external effects.
- Component callbacks express intentions; components do not perform hidden I/O.
- Shared visual values come from Atlas tokens.
- Stable exports follow SemVer. Preview exports may change in minor releases.
- Existing shared components are composed, not copied into applications.
- Slint is pinned in the workspace `Cargo.toml`.
- Consumer markup imports Atlas through the named `@atlas-ui` library configured
  by `atlas_ui::slint_library_paths()`.

## Verification

```bash
sh scripts/quality-gate.sh
```

The public checks remain executable without the ignored local `ai/` directory.

## Project language

English is the project's official and only language. Use English for source
comments, user-facing copy, documentation, tests, fixtures, and contribution
material.
