# Binary efficiency and dead-code policy

Atlas UI aims to remain compatible with Rust's efficiency goals: applications
should pay primarily for the interface, runtime capabilities, and resources
they actually use. This document distinguishes the different kinds of footprint,
records the current architecture, and defines how Atlas will detect and limit
dead weight.

## Four different costs

| Level | What it contains | What it affects | Does it ship in the application? |
|---|---|---|---|
| Downloaded package | Published Rust crates, Slint source files, documentation, and packaged assets | Registry download size and local Cargo cache | Not by itself |
| Compilation input | The application's Slint entry point, resolved imports, Rust crates, build scripts, and selected Cargo features | Clean and incremental build time | Not necessarily |
| Embedded output | Generated UI code, reachable resources, selected Slint backends and renderers, and linked Rust code | Release binary size, memory mapping, and startup behavior | Yes |
| Dead weight | Embedded code or resources that the application cannot use but the toolchain could not remove or Atlas selected too broadly | Binary size and sometimes build time or memory | Yes, and should be measured and reduced |

A large source package does not imply an equally large executable. Conversely,
a small facade can still select a large runtime feature or embed a shared asset.
Atlas therefore reports these costs separately.

## Current Atlas behavior

Applications compile their own `.slint` entry point with `slint-build` and use
`atlas_ui::slint_library_paths()` to resolve named Atlas imports. The recommended
consumer keeps `atlas-ui` and `slint-build` in build dependencies while the
`slint` runtime remains a normal dependency.

```toml
[dependencies]
slint = "=1.17.1"

[build-dependencies]
atlas-ui = "=0.2.1"
slint-build = "=1.17.1"
```

Consumers import explicit symbols from the stable or preview facade:

```slint
import { AtlasButton, AtlasTextField } from "@atlas-ui/stable.slint";
```

The facade is a discovery and compatibility surface, not a request to instantiate
every exported component. Slint resolves the imported components and their
transitive dependencies when it generates Rust code. Components that are not
reachable from the consumer UI are not expected to produce component instances
or generated component implementations. Release linking can then remove
unreferenced Rust code where the target and link configuration permit it.

Atlas's Rust facade and layer crates are deliberately small. They primarily
expose package-safe paths to Slint source directories. The optional document
engine is disabled by default and is only included through the `documents`
Cargo feature.

## Known sources of retained weight

### Fonts

The current typography layer imports the Inter Variable and JetBrains Mono
Variable font files. Together they represent approximately 1 MiB of source
assets. When the typography dependency graph is compiled, both fonts may be
embedded even if a consumer does not render monospace content.

Configurable font profiles are therefore planned. The intended choices are an
Atlas embedded-font profile, a sans-only profile where feasible, and a
host-provided system-font profile. Any alternative must preserve predictable
metrics or clearly document that layout can differ.

### Icons

`AtlasIcon` selects an icon from a runtime enum. All assets referenced by that
dynamic selection may remain reachable when `AtlasIcon` is compiled. The current
registry is small and its SVG payload is negligible compared with fonts and the
UI runtime, but Atlas will revisit generated subsets or icon packs if the
registry grows materially.

### Slint runtime features

Slint backends and renderers are selected through Cargo features. Enabling more
than one supported renderer can intentionally place multiple implementations in
the final program so an application can select one at runtime. This flexibility
has a size and compile-time cost.

Atlas will publish verified runtime profiles rather than prescribe one universal
feature set. A desktop application, a software-rendered embedded target, and a
WebAssembly application have different requirements. Accessibility will not be
removed from a supported profile solely to produce a smaller number.

### Embedded images and translations

Slint compiler configuration determines how images, fonts, and bundled
translations are processed. Applications should only bundle translations they
ship and should choose a resource strategy appropriate to their renderer and
deployment target. Atlas examples must make these choices explicit when they
differ from Slint defaults.

## Measurement policy

Atlas will use optimized release artifacts for footprint decisions. Debug
binaries are not valid size evidence because they contain debug information and
use different optimization and linking behavior.

The planned measurement matrix includes:

| Consumer | Purpose |
|---|---|
| Minimal Slint | Establish the runtime and renderer baseline without Atlas |
| Atlas control | Measure the incremental cost of tokens, one control, icons, and typography |
| Atlas form | Measure stable edge chrome, workspace tabs, icon-only action, metric card, and categorical accent in the compiled getting-started consumer |
| Atlas data workspace | Measure preview data and responsive composition features |
| Atlas documents | Measure the optional Rust document engine and rich-content UI separately |

Each profile should record at least the uncompressed release binary, compressed
artifact size, enabled Cargo features, backend, renderer, embedded resource
inventory, clean build time, and incremental build time. Measurements must use a
named operating system, architecture, Rust version, Slint version, and linker so
results remain comparable.

## Regression rules

- New components should not pull unrelated component families into generated UI
  output.
- Large assets require an explicit ownership and reachability decision.
- Optional Rust subsystems remain behind Cargo features.
- A new default Slint feature or renderer requires a footprint review.
- Resource and binary-size changes must be compared with the appropriate
  baseline consumer, not only with the gallery.
- Size optimizations must not silently remove accessibility, keyboard behavior,
  rendering support, or documented visual guarantees.

The active implementation priorities are tracked in
[`ROADMAP.md`](../ROADMAP.md). Upstream capabilities that may improve resource or
runtime selection are tracked in
[`TECHNOLOGY_WATCHLIST.md`](../TECHNOLOGY_WATCHLIST.md). Exact supported versions
and platforms remain defined by [`COMPATIBILITY.md`](COMPATIBILITY.md).

## Upstream references

- [Slint build-time compilation](https://docs.slint.dev/latest/docs/rust/slint_build/)
- [Slint compiler resource configuration](https://docs.slint.dev/latest/docs/rust/slint_build/struct.CompilerConfiguration)
- [Slint backends and renderers](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backends_and_renderers/)
