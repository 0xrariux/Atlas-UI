# Component catalog

For a quick task-oriented selection, especially from an AI agent, also see the
[component index](AGENT_COMPONENT_INDEX.md).

Atlas `0.2.1` exposes 89 components. Their public classification is described
below and enforced by the `stable.slint` and `preview.slint` facades.
Non-responsive preview contracts are also available from
`preview-nonresponsive.slint` without experimental Slint features. The full
`preview.slint` and `components.slint` aggregates both load the responsive
module and therefore require `SLINT_ENABLE_EXPERIMENTAL_FEATURES=1`.

## Stable foundations

The following contracts are available from `stable.slint`:

- surfaces: `Surface`, `ComponentFrame`, `ContentFrame`, `FocusRing`;
- controls: `AtlasButton`, `AtlasTextField`, `AtlasCheckbox`, `AtlasSwitch`,
  `AtlasBadge`, `AtlasIconButton`;
- icons: `AtlasIcon`;
- workspace navigation: `AtlasWorkspaceTab`, `AtlasWorkspaceTabList`;
- application chrome and metrics: `AtlasEdgeSurface`, `AtlasMetricCard`;
- states: `AtlasSkeleton`, `AtlasEmptyState`, `AtlasErrorState`;
- editorial: `AtlasHeading`, `AtlasParagraph`, `AtlasStyledText`,
  `AtlasInlineCode`, `AtlasCodeBlock`, `AtlasBlockQuote`, `AtlasDivider`;
- globals and types associated with themes, tokens, density, motion, and typography.

These 57 symbols—components, types, and globals combined—follow SemVer.

`AtlasIconButton` requires the built-in Slint `accessible-label` property at
every use site. Its visible icon is decorative because the button owns the
name. Neutral, primary, danger, loading, selected/toggle, disabled, hover,
pressed, and focus-visible states share the button token contract. The minimum
pointer target remains `AtlasDensity.pointer-target-min` even at compact
density. `automation-id` defaults deterministically from the accessible label
and may be overridden when labels are localized.

`AtlasWorkspaceTab` is distinct from the compatible preview `AtlasTab`. It
keeps the tab target and close target as accessible siblings under a grouping
node, avoiding button-inside-button semantics. Left/Right, Home, and End emit
navigation through `AtlasWorkspaceTabList`; Enter and Space activate. Delete
requests close when `closable` is true. Backspace is deliberately unclaimed.
After the host accepts a close and updates its model, call
`settle-after-close`; it focuses the new selected tab or the nearest survivor.
Labels have a bounded width and elide; `overflowed` and `overflow-requested`
let the host provide a menu without hidden model mutation.

`AtlasMetricCard` exposes its label as the accessible name, value as the
accessible value, and metadata/help as the description. Compact and normal
geometry use tokenized padding and bounded elision for localized content.

`AtlasEdgeSurface` has no experimental dependency. Use a bottom divider for a
top bar, a top divider for bottom chrome, a right divider for a left sidebar,
and a left divider for a right sidebar. It does not draw a full outline.

`AtlasBadge` is intrinsically sized and uses a compact soft rectangle. Keep the
intrinsic width, select a semantic `BadgeTone`, and enable `dot` only when the
extra status signal helps scanning. Do not rebuild badges from rectangles with
`AtlasShape.radius-round`, and do not stretch a short status across its cell.

Atlas normalizes the embedded body sans and monospace fonts to a `1.6em` line
height. Wrapped body text therefore keeps readable leading consistently in
`Text`, `TextInput`, `StyledText`, and the shared editorial components. Display
headings retain Inter's original metrics so their established vertical rhythm
does not change.

## Preview core and composition

Use `preview-nonresponsive.slint` for the interaction, grid-overlay, and scroll
contracts below. Intrinsic, split, sticky, and responsive recipes are available
only through the full `preview.slint` aggregate.

- interaction: `ActionArea`, `OverlayFocusController`,
  `RovingFocusController`, `SelectionController`;
- geometry: `LayoutGridOverlay`, `AtlasIntrinsicFrame`, `AtlasStickyRegion`;
- scrolling and panes: `AtlasScrollViewport`, `AtlasSplitView`,
  `AtlasSplitPane`, `AtlasResizeHandle`;
- responsive recipes: `AtlasStack`, `AtlasCluster`, `AtlasSidebar`,
  `AtlasSwitcher`, `AtlasAutoGrid`, `AtlasColumnGrid`,
  `AtlasGridItem`.

