# Atlas UI roadmap

Atlas UI evolves from verified application needs, but foundations are ordered
before the components that depend on them. This roadmap records priorities,
dependency chains, and the boundary between Slint, Atlas, and application code.
It communicates direction rather than delivery dates.

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

```text
Slint language, runtime, renderers, platform, and native semantics
                         │
                         ├──────────────────────────────┐
                         │                              │
                         ▼                              ▼
          Atlas Rust foundations                Atlas Slint adapters
          ├── track allocation                  ├── ModelRc bridges
          ├── overlay placement                 ├── globals/callbacks
          ├── stable identity                   └── geometry conversion
          ├── tree flattening                           │
          └── controlled state                          │
                         └──────────────┬───────────────┘
                                        ▼
                                  atlas-ui-core
                                  ├── surfaces
                                  ├── layout recipes
                                  ├── focus controllers
                                  └── semantic primitives
                                        │
                                        ▼
                               atlas-ui-components
                                        │
                                        ▼
                         templates and host applications
```

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

Status markers in the trees:

- **existing** — implemented with a usable contract;
- **evolve** — implemented, but its foundation or evidence must improve;
- **new** — an Atlas implementation is required;
- **upstream** — Atlas can polyfill part of the behavior, but the complete
  primitive belongs in Slint.

## P0.1 — Measurement and layout

This is the highest-leverage dependency tree. Typography, forms, data views,
responsive compositions, and application shells all consume its results.

```text
[SLINT] Reliable measurement and constraint semantics                 upstream
│
├── [SLINT] Explicit line height and multiline intrinsic measurement  upstream
│   └── [ATLAS-UI] Atlas typography contracts                         evolve
│       ├── editorial components
│       ├── field descriptions and validation
│       ├── rich data cells
│       ├── alerts and notifications
│       └── documentation content
│
├── [SLINT] Stable flex layout and container context                  upstream
│   └── [ATLAS-UI] compact / normal / wide size classes               new
│       ├── AtlasCluster                                              evolve
│       ├── AtlasSidebar                                              evolve
│       ├── AtlasSwitcher                                             evolve
│       ├── AtlasAutoGrid                                             evolve
│       ├── AtlasColumnGrid / AtlasGridItem                           evolve
│       └── AtlasSplitView                                            evolve
│
├── [SLINT] Dynamic grid placement metadata                           upstream
│   └── [ATLAS-UI] span, offset, order, and wrapping contracts        new
│       ├── forms
│       ├── dashboards
│       └── asymmetric application layouts
│
└── [ATLAS-RUST] Deterministic track allocator                        new
    ├── AtlasDataTable                                                evolve
    ├── AtlasDocumentTable                                            evolve
    ├── AtlasKeyValueList                                             evolve
    ├── resizable columns                                             evolve
    └── split panes and property grids                                evolve
```

### Required outcomes

| Work item | Primary owner | Atlas interim strategy | Completion signal |
|---|---|---|---|
| Line height and multiline measurement | Slint | Preserve normalized Atlas font metrics and explicit typography tokens | Atlas can remove font-specific leading workarounds without visual regression |
| Stable responsive layout | Slint | Keep experimental flex usage behind preview Atlas recipes | Nested, wrapping, min/max, and boundary fixtures pass without experimental compiler features |
| Container context | Slint | Continue explicit `reference-width` bindings | Child components consume a scoped container size without global state or repetitive bindings |
| Dynamic grid placement | Slint | Keep the initial Atlas grid width-based | Repeated and conditional items support dynamic placement metadata predictably |
| Track allocation | Atlas Rust | Implement one tested allocator for min, preferred, max, grow, and overflow | Headers, rows, frozen regions, and compact transformations share identical geometry |

## P0.2 — Focus, accessibility, and overlays

