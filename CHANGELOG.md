# Changelog

## 0.1.1 — 2026-09-04

- Pins `tinyvec` to `1.12.0` in the published facade so fresh Slint `1.17.1`
  consumer resolutions avoid the upstream `tinyvec 1.13.0` alloc-only
  compilation regression. Atlas UI and Slint API contracts are unchanged.

## 0.1.0 — 2026-09-03

- Introduces the Atlas UI workspace architecture: tokens, core primitives,
  icons, components, documents, testing helpers, the Rust facade, native
  tooling, the gallery, and a compiled consumer example.
- Exposes 58 stable symbols governed by SemVer and 122 preview symbols that may
  evolve between minor releases.
- Adds preview `AtlasScrollbar`, a controlled standalone vertical scrollbar
  with a 16-pixel interaction corridor, proportional thumb, six-pixel rail,
  two-pixel corners, accessible value semantics, and bounded appearance inputs;
  `AtlasScrollViewport` now composes the same primitive.
- Adds stable `AtlasStatusIndicator` for accessible standalone semantic state
  signals.
- Adds preview `AtlasSettingsRow`, `AtlasChartFrame`, `AtlasModalFrame`,
  `AtlasMetric`, `AtlasCopyableValue`, and `AtlasDrawerFrame` composition
  contracts.
- Extends preview `AtlasProgressBar` with configurable track height, radius,
  track color, and indicator color while preserving its existing defaults.
- Extends stable `AtlasSwitch` with additive label, track, border, background,
  and thumb anatomy properties while preserving its existing behavior and
  defaults.
- Refactors preview `AtlasModal` and `AtlasDrawer` to inherit their slotted
  frame components without removing their standard title, body, or action
  APIs.
- Adds `activity`, `analytics`, `bell`, `clock`, `cloud`, `copy`, `database`,
  `download`, `filter`, `globe`, `settings`, `trash`, `users`, and `webhook` to
  the stable `IconName` registry.
- Removes, renames, and deprecates no public API. The template campaign adds no
  typography roles, enum types, structs, globals, or Rust runtime helpers; the
  scrollbar follow-up extends the existing stable viewport-token global.
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
- Makes `@atlas-ui/stable.slint` compile without Slint experimental features.
- Adds a dedicated non-experimental Atlas Core facade for stable components.
- Keeps `FlexboxLayout` and responsive preview contracts isolated from the
  stable import graph.
- Adds a regression test for the stable-to-preview dependency boundary.
- Sets the effective Rust MSRV to 1.92, as required by Slint 1.17.1.
- Includes native Rust maintenance tooling and agent-oriented documentation.
- Retains `components.slint` as the aggregate compatibility facade.
- Automatically enforces the stable/preview partition and its SemVer rules.
- Adopts the MIT License for Atlas code.
- Tightens documentation rhythm and improves the hierarchy of link cards and callouts.
- Adds dark and light themes, density modes, and reduced motion.
- Adds foundational components, overlays, navigation, and data presentation.
- Provides 77 deterministic visual scenarios across 32 gallery pages.
- Publishes seven `0.1.0` library packages on crates.io, with `atlas-ui` as the
  canonical facade and the GitHub tag as the matching source snapshot.
- Adds an external consumer gate for the four `template-atlas` applications,
  covering 97 rendered states during Atlas upgrades, and records the
  no-overflow scrollbar state found through the Talos adoption audit.
- Adds panels, metric cards, selects, segmented controls, progress, ranges,
  pagination, and key/value lists.
- Adds sparklines, icons, alerts, notices, workflow banners, steppers, drawers,
  and error pages.
- Adds an editorial set with headings, paragraphs, code, quotations, and
  dividers.
- Ships 38 original SVG icons with an asset registry and provenance validation.
- Bundles Inter Variable and JetBrains Mono Variable under OFL-1.1 with
  checksums.
- Adds Stack, Cluster, Sidebar, Switcher, and AutoGrid with evidence from
  360–1440 px.
