# Changelog

## Unreleased

- Adds stable `AtlasIconButton`, `AtlasWorkspaceTab`, and
  `AtlasWorkspaceTabList` contracts with controlled intentions, keyboard focus,
  deterministic automation IDs, and accessible close behavior.
- Adds preview `AtlasSpinner` and `AtlasRadialProgress` activity indicators,
  including reduced-motion behavior and explicit accessibility values.
- Adds preview `AtlasColumnGrid` and `AtlasGridItem` responsive 12-column
  composition contracts.
- Promotes `AtlasEdgeSurface`, `DividerEdge`, `AtlasMetricCard`, and `ValueTone`
  to the stable facade while retaining preview-facade compatibility exports.
- Adds `AtlasCategoryTokens` with six non-semantic light/dark identity colors
  and automated graphical-contrast evidence.
- Adds grid, terminal, gamepad, CPU, memory, play, stop, chevron-right, and
  layers icons to the checksum-controlled monochrome registry.
- Keeps `AtlasAutoGrid` preview because Slint 1.17.1 still requires the
  experimental `FlexboxLayout` capability for wrapping actionable cards.
- Adds `@atlas-ui/preview-nonresponsive.slint` and
  `atlas_ui::preview_nonresponsive_slint_path()` so evolving controls such as
  `AtlasProgressBar` and `AtlasTab` compile without experimental Slint
  features.
- Documents that `preview.slint` and `components.slint` remain compatibility
  aggregates that eagerly load responsive `FlexboxLayout` contracts.

## 0.2.1 — 2026-08-09

- Makes `@atlas-ui/stable.slint` compile without Slint experimental features.
- Adds a dedicated non-experimental Atlas Core facade for stable components.
- Keeps `FlexboxLayout` and responsive preview contracts isolated from the
  stable import graph.
- Adds a regression test for the stable-to-preview dependency boundary.
- Corrects the effective Rust MSRV to 1.92, as required by Slint 1.17.1.
- Includes the native Rust maintenance-tooling migration and refreshed agent
  documentation introduced after the 0.2.0 tag.

## 0.2.0 — 2026-08-09

- Establishes 48 stable-contract symbols in `stable.slint`.
- Isolates 110 evolving symbols in `preview.slint`.
- Retains `components.slint` as the aggregate compatibility facade.
- Automatically enforces the stable/preview partition and its SemVer rules.
- Adopts the MIT License for Atlas code.
- Tightens documentation rhythm and improves the hierarchy of link cards and callouts.

## 0.1.0-preview — 2026-08-08

- Introduced the tokens/core/icons/components/testing/gallery architecture.
- Added dark and light themes, density modes, and reduced motion.
- Added foundational components, overlays, navigation, and data presentation.
- Added 27 deterministic visual scenarios.
- Snapshotted 101 public symbols.
- Added panels, metric cards, selects, segmented controls, progress, ranges,
  pagination, and key/value lists.
- Added sparklines, icons, alerts, notices, workflow banners, steppers, drawers,
  and error pages.
- Added the first editorial set: headings, paragraphs, code, quotations, and dividers.
- Added 15 original SVG icons, an asset registry, and provenance validation.
- Bundled Inter Variable and JetBrains Mono Variable under OFL-1.1 with checksums.
- Added Stack, Cluster, Sidebar, Switcher, and AutoGrid with evidence from 360–1440 px.
- Kept the API in preview pending validation of the desktop matrix.
