# Atlas UI

Atlas UI is an experimental native Rust and Slint design system with shared
tokens, accessible components, responsive compositions, application templates,
and deterministic visual validation.

Atlas does not replace Slint. Slint provides the declarative language, runtime,
and rendering foundation; Atlas provides reusable visual and interaction
contracts on top of it.

Atlas `0.2.0` requires Rust `1.88` and pins Slint `1.17.1`. The currently proven
profile is macOS arm64 with Slint's software renderer at scale factor 1.

- [Project repository](https://github.com/rariux/Atlas-UI)
- [Getting started](https://github.com/rariux/Atlas-UI/blob/main/GETTING_STARTED.md)
- [Compatibility matrix](https://github.com/rariux/Atlas-UI/blob/main/docs/COMPATIBILITY.md)
- [Component index](https://github.com/rariux/Atlas-UI/blob/main/docs/AGENT_COMPONENT_INDEX.md)

Use `atlas_ui::slint_library_paths()` from `build.rs`, then import stable Slint
contracts from `@atlas-ui/stable.slint`. Preview APIs may change in minor Atlas
releases.
