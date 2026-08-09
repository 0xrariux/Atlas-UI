# Compatibility matrix

This document separates pinned dependencies, verified configurations, expected
compatibility, and unsupported claims. “Expected” means the architecture is
intended to support the configuration but Atlas has not collected complete
release evidence for it.

| Dimension | Version or target | Status | Evidence or constraint |
|---|---|---|---|
| Atlas | `0.2.0` | Experimental | Stable and preview facades are separated; the release remains blocked until visual baselines are approved |
| Rust | `1.88` | Required | Workspace MSRV |
| Rust edition | `2024` | Required | Workspace package edition |
| Slint | `1.17.1` | Pinned | Workspace dependencies use exact versions for `slint` and `slint-build` |
| macOS arm64 | Software renderer, scale factor 1 | Verified | Current deterministic capture and performance profile |
| Other macOS configurations | Other renderer or scale factor | Expected, unverified | Validate rendering, input, focus, fonts, and performance in the consumer |
| Linux | Any backend | Expected, unverified | No release claim until platform evidence is recorded |
| Windows | Any backend | Expected, unverified | No release claim until platform evidence is recorded |
| Embedded targets | Any renderer | Unverified | Component geometry, assets, memory, input, and licensing require target-specific evaluation |
| Stable facade | `@atlas-ui/stable.slint` | SemVer-governed | Prefer for applications |
| Preview facade | `@atlas-ui/preview.slint` | Evolving | May change in minor Atlas releases |
| Aggregate facade | `@atlas-ui/components.slint` | Compatibility entry point | Includes stable and preview exports; maturity is less explicit at each import site |

Atlas's MIT license does not replace Slint's licensing terms. Consumers remain
responsible for selecting a Slint license appropriate to their application.

When updating Slint, follow the compatibility process in
`docs/SLINT_INTEGRATION.md` and regenerate all API and visual evidence before
changing this matrix.
