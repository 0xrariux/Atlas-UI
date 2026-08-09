# Engineering and quality

Atlas is developed from the bottom up: tokens, geometry, headless behavior,
components, compositions, and then templates. A generic visual feature belongs
in the lowest relevant layer; a domain rule remains in the application.

## Component contract

Before a component is considered complete, it has:

- a use case and a bounded API;
- default, hover, pressed, focused, and disabled states when relevant;
- controlled properties and intention callbacks;
- tokens with no arbitrary visual values;
- a keyboard, focus, and automated-accessibility strategy;
- short, long, empty, and localized text coverage;
- evidence in light/dark themes and a constrained viewport;
- a deterministic scenario, fixture, and metadata;
- a stable or preview classification;
- an entry in registries, documentation, and the changelog.

## Deterministic validation

The 72 scenarios are addressable by identifier. A comparison is valid only
when its scenario, fixture, viewport, platform profile, and metadata match.
Every recapture resets human approval so a new image is never accepted implicitly.

The quality gate runs, among other checks:

- `cargo fmt`, `cargo check`, Clippy, and all tests;
- dependency rules and cycle detection;
- token and visual-literal checks;
- public API snapshots and facade partitioning;
- scenarios, contrast, and automated accessibility;
- Rust and software-rendering budgets;
- font and icon checksums and licenses;
- release readiness and Slint compatibility tracking.

## API policy

The stable facade is additive within a minor version. Removing, renaming, or
incompatibly changing an item requires a major version. The preview facade may
evolve in a minor version, but every change is recorded in the changelog and registries.

## Public sources of truth

- `docs/`: architecture, components, integration, and methodology;
- `stable.slint`: the SemVer-governed contract;
- `preview.slint`: the explicitly evolving surface;
- `Cargo.toml` and `Cargo.lock`: Rust and Slint versions;
- `screenshots/`: scenarios, metadata, and baselines;
- `CHANGELOG.md`: evolution of published contracts.

Local planning records and manifests supplement these sources during
development but are not part of the public distribution.

Changes must update the affected implementation, examples, registries,
documentation, tests, and visual evidence together.
