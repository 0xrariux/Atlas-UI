# Atlas UI roadmap

Atlas UI evolves from verified application needs, but foundations are ordered
before the components that depend on them. This roadmap records priorities,
dependency chains, and the boundary between Slint, Atlas, and application code.
It communicates direction rather than delivery dates.

## At a glance

| Horizon | Focus |
|---|---|
| Now | Finish Slint 1.18 visual, platform, and accessibility evidence for existing layout, focus, scrolling, and collection contracts. |
| Next | Validate preview forms and overlays in real consumers; build the shared field frame and multiline text area. |
| Later | Advance tree semantics, date/time controls, RTL, drag and drop, and platform-specific interaction breadth. |

New Atlas APIs normally begin in the preview facade. They move to stable only
after their behavior, accessibility, responsive layout, performance, and
migration risks are validated in real applications. Small additive contracts
may enter stable directly only when equivalent evidence already covers their
complete behavior, as with `AtlasStatusIndicator`.

## Responsibility model

The labels used throughout the roadmap identify the expected implementation
owner:

- **[SLINT]** — language, compiler, runtime, renderer, platform, or native
  semantic capability that is broadly useful outside Atlas;
- **[ATLAS-RUST]** — deterministic algorithms, model adapters, identity, and
  state coordination that Atlas can implement without owning rendering;
- **[ATLAS-UI]** — tokens, visual policy, headless controllers, reusable Slint
  components, and application-independent composition recipes;
- **[HOST]** — domain data, navigation, persistence, networking, filesystem,
  security decisions, and other external effects;
- **[EVIDENCE]** — fixtures, accessibility checks, performance measurements,
  compatibility records, and migration tests required for stabilization.

### What Atlas can extend

| Capability | Atlas can own it without an upstream change | Boundary |
|---|---|---|
| Reusable `.slint` components, globals, structs, tokens, and recipes | Yes | Consumers explicitly import and use the Atlas type |
| Layout algorithms, model adapters, tree flattening, paging, selection, and overlay placement math | Yes, in Rust | Slint still owns measurement, rendering, focus, and the accessible tree |
| Platform integration | Technically yes, through Slint's platform abstraction | Exceptional work only; it is too broad a mechanism for fixing individual widgets |
| Global changes to built-in `Text`, layouts, input semantics, or accessibility behavior | No | Requires an upstream Slint change or a maintained fork |
| A replacement renderer | No through the public API | Slint's renderer interface is sealed; a fork would be required |

Atlas extends Slint; it does not hide or globally monkey-patch it. A native
semantic control must remain represented by Slint elements so focus, input,
accessibility, scaling, and renderer behavior remain visible to the runtime.

## Target architecture

Slint owns the language, runtime, rendering, native input, and accessibility.
Atlas Rust supplies deterministic algorithms and thin Slint adapters; Atlas
core and components turn them into reusable UI contracts. Templates and host
applications consume those contracts.

Pure algorithms should remain independent of Slint where practical. Thin
adapters convert their results into Slint properties and models. This keeps the
algorithms testable and reusable while preserving Slint as the rendering and
native interaction foundation.

## Priority model

- **P0 — multiplier foundations:** unblock or stabilize several component
  families and prevent local reimplementation;
- **P1 — dependent component families:** reusable controls and compositions
  built after their required foundations are proven;
- **P2 — platform breadth and advanced interaction:** important work with a
  narrower immediate impact or a higher cross-platform validation cost.

Status terms below distinguish implemented contracts, work that must evolve,
new Atlas work, merged Slint features pending integration, experimental
upstream features, and capabilities still awaiting Slint support.

## P0.1 — Measurement and layout

This is the highest-leverage dependency tree. Typography, forms, data views,
responsive compositions, and application shells all consume its results.

Slint 1.18 integrates multiline measurement fixes, stable flex layout, item
ordering, and repeated-grid structure fixes. Atlas's typography and responsive
recipes continue to evolve. Preview AtlasContainer supplies explicit local
metrics, while the existing Rust track allocator serves tables and lists.
Scoped context and dynamic grid placement remain upstream dependencies.

### Upstream integration baseline

