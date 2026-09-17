//! Logical-pixel placement for host-controlled anchored overlays.
//!
//! The host reads Slint geometry, calls [`place_overlay`], then binds the
//! returned origin to its overlay. Focus and rendering remain in Slint.

/// A point in the containing window's logical coordinate space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
}

/// A size in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

/// A rectangle in the containing window's logical coordinate space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

/// The edge of the anchor next to the overlay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
    /// Above the anchor.
    Top,
    /// Below the anchor.
    Bottom,
    /// Left of the anchor.
    Left,
    /// Right of the anchor.
    Right,
}

/// Alignment along the edge shared with the anchor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
    /// Leading edges coincide.
    Start,
    /// Centers coincide.
    Center,
    /// Trailing edges coincide.
    End,
}

/// Geometry and policy needed to place one anchored overlay.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementRequest {
    /// Anchor geometry; must have positive dimensions.
    pub anchor: Rect,
    /// Allowed viewport; must have positive dimensions.
    pub viewport: Rect,
    /// Desired overlay size; must have positive dimensions.
    pub overlay: Size,
    /// First edge to try.
    pub preferred_side: Side,
    /// Alignment along that edge.
    pub alignment: Alignment,
    /// Nonnegative distance from the anchor.
    pub gap: f32,
}

/// The selected origin and edge. A large overlay may still overflow its bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    /// Overlay origin in the same coordinate space as the input rectangles.
    pub origin: Point,
    /// Edge selected after collision fallback.
    pub side: Side,
    /// Whether the entire overlay fits inside the viewport.
    pub fully_visible: bool,
}

impl Rect {
    fn valid(self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.width > 0.0
            && self.height > 0.0
    }
}

fn clamp(value: f32, start: f32, extent: f32, item_extent: f32) -> f32 {
    value
        .max(start)
        .min((start + extent - item_extent).max(start))
}

fn aligned(start: f32, anchor_extent: f32, overlay_extent: f32, alignment: Alignment) -> f32 {
    match alignment {
        Alignment::Start => start,
        Alignment::Center => start + (anchor_extent - overlay_extent) / 2.0,
        Alignment::End => start + anchor_extent - overlay_extent,
    }
}

fn opposite(side: Side) -> Side {
    match side {
        Side::Top => Side::Bottom,
        Side::Bottom => Side::Top,
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    }
}

fn candidates(preferred: Side) -> [Side; 4] {
    match preferred {
        Side::Top | Side::Bottom => [preferred, opposite(preferred), Side::Right, Side::Left],
        Side::Left | Side::Right => [preferred, opposite(preferred), Side::Bottom, Side::Top],
    }
}

fn position(request: PlacementRequest, side: Side) -> Point {
    let anchor = request.anchor;
    let viewport = request.viewport;
    let overlay = request.overlay;
    match side {
        Side::Top => Point {
            x: clamp(
                aligned(anchor.x, anchor.width, overlay.width, request.alignment),
                viewport.x,
                viewport.width,
                overlay.width,
            ),
            y: anchor.y - request.gap - overlay.height,
        },
        Side::Bottom => Point {
            x: clamp(
                aligned(anchor.x, anchor.width, overlay.width, request.alignment),
                viewport.x,
                viewport.width,
                overlay.width,
            ),
            y: anchor.y + anchor.height + request.gap,
        },
        Side::Left => Point {
            x: anchor.x - request.gap - overlay.width,
            y: clamp(
                aligned(anchor.y, anchor.height, overlay.height, request.alignment),
                viewport.y,
                viewport.height,
                overlay.height,
            ),
        },
        Side::Right => Point {
            x: anchor.x + anchor.width + request.gap,
            y: clamp(
                aligned(anchor.y, anchor.height, overlay.height, request.alignment),
                viewport.y,
                viewport.height,
                overlay.height,
            ),
        },
    }
}

fn overflow(request: PlacementRequest, origin: Point) -> f32 {
    let viewport = request.viewport;
    let overlay = request.overlay;
    (viewport.x - origin.x).max(0.0)
        + (viewport.y - origin.y).max(0.0)
        + (origin.x + overlay.width - viewport.x - viewport.width).max(0.0)
        + (origin.y + overlay.height - viewport.y - viewport.height).max(0.0)
}

