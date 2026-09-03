# Compatibility matrix

This document separates pinned dependencies, verified configurations, expected
compatibility, and unsupported claims. “Expected” means the architecture is
intended to support the configuration but Atlas has not collected complete
release evidence for it.

| Dimension | Version or target | Status | Evidence or constraint |
|---|---|---|---|
| Atlas | `0.1.0` | Experimental | Stable imports no longer load experimental responsive modules; 72 release baselines are approved and five rich-table scenarios await review |
| Rust | `1.92` | Required | Effective MSRV imposed by `slint-build 1.17.1` |
| Rust edition | `2024` | Required | Workspace package edition |
| Slint | `1.17.1` | Pinned | Workspace dependencies use exact versions for `slint` and `slint-build` |
| Slint experimental compiler features | Responsive preview aggregates only | Not required by `stable.slint` or `preview-nonresponsive.slint`; required by `preview.slint` and `components.slint` because both eagerly load contracts that use `FlexboxLayout` |
| macOS | GitHub-hosted runner, Rust `1.92` | CI verified | Workspace compilation, Clippy, tests, and public contract validation run on every change |
| macOS arm64 | Software renderer, scale factor 1 | Visually verified | Current deterministic capture and performance profile |
| Other macOS renderers and scale factors | Consumer-selected | Expected, visually unverified | Validate rendering, input, focus, fonts, and performance in the consumer |
| Linux | GitHub-hosted Ubuntu runner, Rust `1.92` | CI verified | Workspace compilation, Clippy, tests, and public contract validation run on every change |
| Windows | GitHub-hosted Windows runner, Rust `1.92` | CI verified | Workspace compilation, Clippy, tests, and public contract validation run on every change |
| Embedded targets | Any renderer | Unverified | Component geometry, assets, memory, input, and licensing require target-specific evaluation |
| Stable facade | `@atlas-ui/stable.slint` | SemVer-governed | Prefer for applications |
| Non-responsive preview facade | `@atlas-ui/preview-nonresponsive.slint` | Evolving | May change in minor Atlas releases; compiles without experimental Slint features |
| Preview facade | `@atlas-ui/preview.slint` | Evolving compatibility aggregate | May change in minor Atlas releases; eagerly loads responsive contracts and requires experimental Slint features |
| Aggregate facade | `@atlas-ui/components.slint` | Compatibility entry point | Includes stable and all preview exports; eagerly loads responsive contracts and requires experimental Slint features |

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
