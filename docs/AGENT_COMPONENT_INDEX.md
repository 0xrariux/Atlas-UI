# Component index for AI agents

This document is the entry point for agents building an interface with Atlas UI.
Use it to find the right component before writing Slint.

## Selection rules

1. Look for an existing Atlas component in the table below first.
2. Prefer `stable.slint` whenever it covers the requirement.
3. Import from `preview-nonresponsive.slint` when a non-responsive preview API
   is required.
4. Import from `preview.slint` only when responsive preview APIs are required.
5. Never copy an Atlas component implementation into an application.
6. Compose existing components before proposing a new primitive.
7. Keep data, navigation, networking, files, and persistence in Rust.
8. Treat Atlas callbacks as intentions controlled by the host.
9. Never invent a property; inspect the public `.slint` declaration.

## Facades

```slint
// Contracts guaranteed by SemVer
import { AtlasButton, AtlasTextField, AtlasTheme } from "@atlas-ui/stable.slint";

// Non-responsive contracts that may change between minor versions
import { AtlasDataTable, AtlasModal } from "@atlas-ui/preview-nonresponsive.slint";

// Responsive preview contracts; requires experimental Slint features
import { AtlasAutoGrid } from "@atlas-ui/preview.slint";
```

`preview.slint` and `components.slint` both load the experimental responsive
module as compatibility aggregates. New code should prefer `stable.slint` or
`preview-nonresponsive.slint` unless it actually uses responsive preview APIs.

## Quick lookup by need

| Need | Recommended component | Maturity |
|---|---|---|
| Primary or secondary action | `AtlasButton` | stable |
| Icon-only action | `AtlasIconButton` | stable |
| Single-line input | `AtlasTextField` | stable |
| Boolean choice | `AtlasCheckbox`, `AtlasSwitch` | stable |
| Short status | `AtlasBadge` | stable |
| Semantic icon | `AtlasIcon` | stable |
| Content surface or frame | `Surface`, `ComponentFrame`, `ContentFrame` | stable |
| Loading, empty, or error state | `AtlasSkeleton`, `AtlasEmptyState`, `AtlasErrorState` | stable |
| Compact indeterminate progress | `AtlasSpinner` | preview |
| Heading, paragraph, or code | stable editorial family | stable |
| Closable workspace tabs | `AtlasWorkspaceTab`, `AtlasWorkspaceTabList` | stable |
| Simple tabs | `AtlasTab`, `AtlasTabPanel` | preview |
| Page-edge chrome | `AtlasEdgeSurface` | stable |
| Metric presentation | `AtlasMetricCard` | stable |
| Tooltip | `AtlasTooltip` | preview |
| Contextual menu | `AtlasMenu` | preview |
| Blocking dialog | `AtlasModal` | preview |
| Side panel | `AtlasDrawer` | preview |
| Complex application table | `AtlasDataTable` | preview |
| Virtualized list | `AtlasDataList` | preview |
| Select, range, or pagination | application controls | preview |
| Message or workflow | feedback family | preview |
| Responsive layout | composition family | preview |
| Documentation or rendered Markdown | rich-content family | preview |
| Search or command palette | documentation tools | preview |
| Dashboard or settings | application templates | preview |

## 1. Foundations and surfaces

### Stable — `stable.slint`

- `Surface` — tokenized visual surface;
- `ComponentFrame` — shared component geometry;
- `ContentFrame` — responsive content width and margins;
- `FocusRing` — consistent focus indication.
- `AtlasEdgeSurface` — stable top, bottom, left, or right chrome with one
  content-facing divider.

### Non-responsive preview — `preview-nonresponsive.slint`

- `ActionArea` — headless activation and focus;
- `LayoutGridOverlay` — grid visualization;
- `OverlayFocusController` — focus containment and restoration;
- `RovingFocusController` — navigation within a group;
- `SelectionController` — controlled selection;
- `AtlasScrollViewport` — controlled documentation viewport.

### Responsive preview — `preview.slint`

- `AtlasIntrinsicFrame` — bounded intrinsic dimensions;
- `AtlasSplitView`, `AtlasSplitPane`, `AtlasResizeHandle` — resizable panes;
- `AtlasStickyRegion` — region attached to an edge.

