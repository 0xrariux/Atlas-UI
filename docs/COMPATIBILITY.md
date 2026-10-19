# Compatibility matrix

This document separates pinned dependencies, verified configurations, expected
compatibility, and unsupported claims. “Expected” means the architecture is
intended to support the configuration but Atlas has not collected complete
release evidence for it.

## Slint 1.18.1 source candidate

The current source branch pins both `slint` and `slint-build` to `=1.18.1`.
It retains the Atlas 0.2.2 workspace version while evaluating this dependency
update; no new Atlas package or tag has been published.

Local validation passed on 2026-10-19 with Rust 1.97.1 on macOS 26.5.1 arm64.
The quality gate passes all 121 tests, formatting, compilation, Clippy and
contract/package checks. All 77 software captures pass against the approved
1.18.0 references: 76 PNGs have identical hashes; the remaining capture differs
at seven pixels above the comparison's channel tolerance, confined to its
animated spinner. All four render budgets pass, and Command, Forge, Fleet and
Ledger compile with matching pins in temporary source copies.

The complete quality gate also passes all 121 tests with Rust 1.92.0 in a
separate clean target directory on macOS arm64. CI uses `--locked` so every
platform validates the reviewed dependency resolution.

The baseline images and their original approval metadata are unchanged.
Candidate hosted CI remains pending. These results do
not establish native accessibility, IME, touch, touchpad or another platform's
behavior. The 97 external native captures were not rerun. See the
[patch audit](SLINT_1_18_AUDIT.md#slint-1181-candidate-review-on-2026-10-19).
The [validation record](slint-1.18.1-validation.json) contains the measured
results and their limits.

Local path consumers must pin both Slint dependencies to 1.18.1. Published
Atlas 0.2.2 consumers must retain 1.18.0. The existing companion templates
require the same runtime/build pin update to consume this candidate.

## Published Atlas 0.2.2 evidence

The following matrix records the published release and its 1.18.0 source tag.
Its hosted CI and visual approvals do not establish candidate validation.

| Dimension | Version or target | Status | Evidence or constraint |
|---|---|---|---|
| Atlas tagged source and published crate | `v0.2.2` | CI and visual references verified | Stable imports remain separate from responsive preview; 77 macOS arm64 software references are approved under Slint 1.18.0 |
| Rust | `1.92` | Required | Effective MSRV imposed by `slint-build 1.18.0` |
| Rust edition | `2024` | Required | Workspace package edition |
| Slint | `1.18.0` | Pinned in the published release | Tagged workspace dependencies use exact versions for `slint` and `slint-build`; `const-field-offset` resolves to 0.2.1 |
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
