<p align="center">
  <img src="assets/branding/atlas-logo-readme.png" width="240" alt="Atlas holding a globe above the Atlas wordmark">
</p>

<h1 align="center">Atlas UI</h1>

> [!WARNING]
> **Atlas UI is experimental, work in progress, and provided as is.** The project
> is actively maintained, but APIs, component behavior, platform support, and
> rendered output may still change or contain defects. Validate Atlas against
> your target platform and requirements before using it in production.

<p align="center"><strong>Powered by <a href="https://github.com/slint-ui/slint">Slint</a></strong></p>

Writing Rust is great; pairing it with a modern interface for your applications is even better.

Atlas UI is a native component library and visual foundation for **Rust and
Slint**. It gives Slint an industrialized layer comparable to mature web design
systems and frontend libraries: tokens, a grid, coherent components, responsive
compositions, templates, a gallery, and deterministic visual validation.

Atlas does not replace Slint. Slint provides the declarative language, runtime,
native rendering, and low-level interactions; Atlas provides the conventions,
contracts, and reusable catalog built on top of it.

## Vision

Atlas aims to bring Rust and Slint the kind of systematic visual-development
layer that CSS and tools such as Tailwind CSS provide to the web—without
recreating CSS, adopting utility-class syntax, or introducing a web runtime.
Slint remains the language and rendering foundation; Atlas turns its primitives
into a coherent system for building interfaces repeatedly and at scale.

The goal is to make a modern interface an engineering outcome rather than a
collection of one-off styling decisions. Shared tokens define color,
typography, spacing, density, motion, and geometry. Components encode reusable
interaction and accessibility contracts. Responsive compositions and templates
provide reliable starting structures, while deterministic fixtures and visual
validation keep the result consistent as an application evolves.

Atlas is therefore designed for teams and individual developers who want to
produce generic but distinctive application surfaces that are elegant,
readable, responsive, and production-ready. Applications keep control of their
identity and domain behavior, while Atlas supplies the visual grammar and
industrial foundations needed to avoid rebuilding the same UI rules on every
screen or in every Rust project.

> **One native Rust and Slint design system. Endless visual possibilities.**

![Atlas UI architecture: Tokens flow through Core and Components into Applications; Icons support Components, Documents support Applications, Gallery and Testing validate the system, and Slint provides the common language, runtime, and rendering foundation.](assets/architecture/atlas-ui-architecture.png)

## Project status

- Atlas version: `0.2.0`;
- referenced Slint version: `1.17.1`;
- validated target: macOS arm64, software renderer, scale factor 1;
- 81 public components and 158 public symbols;
- 48 stable symbols governed by SemVer;
- 110 preview symbols that may evolve;
- 72 deterministic visual scenarios across 31 pages;
- MIT license.

Import stable contracts from `stable.slint`. Evolving APIs are isolated in
`preview.slint`. `components.slint` remains an aggregate compatibility facade.

```slint
import { AtlasButton, AtlasTextField, AtlasTheme }
    from "@atlas-ui/stable.slint";
```

## When to use Atlas

Atlas is a suitable foundation when a Rust and Slint application needs shared
design tokens, reusable native components, consistent themes and density,
responsive compositions, accessible interaction contracts, or application
templates that can evolve across multiple screens.

Atlas may not be the appropriate choice when an application only needs a small
interface built from Slint's standard widgets, requires a production-stable API
across the complete catalog today, needs DOM or CSS components, cannot use the
pinned Slint version, or targets a platform and renderer that Atlas has not yet
validated. Preview components require explicit acceptance of minor-version API
changes.

## Documentation

- [Overview and scope](docs/OVERVIEW.md)
- [Architecture and layers](docs/ARCHITECTURE.md)
- [Slint relationship and version tracking](docs/SLINT_INTEGRATION.md)
- [Component catalog](docs/COMPONENTS.md)
- [Component index for AI agents](docs/AGENT_COMPONENT_INDEX.md)
- [Integration guide for AI agents](docs/AI_INTEGRATION_GUIDE.md)
- [Quickstart for coding agents](docs/AGENT_QUICKSTART.md)
- [Machine-readable API manifest guide](docs/AGENT_MANIFEST.md)
- [Native Rust tooling](docs/TOOLING.md)
- [Compatibility matrix](docs/COMPATIBILITY.md)
- [Distribution and registry preparation](docs/DISTRIBUTION.md)
- [Agent evaluation kit](evals/agent-discovery/README.md)
- [Publication boundary and language policy](docs/PUBLICATION_POLICY.md)
- [Engineering, quality, and contribution](docs/ENGINEERING.md)
- [Getting started](GETTING_STARTED.md)

