# Deterministic track allocation

`atlas_ui::core::tracks::allocate_tracks` allocates one horizontal set of
tracks in integer logical pixels. A host can reuse the same result for a table
header, every row, and related frozen or compact projections. The algorithm
honors each track's minimum, preferred width, maximum, and growth weight. It
reports real occupied width, overflow, and space left unused when all tracks
reach their maximum.

```rust
use atlas_ui::core::tracks::{TrackConstraint, allocate_tracks};

let columns = [
    TrackConstraint { preferred: 160, minimum: 120, maximum: 240, grow: 1 },
    TrackConstraint { preferred: 240, minimum: 180, maximum: 400, grow: 2 },
];
let allocation = allocate_tracks(640, 12, 8, &columns)?;
assert_eq!(allocation.widths.len(), columns.len());
```

The host reads the current Slint viewport width in logical pixels, chooses an
integer rounding policy, and maps the returned widths to the
`AtlasDataTable.allocated-column-widths` length model. Supply one width for
each `DataColumn` in the same order. When the model length differs, the table
falls back to its native Slint min/preferred/max/grow constraints. Recompute
after viewport, column, density, or padding/gap changes. Keep the table's
`cell-padding-x` and `column-gap` equal to the values passed to the allocator.

The native gallery wires both `AtlasDataTable` instances, its
`AtlasDocumentTable` instances, and its `AtlasKeyValueList` through pure Slint
callbacks handled in Rust. The bindings pass current width, padding, gap, and
column models to `allocate_tracks`. The returned width model drives each
table's header and rows together. The gallery also handles
`column-resize-requested` by storing the preferred width with
`TrackWidthOverrides`, keyed by column ID. See
[`apps/gallery`](../apps/gallery/src/main.rs). An invalid allocation returns an
empty model so the components use their native constraints.

The allocator returns an error for inverted ranges or geometry exceeding
`u32` logical pixels. It preserves minimums when the viewport is too narrow;
the table then needs horizontal scrolling. A finite maximum or zero growth
weight may leave `unused_width` instead of stretching a track past its limit.
The host owns column identity across insertion, removal, and reordering.

`AtlasContainer` is a separate preview layout boundary. It exposes
`content-width`, `content-height`, and a local `size-class` after `inset`.
Slint does not propagate these metrics through `@children` as implicit scoped
context: bind nested widths and `reference-width` properties explicitly to
the container's `content-width`. It does not draw a decorative surface or
intercept pointer input. `clip-content` is opt-in.

Slint 1.18's `layout-order` changes visual positions in linear and flex
layouts. The Atlas software-window fixture confirms that keyboard Tab still
follows declaration order for interactive children. Keep visual, keyboard,
and accessible order aligned; use `layout-order` for interactive content only
when the host supplies an intentional focus policy and validates it on target
accessibility backends.
