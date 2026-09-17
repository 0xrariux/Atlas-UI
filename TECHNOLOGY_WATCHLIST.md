# Atlas UI technology watchlist

This watchlist records the upstream capabilities and engineering evidence that
may unlock Atlas roadmap work or allow preview contracts to become stable. It
distinguishes the pinned Slint baseline from merged upstream capabilities rather
than restating product priority. Priority,
dependency order, and ownership are defined in [`ROADMAP.md`](ROADMAP.md).

The status described here is not a commitment from the Slint project. Every item
is reassessed when Atlas changes its pinned Slint version, validates a new
platform, or starts work on an affected dependency tree.

## Slint 1.17.1 historical capability baseline

| Area | Pinned baseline | Capability or evidence still required | Atlas interim strategy | Review trigger |
|---|---|---|---|---|
| Text measurement and leading | `Text` exposes wrapping and font metrics, but no explicit line-height contract | Controllable line height and reliable multiline intrinsic measurement across renderers | Use normalized Atlas body-font metrics and explicit typography tokens | `Text`, `StyledText`, font metrics, shaping, or intrinsic-size behavior changes |
| Responsive layout | Atlas preview recipes use experimental `FlexboxLayout` behavior | Stable wrapping, basis, growth, shrinkage, gaps, and nested constraints | Keep responsive recipes in preview and cover boundary widths with fixtures | Flex layout becomes stable or its constraint API changes |
| Container context | Reusable children require explicit `reference-width` bindings | Component-scoped container size or semantic size context | Keep size input explicit and avoid global viewport state | Slint adds container context, queries, attached layout properties, or an equivalent primitive |
| Shared environment context | Globals and explicit root properties can distribute application-wide values, but globals are not scoped providers | Typed provider/consumer context with nested overrides and predictable ownership | Reserve globals for application-wide design settings; use explicit environment structs and root adapter recipes | Globals, component inheritance, context, dependency injection, or scoped property propagation changes |
| Dynamic grid placement | Placement values remain constrained; merged repeated/conditional structure fixes are tracked in the upstream delta below | Dynamic row/column starts and spans, with predictable wrapping; linear/flex ordering is separate | Keep the initial Atlas grid width-based; calculate tracks and gutters explicitly | Grid and repeater layout metadata changes |
| Reusable delegates | Static child placement is available through `@children` | Typed reusable presentation delegates for model-driven controls | Keep Atlas data contracts explicit and share internal cell components | Slint adds component-valued delegates, typed slots, or an equivalent model presentation contract |
| Accessibility | Roles, labels, values, selection, landmarks, and live properties are available | Verified relationships, focus behavior, advanced collection semantics, and backend parity | Define Atlas semantic and keyboard contracts; validate each platform | Accessibility properties, roles, platform bridges, or backend behavior changes |
| Focus and overlays | Focus scopes and popup primitives exist, while complete placement and restoration remain component work | Reliable containment, restoration, outside dismissal, anchor geometry, collision handling, and nested overlays | Keep overlays controlled; share Atlas focus and placement contracts | Popup, focus, coordinate mapping, or window APIs change |
| Pointer observation and propagation | `TouchArea` exposes hover and activation but interactive layers can compete for pointer events | Passive hover observation, explicit propagation/capture policy, global pointer coordinates, and reliable nested hit testing | Keep `ActionArea` as the shared pressable contract; avoid claiming passive ancestor hover where Slint cannot express it | Pointer events, `TouchArea`, hit testing, capture, coordinate mapping, or window pointer APIs change |
| Component lifecycle | Component state and visibility can be observed, but no general destruction notification is available to reusable Slint components | Reliable unmount/destruction or anchor-invalidated notification for transient UI cleanup | Keep overlays controlled by a surviving owner and require explicit close/invalidation intents where possible | Component lifecycle, repeater removal, popup ownership, or destruction callbacks change |
| Text editing | Text input and editing primitives exist | Mature multiline editing, selection, cursor visibility, IME, scrolling, and error relationships across platforms | Keep text controlled and domain validation in the host | TextEdit, IME, selection, mobile, or platform input behavior changes |
| Visible-item virtualization | `ListView` instantiates visible items on demand | Variable-height measurement, sticky regions, focus retention, and stable identity during mutations | Reuse `ListView`; add Atlas identity, paging, selection, and tree adapters | ListView, Flickable, repeater identity, or viewport APIs change |
| Scrolling and viewport input | `Flickable` supplies native movement, while axis restriction, nested propagation, and device-specific tuning are limited | Explicit axes, boundary pass-through, controllable smoothing/inertia, and parity across wheel, precision touchpad, touch, and keyboard | Wrap controlled positive offsets in `AtlasScrollViewport`; keep unsupported native gesture policy upstream | `Flickable`, wheel events, gesture APIs, viewport physics, or platform input behavior changes |
| Model adapters | Rust exposes `Model`, `ModelRc`, map, filter, sort, reverse, and mutable vector models | Atlas-specific stable identity, tree projection, paging, and cell projection | Implement deterministic adapters in Atlas Rust foundations | Slint model notification, identity, threading, or adapter APIs change |
| Drag and drop | `DragArea`, `DropArea`, actions, images, and opaque host-language data transfer are available | Cross-platform pointer and keyboard evidence plus reusable Atlas recipes | Validate native primitives; keep payload validation and filesystem work in the host | Drag/drop API, backend coverage, or platform payload behavior changes |
| Localization and direction | Text supports start/end alignment and host-owned copy | Locale-aware formatting hooks, bidi evidence, logical layout direction, and RTL parity | Keep formatting host-owned and avoid stable RTL claims until verified | Locale, translation, bidi, or logical-direction APIs change |
| Rendering and text shaping | Multiple renderers and font metrics are available | Consistent clipping, fractional geometry, SVG, shaping, and typography across renderers and scale factors | Maintain deterministic software baselines and targeted native-platform captures | Renderer, font, SVG, scale-factor, or shaping dependencies change |
| Input modalities | Keyboard, pointer, touch, wheel, focus, and drag primitives are available at different maturity levels | Consistent mouse, touch, pen, wheel, high-resolution scrolling, and keyboard behavior | Expose controlled intentions and document validated modalities per component | Event structures, gesture APIs, or platform coverage changes |
| Rich inline content | `StyledText` and Markdown cover part of rich presentation; ordinary text styling and component-valued inline runs remain constrained | Typed styled runs, links, images, badges, and accessible inline components without parsing arbitrary Markdown | Keep Atlas editorial structures explicit and use standalone semantic components; do not imply general inline-component support | `Text`, `StyledText`, Markdown, inline layout, or component-valued content changes |
| Animation control | Declarative transitions and `animation-tick()` are available | Direction-specific enter/exit timing, cancellation, sequencing, and interruption semantics | Standardize Atlas motion tokens and bounded component state transitions; preserve reduced-motion behavior | Animation, transition, timeline, cancellation, or reduced-motion APIs change |
| Platform abstraction | Custom `Platform` and `WindowAdapter` implementations are possible; renderers remain Slint-provided | Only platform-specific integration that cannot be expressed through normal components | Treat custom platform work as exceptional and never as a widget workaround | Platform, window-adapter, clipboard, URL, event-loop, or renderer APIs change |

