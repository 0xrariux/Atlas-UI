# Compatibility matrix

This document separates pinned dependencies, verified configurations, expected
compatibility, and unsupported claims. “Expected” means the architecture is
intended to support the configuration but Atlas has not collected complete
release evidence for it.

| Dimension | Version or target | Status | Evidence or constraint |
|---|---|---|---|
| Atlas | `0.2.1` | Experimental | Stable imports no longer load experimental responsive modules; all 72 release baselines are approved |
| Rust | `1.92` | Required | Effective MSRV imposed by `slint-build 1.17.1` |
| Rust edition | `2024` | Required | Workspace package edition |
| Slint | `1.17.1` | Pinned | Workspace dependencies use exact versions for `slint` and `slint-build` |
| Slint experimental compiler features | Preview only | Required only when importing responsive preview contracts that use `FlexboxLayout` |
| macOS | GitHub-hosted runner, Rust `1.92` | CI verified | Workspace compilation, Clippy, tests, and public contract validation run on every change |
| macOS arm64 | Software renderer, scale factor 1 | Visually verified | Current deterministic capture and performance profile |
| Other macOS renderers and scale factors | Consumer-selected | Expected, visually unverified | Validate rendering, input, focus, fonts, and performance in the consumer |
| Linux | GitHub-hosted Ubuntu runner, Rust `1.92` | CI verified | Workspace compilation, Clippy, tests, and public contract validation run on every change |
| Windows | GitHub-hosted Windows runner, Rust `1.92` | CI verified | Workspace compilation, Clippy, tests, and public contract validation run on every change |
| Embedded targets | Any renderer | Unverified | Component geometry, assets, memory, input, and licensing require target-specific evaluation |
| Stable facade | `@atlas-ui/stable.slint` | SemVer-governed | Prefer for applications |
| Preview facade | `@atlas-ui/preview.slint` | Evolving | May change in minor Atlas releases |
| Aggregate facade | `@atlas-ui/components.slint` | Compatibility entry point | Includes stable and preview exports; maturity is less explicit at each import site |

Atlas's MIT license does not replace Slint's licensing terms. Consumers remain
responsible for selecting a Slint license appropriate to their application.

## Meaning of platform validation

The public GitHub Actions workflow validates the complete workspace on Linux,
Windows, and macOS. Each platform compiles all targets, runs Clippy with
warnings denied, executes the full test suite, and checks Atlas's public API,
package, agent-manifest, and release contracts.

Cross-platform CI does not establish pixel-identical rendering across operating
systems, graphics backends, font stacks, display scales, or input devices. The
deterministic visual baselines remain tied to the explicitly listed macOS arm64
software-renderer profile. Consumers should capture and review rendering on
their own deployment configuration before making a production support claim.

When updating Slint, follow the compatibility process in
`docs/SLINT_INTEGRATION.md` and regenerate all API and visual evidence before
changing this matrix.
