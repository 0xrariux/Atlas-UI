# Atlas UI

Atlas UI is an experimental native Rust and Slint design system with shared
tokens, accessible components, responsive compositions, application templates,
and deterministic visual validation.

Atlas does not replace Slint. Slint provides the declarative language, runtime,
and rendering foundation; Atlas provides reusable visual and interaction
contracts on top of it.

Atlas `0.2.0` requires Rust `1.92` and pins Slint `1.18.0`. All facades compile
without experimental Slint configuration. The locally validated visual profile
is macOS arm64 with Slint's software renderer at scale factor 1; the Slint 1.18
reference set and native Linux/Windows runs are pending release review.

- [Project repository](https://github.com/0xrariux/Atlas-UI)
- [Getting started](https://github.com/0xrariux/Atlas-UI/blob/main/GETTING_STARTED.md)
- [Compatibility matrix](https://github.com/0xrariux/Atlas-UI/blob/main/docs/COMPATIBILITY.md)
- [Component index](https://github.com/0xrariux/Atlas-UI/blob/main/docs/AGENT_COMPONENT_INDEX.md)
- [Visual workflow for coding agents](https://github.com/0xrariux/Atlas-UI/blob/main/docs/AGENT_VISUAL_WORKFLOW.md)
- [Companion template showcase](https://github.com/0xrariux/template-atlas)

The `0.2.0` source is a release candidate. The published crates.io `0.1.1`
package and tagged `v0.1.1` source remain on Slint `1.17.1`. Use a path
dependency when testing the candidate before registry publication.

Use `atlas_ui::slint_library_paths()` from `build.rs`, then import stable Slint
contracts from `@atlas-ui/stable.slint`. Prefer
`@atlas-ui/preview-nonresponsive.slint` for evolving controls that do not need
responsive layout. Preview APIs may change in minor Atlas releases.

For agent-generated screens, the dependency path alone is not a visual brief.
Give the agent a product reference and target viewports, make the Atlas agent
documents explicit in the consumer's instructions, and require rendered
screenshot review instead of treating compilation as visual validation.
