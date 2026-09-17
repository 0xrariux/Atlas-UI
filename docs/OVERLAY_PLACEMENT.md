# Anchored overlay placement

`atlas_ui::core::overlay::place_overlay` computes an overlay origin in logical
pixels. It accepts an anchor rectangle and viewport rectangle in the same
containing-window coordinate space, the overlay's measured size, a preferred
side, alignment, and gap. It tries the preferred side, its opposite, then the
perpendicular sides. It shifts along the cross axis to remain inside the
viewport and reports whether the whole overlay is visible.

```rust
use atlas_ui::core::overlay::{
    place_overlay, Alignment, PlacementRequest, Rect, Side, Size,
};

let placement = place_overlay(PlacementRequest {
    anchor: Rect { x: 120.0, y: 80.0, width: 32.0, height: 24.0 },
    viewport: Rect { x: 0.0, y: 0.0, width: 480.0, height: 320.0 },
    overlay: Size { width: 200.0, height: 160.0 },
    preferred_side: Side::Bottom,
    alignment: Alignment::Start,
    gap: 8.0,
});
```

The host passes `placement.origin.x/y` to the overlay's Slint position. Measure
the actual overlay size after layout, then recompute when its size, the anchor,
or the viewport changes. Do not mix window and component-local coordinates.
`None` means that geometry is invalid or the anchor is fully outside the
viewport; close the overlay or leave it unplaced. If `fully_visible` is false,
the overlay is larger than available space and the host must choose a size or
scroll policy. In a tight viewport, a placed overlay may overlap its anchor
after the final viewport clamp.

For `AtlasMenu`, bind `anchor-eligible` to the host's current anchor state.
Set it to false when the anchor is removed, disabled, or otherwise ineligible.
The menu emits `dismissed()` and stops rendering. The host must set `open` to
false in that callback and route `focus-restore-requested()` to an eligible
control. The host also owns outside-click handling. Atlas components do not
read window geometry or perform hidden I/O.

For `AtlasPopover`, use a viewport-sized root and pass the calculated origin
relative to that root through `panel-x` and `panel-y`. Its panel contains
consumer children. The component handles outside clicks, Escape, anchor loss,
and focus restoration intentions; the host still controls `open` and the
destination of restored focus.

For `AtlasCombobox` and `AtlasAutocomplete`, convert the returned window origin
to the field component's local coordinate space and bind `menu-x` and `menu-y`.
Their defaults place the menu below the field. Recompute on resize, model
height changes, and anchor movement. Bind `anchor-eligible` to the host's
anchor lifecycle; on invalidation, the menu closes without focusing that
ineligible anchor. The host chooses a separate fallback focus destination.

To add outside-click dismissal to `AtlasMenu`, compose it inside an
`AtlasPopover` panel. Bind both layers to one controlled `open` value and set
`focus-on-open: false` on the popover, leaving initial keyboard focus with the
menu. The popover handles outside clicks; the menu handles keyboard dismissal
and the single focus restoration callback.

The [P0.2 manual review](P0_2_MANUAL_REVIEW.md) lists the viewport and native
accessibility checks to run in consumer templates.