## 2. Foundational controls

### Stable — `stable.slint`

- `AtlasButton` — explicit action, tones, loading, and icon;
- `AtlasIconButton` — accessible icon-only neutral, primary, or danger action;
- `AtlasTextField` — controlled input, hint, error, and required state;
- `AtlasCheckbox` — independent Boolean choice;
- `AtlasSwitch` — immediate Boolean setting;
- `AtlasBadge` — short, non-interactive status. Keep its intrinsic width and
  use `dot: true` when a compact operational signal is useful. Do not recreate
  status pills with `radius-round` or stretch a badge to fill a table cell;
  Atlas deliberately uses a compact soft rectangle for better scanning.

## 3. Icons

### Stable — `stable.slint`

- `AtlasIcon` — decorative or informative SVG icon with semantic size and tone.

Operational names include `grid`, `terminal`, `gamepad`, `cpu`, `memory`,
`play`, `stop`, `chevron-right`, and `layers`.

Use only a registered `IconName`. Do not use a font glyph, emoji, or local SVG
to replace an existing Atlas icon.

## 4. Data states

### Stable — `stable.slint`

- `AtlasSkeleton` — deterministic loading with reduced-motion support;
- `AtlasEmptyState` — collection or search without results;
- `AtlasErrorState` — local error with controlled recovery.

## 5. Typography and editorial content

### Stable — `stable.slint`

- `AtlasHeading` — semantic h1 through h6 headings;
- `AtlasParagraph` — paragraph with bounded reading measure;
- `AtlasStyledText` — semantic emphasis, monospace, and underlining;
- `AtlasInlineCode` — inline code fragment;
- `AtlasCodeBlock` — code block and copy intention;
- `AtlasBlockQuote` — block quotation;
- `AtlasDivider` — editorial separator.

### Stable — `stable.slint`

- `AtlasWorkspaceTab`, `AtlasWorkspaceTabList` — closable workspace tabs with
  controlled selection, roving focus, Delete-to-close, bounded labels, and
  overflow intention.

### Non-responsive preview — `preview-nonresponsive.slint`

- `AtlasSelectableText` — Unicode selection and controlled copying.

## 6. Navigation and overlays

### Non-responsive preview — `preview-nonresponsive.slint`

- `AtlasTab`, `AtlasTabPanel` — tabs with roving focus;
- `AtlasTooltip` — hover/focus help and truncated values;
- `AtlasMenu` — model-driven contextual menu;
- `AtlasModal` — controlled modal dialog;
- `AtlasDrawer` — side panel with a focus boundary.

## 7. Data and lists

### Non-responsive preview — `preview-nonresponsive.slint`

- `AtlasDataTable` — virtualized responsive table with shared min/max/grow
  tracks, rich cells, sorting, filters, multi-selection, resizable columns,
  inline editing, contextual menus, expansion, and compact semantic cards;
- `AtlasDataList` — virtualized list and controlled selection;
- `AtlasKeyValueList` — key/value property presentation.

Operations are emitted as intentions. Components never silently sort, filter,
or persist data.

Prefer `AtlasDataTable` over manually assembled row rectangles. When a custom
table is necessary, the enclosing surface owns the outer border and row
dividers are rendered only between rows. The final row must not add a bottom
divider, otherwise it visually doubles the surface border.

## 8. Application controls

### Non-responsive preview — `preview-nonresponsive.slint`

- `AtlasPanel` — composition panel;
- `AtlasSelectField` — bounded selection;
- `AtlasSegmentedControl` — compact exclusive choice;
- `AtlasSpinner` — compact indeterminate progress with an explicit accessible
  label and value text;
- `AtlasProgressBar` — determinate progress, or an indeterminate activity rail
  when `indeterminate` is true. Set `show-labels` to false only when visible
  context supplies the label; the required accessible `label` and exposed
  `value-text` remain available;
- `AtlasRadialProgress` — compact determinate progress displayed as a ring;
- `AtlasRangeControl` — value within a range;
- `AtlasPagination` — page navigation;
- `AtlasSparkline` — compact trend.

