# Component catalog

For a quick task-oriented selection, especially from an AI agent, also see the
[component index](AGENT_COMPONENT_INDEX.md).

The `v0.1.1` release exposes 97 components. Public classification is enforced
by the `stable.slint` and `preview.slint` facades.
Non-responsive preview contracts are also available from
`preview-nonresponsive.slint` without experimental Slint features. The full
`preview.slint` and `components.slint` aggregates both load the responsive
module and therefore require `SLINT_ENABLE_EXPERIMENTAL_FEATURES=1`.

## Stable foundations

The following contracts are available from `stable.slint`:

- surfaces: `Surface`, `ComponentFrame`, `ContentFrame`, `FocusRing`;
- controls: `AtlasButton`, `AtlasTextField`, `AtlasCheckbox`, `AtlasSwitch`,
  `AtlasBadge`, `AtlasStatusIndicator`, `AtlasIconButton`;
- icons: `AtlasIcon`;
- workspace navigation: `AtlasWorkspaceTab`, `AtlasWorkspaceTabList`;
- application chrome and metrics: `AtlasEdgeSurface`, `AtlasMetricCard`;
- states: `AtlasSkeleton`, `AtlasEmptyState`, `AtlasErrorState`;
- editorial: `AtlasHeading`, `AtlasParagraph`, `AtlasStyledText`,
  `AtlasInlineCode`, `AtlasCodeBlock`, `AtlasBlockQuote`, `AtlasDivider`;
- globals and types associated with themes, tokens, density, motion, and typography.

These 58 symbols—components, types, and globals combined—follow SemVer.

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

`AtlasStatusIndicator` is the standalone semantic signal for dense rows,
adjacent labels, avatars, and headings. It shares `BadgeTone` with
`AtlasBadge`, requires an accessible `label`, and permits an
`indicator-color` override for product theming. Do not use the signal without
adjacent text or another non-color state cue.

`AtlasSwitch` keeps its original stable interaction, accessibility, checked
state, and `toggled(bool)` callback while allowing dense desktop presentation
to adapt. Its additive inputs and stable defaults are:

- `show-label: true`;
- `track-offset-x: AtlasGrid.space-1`, `track-width: AtlasGrid.space-10`, and
  `track-height: AtlasGrid.space-6`;
- `track-padding: AtlasGrid.space-1` and
  `track-radius: AtlasShape.radius-round`;
- `track-border-width: AtlasShape.border-width`, with border color derived
  from `SwitchTone` and checked state;
- `track-background`, which defaults to selected or normal surface color;
- `thumb-size: AtlasGrid.space-4`, with thumb color derived from checked state.

Use `track-border-color`, `track-background`, and `thumb-color` only for
bounded product composition. A visually compact track does not justify a tiny
isolated pointer target: keep the switch itself at
`AtlasDensity.pointer-target-min`, or place it inside a setting row whose hit
area meets the target.

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
- scrolling and panes: `AtlasScrollbar`, `AtlasScrollViewport`, `AtlasSplitView`,
  `AtlasSplitPane`, `AtlasResizeHandle`;
- responsive recipes: `AtlasStack`, `AtlasCluster`, `AtlasSidebar`,
  `AtlasSwitcher`, `AtlasAutoGrid`, `AtlasColumnGrid`,
  `AtlasGridItem`.

`AtlasScrollbar` is the controlled visual/input primitive for vertical
overflow. Bind `viewport-y`, `viewport-height`, `content-height`, and optional
`maximum-y`, then apply `scroll-requested(length)` to the host-owned scroll
state. Its default anatomy draws a six-pixel track and proportional thumb with
two-pixel corners inside a 16-pixel interaction corridor. The track, thumb,
minimum height, inset, radius, and idle/hover/active colors are bounded
per-instance inputs. `AtlasScrollViewport` composes this same primitive and
continues to own native flicking plus Page Up, Page Down, Home, and End.
The component hides when there is no overflow and clamps its rendered and
accessible position when host state is temporarily outside the valid range.
Provide a contextual `accessible-label` when the surrounding content is not
obvious.

```slint
import { AtlasScrollbar } from "@atlas-ui/preview-nonresponsive.slint";

export component ExistingScrollSurface inherits Rectangle {
    in-out property <length> scroll-y;
    in property <length> content-height;

    AtlasScrollbar {
        x: parent.width - self.width;
        height: parent.height;
        viewport-y: root.scroll-y;
        viewport-height: root.height;
        content-height: root.content-height;
        accessible-label: "Project list scroll position";
        scroll-requested(offset) => { root.scroll-y = offset; }
    }
}
```

Do not render the standalone primitive alongside the viewport's built-in
scrollbar. For per-instance scrollbar colors or geometry inside an
`AtlasScrollViewport`, set `show-scrollbar: false`, reserve
`AtlasViewportTokens.scrollbar-hit-width`, and bind one `AtlasScrollbar` to the
same controlled offset.

`AtlasAutoGrid` remains preview: its wrapping, basis, growth, and shrinkage
depend on experimental `FlexboxLayout` in Slint 1.17.1. Stable consumers should
choose a deterministic column count in application state and compose explicit
`HorizontalLayout`/`VerticalLayout` groups. Promotion requires a stable Slint
flex contract or a non-experimental Atlas implementation, breakpoint and nested
overflow evidence, three-platform verification, and a clean stable consumer.

