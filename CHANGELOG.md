# Changelog

## 0.2.2 — 2026-09-17

- Publishes the Slint 1.18 migration recorded under the `v0.2.0` source tag.
  The previously published `0.2.0` and `0.2.1` crate versions were yanked and
  cannot be reused on crates.io.
- Removes the `atlas-ui-testing` build dependency on the facade to keep all
  seven registry packages publishable in dependency order. Runtime layout
  fixtures still compile against the same public Slint library paths.

## 0.2.0 — 2026-09-17

- Pins Slint and `slint-build` to 1.18.0 and resolves `const-field-offset`
  0.2.1. The published Atlas 0.1.1 package remains on Slint 1.17.1.
- Migrates Atlas responsive recipes and in-repo application templates to
  stable upstream `FlexboxLayout`; all source facades now compile without
  experimental Slint compiler features. Responsive Atlas contracts remain
  preview.
- Adapts native scroll internals to Slint's `content-*` properties while
  preserving the public Atlas viewport contract.
- Forwards `line-height-factor` and `max-lines` in `AtlasStyledText`,
  `line-height-factor` in `AtlasSelectableText`, and `input-method-hints` in
  `AtlasTextField`.
- Corrects rich-text fragment spacing and repeated document-list row sizing
  under Slint 1.18 measurement; adjusts the gallery list-grid layout for
  larger typography.
- Regenerates the agent API manifest and records the full 1.18 changelog
  impact audit. All 77 macOS software-renderer references were reviewed and
  updated for Slint 1.18.0.
- Adds a rendered Slint geometry fixture for responsive pane transitions,
  nested grid columns, and repeated document rows under text/model mutations.
- Adds runtime keyboard and pointer conformance checks for `ActionArea` and
  `AtlasModal`. Explicit keyboard focus now keeps its visible indicator,
  disabling an action clears its focus and pressed state without restoring
  stale focus on reenable, and activation requires release of the same key
  that was pressed.
- Restores keyboard focus to zero-size overlay controllers under Slint 1.18,
  adds menu arrow/Home/End and Enter/Space navigation, prevents activation of
  disabled menu entries, and verifies menu/drawer dismissal and focus return.
- Adds a runtime workspace-tab keyboard fixture covering roving arrows,
  Home/End, activation, close settlement, and disabled-tab recovery. The stable
  tab list now exposes `focus-index(index)` so hosts can focus an eligible tab
  after their model changes.
- Adds a nested modal/menu runtime fixture that verifies Escape unwinds one
  overlay at a time and restores focus to the correct invoking control. The
  preview modal frame and modal now expose `focus-on-open` so a host can give
  a menu priority when both layers open in one update.
- Adds a host-side logical-pixel overlay placement API with flip, shift,
  viewport fallback, and invalid-anchor handling. `AtlasMenu` now emits
  dismissal when its anchor becomes ineligible.
- Adds preview `AtlasPopover` with a slotted panel, host-provided placement,
  outside-click and Escape dismissal, anchor-loss handling, and focus return.
- Adds preview `AtlasCombobox`, composing the select field and menu with
  keyboard selection, expanded accessibility state, and disabled-anchor
  dismissal. A three-layer modal/popover/menu fixture verifies focus unwinding.
- Makes `AtlasTooltip` hide when its anchor becomes ineligible, including while
  `force-open` is set.
- Adds preview `AtlasRadioGroup` with controlled choice IDs, accessible radio
  states, token styling, and host-selected roving focus around disabled items.
- Verifies `AtlasMenu` inside `AtlasPopover` for outside-click and Escape
  dismissal with one focus restoration.
- Adds preview `AtlasAutocomplete`, composing the stable text field and preview
  menu. Typing retains editor focus; arrow keys navigate suggestions, Enter
  selects, and Escape or Tab closes the list. The host supplies query results
  and handles selection. `AtlasMenu.focus-on-open` permits this composition.
- Lets the host place combobox and autocomplete menus with component-local
  `menu-x`/`menu-y` coordinates and mark their anchors ineligible. The menu
  clamps its active index when items change; the combobox closes when its
  options become empty. Both expose a controlled active index for model updates.
- Adds a public Rust track allocator for deterministic min/preferred/max/grow
  widths, overflow, and unused-space reporting. `AtlasDataTable` can consume
  one host-provided width model for matching header and row tracks. Both native
  gallery tables now calculate that model from their live width and columns.
- Adds preview `AtlasContainer` with local content metrics and size class.
  A Slint 1.18 runtime fixture confirms that `layout-order` changes visual
  positions while keyboard Tab keeps declaration order. Another fixture covers
  wrapped repeated grid rows and conditional spans through model mutations.
- Extends the host allocator to gallery document tables and key/value lists,
  with column resize overrides keyed by stable IDs. `AtlasAutoGrid.basis-width`
  lets an intrinsically sized pane use a parent-owned width without a layout
  cycle.
- Adds explicit `AtlasEnvironment` forwarding through nested containers;
  extends `AtlasScrollViewport` to controlled two-axis offsets, reveal requests,
  horizontal scrollbar presentation, and logical-direction keyboard paging.
- Bounds long menus and reveals their active item while scrolling. Adds Rust
  collection identity, paging, grouping, cell projection, and incremental tree
  projection adapters plus preview `AtlasTreeView` with ID-based intentions.

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
