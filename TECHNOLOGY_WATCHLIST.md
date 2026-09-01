# Atlas UI technology watchlist

This watchlist records the upstream capabilities and engineering evidence that
may unlock Atlas roadmap work or allow preview contracts to become stable. It
tracks the pinned Slint version rather than restating product priority. Priority,
dependency order, and ownership are defined in [`ROADMAP.md`](ROADMAP.md).

The status described here is not a commitment from the Slint project. Every item
is reassessed when Atlas changes its pinned Slint version, validates a new
platform, or starts work on an affected dependency tree.

## Slint 1.17.1 capability baseline

| Area | Pinned baseline | Capability or evidence still required | Atlas interim strategy | Review trigger |
|---|---|---|---|---|
| Text measurement and leading | `Text` exposes wrapping and font metrics, but no explicit line-height contract | Controllable line height and reliable multiline intrinsic measurement across renderers | Use normalized Atlas body-font metrics and explicit typography tokens | `Text`, `StyledText`, font metrics, shaping, or intrinsic-size behavior changes |
| Responsive layout | Atlas preview recipes use experimental `FlexboxLayout` behavior | Stable wrapping, basis, growth, shrinkage, gaps, and nested constraints | Keep responsive recipes in preview and cover boundary widths with fixtures | Flex layout becomes stable or its constraint API changes |
| Container context | Reusable children require explicit `reference-width` bindings | Component-scoped container size or semantic size context | Keep size input explicit and avoid global viewport state | Slint adds container context, queries, attached layout properties, or an equivalent primitive |
| Dynamic grid placement | Grid placement and repeated/conditional layout metadata remain constrained | Dynamic spans, starts, offsets, ordering, and predictable wrapping | Keep the initial Atlas grid width-based; calculate tracks and gutters explicitly | Grid and repeater layout metadata changes |
| Reusable delegates | Static child placement is available through `@children` | Typed reusable presentation delegates for model-driven controls | Keep Atlas data contracts explicit and share internal cell components | Slint adds component-valued delegates, typed slots, or an equivalent model presentation contract |
| Accessibility | Roles, labels, values, selection, landmarks, and live properties are available | Verified relationships, focus behavior, advanced collection semantics, and backend parity | Define Atlas semantic and keyboard contracts; validate each platform | Accessibility properties, roles, platform bridges, or backend behavior changes |
| Focus and overlays | Focus scopes and popup primitives exist, while complete placement and restoration remain component work | Reliable containment, restoration, outside dismissal, anchor geometry, collision handling, and nested overlays | Keep overlays controlled; share Atlas focus and placement contracts | Popup, focus, coordinate mapping, or window APIs change |
| Text editing | Text input and editing primitives exist | Mature multiline editing, selection, cursor visibility, IME, scrolling, and error relationships across platforms | Keep text controlled and domain validation in the host | TextEdit, IME, selection, mobile, or platform input behavior changes |
| Visible-item virtualization | `ListView` instantiates visible items on demand | Variable-height measurement, sticky regions, focus retention, and stable identity during mutations | Reuse `ListView`; add Atlas identity, paging, selection, and tree adapters | ListView, Flickable, repeater identity, or viewport APIs change |
| Model adapters | Rust exposes `Model`, `ModelRc`, map, filter, sort, reverse, and mutable vector models | Atlas-specific stable identity, tree projection, paging, and cell projection | Implement deterministic adapters in Atlas Rust foundations | Slint model notification, identity, threading, or adapter APIs change |
| Drag and drop | `DragArea`, `DropArea`, actions, images, and opaque host-language data transfer are available | Cross-platform pointer and keyboard evidence plus reusable Atlas recipes | Validate native primitives; keep payload validation and filesystem work in the host | Drag/drop API, backend coverage, or platform payload behavior changes |
| Localization and direction | Text supports start/end alignment and host-owned copy | Locale-aware formatting hooks, bidi evidence, logical layout direction, and RTL parity | Keep formatting host-owned and avoid stable RTL claims until verified | Locale, translation, bidi, or logical-direction APIs change |
| Rendering and text shaping | Multiple renderers and font metrics are available | Consistent clipping, fractional geometry, SVG, shaping, and typography across renderers and scale factors | Maintain deterministic software baselines and targeted native-platform captures | Renderer, font, SVG, scale-factor, or shaping dependencies change |
| Input modalities | Keyboard, pointer, touch, wheel, focus, and drag primitives are available at different maturity levels | Consistent mouse, touch, pen, wheel, high-resolution scrolling, and keyboard behavior | Expose controlled intentions and document validated modalities per component | Event structures, gesture APIs, or platform coverage changes |
| Platform abstraction | Custom `Platform` and `WindowAdapter` implementations are possible; renderers remain Slint-provided | Only platform-specific integration that cannot be expressed through normal components | Treat custom platform work as exceptional and never as a widget workaround | Platform, window-adapter, clipboard, URL, event-loop, or renderer APIs change |