## Upstream delta reviewed on 2026-09-09

Atlas's source checkout now pins Slint 1.18.0. The baseline above records the
earlier 1.17.1 state; the table below records the upstream evidence available
on 2026-09-09. The released [1.18.0 changelog](https://github.com/slint-ui/slint/blob/master/CHANGELOG.md#1180---2026-09-16)
and [Atlas audit](docs/SLINT_1_18_AUDIT.md) supersede its release-status
predictions. Atlas visual and cross-platform validation remains in progress.

| Foundation | Upstream evidence | Atlas action and remaining boundary |
|---|---|---|
| Explicit leading | [#12649](https://github.com/slint-ui/slint/pull/12649), merged 2026-08-19, adds `line-height-factor` | Atlas editorial wrappers now forward the property. The factor scales natural line height; retain font normalization until cross-font and renderer evidence supports changing it |
| Multiline measurement | [#13003](https://github.com/slint-ui/slint/pull/13003) fixes box-layout row measurement; [#13071](https://github.com/slint-ui/slint/pull/13071) fixes repeated wrapped cells in grid layouts; both merged | Slint 1.18 is pinned and all 77 visual scenarios have been compared with 1.17.1 references; test resize, model/text mutations, nesting, and explicit constraints before proposing residual fixes or approving new baselines |
| Stable flexbox | [#12767](https://github.com/slint-ui/slint/pull/12767), merged 2026-08-05, removes experimental status | Atlas responsive recipes and all facades compile without experimental compiler flags. Complete runtime layout and external-consumer evidence before promoting preview APIs |
| Item order | `layout-order` is on master for flexbox and horizontal/vertical layouts; see the master changelog | Validate visual order, keyboard order, and accessibility separately; do not classify this as dynamic grid placement |
| Named slots | [#12552](https://github.com/slint-ui/slint/pull/12552), merged 2026-07-31, provides an experimental initial implementation | Evaluate static composition without reimplementing named slots; initial slots cannot be placed in `if`, `for`, `PopupWindow`, or `ComponentContainer`, so customizable comboboxes and overlays are not automatically unlocked; typed dynamic delegates remain separate |
| Repeated and conditional grids | [#10739](https://github.com/slint-ui/slint/pull/10739) fixes spans in repeated rows; [#12257](https://github.com/slint-ui/slint/pull/12257) fixes conditional cells; both merged | Validate the corrected structures separately from dynamic placement values: published [GridLayout documentation](https://docs.slint.dev/latest/docs/slint/reference/layouts/gridlayout/) still requires constants for `row`, `col`, `rowspan`, and `colspan` |
| Scoped provider/consumer context | [#3508](https://github.com/slint-ui/slint/issues/3508) remains open, with scope, defaults, and preview behavior under discussion | Keep explicit context adapters; frame any language contribution with maintainers before implementation |
| Popup placement | [#4870](https://github.com/slint-ui/slint/issues/4870) remains open; repositioning already exists, but the requested anchor, gravity, and constraint contract is incomplete | Keep Atlas Rust placement policy and adapters; do not describe all native collision handling as absent |
| Neutral container | Related [CellLayout request #4800](https://github.com/slint-ui/slint/issues/4800) remains open | Keep `ContentFrame` Atlas-owned; assess a smaller generic alignment and overlapping-constraint primitive as a possible upstream contribution |

These are review inputs, not automatic promotions. For named slots, record the
exact compiler revision and supported placement contexts in any experiment.
For layout fixes, a new upstream correction needs a failing standalone Slint
reproducer on the current implementation, with no Atlas dependency.

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
preview contract. Slint 1.18 supplies the released stable wrapping primitive;
the remaining stabilization criteria are deterministic
compact/normal/wide, exact-breakpoint, nested, long-label, and overflow
fixtures; keyboard order matching visual order; macOS/Linux/Windows compile and
interaction evidence; and a stable-only consumer exercising the contract.
Until then, stable consumers choose a host-controlled
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
