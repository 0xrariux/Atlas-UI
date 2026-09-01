//! Deterministic geometry oracle for the shared Slint data-table track contract.

#[derive(Clone, Copy, Debug)]
struct ColumnConstraint {
    preferred: u32,
    minimum: u32,
    maximum: u32,
    grow: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Allocation {
    widths: Vec<u32>,
    offsets: Vec<u32>,
    content_width: u32,
    overflow: bool,
}

fn distribute_growth(widths: &mut [u32], columns: &[ColumnConstraint], mut remaining: u32) {
    while remaining > 0 {
        let eligible: Vec<usize> = columns
            .iter()
            .enumerate()
            .filter_map(|(index, column)| {
                (column.grow > 0 && widths[index] < column.maximum).then_some(index)
            })
            .collect();
        if eligible.is_empty() {
            break;
        }
        let total_grow: u32 = eligible.iter().map(|&index| columns[index].grow).sum();
        let pass = remaining;
        let mut consumed = 0;
        for &index in &eligible {
            let share = pass.saturating_mul(columns[index].grow) / total_grow;
            let granted = share.min(columns[index].maximum - widths[index]);
            widths[index] += granted;
            consumed += granted;
        }
        remaining -= consumed;
        if consumed == 0 || remaining > 0 {
            for &index in eligible.iter().rev() {
                let capacity = columns[index].maximum - widths[index];
                let granted = capacity.min(remaining);
                widths[index] += granted;
                remaining -= granted;
                if remaining == 0 {
                    break;
                }
            }
        }
        if consumed == 0 && remaining > 0 {
            break;
        }
    }
}

fn distribute_shrink(widths: &mut [u32], columns: &[ColumnConstraint], mut deficit: u32) {
    while deficit > 0 {
        let eligible: Vec<usize> = columns
            .iter()
            .enumerate()
            .filter_map(|(index, column)| (widths[index] > column.minimum).then_some(index))
            .collect();
        if eligible.is_empty() {
            break;
        }
        let total_capacity: u32 = eligible
            .iter()
            .map(|&index| widths[index] - columns[index].minimum)
            .sum();
        let pass = deficit;
        let mut consumed = 0;
        for &index in &eligible {
            let capacity = widths[index] - columns[index].minimum;
            let share = pass.saturating_mul(capacity) / total_capacity;
            let removed = share.min(capacity);
            widths[index] -= removed;
            consumed += removed;
        }
        deficit -= consumed;
        if consumed == 0 || deficit > 0 {
            for &index in eligible.iter().rev() {
                let capacity = widths[index] - columns[index].minimum;
                let removed = capacity.min(deficit);
                widths[index] -= removed;
                deficit -= removed;
                if deficit == 0 {
                    break;
                }
            }
        }
        if consumed == 0 && deficit > 0 {
            break;
        }
    }
}

fn allocate(
    viewport_width: u32,
    horizontal_padding: u32,
    column_gap: u32,
    columns: &[ColumnConstraint],
) -> Allocation {
    let gap_width = column_gap * u32::try_from(columns.len().saturating_sub(1)).unwrap();
    let chrome = horizontal_padding * 2 + gap_width;
    let available = viewport_width.saturating_sub(chrome);
    let minimum_total: u32 = columns.iter().map(|column| column.minimum).sum();
    let overflow = minimum_total > available;
    let target = if overflow { minimum_total } else { available };
    let mut widths: Vec<u32> = columns
        .iter()
        .map(|column| column.preferred.clamp(column.minimum, column.maximum))
        .collect();
    let preferred_total: u32 = widths.iter().sum();
    if preferred_total < target {
        distribute_growth(&mut widths, columns, target - preferred_total);
    } else if preferred_total > target {
        distribute_shrink(&mut widths, columns, preferred_total - target);
    }

    let mut cursor = horizontal_padding;
    let offsets = widths
        .iter()
        .map(|width| {
            let offset = cursor;
            cursor += width + column_gap;
            offset
        })
        .collect();
    Allocation {
        widths,
        offsets,
        content_width: target + chrome,
        overflow,
    }
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
