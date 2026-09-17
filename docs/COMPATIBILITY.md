# Compatibility matrix

This document separates pinned dependencies, verified configurations, expected
compatibility, and unsupported claims. “Expected” means the architecture is
intended to support the configuration but Atlas has not collected complete
release evidence for it.

| Dimension | Version or target | Status | Evidence or constraint |
|---|---|---|---|
| Atlas tagged source | `v0.2.0` | CI and visual references verified | Stable imports remain separate from responsive preview; 77 macOS arm64 software references are approved under Slint 1.18.0 |
| Published Atlas | `0.1.1` | Released | The published package and tagged source use Slint 1.17.1 |
| Rust | `1.92` | Required | Effective MSRV imposed by `slint-build 1.18.0` |
| Rust edition | `2024` | Required | Workspace package edition |
| Slint | `1.18.0` | Pinned in source checkout | Workspace dependencies use exact versions for `slint` and `slint-build`; `const-field-offset` resolves to 0.2.1 |
| Slint experimental compiler features | None required | Verified locally | All facades compile without `SLINT_ENABLE_EXPERIMENTAL_FEATURES`; Atlas keeps responsive recipes in preview by its own API policy |
| macOS | Local Rust `1.92.0` and `1.97.1` | Build and tests verified | Clean-target workspace compilation, tests, and the full quality gate pass with Rust 1.92.0; Rust 1.92 hosted CI passed |
| macOS arm64 | Software renderer, scale factor 1 | Approved visual references | All 77 Slint 1.18.0 baselines are approved; `docs/slint-1.18-visual-deltas.json` preserves the comparison with the former 1.17.1 references |
| Other macOS renderers and scale factors | Consumer-selected | Expected, visually unverified | Validate rendering, input, focus, fonts, and performance in the consumer |
| Linux | GitHub-hosted Ubuntu runner, Rust `1.92` | CI passed; native UI behavior unverified | Workspace compilation, Clippy, tests, and public validation passed in CI; renderer and input behavior require target-specific checks |
| Windows | GitHub-hosted Windows runner, Rust `1.92` | CI passed; native UI behavior unverified | Workspace compilation, Clippy, tests, and public validation passed in CI; renderer and input behavior require target-specific checks |
| Embedded targets | Any renderer | Unverified | Component geometry, assets, memory, input, and licensing require target-specific evaluation |
| Stable facade | `@atlas-ui/stable.slint` | SemVer-governed | Prefer for applications |
| Non-responsive preview facade | `@atlas-ui/preview-nonresponsive.slint` | Evolving | May change in minor Atlas releases; excludes responsive recipes |
| Preview facade | `@atlas-ui/preview.slint` | Evolving compatibility aggregate | Eagerly loads responsive recipes; compiles without experimental Slint features |
| Aggregate facade | `@atlas-ui/components.slint` | Compatibility entry point | Includes stable and all preview exports; compiles without experimental Slint features |

Atlas's MIT license does not replace Slint's licensing terms. Consumers remain
responsible for selecting a Slint license appropriate to their application.

## Meaning of platform validation

The public GitHub Actions workflow validated the complete Slint 1.18.0
workspace on Linux, Windows, and macOS with Rust 1.92.

Cross-platform CI does not establish pixel-identical rendering across operating
systems, graphics backends, font stacks, display scales, or input devices. The
approved deterministic visual baselines are tied to Slint 1.18.0 on the
macOS arm64 software-renderer profile. Consumers should capture and review
rendering on their own deployment configuration before making a production
support claim.

When updating Slint, follow the compatibility process in
`docs/SLINT_INTEGRATION.md` and regenerate all API and visual evidence before
changing this matrix.