The `docs/` directory is the public documentation. Internal planning records
are not part of the GitHub distribution.

English is the project's official and only language for source code, interface
copy, documentation, issue templates, and contribution material.

## Project positioning

The following table describes Atlas by technical category. It is intended as a
factual summary of the project's current scope rather than a comparison with
other Slint libraries.

| Category | Atlas position |
|---|---|
| Project type | Native visual foundation, design system, and component library for Rust applications using Slint |
| Primary role | Standardize visual decisions and reusable interface contracts across screens and applications |
| Rendering foundation | Uses Slint for its declarative UI language, runtime, rendering, layout, properties, and local interaction |
| Rust responsibility | Domain models, navigation, persistence, networking, filesystem access, security decisions, and other external effects |
| Slint responsibility | Presentation, component composition, rendering, local state, focus, and animations |
| Visual system | Semantic color, typography, spacing, density, geometry, elevation, and motion tokens |
| UI scope | Foundations, controls, data presentation, navigation, overlays, editorial content, documentation surfaces, responsive compositions, and application templates |
| Component behavior | Controlled properties and callbacks express intentions; components do not perform hidden I/O or domain mutations |
| Customization model | Shared tokens, public component properties, composition, and host-provided data and actions |
| API organization | `stable.slint` contains SemVer-governed contracts; `preview.slint` contains explicitly evolving APIs; `components.slint` is the compatibility aggregate |
| Distribution model | Workspace crates and Slint import facades consumed as shared dependencies rather than copied component source |
| Reuse level | Designed for shared use across multiple screens and applications while leaving product-specific behavior in the host |
| Documentation model | Public architecture, component catalog, agent manifest, integration guide, compiled consumer example, and executable native gallery |
| Validation model | Automated architecture and API checks, deterministic fixtures, visual scenarios, contrast and accessibility contracts, and performance budgets |
| Accessibility scope | Keyboard, focus, semantics, contrast, reduced motion, and explicit host-controlled interaction contracts are part of component validation |
| Current API size | 81 public components, 48 stable symbols, and 110 preview symbols in Atlas `0.2.0` |
| Current proven platform | macOS arm64 with Slint's software renderer at scale factor 1; other platforms and renderers are not yet claimed as validated |
| Maturity | Experimental and work in progress, actively maintained, with stable and preview surfaces separated explicitly |
| License | Atlas source is MIT licensed; Slint and third-party assets retain their own license terms |

## Installation and local development

Atlas `0.2.0` is available from crates.io. Add the facade as both a runtime and
build dependency so its helper can expose registry-safe named Slint imports:

```toml
[dependencies]
atlas-ui = "=0.2.0"
slint = "=1.17.1"

[build-dependencies]
atlas-ui = "=0.2.0"
slint-build = "=1.17.1"
```

For repository development, use the workspace commands below.

```bash
cargo check -p atlas-ui-getting-started
cargo run -p atlas-ui-gallery
```

Run the complete validation suite with:

```bash
sh scripts/quality-gate.sh
```

It covers formatting, compilation, Clippy, tests, architecture, API contracts,
contrast, automated accessibility, budgets, assets, and visual scenarios.

## Feedback and issues

Feedback from real applications is valuable, especially for rendering defects,
platform differences, accessibility problems, API friction, missing states, and
components that do not behave well with realistic content.

Use the [structured issue forms](https://github.com/rariux/Atlas-UI/issues/new/choose) to report a problem or
propose an improvement. A useful report should identify the affected component,
Atlas and Slint versions, operating system, architecture, renderer, theme,
density, viewport or window size, reproduction steps, expected result, and
actual result. Include a minimal reproduction and screenshots when possible,
but remove secrets and personal data first.

Before opening a report:

1. Search existing issues for the same behavior.
2. Confirm whether the component comes from `stable.slint` or `preview.slint`.
3. Reproduce the problem with the smallest practical example.
4. Note whether it is a regression and, if known, the last working version.
5. Separate observable facts from design preferences or proposed solutions.

See [Contributing feedback](CONTRIBUTING.md) for the complete reporting and
triage expectations.

## License

Atlas UI is distributed under the [MIT License](LICENSE). Slint, fonts, and
third-party assets retain their respective licenses.