Reviewed on 2026-09-09 and revisited for the 2026-09-16 Slint 1.18.0 source
migration. The
[upstream delta in the watchlist](TECHNOLOGY_WATCHLIST.md#upstream-delta-reviewed-on-2026-09-09)
records merged work separately from this baseline. The
[Slint master changelog](https://github.com/slint-ui/slint/blob/master/CHANGELOG.md)
records the 1.18.0 release. Atlas v0.2.2 pins 1.18.0; its macOS arm64 software
baselines and Rust 1.92 CI matrix are approved. Other renderers and native
interaction paths need target-specific evidence.

- **Explicit leading:** [#12649](https://github.com/slint-ui/slint/pull/12649),
  merged 2026-08-19, adds `line-height-factor`. It multiplies natural line height,
  so equal factors do not guarantee equal rhythm across fonts. Preserve Atlas
  typography metrics until cross-font and cross-renderer evidence supports
  removing individual workarounds.
- **Multiline measurement:** merged
  [#13003](https://github.com/slint-ui/slint/pull/13003) and
  [#13071](https://github.com/slint-ui/slint/pull/13071) address box-layout rows
  and repeated grid cells measured at an incorrect width. These fixes are now
  in Atlas's Slint pin; investigate only reproducible residual defects.
- **Stable flexbox:** [#12767](https://github.com/slint-ui/slint/pull/12767),
  merged 2026-08-05, removes experimental status. General implementation and
  stabilization are no longer upstream roadmap tasks; Atlas recipe validation
  migration is complete in the source checkout. Atlas recipe evidence remains
  required.
- **Item order:** `layout-order` exists on master for `FlexboxLayout`,
  `HorizontalLayout`, and `VerticalLayout`. Validate the Atlas ordering contract
  independently from dynamic grid placement and keyboard/accessibility order.
- **Grid structures:** merged
  [#10739](https://github.com/slint-ui/slint/pull/10739) and
  [#12257](https://github.com/slint-ui/slint/pull/12257) cover spans in repeated
  rows and conditional cells. These structural fixes do not establish dynamic
  placement values: published [GridLayout documentation](https://docs.slint.dev/latest/docs/slint/reference/layouts/gridlayout/)
  still requires constants for `row`, `col`, `rowspan`, and `colspan`.
- **Neutral container:** `ContentFrame` can remain Atlas-owned. The open related
  [CellLayout request #4800](https://github.com/slint-ui/slint/issues/4800)
  is a possible home for a generic alignment and overlapping-constraint
  primitive, subject to a concrete, reusable contract.

### Atlas implementation audit and next work

The 2026-09-09 source audit distinguishes existing Atlas code from work unlocked
by the upstream delta:

| Area | Current Atlas implementation | Next concrete work |
|---|---|---|
| Typography | [Typography tokens](crates/atlas-ui-tokens/ui/typography.slint) and [editorial components](crates/atlas-ui-components/ui/editorial.slint) retain embedded font metrics; `AtlasStyledText` and `AtlasSelectableText` now forward `line-height-factor`, and `AtlasStyledText` forwards `max-lines` | Compare fonts, scales, wrapping, and renderers before changing global leading tokens or removing metric assertions |
| Responsive layouts | [Responsive recipes](crates/atlas-ui-core/ui/responsive-layout.slint) compose stable Slint 1.18.0 `FlexboxLayout`; the experimental flag has been removed | Validate narrow and wide captures, nested constraints, and external consumers before promoting Atlas preview APIs |
| Measurement evidence | [Runtime Slint fixture](crates/atlas-ui-testing/tests/layout_runtime.rs) renders real Atlas switcher, nested grid, and document-list components across wide/narrow/wide transitions, short/long/short text, repeated-row insertion/removal, and bounded panes. The nested grid now binds `reference-width` and `basis-width` to parent-owned geometry while children bind preferred width to `item-basis`; this compiles without the previous layout cycle. A separate repeated-grid fixture covers wrapping, insertion, removal, and conditional spans. The public track allocator is exercised by [data-table geometry tests](crates/atlas-ui-testing/tests/data_table_layout.rs). | Extend rendered table-cell fixtures, mixed font scales, explicit min/max conflicts, and cross-renderer/platform measurements |
| Ordering and grids | A Slint 1.18 fixture verifies that `layout-order` changes visual order while Tab keeps declaration order; recipes compute spans and widths explicitly | Validate accessibility-tree order on target backends before exposing order controls on interactive Atlas compositions; retain the width-based grid adapter until dynamic placement values have a supported contract |
| Container metrics | Stable `ContentFrame` computes page margins; preview `AtlasContainer` exposes local content width, height, and size class with an explicit child boundary | Validate nested consumer compositions and clipping. Slint does not propagate scoped metrics through `@children`; consumers bind child widths and `reference-width` explicitly until upstream context exists |
| Track allocation | Public `atlas_ui::core::tracks::allocate_tracks` computes deterministic integer widths, offsets, overflow, and unused space. The gallery binds both data tables, document tables, and a key/value list to the allocator. `TrackWidthOverrides` keeps resized preferred widths by column ID. | Validate density and column-model mutation, then compare rendered tracks in external consumers and across platforms |
| Scoped context and slots | Explicit `reference-width` properties and `@children` composition remain in use | Keep explicit context wiring. Evaluate named slots only in supported static contexts on an identified upstream revision; dynamic delegates and popup composition need separate evidence |

The runtime layout fixtures now exercise Atlas's responsive panes, repeated
document rows, and a Slint grid with wrapped repeated rows and a conditional
span. Extend the versioned migration comparison with standalone Slint forms,
notification rows, and rendered table cells, then validate the corresponding
Atlas components. Check
existing upstream tests and open contributions before proposing any residual
correction. The Slint source upgrade and flag removal do not imply Atlas
stability promotion or visual approval.
The [P0.1 template review](docs/P0_1_MANUAL_REVIEW.md) records the remaining
cross-renderer, typography, accessibility-order, and host-wiring checks.
The [P0 foundations guide](docs/P0_FOUNDATIONS.md) records the Atlas-owned
adapters added after the 1.18 source migration.

### Required outcomes

| Work item | Primary owner | Atlas interim strategy | Completion signal |
|---|---|---|---|
| Line height and multiline measurement | Atlas integration + Slint residual fixes | Slint 1.18 and editorial property forwarding are integrated; retain normalized metrics where needed | Resize, text/model mutation, nested layouts, and cross-font/renderer fixtures pass; remove only workarounds proven redundant |
| Stable responsive layout | Atlas UI + evidence | Preview recipes now compile against stable upstream flexbox without experimental compiler features | Nested, wrapping, min/max, and boundary fixtures pass; Atlas promotion still meets its own evidence gates |
| Linear/flex item order | Atlas UI + evidence | Integrate `layout-order` separately from grid placement | Visual order, keyboard traversal, and accessibility behavior satisfy the documented Atlas contract |
| Neutral content container | Atlas UI | Compose `@children` in a non-painting frame instead of using incidental `Rectangle` semantics | Layouts can introduce stacking, clipping, constraints, and content bounds without creating a decorative surface |
| Container context | Slint + Atlas UI | Define explicit container metrics and size-class inputs; continue `reference-width` bindings until scoped propagation exists upstream | Child components consume a scoped container size without application-global viewport state or repetitive bindings |
| Repeated/conditional grid structures | Atlas integration + evidence | Validate merged span, conditional-cell, and multiline fixes | Repeated and conditional fixtures preserve geometry during resize and model changes |
| Dynamic grid placement values | Slint | Keep the initial Atlas grid width-based while placement properties require constants | Runtime `row`, `col`, `rowspan`, and `colspan` values have a documented, predictable contract |
| Track allocation | Atlas Rust | Implement one tested allocator for min, preferred, max, grow, and overflow | Headers, rows, frozen regions, and compact transformations share identical geometry |

## P0.2 — Focus, accessibility, and overlays

Slint owns native focus and accessible semantics. Atlas's shared ActionArea
and focus controllers define behavior for existing buttons, toggles, menus,
modals, and drawers. Radio group, popover, combobox, and autocomplete are
preview components; date and time pickers remain future work. Placement,
dismissal, and focus restoration need broader native-platform evidence.

Overlay geometry can be computed by Atlas, but native focus state and the
accessible tree must remain in Slint. Components expose controlled intentions;
the host owns navigation and external effects. The existing modal and drawer
frames cover controlled panel, dismissal, traversal, and focus-restoration
boundaries. A modal/menu stack now has a tested focus priority contract for
same-update opening through `focus-on-open`. A host-side placement function now
covers an anchor, viewport collision, flip, and shift; menus can dismiss when
their anchor becomes ineligible. A slotted preview popover now consumes the
host's placement result and covers outside-click dismissal. A preview combobox
composes the select field with a keyboard menu, and tooltips close when their
trigger becomes ineligible. A preview autocomplete composes the text field
and menu while keeping text-input focus during keyboard selection. The host
owns filtering and suggestion results. Combobox and autocomplete now accept
host-computed menu coordinates and anchor eligibility, while menu model changes
clamp the active index and empty combobox options close the list. The unresolved
`evolve` work includes shared placement wiring across applications, long-list
scrolling, item-identity settlement after reordering, and broader platform
evidence.

Atlas menus now bound long lists by `maximum-menu-height`, expose
`scroll-offset`, and reveal the active item after keyboard navigation. Runtime
fixtures cover this path. Placement in applications and active-ID settlement
after reorder remain host integration work.

Slint already repositions popups in some cases. The open
[popup placement request #4870](https://github.com/slint-ui/slint/issues/4870)
concerns a fuller anchor, gravity, and constraint contract. Atlas Rust placement
remains relevant for explicit policy and deterministic adapters; native
collision handling must not be described as wholly absent.

`ActionArea` is Atlas's headless pressable foundation rather than merely an
implementation detail of `AtlasButton`. It should specify one reusable
activation contract for button-like controls, toggles, tabs, disclosures, and
other semantic actions. Atlas can normalize public Slint behavior and test its
consumers, but it cannot manufacture passive pointer observation, event
propagation, global pointer coordinates, or component destruction signals when
the runtime does not expose them.

The preview `AtlasRadioGroup` exposes controlled choice IDs, radio semantics,
one tab stop, visible focus, and host-selected roving focus. Its arrow callback
reports direction; the host selects an eligible index from its model and calls
`focus-index(index)`. A native Slint 1.18 radio prototype did not expose a
controlled selected index and did not move selection with arrow keys in the
software-window fixture, so Atlas uses its existing `ActionArea` policy here.

A software-window interaction fixture now exercises `ActionArea` pointer and
Enter/Space activation, disabled transitions, keyboard focus indication,
`AtlasModal` traversal and dismissal, `AtlasMenu` keyboard navigation and
disabled entries, and `AtlasDrawer` dismissal and focus return. Slint 1.18
requires the otherwise headless overlay focus scope to remain visible at zero
size; a hidden scope cannot hold keyboard focus. A second software-window
fixture covers stable workspace-tab roving navigation, close settlement, and
host-selected focus after a tab becomes disabled. A radio-group fixture covers
host-selected focus around a disabled choice and Enter/Space selection. An
autocomplete fixture covers live typing, arrow selection, Enter, Escape, and
Tab without moving focus to its menu. A
modal/menu stack also
passes sequential and same-update opening in either order, followed by Escape
and focus restoration for each overlay. The menu also dismisses on anchor
ineligibility; the popover covers Escape, outside click, and anchor loss. A
three-layer modal/popover/menu fixture verifies same-update focus unwinding.
Menu composition inside a popover covers outside dismissal and one focus
return. Other roving groups, unbounded overlay stacks,
native accessibility backends, and assistive technology still need separate
evidence before P0.2 is complete.

### Required outcomes

| Work item | Primary owner | Completion signal |
|---|---|---|
| Accessible semantic coverage | Slint + Atlas evidence | Required roles and relationships behave consistently on every supported backend |
| Headless activation behavior | Atlas UI | Every applicable control consumes one documented pointer, Enter/Space, disabled, focus-visible, and accessibility contract |
| Pointer layering and passive hover | Slint + Atlas evidence | Parent hover observation coexists with interactive descendants without duplicate activation or blocked events |
| Keyboard modality and roving focus | Atlas UI | Shared controllers cover every applicable component family without local key handling divergence |
| Overlay placement | Atlas Rust + Atlas UI | One placement contract covers anchors, collision, dismissal, nested overlays, and deterministic fixtures |
| Overlay lifecycle | Slint + Atlas UI | Open overlays close or re-anchor predictably when their anchor disappears, becomes disabled, or loses eligibility |
| Focus containment and restoration | Slint + Atlas UI | Modal, drawer, menu, popover, and combobox keyboard matrices pass on supported platforms |

## P0.3 — Scoped context and shared environment

Atlas already exposes global design tokens and settings, but reusable
components also need a disciplined way to consume shared environment without
turning domain state into an implicit singleton.

Slint's scoped provider/consumer context remains upstream work. Atlas uses
explicit container metrics and AtlasEnvironment forwarding until a language
contract can propagate local context safely.

The [provider/consumer proposal #3508](https://github.com/slint-ui/slint/issues/3508)
remains open. Scope, default values, and preview behavior still need a language
contract agreed with Slint maintainers before a broad implementation effort.
Stable flexbox and named slots do not supply this scoped environment contract.

Until Slint provides scoped context, Atlas should prefer explicit properties,
small environment structs, root-owned globals, and documented adapter recipes.
Atlas globals remain appropriate for truly application-wide design settings;
they must not become a service locator for navigation, domain data, persistence,
or arbitrary component state. Nested theme or density overrides require a
scoped contract and must not silently mutate application-global settings.

Atlas now exposes an explicit `AtlasEnvironment` value on `AtlasContainer`.
Nested containers bind `environment` to their parent's `content-environment`,
which replaces viewport dimensions with local content metrics while preserving
the other fields. This is an opt-in property adapter; a language-level scoped
provider remains upstream work.

### Required outcomes

| Work item | Primary owner | Completion signal |
|---|---|---|
| Environment vocabulary | Atlas UI | Container, viewport, modality, locale, direction, theme, and density inputs use shared types and naming across component families |
| Root adapter recipe | Atlas Rust + Atlas UI | A consumer wires window and host context once at an application boundary without repetitive leaf-level bindings |
| Scoped overrides | Slint + Atlas UI | Nested visual regions can override supported environment values without affecting siblings or relying on hidden domain state |

## P0.4 — Scrolling and viewport behavior

AtlasScrollViewport and AtlasScrollbar are preview controls. Atlas owns
positive controlled offsets, axis policy, keyboard paging, reveal requests,
and scrollbar presentation. Slint still owns native gesture inertia and input
propagation at nested scroll boundaries.

Atlas can hide coordinate-sign details, expose controlled offsets, constrain
allowed axes, and standardize keyboard and scrollbar behavior. Native gesture
inertia, high-resolution touchpad behavior, event propagation at nested scroll
boundaries, and platform-specific wheel policy remain upstream concerns unless
Slint exposes sufficient controls for a portable adapter.

`AtlasScrollViewport` now supports controlled positive offsets on both axes,
clamping after content shrink, rectangle reveal, an overlaid horizontal
scrollbar, and logical-direction keyboard paging. A software-window fixture
checks two-axis clamping, reveal, and RTL key behavior. Native nested gesture
chaining remains upstream/platform behavior.

### Required outcomes

| Work item | Primary owner | Completion signal |
|---|---|---|
| Axis contract | Atlas UI | Vertical and horizontal consumers cannot accidentally create cross-axis overflow or a zero-sized viewport |
| Input parity | Slint + Atlas evidence | Mouse wheel, precision touchpad, touch, keyboard, and programmatic scrolling behave predictably on supported platforms |
| Nested scrolling | Slint + Atlas UI | Boundary events pass to the intended ancestor without loops, dead zones, or double movement |
| Viewport control | Atlas UI | Components share positive controlled offsets, clamping, paging, reveal requests, and accessible range semantics |

## P0.5 — Models, delegates, and virtualization

Slint `ListView` already instantiates visible rows on demand. Atlas should build
identity and collection behavior around it instead of replacing basic visible
item virtualization.

Slint ListView already virtualizes visible rows. Atlas Rust adds stable
identity, paging, grouping, cell projection, and tree flattening for existing
data views and the preview AtlasTreeView. Variable-height and sticky
virtualization, reusable dynamic delegates, and some focus guarantees remain
upstream work; named slots are still experimental.

Named slots already have an initial implementation in
[#12552](https://github.com/slint-ui/slint/pull/12552), merged 2026-07-31,
but remain experimental. The initial implementation forbids placing slots in
`if`, `for`, `PopupWindow`, and `ComponentContainer`. Evaluate supported static
composition before adding Atlas workarounds; do not assume this unlocks
customizable comboboxes or overlays. Typed, model-driven, dynamic delegates
remain a separate upstream contract.

Atlas Rust now provides ID-based collection state, bounded paging, grouping,
cell projection by column ID, borrowed visible-tree flattening, and an indexed
tree projection with incremental expand/collapse. Preview
`AtlasTreeView` renders host-projected fixed-height rows and emits intentions
by ID; a software-window fixture covers keyboard disclosure and selection.
Variable-height/sticky virtualization and reusable dynamic delegates remain
upstream dependencies.

### Required outcomes

| Work item | Primary owner | Completion signal |
|---|---|---|
| Stable identity adapters | Atlas Rust | Selection, focus, and expansion survive insertion, removal, filtering, sorting, and paging |
| Tree projection | Atlas Rust | Large host-owned hierarchies flatten incrementally without domain logic in Slint |
| Advanced virtualization | Slint | Variable-height and sticky collections retain correct geometry, identity, and focus |
| Named slots | Atlas experiments + Slint stabilization | Supported composition contexts are validated on an explicit compiler revision; stable use waits for upstream stabilization |
| Reusable typed/dynamic delegates | Slint | Model-driven controls can accept reusable typed presentation without copying implementations, including required dynamic and popup contexts |

## P1.1 — Forms and text editing

The next Atlas field foundation will share labels, descriptions, required
state, validation presentation, disabled/read-only behavior, and focus routing.
It will serve existing text and select controls, preview radio group, combobox,
and autocomplete, plus a future multiline text area and date/time fields.
Mature TextEdit, IME, and accessible error relationships still need Slint and
platform evidence.

Validation remains controlled by the host. Atlas owns presentation, focus
intentions, and the relationship between labels, descriptions, and messages;
it does not introduce hidden domain validation.

## P1.2 — Derived components ordered by dependency readiness

| Order | Component family | Required foundations | Primary owner |
|---|---|---|---|
| 1 | Radio group | Field frame, roving focus, accessible grouping | Atlas UI |
| 2 | Accordion and disclosure | ActionArea, roving focus, expanded semantics | Atlas UI |
| 3 | Breadcrumb | Navigation semantics, responsive overflow | Atlas UI + host routing |
| 4 | Avatar and persona | Image fallback, loading state, accessible identity | Atlas UI |
| 5 | Multiline text area | Field frame, TextEdit/IME evidence, scrolling | Atlas UI + Slint |
| 6 | Popover | Overlay placement, dismissal, focus restoration | Atlas Rust + Atlas UI |
| 7 | Combobox and autocomplete | Field frame, overlay, models, keyboard navigation | Atlas Rust + Atlas UI + Slint |
| 8 | Tree view | Stable identity, tree projection, virtualization, tree semantics | Atlas Rust + Atlas UI + Slint |
| 9 | Date and time pickers | Overlay, text input, localization, host formatting | Atlas UI + host + Slint |

## P1.3 — Atlas-owned visual and composition system

These capabilities express Atlas design policy. They should not be proposed as
Slint built-ins unless a smaller, brand-neutral primitive is discovered.

Atlas owns design tokens, surfaces, status and progress controls, scrolling,
domain-neutral frames, overlay compositions, controlled data states, responsive
recipes, and application-independent templates. The component catalog records
each public contract; application code retains domain data and external
effects.

`AtlasScrollbar` and `AtlasScrollViewport` remain preview. They have a real
Talos consumer plus direct and composed dark/light gallery evidence; promotion
still requires repeatable keyboard and accessibility inspection,
cross-platform rendering review, and migration evidence from an external
consumer.

## P2 — Direction, drag and drop, and platform breadth

Slint supplies the lower-level direction and drag/drop primitives. Atlas
still needs verified RTL layout contracts and controlled recipes for collection
reordering, movable columns and panels, and file-drop presentation. Hosts
retain payload validation and filesystem operations.

Atlas must validate the Slint drag-and-drop primitives across supported
platforms before defining stable recipes. It should not recreate the underlying
drag source, drop target, or payload negotiation mechanism.

Additional P2 candidates include advanced desktop navigation, nested context
menus, desktop menubars, floating panels, color pickers, dynamic
data-visualization series and legends, responsive visibility, aspect-ratio,
and overflow helpers. Direction-aware motion recipes and richer predetermined
text runs are also candidates where public Slint primitives are sufficient;
arbitrary inline components, transition cancellation, and per-direction
animation control remain upstream dependencies. `AtlasChartFrame` already
supplies the accessible plot background and grid; it does not close the
dynamic-series work.

## Cross-cutting evidence gates

These are required workstreams, not optional cleanup after component delivery.

| Gate | Required evidence | Unlocks |
|---|---|---|
| Responsive layout | Boundary widths, nesting, wrapping, overflow, density, and reduced-motion fixtures | Stable layout recipes and application templates |
| Accessibility | Repeatable keyboard paths and platform accessibility inspection | Promotion of interactive preview components |
| Cross-platform rendering | Reviewed Linux, Windows, and macOS captures with renderer and scale-factor metadata | Broader support claims |
| Localization | Text expansion, Unicode, bidi, RTL, and host-formatting fixtures | Date/time controls and localized application layouts |
| Performance | Budgets for large models, resize behavior, overlays, and text-heavy views | Advanced collections and document workspaces |
| Binary footprint | Differential release measurements and resource reachability | Enforceable runtime and asset budgets |
| API evolution | Migration fixtures and external consumers | Stable facade growth |
| Packaging | Registry-only consumers and verified feature isolation | Reproducible adoption outside the workspace |

## Delivery sequence

The dependency order is more important than the exact release containing each
wave.

1. **Foundation contracts and evidence:** define neutral behavior, ownership,
   fixtures, and budgets before adding APIs.
2. **Measurement and layout:** complete runtime validation of Slint 1.18's
   leading, box/grid measurement, stable flexbox, and item-order behavior;
   implement the Atlas track allocator and size classes. Target upstream work
   at reproduced residual defects, dynamic grid placement values, and scoped
   container capabilities rather than reimplementing merged features.
3. **Interaction, context, and overlays:** consolidate headless activation,
   environment adapters, overlay placement, accessible semantics, and focus
   restoration.
4. **Scrolling and collection models:** define axis and viewport contracts,
   then add stable identity, tree projection, paging, and
   advanced virtualization evidence.
5. **Forms and editing:** introduce the shared field frame and text area;
   validate the preview radio group, popover, combobox, and autocomplete in
   consumer templates and native accessibility tools.
6. **Advanced components and platform breadth:** tree view, date/time controls,
   RTL contracts, drag-and-drop recipes, and advanced desktop compositions.

If only three engineering programs can be funded, the order is:

1. measurement, responsive layout, and track allocation;
2. focus, accessibility, and overlay placement;
3. stable model identity, delegates, and advanced virtualization.

## Atlas polyfill and upstream policy

Atlas does not need to wait for every Slint improvement, but it must avoid
creating permanent forks of generic runtime behavior.

1. Check merged and in-flight Slint work against the pinned baseline. Specify
   the remaining behavior in brand-neutral terms and add deterministic fixtures
   with standalone Slint reproducers for upstream defects.
2. Implement an Atlas preview polyfill using public Slint and Rust APIs.
3. Propose upstream the smallest capability that is useful without Atlas
   tokens, naming, or product policy.
4. Keep the Atlas facade stable while replacing the internal polyfill when an
   upstream implementation becomes available.
5. Maintain a Slint fork only for a critical compiler/runtime blocker that
   cannot be expressed through public APIs, with an explicit synchronization
   and removal plan.

A capability is an upstream candidate when it is domain-neutral, useful to
multiple component families, independent of Atlas styling, or requires access
to compiler/runtime/platform internals. It remains Atlas-owned when it expresses
visual policy, controlled state, a composition recipe, or a replaceable adapter
over public Slint APIs.

## Maintenance

The pinned versions and supported platforms in
[`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md) remain authoritative. Upstream
capability status and review triggers live in
[`TECHNOLOGY_WATCHLIST.md`](TECHNOLOGY_WATCHLIST.md). When a dependency changes,
maintainers update the watchlist first, then reassess the affected roadmap tree
and its stabilization evidence.

Feedback should include a concrete application case, the missing primitive,
the expected ownership boundary, and at least one credible reuse case. Use the
[structured issue forms](https://github.com/0xrariux/Atlas-UI/issues/new/choose)
or the pinned
[Atlas UI 0.1 early-adopter feedback issue](https://github.com/0xrariux/Atlas-UI/issues/2).