### Stable — `stable.slint`

- `AtlasMetricCard` — accessible label, value, metadata, and semantic value
  tone in compact or normal geometry.

## 9. Feedback, workflow, and recovery

### Non-responsive preview — `preview-nonresponsive.slint`

- `AtlasInlineAlert` — contextual message;
- `AtlasNoticeStack` — notification stack;
- `AtlasWorkflowBanner` — global workflow state;
- `AtlasStepper` — step-based progress;
- `AtlasErrorPage` — page-level recovery.

## 10. Responsive composition

### Responsive preview — `preview.slint`

- `AtlasStack` — stack with tokenized gap and inset;
- `AtlasCluster` — wrapping horizontal group;
- `AtlasSidebar` — main content with a side panel;
- `AtlasSwitcher` — width-dependent horizontal/vertical switch;
- `AtlasAutoGrid` — grid with an automatic column count.
- `AtlasColumnGrid`, `AtlasGridItem` — explicit responsive 12-column grid and
  compact, normal, and wide item spans.

Bind every item's `reference-width` to its containing grid. If `columns` or
`gap` differs from the defaults, bind those values to the item as well so both
components use the same geometry.

These recipes currently depend on experimental Slint layout capabilities and
must not be presented as stable.

The stable `AtlasEdgeSurface` is independent of these experimental recipes.
Use `DividerEdge.bottom` for top bars and navigation bars,
`DividerEdge.right` for left sidebars, and `DividerEdge.left` for right
sidebars. Page chrome must not use a full rectangular border.

## 11. Rich content

### Responsive preview — `preview.slint`

- `AtlasAdmonition`, `AtlasCallout` — semantic information and action;
- `AtlasLink`, `AtlasLinkCard` — controlled navigation and
  default/hover/selected hierarchy. `AtlasLink` reserves
  `AtlasControlTokens.gap` before its directional icon;
- `AtlasRichText`, `AtlasDocumentList` — rich presentation;
- `AtlasDocumentTable` — responsive editorial table;
- `AtlasFigure` — media with loading, ready, empty, and error states;
- `AtlasTerminalBlock` — command and copy intention;
- `AtlasContentTabs` — tabbed documentation content;
- `AtlasCaption`, `AtlasCrossReference` — captions and references;
- `AtlasFootnoteReference`, `AtlasFootnoteList` — notes and focus return.

## 12. Documentation shell and tools

### Non-responsive preview — `preview-nonresponsive.slint`

- `AtlasDocumentationShell` — header, sidebar, content, table of contents, and footer;
- `AtlasAnchorAction` — deep-link copy intention;
- `AtlasDocumentSearch` — search results prepared by Rust;
- `AtlasCommandPalette` — commands and navigation;
- `AtlasThemeControl` — controlled system/light/dark preference.

## 13. Templates

### Responsive preview — `preview.slint`

- `AtlasRoadmapContentTemplate` — documentation roadmap;
- `AtlasSettingsTemplate` — settings navigation and form;
- `AtlasDashboardTemplate` — metrics, activity, and a domain slot.

An Atlas template provides a generic composition. The agent injects the domain
model and actions from the application without modifying the template.

## When no component fits

1. Check whether listed components can be composed to meet the need.
2. Inspect public types in `stable.slint`, `preview-nonresponsive.slint`, and
   `preview.slint`.
3. Decide whether the need is generic or application-specific.
4. Keep domain composition in the application.
5. Propose an Atlas component only when multiple screens can reuse it.
6. For a new Atlas component, provide tokens, states, keyboard behavior, a
   scenario, fixture, documentation, tests, and explicit maturity.

## Additional references

- [Visual workflow for coding agents](AGENT_VISUAL_WORKFLOW.md)
- [Detailed catalog](COMPONENTS.md)
- [Architecture and layers](ARCHITECTURE.md)
- [Slint integration](SLINT_INTEGRATION.md)
- [Engineering and quality](ENGINEERING.md)
- [Talos consumer gap audit](TALOS_CONSUMER_GAP_AUDIT.md)
- [Getting started](../GETTING_STARTED.md)