```text
[SLINT] Reliable native focus and accessibility semantics             upstream
│
├── names, descriptions, errors, values, and live updates
├── detailed table, collection, and tree relationships
├── focus traversal and restoration
└── backend parity
    │
    ▼
[ATLAS-UI] Shared interaction controllers                             evolve
├── ActionArea
├── RovingFocusController
├── SelectionController
└── OverlayFocusController
    │
    ├── foundational controls
    │   ├── AtlasButton                                               existing
    │   ├── AtlasCheckbox / AtlasSwitch                               existing
    │   └── AtlasRadioGroup                                           new
    │
    └── [ATLAS-RUST] Overlay placement and lifecycle                  new
        ├── anchor geometry
        ├── viewport collision and fallback placement
        ├── outside dismissal
        ├── focus containment
        └── focus restoration
            ├── AtlasTooltip                                          evolve
            ├── AtlasMenu                                             evolve
            ├── AtlasPopover                                          new
            ├── AtlasCombobox / AtlasAutocomplete                     new
            ├── date and time pickers                                 new
            ├── AtlasModalFrame                                       existing
            │   └── AtlasModal                                        evolve
            └── AtlasDrawerFrame                                      existing
                └── AtlasDrawer                                       evolve
```

Overlay geometry can be computed by Atlas, but native focus state and the
accessible tree must remain in Slint. Components expose controlled intentions;
the host owns navigation and external effects. The existing modal and drawer
frames cover controlled panel, dismissal, traversal, and focus-restoration
boundaries; the unresolved `evolve` work is shared anchor placement, collision,
nested overlay, and broader platform evidence.

### Required outcomes

| Work item | Primary owner | Completion signal |
|---|---|---|
| Accessible semantic coverage | Slint + Atlas evidence | Required roles and relationships behave consistently on every supported backend |
| Keyboard modality and roving focus | Atlas UI | Shared controllers cover every applicable component family without local key handling divergence |
| Overlay placement | Atlas Rust + Atlas UI | One placement contract covers anchors, collision, dismissal, nested overlays, and deterministic fixtures |
| Focus containment and restoration | Slint + Atlas UI | Modal, drawer, menu, popover, and combobox keyboard matrices pass on supported platforms |

## P0.3 — Models, delegates, and virtualization

Slint `ListView` already instantiates visible rows on demand. Atlas should build
identity and collection behavior around it instead of replacing basic visible
item virtualization.

```text
[SLINT] ListView visible-item virtualization                          existing
│
├── [SLINT] Advanced collection virtualization                       upstream
│   ├── measured variable-height items
│   ├── sticky regions
│   ├── focus retention
│   └── stable identity during model mutations
│
├── [SLINT] Reusable typed delegate/slot contracts                    upstream
│
└── [ATLAS-RUST] Collection model adapters                           new
    ├── stable item identity
    ├── tree flattening and disclosure state
    ├── paging
    ├── grouping
    ├── cell projection
    └── selection by identifier
        ├── AtlasDataList                                             evolve
        ├── AtlasDataTable                                            evolve
        ├── AtlasTreeView                                             new
        ├── AtlasCommandPalette                                       evolve
        ├── AtlasDocumentSearch                                       evolve
        └── AtlasCombobox / AtlasAutocomplete                         new
```

### Required outcomes

| Work item | Primary owner | Completion signal |
|---|---|---|
| Stable identity adapters | Atlas Rust | Selection, focus, and expansion survive insertion, removal, filtering, sorting, and paging |
| Tree projection | Atlas Rust | Large host-owned hierarchies flatten incrementally without domain logic in Slint |
| Advanced virtualization | Slint | Variable-height and sticky collections retain correct geometry, identity, and focus |
| Reusable delegates | Slint | Model-driven controls can accept reusable typed presentation without copying implementations |

## P1.1 — Forms and text editing

```text
[ATLAS-UI] Shared field frame                                         new
├── label
├── description
├── required state
├── validation message
├── disabled and read-only state
└── focus routing
    │
    ├── AtlasTextField                                                evolve
    ├── AtlasRadioGroup                                               new
    ├── AtlasTextArea                                                 new
    ├── AtlasSelectField                                              evolve
    ├── AtlasCombobox / AtlasAutocomplete                             new
    ├── date and time fields                                          new
    └── inline table editing                                          evolve
        │
        └── [SLINT] Mature TextEdit and IME behavior                  upstream
            ├── selection
            ├── cursor visibility
            ├── composition input
            ├── automatic scrolling
            └── accessible error relationships
```

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

