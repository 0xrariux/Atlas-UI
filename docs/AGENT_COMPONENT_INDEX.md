# Component index for AI agents

This document is the entry point for agents building an interface with Atlas UI.
Use it to find the right component before writing Slint.

## Selection rules

1. Look for an existing Atlas component in the table below first.
2. Prefer `stable.slint` whenever it covers the requirement.
3. Import from `preview.slint` only when the stable API is insufficient.
4. Never copy an Atlas component implementation into an application.
5. Compose existing components before proposing a new primitive.
6. Keep data, navigation, networking, files, and persistence in Rust.
7. Treat Atlas callbacks as intentions controlled by the host.
8. Never invent a property; inspect the public `.slint` declaration.

## Facades

```slint
// Contracts guaranteed by SemVer
import { AtlasButton, AtlasTextField, AtlasTheme } from "@atlas-ui/stable.slint";

// Contracts that may change between minor versions
import { AtlasDataTable, AtlasModal } from "@atlas-ui/preview.slint";
```

`components.slint` aggregates both facades for compatibility, but new code
should prefer an explicit stable or preview import.

## Quick lookup by need

| Need | Recommended component | Maturity |
|---|---|---|
| Primary or secondary action | `AtlasButton` | stable |
| Single-line input | `AtlasTextField` | stable |
| Boolean choice | `AtlasCheckbox`, `AtlasSwitch` | stable |
| Short status | `AtlasBadge` | stable |
| Semantic icon | `AtlasIcon` | stable |
| Content surface or frame | `Surface`, `ComponentFrame`, `ContentFrame` | stable |
| Loading, empty, or error state | `AtlasSkeleton`, `AtlasEmptyState`, `AtlasErrorState` | stable |
| Heading, paragraph, or code | stable editorial family | stable |
| Tabs | `AtlasTab`, `AtlasTabPanel` | preview |
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

### Preview — `preview.slint`

- `ActionArea` — headless activation and focus;
- `LayoutGridOverlay` — grid visualization;
- `OverlayFocusController` — focus containment and restoration;
- `RovingFocusController` — navigation within a group;
- `SelectionController` — controlled selection;
- `AtlasScrollViewport` — controlled documentation viewport;
- `AtlasIntrinsicFrame` — bounded intrinsic dimensions;
- `AtlasSplitView`, `AtlasSplitPane`, `AtlasResizeHandle` — resizable panes;
- `AtlasStickyRegion` — region attached to an edge.

## 2. Foundational controls

### Stable — `stable.slint`

- `AtlasButton` — explicit action, tones, loading, and icon;
- `AtlasTextField` — controlled input, hint, error, and required state;
- `AtlasCheckbox` — independent Boolean choice;
- `AtlasSwitch` — immediate Boolean setting;
- `AtlasBadge` — short, non-interactive status.

## 3. Icons

### Stable — `stable.slint`

- `AtlasIcon` — decorative or informative SVG icon with semantic size and tone.

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

### Preview — `preview.slint`

- `AtlasSelectableText` — Unicode selection and controlled copying.

## 6. Navigation and overlays

### Preview — `preview.slint`

- `AtlasTab`, `AtlasTabPanel` — tabs with roving focus;
- `AtlasTooltip` — hover/focus help and truncated values;
- `AtlasMenu` — model-driven contextual menu;
- `AtlasModal` — controlled modal dialog;
- `AtlasDrawer` — side panel with a focus boundary.

## 7. Data and lists

### Preview — `preview.slint`

- `AtlasDataTable` — virtualized table, sorting, filters, multi-selection,
  resizable columns, inline editing, contextual menus, and expansion;
- `AtlasDataList` — virtualized list and controlled selection;
- `AtlasKeyValueList` — key/value property presentation.

Operations are emitted as intentions. Components never silently sort, filter,
or persist data.

## 8. Application controls

### Preview — `preview.slint`

- `AtlasPanel` — composition panel;
- `AtlasMetricCard` — metric and trend;
- `AtlasSelectField` — bounded selection;
- `AtlasSegmentedControl` — compact exclusive choice;
- `AtlasProgressBar` — determinate or indeterminate progress;
- `AtlasRangeControl` — value within a range;
- `AtlasPagination` — page navigation;
- `AtlasSparkline` — compact trend.

## 9. Feedback, workflow, and recovery

### Preview — `preview.slint`

- `AtlasInlineAlert` — contextual message;
- `AtlasNoticeStack` — notification stack;
- `AtlasWorkflowBanner` — global workflow state;
- `AtlasStepper` — step-based progress;
- `AtlasErrorPage` — page-level recovery.

## 10. Responsive composition

### Preview — `preview.slint`

- `AtlasStack` — stack with tokenized gap and inset;
- `AtlasCluster` — wrapping horizontal group;
- `AtlasSidebar` — main content with a side panel;
- `AtlasSwitcher` — width-dependent horizontal/vertical switch;
- `AtlasAutoGrid` — grid with an automatic column count.

These recipes currently depend on experimental Slint layout capabilities and
must not be presented as stable.

## 11. Rich content

### Preview — `preview.slint`

- `AtlasAdmonition`, `AtlasCallout` — semantic information and action;
- `AtlasLink`, `AtlasLinkCard` — controlled navigation and
  default/hover/selected hierarchy;
- `AtlasRichText`, `AtlasDocumentList` — rich presentation;
- `AtlasDocumentTable` — responsive editorial table;
- `AtlasFigure` — media with loading, ready, empty, and error states;
- `AtlasTerminalBlock` — command and copy intention;
- `AtlasContentTabs` — tabbed documentation content;
- `AtlasCaption`, `AtlasCrossReference` — captions and references;
- `AtlasFootnoteReference`, `AtlasFootnoteList` — notes and focus return.

## 12. Documentation shell and tools

### Preview — `preview.slint`

- `AtlasDocumentationShell` — header, sidebar, content, table of contents, and footer;
- `AtlasAnchorAction` — deep-link copy intention;
- `AtlasDocumentSearch` — search results prepared by Rust;
- `AtlasCommandPalette` — commands and navigation;
- `AtlasThemeControl` — controlled system/light/dark preference.

## 13. Templates

### Preview — `preview.slint`

- `AtlasRoadmapContentTemplate` — documentation roadmap;
- `AtlasSettingsTemplate` — settings navigation and form;
- `AtlasDashboardTemplate` — metrics, activity, and a domain slot.

An Atlas template provides a generic composition. The agent injects the domain
model and actions from the application without modifying the template.

## When no component fits

1. Check whether listed components can be composed to meet the need.
2. Inspect public types in `stable.slint` and `preview.slint`.
3. Decide whether the need is generic or application-specific.
4. Keep domain composition in the application.
5. Propose an Atlas component only when multiple screens can reuse it.
6. For a new Atlas component, provide tokens, states, keyboard behavior, a
   scenario, fixture, documentation, tests, and explicit maturity.

## Additional references

- [Detailed catalog](COMPONENTS.md)
- [Architecture and layers](ARCHITECTURE.md)
- [Slint integration](SLINT_INTEGRATION.md)
- [Engineering and quality](ENGINEERING.md)
- [Getting started](../GETTING_STARTED.md)
