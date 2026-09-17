<h1 align="center">Atlas UI</h1>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT license"></a>
  <a href="https://github.com/0xrariux/Atlas-UI/releases/tag/v0.2.0"><img src="https://img.shields.io/badge/source-v0.2.0-2379F4.svg" alt="Atlas UI source release"></a>
</p>

Atlas UI is a component library and design system for native **Rust + Slint**
applications. It provides tokens, controls, responsive layouts, and reusable
interaction contracts while the application owns its data and behavior.

## Start here

To explore the current source checkout, run the gallery:

```bash
cargo run -p atlas-ui-gallery
```

To use Atlas in your own application, follow [Getting started](GETTING_STARTED.md).
For a complete application starting point, browse the [four template previews](https://github.com/0xrariux/template-atlas#preview).

## What Atlas provides

- Stable controls, typography, themes, icons, and design tokens.
- Preview navigation, data views, overlays, responsive compositions, and templates.
- A native gallery and repeatable checks for layout, input, and rendering.
- Explicit callbacks so domain state and external effects stay in Rust.

Browse the [component catalog](docs/COMPONENTS.md) or the
[task-oriented index for coding agents](docs/AGENT_COMPONENT_INDEX.md).

## Version and maturity

| | Atlas | Slint | How to consume |
|---|---|---|---|
| Published crate | `v0.1.1` | `1.17.1` | crates.io |
| Tagged source | `v0.2.0` | `1.18.0` | local path dependency |

The current source exposes **103 public components**: 26 stable and 77 preview.
Stable imports follow SemVer; preview contracts can change in a minor release.
The Slint 1.18 migration passes the local release gate, all 97 states of the
four templates, and Rust 1.92 CI on Linux, Windows, and macOS. Atlas's 77
software-renderer references are approved for Slint 1.18. See the [compatibility matrix](docs/COMPATIBILITY.md)
and [component evidence](docs/COMPONENT_EVIDENCE.md) for precise validation.

## Documentation

- [Getting started](GETTING_STARTED.md) — install Atlas and compile a first view.
- [Component catalog](docs/COMPONENTS.md) — choose stable or preview APIs.
- [Template applications](https://github.com/0xrariux/template-atlas) — Command, Forge, Fleet, and Ledger.
- [Coding-agent quickstart](docs/AGENT_QUICKSTART.md) — use Atlas with an AI agent.
- [Documentation index](docs/README.md) — architecture, tooling, roadmap, and reference guides.

## Develop Atlas

```bash
sh scripts/quality-gate.sh
```

The [engineering guide](docs/ENGINEERING.md) explains the component and test
contracts. Report defects or propose improvements with the
[issue forms](https://github.com/0xrariux/Atlas-UI/issues/new/choose).

## License

Atlas UI is [MIT licensed](LICENSE). Slint, fonts, and third-party assets retain
their own licenses.