```text
[ATLAS-UI] Tokens and settings
├── semantic color
├── density
├── typography
├── motion
├── elevation and shape
└── geometry
    │
    ├── surfaces and frames
    │   ├── Surface
    │   ├── ComponentFrame
    │   ├── ContentFrame
    │   └── AtlasEdgeSurface
    │
    ├── status and progress
    │   ├── AtlasBadge
    │   ├── AtlasStatusIndicator
    │   ├── AtlasProgressBar
    │   └── AtlasRadialProgress
    │
    ├── scrolling
    │   ├── AtlasScrollbar
    │   └── AtlasScrollViewport
    │
    ├── domain-neutral composition
    │   ├── AtlasSettingsRow
    │   ├── AtlasChartFrame (frame and grid only)
    │   ├── AtlasMetric (unframed content)
    │   └── AtlasCopyableValue (host-owned clipboard)
    │
    ├── overlay composition
    │   ├── AtlasModalFrame → AtlasModal
    │   └── AtlasDrawerFrame → AtlasDrawer
    │
    ├── controlled data states
    │   ├── AtlasSkeleton
    │   ├── AtlasEmptyState
    │   └── AtlasErrorState
    │
    ├── responsive recipes
    │   ├── stacks, clusters, sidebars, and switchers
    │   ├── grids and split views
    │   └── sticky and intrinsic regions
    │
    └── application-independent templates
        ├── AtlasDocumentationShell
        ├── AtlasSettingsTemplate
        └── AtlasDashboardTemplate
```

`AtlasScrollbar` and `AtlasScrollViewport` remain preview. They have a real
Talos consumer plus direct and composed dark/light gallery evidence; promotion
still requires repeatable keyboard and accessibility inspection,
cross-platform rendering review, and migration evidence from an external
consumer.

## P2 — Direction, drag and drop, and platform breadth

```text
[SLINT] Logical direction and verified RTL behavior                   upstream
└── [ATLAS-UI] Logical start/end layout contracts                     new
    ├── navigation
    ├── tables
    ├── forms
    └── documentation

[SLINT] DragArea / DropArea                                           existing
└── [ATLAS-UI] Controlled drag-and-drop recipes                       new
    ├── reorderable collections
    ├── movable columns
    ├── resizable or movable panels
    └── file drop zones
        └── [HOST] payload validation and filesystem operations
```

Atlas must validate the Slint 1.17.1 drag-and-drop primitives across supported
platforms before defining stable recipes. It should not recreate the underlying
drag source, drop target, or payload negotiation mechanism.

Additional P2 candidates include advanced desktop navigation, nested context
menus, desktop menubars, floating panels, color pickers, dynamic
data-visualization series and legends, responsive visibility, aspect-ratio,
and overflow helpers. `AtlasChartFrame` already supplies the accessible plot
background and grid; it does not close the dynamic-series work.

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
2. **Measurement and layout:** implement the Atlas track allocator and size
   classes; continue upstream work for missing Slint measurement and container
   capabilities.
3. **Interaction and overlays:** consolidate keyboard controllers, overlay
   placement, accessible semantics, and focus restoration.
4. **Collection models:** add stable identity, tree projection, paging, and
   advanced virtualization evidence.
5. **Forms and editing:** introduce the shared field frame, then radio group,
   text area, popover, combobox, and autocomplete.
6. **Advanced components and platform breadth:** tree view, date/time controls,
   RTL contracts, drag-and-drop recipes, and advanced desktop compositions.

If only three engineering programs can be funded, the order is:

1. measurement, responsive layout, and track allocation;
2. focus, accessibility, and overlay placement;
3. stable model identity, delegates, and advanced virtualization.

## Atlas polyfill and upstream policy

Atlas does not need to wait for every Slint improvement, but it must avoid
creating permanent forks of generic runtime behavior.

1. Specify the behavior in brand-neutral terms and add deterministic fixtures.
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
