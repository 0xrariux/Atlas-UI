# Atlas P0 foundations in the Slint 1.18 source checkout

This page describes the Atlas-owned P0 work prepared for a release review. It
does not change the published `v0.1.1` package. The exact Slint and
`slint-build` pins in this checkout are 1.18.0.

## P0.1: measurement and tracks

`atlas_ui::core::tracks::allocate_tracks` is the shared integer-pixel
allocator. `TrackWidthOverrides` retains user-resized widths by column ID so
reordering does not attach a width to the wrong column. The gallery wires
`AtlasDataTable`, `AtlasDocumentTable`, and `AtlasKeyValueList` to Rust
allocation callbacks. Each table's header and body read the same width model;
the key/value list uses two tracks. Invalid input leaves the width model empty
and returns to native constraints. See [track allocation](TRACK_ALLOCATION.md).

`AtlasAutoGrid.basis-width` accepts a parent-owned width. When a grid is inside
an intrinsically sized flex pane and children bind `preferred-width` to
`item-basis`, set both `reference-width` and `basis-width` from an independent
parent width. The runtime fixture compiles and measures this previously cyclic
composition. A binding derived from the pane's measured child width remains a
cycle and is not a valid parent width.

## P0.2: focus and overlays

`AtlasMenu.maximum-menu-height` bounds a long menu. Its rows scroll inside a
`Flickable`; keyboard navigation reveals the active item and `scroll-offset`
exposes its current position. The host still owns overlay placement and active
item identity after reordering. Use
[`place_overlay`](OVERLAY_PLACEMENT.md) for anchored panels and return its
coordinates to the relevant menu or popover properties.

## P0.3: explicit environment

`AtlasEnvironment` carries local viewport dimensions, density, resolved theme,
motion preference, input modality, logical direction, and locale.
`AtlasContainer.environment` accepts the parent or application environment;
`content-environment` returns the same values with this container's local
content dimensions. Nested containers explicitly bind their `environment` to
the parent's `content-environment`. This is an opt-in property contract; Slint
does not propagate context through `@children` automatically.

## P0.4: controlled scrolling

`AtlasScrollViewport` now accepts content width and height, exposes positive
`viewport-x/y` offsets and their maxima, and can enable horizontal, vertical,
or both axes. It clamps offsets when content shrinks, provides `scroll-to-x`,
`scroll-to`, and `reveal-rect`, and overlays a horizontal scrollbar when
needed. Left and Right keyboard paging follows `logical-direction`.
Platform gesture inertia and scroll chaining at nested boundaries remain Slint
runtime behavior.

## P0.5: collection identity

`atlas_ui::core::collections` provides ID-based selection, focus, and
expansion; bounded paging; contiguous grouping; column-ID cell projection;
and borrowed visible-tree flattening. `AtlasTreeView` is a preview presentation
for host-projected `TreeViewRow` models. It emits selection, expansion, and
focus intentions by ID. After insertions, sorting, or filtering, the host maps
its focused ID back to a visible index before calling `focus-index`.
`TreeProjection` inserts and removes visible disclosure branches incrementally
and retains surviving expansion IDs when the host replaces its source tree.
Variable-height and sticky virtualization, dynamic delegates, and native tree
accessibility roles require upstream Slint support.

## Release boundary

The automated quality gate and software fixtures establish compilation,
deterministic algorithms, and targeted runtime geometry and keyboard paths.
The existing 1.17.1 visual references are not approved as 1.18 baselines.
Native platform review, template review, and the release version/tag decision
remain separate gates before publication.