`AtlasAutoGrid` remains preview: its wrapping, basis, growth, and shrinkage
depend on experimental `FlexboxLayout` in Slint 1.17.1. Stable consumers should
choose a deterministic column count in application state and compose explicit
`HorizontalLayout`/`VerticalLayout` groups. Promotion requires a stable Slint
flex contract or a non-experimental Atlas implementation, breakpoint and nested
overflow evidence, three-platform verification, and a clean stable consumer.

## Preview navigation and overlays

`AtlasTab`, `AtlasTabPanel`, `AtlasTooltip`, `AtlasMenu`, `AtlasModal`, and
`AtlasDrawer` cover selection, contextual help, menus, dialogs, and panes with
controlled state and explicit focus restoration.

## Preview data and application controls

- data: `AtlasDataTable`, `AtlasDataList`, `AtlasKeyValueList`;
- domain-neutral surfaces: `AtlasPanel`, `AtlasSparkline`;
- input: `AtlasSelectField`, `AtlasSegmentedControl`, `AtlasRangeControl`;
- progress and navigation: `AtlasProgressBar`, `AtlasRadialProgress`,
  `AtlasSpinner`, `AtlasPagination`, `AtlasStepper`;
- feedback: `AtlasInlineAlert`, `AtlasNoticeStack`, `AtlasWorkflowBanner`,
  `AtlasErrorPage`.

The table supports virtualization, a sticky header, resizable columns,
multi-selection, sorting, filtering, inline editing, contextual menus,
expandable details, rich compound cells, and a compact responsive card mode.
`DataColumn.width` is the preferred width; the shared track allocator clamps it
to `min-width`/`max-width`, distributes remaining space by `grow`, and gives
the deterministic rounding remainder to the final track. Header and row tracks
share the same padding and gap inputs. If the sum of minimum widths is wider
than the viewport, desktop mode preserves those minimums and enables horizontal
scrolling. Domain operations remain in the Rust host.

`AtlasSpinner` is the compact indeterminate primitive for controls and small
status surfaces. Its `label` and `value-text` form the progress accessibility
contract; `indicator-color` defaults to the semantic info color and `size`
tracks the current typography scale. `AtlasProgressBar` remains the horizontal
primitive. Set `indeterminate: true` for unknown-duration work and
`show-labels: false` for a rail embedded at a surface edge. The rail still
requires `label` and exposes `value-text`, even when its visual labels are
hidden. Full motion uses the shared continuous-cycle tokens. Reduced motion
uses a centered static segment or incomplete spinner arc, preserving an
unambiguous in-progress state without movement.

`DataCell.accessible-text` overrides the generated compound-cell label. Keep
`DataCell.text` populated as the plain-text fallback and ordered semantic
equivalent for tag cells; Atlas exposes that value once and keeps the visual
badges decorative, avoiding duplicate spoken content in table and card modes.

## Categorical identity

`AtlasCategoryTokens.category-1` through `category-6` are stable ordinal colors
for persistent identities and data series. They are separate from success,
warning, danger, and info. Applications may use them on card accents, icons,
or series, but must pair color with a label, icon, pattern, or shape.

```slint
Surface { border-color: AtlasCategoryTokens.category-3; }
AtlasIcon { icon-color: AtlasCategoryTokens.category-3; name: IconName.layers; }
```

Decorative icons keep `decorative: true`. Informative standalone icons set
`decorative: false` and a specific `accessible-name`, such as
`"CPU utilization"`; generic names such as `"icon"` are invalid.

## Preview rich content and documentation

- blocks: `AtlasAdmonition`, `AtlasCallout`, `AtlasFigure`,
  `AtlasDocumentTable`, `AtlasTerminalBlock`, `AtlasContentTabs`;
- links and text: `AtlasLink`, `AtlasLinkCard`, `AtlasRichText`,
  `AtlasSelectableText`, `AtlasDocumentList`;
- references: `AtlasCaption`, `AtlasCrossReference`,
  `AtlasFootnoteReference`, `AtlasFootnoteList`;
- tools: `AtlasAnchorAction`, `AtlasDocumentSearch`, `AtlasCommandPalette`;
- shell: `AtlasDocumentationShell`, `AtlasThemeControl`.

`AtlasLink` separates its label and directional icon with the shared
control-gap token.

## Preview templates

`AtlasRoadmapContentTemplate`, `AtlasSettingsTemplate`, and
`AtlasDashboardTemplate` demonstrate how to compose foundations without adding
a domain dependency to the library.

## Maturity

`stable` means that names, properties, callbacks, types, and semantics follow
SemVer. `preview` means that a component is testable and documented but may
still change in a minor release. Promotion requires an API audit,
representative scenarios, and a recorded decision.
