# Talos consumer gap audit

## Packaging follow-up

Talos should import `AtlasProgressBar`, `AtlasSpinner`, `AtlasTab`, and other
non-responsive evolving controls from `@atlas-ui/preview-nonresponsive.slint`.
That facade has no transitive `FlexboxLayout` dependency and needs no
`SLINT_ENABLE_EXPERIMENTAL_FEATURES` setting. The full `preview.slint` and
`components.slint` facades remain compatibility aggregates and eagerly load
responsive preview contracts, so both still require the experimental setting.

This audit uses Talos as an integration case without importing its domain or
modifying the sibling repository.

## Decisions

| Gap | Decision | Atlas contract |
|---|---|---|
| Compact icon-only actions | Accepted as a new stable primitive | `AtlasIconButton`, `IconButtonTone` |
| Browser/workspace tabs | Changed from extending preview `AtlasTab` to a distinct stable family | `AtlasWorkspaceTab`, `AtlasWorkspaceTabList` |
| Operational symbols | Accepted as reusable registry additions | `grid`, `terminal`, `gamepad`, `cpu`, `memory`, `play`, `stop`, `chevron-right`, `layers` |
| Page-edge chrome | Accepted by extracting the existing implementation from experimental responsive code | stable `AtlasEdgeSurface`, `DividerEdge` |
| Metric presentation | Accepted by moving the existing component, not duplicating it | stable `AtlasMetricCard`, `ValueTone` |
| Persistent identity colors | Accepted as an ordinal token family separate from status | `AtlasCategoryTokens.category-1` through `category-6` |
| Responsive actionable cards | Rejected for stable promotion on Slint 1.17.1 | `AtlasAutoGrid` remains preview |
| Talos shell or project model | Rejected as application-specific | remains in Talos |
| Hover and pressed transitions | Accepted as behavior on existing controls and tabs | token-driven `AtlasMotion.fast` transitions |
| Compact indeterminate control progress | Accepted as a new preview primitive | `AtlasSpinner` |
| Long-running surface activity rail | Accepted by extending the existing preview progress contract | `AtlasProgressBar.indeterminate`, `show-labels`, `value-text` |
| Talos navigation geometry, accents, and project selection model | Rejected as product-specific composition | reuse Atlas tab/action contracts; keep styling and state in Talos |

## Interaction-primitives follow-up

The interaction review improves existing contracts before adding a new one.
`AtlasButton`, `AtlasIconButton`, `AtlasCheckbox`, `AtlasSwitch`, `AtlasTab`,
and `AtlasWorkspaceTab` now transition token-selected background, border, and
opacity states with `AtlasMotion.fast`. Keyboard activation, focus-visible,
disabled, selected, loading, roving-focus, and minimum pointer-target behavior
are unchanged. Reduced motion resolves these transitions to zero duration.

The stable button components add `loading-label: "Loading"`. Loading buttons
remain non-activatable and no longer take disabled opacity; they render the
same preview spinner while preserving their action label. `AtlasMotion` adds
the stable non-zero `spinner-cycle` and `indeterminate-cycle` divisor tokens.
Components explicitly replace these continuous animations with static reduced-
motion states.

`AtlasSpinner` is new and preview. Its public inputs are `label`, `value-text`,
`indicator-color`, and `size`. `AtlasProgressBar` remains preview and adds
`indeterminate`, `show-labels`, and `value-text`; its existing determinate API
and `percent` output remain intact. Spinner and rail stay separate because one
is intrinsic control content while the other consumes available horizontal
surface width.

The Talos top-bar tabs, project registry rows, detail tabs, Palladium sidebar,
Promethee controls, template rows, and project-create choices retain their
product-specific icons, fixed geometry, palette, copy, and domain state. Atlas
does not promote those compositions or any responsive behavior.

## Stable API additions and promotions

- New components: `AtlasIconButton`, `AtlasWorkspaceTab`,
  `AtlasWorkspaceTabList`.
- Promoted components: `AtlasEdgeSurface`, `AtlasMetricCard`.
- New type: `IconButtonTone`.
- Promoted types: `DividerEdge`, `ValueTone`.
- New global: `AtlasCategoryTokens`.
- Additive `IconName` values: `grid`, `terminal`, `gamepad`, `cpu`, `memory`,
  `play`, `stop`, `chevron-right`, and `layers`.

Preview keeps compatibility re-exports for `AtlasEdgeSurface`, `DividerEdge`,
`AtlasMetricCard`, and `ValueTone`. The aggregate facade continues to export
every stable and preview contract. Existing `AtlasTab` properties, callbacks,
defaults, and preview maturity are unchanged.

The change is additive under SemVer. Preview consumers may migrate promoted
imports to `stable.slint` immediately; no source migration is forced.

## Accessibility and interaction

`AtlasIconButton` uses the built-in Slint `accessible-label` as mandatory
consumer input, exposes enabled/loading/toggle state, supports Enter and Space,
uses keyboard-modality focus visibility, and never shrinks below the shared
minimum pointer target.

Workspace tabs expose tab-list, tab, and sibling close-button semantics. They
announce label, dirty description, selection, zero-based index, count, enabled
state, and deterministic IDs. Left/Right, Home, and End drive roving focus;
Enter and Space activate; Delete requests close; Backspace is not captured.
The host accepts or rejects selection and removal. After accepted removal it
calls `settle-after-close`, which focuses the new selected tab or nearest
surviving tab. Long labels elide within bounded tab widths. Overflow emits an
intention so the host may present a menu.

Metric cards expose name, value, and description separately. Category colors
meet the 3:1 graphical-object threshold against light/dark canvas and surface,
but must always be paired with a label, icon, pattern, or shape.

## Responsive collection blocker

`AtlasAutoGrid` still relies on Slint 1.17.1's experimental
`FlexboxLayout`. Stable applications should keep the column count in host or
application state and compose explicit linear rows. Atlas will reconsider
promotion only after a stable upstream wrapping contract or a
non-experimental replacement passes breakpoint, nesting, overflow, keyboard
order, three-platform, visual, and external-consumer evidence.

## Talos follow-up replacements

After Talos updates its Atlas dependency, it can replace:

- button-composed workspace tabs with `AtlasWorkspaceTab` and
  `AtlasWorkspaceTabList`;
- textual add/close/stop affordances with `AtlasIconButton`;
- raw topbar `Rectangle` chrome with `AtlasEdgeSurface`;
- local `TelemetryCell` with `AtlasMetricCard`;
- status-token identity mappings with `AtlasCategoryTokens`;
- shield/refresh/log substitutions with the matching operational `IconName`;
- `WorkspaceButton` with `AtlasButton`, mapping `primary` to
  `ButtonTone.primary`, `icon-name` to `show-icon` plus `icon-name`, and
  `loading` plus operation-specific `loading-label` directly;
- `WorkspaceActivityIndicator` with preview `AtlasSpinner` where a standalone
  compact progress object remains necessary;
- `WorkspaceLoadingRail` with preview `AtlasProgressBar { indeterminate: true;
  show-labels: false; }`, retaining an explicit operation `label` and
  `value-text`.

`TalosTopBar`, `ProjectCard`, `ProjectItem`, `HostedProjectView`, runtime
adapter terminology, project identity data, versions, execution state, and
telemetry values remain application-owned. The fixed actionable-card
collection also remains application-owned until the responsive blocker clears.
