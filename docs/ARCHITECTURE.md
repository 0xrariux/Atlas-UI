# Architecture and layers

Atlas is a Rust workspace divided by responsibility. A lower layer never
depends on a higher layer, and dependency cycles are forbidden.

## 1. Tokens — `atlas-ui-tokens`

The first layer contains atomic visual decisions: semantic palette, themes, a
4 px grid, spacing, sizes, typography, densities, elevation, shapes, motion,
viewports, and z-order. Higher-level components do not introduce arbitrary
colors or dimensions.

## 2. Core — `atlas-ui-core`

Core turns tokens into geometric and behavioral primitives: surfaces, content
frames, focus, grids, scrolling, intrinsic layouts, split views, and headless
controllers. This layer has no domain knowledge.

## 3. Icons — `atlas-ui-icons`

This crate owns the icon registry, deterministic SVGs, sizes, and semantic
tones. Assets are checksum-controlled and separate from typography. Original
icons are licensed under CC0-1.0.

## 4. Components — `atlas-ui-components`

This layer composes tokens, core, and icons into the user-facing API. It
contains controls, navigation, data presentation, editorial content,
documentation, and templates. Four facades define the contract:

- `stable.slint`: SemVer-governed symbols;
- `preview-nonresponsive.slint`: evolving controls without responsive layout
  imports;
- `preview.slint`: the complete evolving surface, plus compatibility
  re-exports for promoted contracts; it also loads responsive contracts;
- `components.slint`: stable-plus-preview compatibility aggregate, which also
  loads responsive contracts.

The current component layer uses a two-level overlay pattern. Preview
`AtlasModalFrame` and `AtlasDrawerFrame` own the controlled visibility,
semantic boundary, panel, dismissal, traversal, and focus-restoration
contracts around consumer-owned child content. `AtlasModal` and `AtlasDrawer`
inherit those frames and add the standard title/body/action composition.
`AtlasSettingsRow` and `AtlasChartFrame` follow the same lower-boundary model:
Atlas owns reusable semantics and presentation while the child owns values,
series, persistence, and domain behavior.

Presentation hooks on scrollbar, progress, switch, metric, settings, chart,
and overlay components are bounded component inputs. The scrollbar defaults
extend the existing `AtlasViewportTokens` global because both
`AtlasScrollbar` and `AtlasScrollViewport` share that anatomy; the other hooks
do not introduce new global design-token contracts. Dynamic chart series, axes
and legends, overlay placement and collision, docking, hierarchical tree
behavior, and numeric formatting remain application or future-foundation
concerns.

## 5. Documents — `atlas-ui-documents`

This Rust crate remains independent of Slint. It provides document models, safe
Markdown, Unicode anchors, history, search, destination policies, asynchronous
loading, and selection controllers. It selects no runtime and performs no I/O.

## 6. Testing — `atlas-ui-testing`

Large fixtures, image comparators, and measurement tools live outside the
public runtime. They support the gallery and quality gate without burdening
consumer applications.

## 7. Gallery — `apps/gallery`

The gallery is executable native documentation. Every scenario has an
identifier, fixture, viewport, metadata, and baseline. It serves as a catalog,
stress test, and visual-regression infrastructure.

The source and scenario registry currently contain 77 scenarios. Interaction
and documentation-viewport specimens exercise `AtlasScrollbar` directly and
through `AtlasScrollViewport`, including the no-overflow hidden state; campaign
specimens cover the other new components and configurable progress/switch
contracts, but affected baseline
images are not implicitly approved by a source or documentation change; they
continue through the normal visual-review workflow.

## Rust/Slint boundary

Slint owns rendering, declarative composition, local focus, and animations.
Rust owns models, computation, loading, security, and external effects. Atlas
callbacks are intentions that the host validates and returns as controlled state.

## Showcase dependency direction

The companion [`template-atlas`](https://github.com/0xrariux/template-atlas)
repository contains native Slint + Atlas consumers for Command, Forge, Fleet,
and Ledger, together with rendered previews and deterministic capture tooling.
Template applications consume Atlas public facades. Atlas does not depend on
their product code.

The four applications provide cross-theme validation evidence; their product
colors, fixed shell geometry, typography choices, and domain components do not
become Atlas tokens or APIs. They are maintained as four external consumer
suites with 97 rendered states. The executable upgrade procedure is documented
in [External consumer scenarios](EXTERNAL_CONSUMER_SCENARIOS.md).

## Quality gate

`sh scripts/quality-gate.sh` is the canonical repository validation. It checks
formatting, workspace/all-target compilation, warnings-as-errors Clippy,
workspace/all-target tests, package contents, public-language and link rules,
agent-manifest/facade consistency, asset and architecture contracts, and the
scenario registry.
