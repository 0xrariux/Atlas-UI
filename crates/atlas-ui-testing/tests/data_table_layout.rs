//! Deterministic geometry contract for the public Atlas track allocator.

use atlas_ui_core::tracks::{
    TrackAllocation, TrackAllocationError, TrackConstraint, allocate_tracks,
};

type ColumnConstraint = TrackConstraint;
type Allocation = TrackAllocation;

fn allocate(
    viewport_width: u32,
    horizontal_padding: u32,
    column_gap: u32,
    columns: &[ColumnConstraint],
) -> Allocation {
    allocate_tracks(viewport_width, horizontal_padding, column_gap, columns)
        .expect("valid data-table constraints")
}

fn access_matrix_columns() -> [ColumnConstraint; 5] {
    [
        ColumnConstraint {
            preferred: 190,
            minimum: 160,
            maximum: 1000,
            grow: 19,
        },
        ColumnConstraint {
            preferred: 160,
            minimum: 140,
            maximum: 1000,
            grow: 16,
        },
        ColumnConstraint {
            preferred: 240,
            minimum: 220,
            maximum: 1200,
            grow: 24,
        },
        ColumnConstraint {
            preferred: 290,
            minimum: 240,
            maximum: 1400,
            grow: 29,
        },
        ColumnConstraint {
            preferred: 120,
            minimum: 110,
            maximum: 700,
            grow: 12,
        },
    ]
}

fn assert_no_overlap(allocation: &Allocation, padding: u32, gap: u32) {
    assert_eq!(allocation.offsets.first(), Some(&padding));
    for index in 1..allocation.widths.len() {
        assert_eq!(
            allocation.offsets[index],
            allocation.offsets[index - 1] + allocation.widths[index - 1] + gap
        );
    }
    let occupied = padding * 2
        + allocation.widths.iter().sum::<u32>()
        + gap * u32::try_from(allocation.widths.len().saturating_sub(1)).unwrap();
    assert_eq!(occupied, allocation.content_width);
}

#[test]
fn header_and_rows_share_identical_effective_tracks() {
    let columns = access_matrix_columns();
    let header = allocate(1_376, 12, 8, &columns);
    let row = allocate(1_376, 12, 8, &columns);
    assert_eq!(header, row);
}

#[test]
fn preferred_minimum_maximum_and_grow_are_honored() {
    let columns = [
        ColumnConstraint {
            preferred: 100,
            minimum: 80,
            maximum: 110,
            grow: 1,
        },
        ColumnConstraint {
            preferred: 100,
            minimum: 90,
            maximum: 300,
            grow: 1,
        },
    ];
    let allocation = allocate(266, 8, 4, &columns);
    assert_eq!(allocation.widths, [110, 136]);
    assert!(!allocation.overflow);

    let constrained = allocate(196, 8, 4, &columns);
    assert_eq!(constrained.widths.iter().sum::<u32>(), 176);
    assert!(constrained.widths[0] >= 80);
    assert!(constrained.widths[1] >= 90);
}

#[test]
fn final_column_absorbs_the_integer_remainder_deterministically() {
    let columns = access_matrix_columns();
    let first = allocate(1_377, 12, 8, &columns);
    let second = allocate(1_377, 12, 8, &columns);
    assert_eq!(first, second);
    assert_eq!(first.content_width, 1_377);
    assert_no_overlap(&first, 12, 8);
}

#[test]
fn constrained_width_activates_overflow_at_the_sum_of_minimums() {
    let columns = access_matrix_columns();
    let allocation = allocate(800, 12, 8, &columns);
    assert!(allocation.overflow);
    assert_eq!(allocation.widths, [160, 140, 220, 240, 110]);
    assert_eq!(allocation.content_width, 926);
    assert_no_overlap(&allocation, 12, 8);
}

#[test]
fn access_matrix_has_zero_overlap_at_all_acceptance_widths() {
    let columns = access_matrix_columns();
    for viewport in [960, 1_376, 2_816] {
        let allocation = allocate(viewport, 12, 8, &columns);
        assert!(!allocation.overflow, "unexpected overflow at {viewport}px");
        assert_eq!(allocation.content_width, viewport);
        assert_no_overlap(&allocation, 12, 8);
    }
}

#[test]
fn capped_tracks_report_unused_space_instead_of_fabricating_content_width() {
    let columns = [ColumnConstraint {
        preferred: 90,
        minimum: 80,
        maximum: 100,
        grow: 1,
    }];
    let allocation = allocate(300, 10, 0, &columns);
    assert_eq!(allocation.widths, [100]);
    assert_eq!(allocation.content_width, 120);
    assert_eq!(allocation.unused_width, 180);
    assert!(!allocation.overflow);
}

#[test]
fn invalid_and_unrepresentable_constraints_return_errors() {
    let inverted = [ColumnConstraint {
        preferred: 50,
        minimum: 100,
        maximum: 80,
        grow: 1,
    }];
    assert_eq!(
        allocate_tracks(300, 0, 0, &inverted),
        Err(TrackAllocationError::InvalidConstraint(0))
    );
    let huge = [ColumnConstraint {
        preferred: u32::MAX,
        minimum: u32::MAX,
        maximum: u32::MAX,
        grow: 0,
    }];
    assert_eq!(
        allocate_tracks(u32::MAX, 1, 0, &huge),
        Err(TrackAllocationError::ArithmeticOverflow)
    );
}