## Atlas and ecosystem evidence

| Area | Work or evidence required | Unlocks or stabilizes | Readiness signal |
|---|---|---|---|
| Responsive verification | Boundary-width, nesting, overflow, span, wrapping, density, and reduced-motion fixtures | Responsive composition contracts | Reviewed scenarios pass at compact, normal, wide, and exact breakpoint widths |
| Cross-platform visuals | Deterministic or reviewed baselines beyond the reference renderer | Broader rendering support claims | Approved Linux, Windows, and macOS evidence records renderer and scale factor |
| Accessibility testing | Repeatable keyboard paths and platform accessibility inspection for each interactive family | Promotion of preview controls | Keyboard matrices and assistive-technology checks pass on supported platforms |
| Overlay verification | Anchor, collision, nested popup, dismissal, and focus restoration scenarios | Menu, modal, drawer, popover, combobox, and date-picker families | Placement and focus matrices pass at viewport edges and under resize |
| Collection identity | Mutation, filtering, sorting, paging, expansion, and focus-retention fixtures | Stable data list, table, tree, search, and command components | Selection and focus remain attached to stable identifiers under every supported mutation |
| Localization | Locale, text expansion, Unicode, bidi, and RTL fixtures with host-owned formatting examples | Date/time controls, forms, navigation, and document layouts | Representative locales pass layout, input, focus, and accessibility review |
| Performance | Budgets for large models, variable rows, nested layouts, overlays, text-heavy documents, and resize behavior | Advanced collections and application templates | Debug and release measurements remain within published budgets |
| Binary footprint | Differential release measurements, resource inventories, and verified Slint feature profiles | Smaller consumers and enforceable dead-weight limits | Minimal, control, form, data, and document consumers have comparable baselines |
| API evolution | Migration fixtures and clean external consumers for every preview contract considered for promotion | Stable facade growth | The contract survives real consumers and a release cycle without unresolved migration risk |
| Packaging | Registry-only consumers, feature isolation, and removal of unnecessary experimental flags | Easier adoption of stable and preview APIs | Published packages compile without workspace paths or implicit repository configuration |
| Component evidence | Multiple reusable application cases with documented ownership and accessibility boundaries | New roadmap candidates | One concrete consumer and a second credible reuse case validate the primitive |

## Review classification

For each new Slint release, record every affected item as one of:

- **unchanged** — Atlas keeps its current implementation and maturity;
- **improved** — Atlas can simplify a workaround or add evidence;
- **replaced** — an Atlas polyfill can be removed behind the same public API;
- **regressed** — affected preview components remain blocked or need adaptation;
- **no longer relevant** — the roadmap dependency has changed or disappeared.

An upstream feature becoming stable does not automatically promote an Atlas
component. Atlas still requires API, accessibility, visual, performance,
packaging, and consumer evidence.

For actionable-card collections specifically, `AtlasAutoGrid` remains the
preview contract. Its stabilization exit criteria are: a released stable Slint
wrapping primitive or a non-experimental Atlas replacement; deterministic
compact/normal/wide, exact-breakpoint, nested, long-label, and overflow
fixtures; keyboard order matching visual order; macOS/Linux/Windows compile and
interaction evidence; and a stable-only consumer that needs no experimental
environment variable. Until then, stable consumers choose a host-controlled
column count and compose explicit linear layout rows.

## Upstream references

- [Slint component libraries](https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/)
- [Slint compiler library paths](https://docs.slint.dev/latest/docs/rust/slint_build/struct.CompilerConfiguration)
- [Text properties and font metrics](https://docs.slint.dev/latest/docs/slint/reference/elements/text/)
- [Positioning, constraints, layouts, and child placement](https://docs.slint.dev/latest/docs/slint/guide/language/coding/positioning-and-layouts/)
- [ListView visible-item virtualization](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/listview/)
- [Rust model API](https://docs.slint.dev/latest/docs/rust/slint/trait.Model)
- [DragArea](https://docs.slint.dev/latest/docs/slint/reference/drag-and-drop/dragarea/)
- [Slint platform abstraction](https://docs.slint.dev/latest/docs/rust/slint/platform/)

## Maintenance policy

The pinned versions and verified platforms remain authoritative in
[`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md). Product order and ownership
remain authoritative in [`ROADMAP.md`](ROADMAP.md). Update this watchlist when
either document changes materially or when an upstream release affects a
tracked capability.
