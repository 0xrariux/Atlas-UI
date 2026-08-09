# Component catalog

For a quick task-oriented selection, especially from an AI agent, also see the
[component index](AGENT_COMPONENT_INDEX.md).

Atlas `0.2.1` exposes 81 components. Their public classification is described
below and enforced by the `stable.slint` and `preview.slint` facades.

## Stable foundations

The following contracts are available from `stable.slint`:

- surfaces: `Surface`, `ComponentFrame`, `ContentFrame`, `FocusRing`;
- controls: `AtlasButton`, `AtlasTextField`, `AtlasCheckbox`, `AtlasSwitch`,
  `AtlasBadge`;
- icons: `AtlasIcon`;
- states: `AtlasSkeleton`, `AtlasEmptyState`, `AtlasErrorState`;
- editorial: `AtlasHeading`, `AtlasParagraph`, `AtlasStyledText`,
  `AtlasInlineCode`, `AtlasCodeBlock`, `AtlasBlockQuote`, `AtlasDivider`;
- globals and types associated with themes, tokens, density, motion, and typography.

These 48 symbols—components, types, and globals combined—follow SemVer.

## Preview core and composition

- interaction: `ActionArea`, `OverlayFocusController`,
  `RovingFocusController`, `SelectionController`;
- geometry: `LayoutGridOverlay`, `AtlasIntrinsicFrame`, `AtlasStickyRegion`;
- scrolling and panes: `AtlasScrollViewport`, `AtlasSplitView`,
  `AtlasSplitPane`, `AtlasResizeHandle`;
- responsive recipes: `AtlasStack`, `AtlasCluster`, `AtlasSidebar`,
  `AtlasSwitcher`, `AtlasAutoGrid`.

## Preview navigation and overlays

`AtlasTab`, `AtlasTabPanel`, `AtlasTooltip`, `AtlasMenu`, `AtlasModal`, and
`AtlasDrawer` cover selection, contextual help, menus, dialogs, and panes with
controlled state and explicit focus restoration.

## Preview data and application controls

- data: `AtlasDataTable`, `AtlasDataList`, `AtlasKeyValueList`;
- domain-neutral surfaces: `AtlasPanel`, `AtlasMetricCard`, `AtlasSparkline`;
- input: `AtlasSelectField`, `AtlasSegmentedControl`, `AtlasRangeControl`;
- progress and navigation: `AtlasProgressBar`, `AtlasPagination`, `AtlasStepper`;
- feedback: `AtlasInlineAlert`, `AtlasNoticeStack`, `AtlasWorkflowBanner`,
  `AtlasErrorPage`.

The table supports virtualization, a sticky header, resizable columns,
multi-selection, sorting, filtering, inline editing, contextual menus,
expandable details, and a compact responsive mode. Domain operations remain in
the Rust host.

## Preview rich content and documentation

- blocks: `AtlasAdmonition`, `AtlasCallout`, `AtlasFigure`,
  `AtlasDocumentTable`, `AtlasTerminalBlock`, `AtlasContentTabs`;
- links and text: `AtlasLink`, `AtlasLinkCard`, `AtlasRichText`,
  `AtlasSelectableText`, `AtlasDocumentList`;
- references: `AtlasCaption`, `AtlasCrossReference`,
  `AtlasFootnoteReference`, `AtlasFootnoteList`;
- tools: `AtlasAnchorAction`, `AtlasDocumentSearch`, `AtlasCommandPalette`;
- shell: `AtlasDocumentationShell`, `AtlasThemeControl`.

## Preview templates

`AtlasRoadmapContentTemplate`, `AtlasSettingsTemplate`, and
`AtlasDashboardTemplate` demonstrate how to compose foundations without adding
a domain dependency to the library.

## Maturity

`stable` means that names, properties, callbacks, types, and semantics follow
SemVer. `preview` means that a component is testable and documented but may
still change in a minor release. Promotion requires an API audit,
representative scenarios, and a recorded decision.