## Preview navigation and overlays

`AtlasTab`, `AtlasTabPanel`, `AtlasTooltip`, `AtlasMenu`, `AtlasModal`,
`AtlasModalFrame`, `AtlasDrawer`, and `AtlasDrawerFrame` cover selection,
contextual help, menus, dialogs, and panes with controlled state and explicit
focus restoration.

Use `AtlasModal` for its standard `title`, `meta`, `description`, `ModalSize`,
danger state, cancel/confirm labels, and `cancelled()`/`confirmed()` callbacks.
It inherits `open`, `contained`, backdrop/panel presentation, and the focus
lifecycle from `AtlasModalFrame`. The standard component derives its semantic
label, panel size, danger border, dismissal, and traversal behavior from its
high-level API; use `AtlasModalFrame` instead of rebinding those managed values
when a product needs custom dialog anatomy. The frame exposes a child slot and
`dismissed()`, `traversal-requested(bool)`, and
`focus-restore-requested()` intentions.

Use `AtlasDrawer` for the standard title/body/actions anatomy. It inherits
controlled `open`, `side`, `contained`, `dismiss-on-backdrop`, backdrop/panel
presentation, and focus lifecycle from `AtlasDrawerFrame`, then maps dismissal
to `close()`. Use the frame for consumer-owned content or panel geometry. On
either drawer, `dismiss-on-backdrop: false` disables pointer dismissal without
removing the focus controller's keyboard dismissal intention. Both frame
components are preview.

## Preview data and application controls

- data: `AtlasDataTable`, `AtlasDataList`, `AtlasKeyValueList`;
- domain-neutral composition: `AtlasPanel`, `AtlasMetric`,
  `AtlasCopyableValue`, `AtlasSettingsRow`, `AtlasChartFrame`,
  `AtlasSparkline`;
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

`AtlasMetric` is the unframed label/value/metadata anatomy for metric strips,
inspectors, and product-owned cards. It exposes semantic `ValueTone` plus
bounded presentation inputs; the consumer continues to own surfaces, icons,
dividers, grouping, and responsive layout. Prefer stable `AtlasMetricCard`
when its complete card composition already fits.

`AtlasCopyableValue` presents one elided value and a keyboard-accessible copy
action. `copy-requested(value)` is an intention only: Rust owns clipboard I/O
and the feedback timer, then returns controlled `copied` state. Use
`copy-enabled: false` for a visible unavailable action and customize geometry
only when embedding the primitive in a compact product composition. Localize
`copy-label` and `copied-label`; disabling the action reduces opacity and
prevents pointer, keyboard, and accessible activation.

```slint
AtlasCopyableValue {
    value: "item_123";
    copied: root.copy-feedback-visible;
    copy-requested(value) => { root.copy-requested(value); }
}
```

`AtlasSettingsRow` is an accessible group box for a setting `title`, optional
`description`, and consumer-owned child control. `enabled` changes the row's
semantics and opacity but does not own or mutate the child value; bind the same
enabled state into an interactive child when it must be disabled. The
`show-divider`/`divider-color` inputs control the optional trailing divider.
`content-right-inset`, title/description y and height inputs, text colors,
sizes, and title weight provide bounded layout hooks. The host still owns
validation, saving, and persistence, and the child may be a switch, field,
select, or action.

```slint
AtlasSettingsRow {
    title: "Automatic deployment";
    description: "Apply validated releases automatically.";
    show-divider: true;
    AtlasSwitch {
        x: parent.width - self.width;
        y: (parent.height - self.height) / 2;
        label: "Automatic deployment";
        show-label: false;
    }
}
```

`AtlasChartFrame` is an accessible image-role plot surface with required
`label` and optional `description`. `horizontal-lines` defaults to five and
`vertical-lines` to zero. When a line count is greater than one, lines are
distributed from the leading/top endpoint to the trailing/bottom endpoint; a
count of one places the line at the origin. Grid width, horizontal and vertical
colors, and plot background are configurable. Child content is rendered over
the grid and owns the series. The frame deliberately does not own data models,
series rendering, axes, legends, sampling, interpolation, animation, or domain
formatting.

```slint
AtlasChartFrame {
    label: "Request rate";
    description: "A host-provided series over a four-by-four grid";
    horizontal-lines: 4;
    vertical-lines: 4;
    // Add product-owned series elements here.
}
```

`AtlasSpinner` is the compact indeterminate primitive for controls and small
status surfaces. Its `label` and `value-text` form the progress accessibility
contract; `indicator-color` defaults to the semantic info color and `size`
tracks the current typography scale. `AtlasProgressBar` remains the horizontal
primitive. Set `indeterminate: true` for unknown-duration work and
`show-labels: false` for a rail embedded at a surface edge. The rail still
requires `label` and exposes `value-text`, even when its visual labels are
hidden. Embedded rails can configure `track-height`, `track-radius`,
`track-color`, and `indicator-color`; the defaults preserve the standard
8 px rounded semantic track. Full motion uses the shared continuous-cycle
tokens. Reduced motion uses a centered static segment or incomplete spinner
arc, preserving an unambiguous in-progress state without movement.

These customization inputs are component-scoped presentation hooks. They do
not introduce new global color, typography, spacing, density, shape, or
application-theme tokens. In particular, `AtlasMetric` font-family inputs do
not define a global serif, numeric, tabular-numeric, or financial type role.

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