/// Chooses the first fitting edge, or the edge with the least overflow.
///
/// The preferred edge, its opposite, then both perpendicular edges are tried
/// in order. Cross-axis coordinates shift into the viewport. When the overlay
/// cannot fit, its origin is clamped to the viewport start on the overflowing
/// axis and [`Placement::fully_visible`] is false. Invalid geometry or an
/// anchor fully outside the viewport returns `None`, which a host can treat
/// as a dismissal or an unplaced state.
#[must_use]
pub fn place_overlay(request: PlacementRequest) -> Option<Placement> {
    if !request.anchor.valid()
        || !request.viewport.valid()
        || !request.overlay.width.is_finite()
        || !request.overlay.height.is_finite()
        || request.overlay.width <= 0.0
        || request.overlay.height <= 0.0
        || !request.gap.is_finite()
        || request.gap < 0.0
    {
        return None;
    }
    if request.anchor.x >= request.viewport.x + request.viewport.width
        || request.anchor.y >= request.viewport.y + request.viewport.height
        || request.anchor.x + request.anchor.width <= request.viewport.x
        || request.anchor.y + request.anchor.height <= request.viewport.y
    {
        return None;
    }

    let mut chosen = None;
    let mut best_overflow = f32::INFINITY;
    for side in candidates(request.preferred_side) {
        let origin = position(request, side);
        let amount = overflow(request, origin);
        if amount < best_overflow {
            chosen = Some((side, origin));
            best_overflow = amount;
        }
        if amount == 0.0 {
            break;
        }
    }
    let (side, origin) = chosen?;
    let origin = Point {
        x: clamp(
            origin.x,
            request.viewport.x,
            request.viewport.width,
            request.overlay.width,
        ),
        y: clamp(
            origin.y,
            request.viewport.y,
            request.viewport.height,
            request.overlay.height,
        ),
    };
    Some(Placement {
        origin,
        side,
        fully_visible: overflow(request, origin) == 0.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> PlacementRequest {
        PlacementRequest {
            anchor: Rect {
                x: 160.0,
                y: 80.0,
                width: 40.0,
                height: 20.0,
            },
            viewport: Rect {
                x: 10.0,
                y: 10.0,
                width: 300.0,
                height: 200.0,
            },
            overlay: Size {
                width: 100.0,
                height: 60.0,
            },
            preferred_side: Side::Bottom,
            alignment: Alignment::Center,
            gap: 8.0,
        }
    }

    #[test]
    fn placement_preserves_preference_and_alignment_when_it_fits() {
        let result = place_overlay(request()).unwrap();
        assert_eq!(result.side, Side::Bottom);
        assert_eq!(result.origin, Point { x: 130.0, y: 108.0 });
        assert!(result.fully_visible);
    }

    #[test]
    fn collision_flips_then_shifts_within_nonzero_viewport() {
        let mut input = request();
        input.anchor.x = 290.0;
        input.anchor.y = 175.0;
        let result = place_overlay(input).unwrap();
        assert_eq!(result.side, Side::Top);
        assert_eq!(result.origin, Point { x: 210.0, y: 107.0 });
        assert!(result.fully_visible);
    }

    #[test]
    fn perpendicular_fallback_uses_available_space() {
        let mut input = request();
        input.anchor.y = 87.0;
        input.overlay.height = 100.0;
        input.viewport.height = 130.0;
        let result = place_overlay(input).unwrap();
        assert_eq!(result.side, Side::Right);
        assert_eq!(result.origin, Point { x: 208.0, y: 40.0 });
        assert!(result.fully_visible);
    }

    #[test]
    fn oversized_overlay_reports_overflow_and_invalid_anchor_is_rejected() {
        let mut input = request();
        input.overlay.width = 400.0;
        input.overlay.height = 300.0;
        let result = place_overlay(input).unwrap();
        assert_eq!(
            result.origin,
            Point {
                x: input.viewport.x,
                y: input.viewport.y,
            }
        );
        assert!(!result.fully_visible);

        input.anchor.width = 0.0;
        assert_eq!(place_overlay(input), None);
        input.anchor.width = f32::NAN;
        assert_eq!(place_overlay(input), None);
    }

    #[test]
    fn offscreen_anchor_has_no_placement() {
        let mut input = request();
        input.anchor.x = 310.0;
        assert_eq!(place_overlay(input), None);
        input.anchor.x = 309.0;
        assert!(place_overlay(input).is_some());
    }
}
