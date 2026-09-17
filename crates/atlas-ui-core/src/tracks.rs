//! Deterministic horizontal track allocation in integer logical pixels.
//!
//! A host can calculate one allocation for a table header and all of its rows,
//! then pass the resulting widths to the Slint presentation. This module does
//! not measure text or change Slint's own layout behavior.

use std::collections::{BTreeMap, BTreeSet};

/// Width constraints for one horizontal track.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrackConstraint {
    /// Preferred width before available space is distributed.
    pub preferred: u32,
    /// Smallest permitted width.
    pub minimum: u32,
    /// Largest permitted width.
    pub maximum: u32,
    /// Relative share of remaining space; zero prevents growth.
    pub grow: u32,
}

/// Allocated widths and left edges in integer logical pixels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrackAllocation {
    /// One width per input track, in the same order.
    pub widths: Vec<u32>,
    /// Left edge of each track, including the leading padding.
    pub offsets: Vec<u32>,
    /// Actual occupied width, including both padding edges and gaps.
    pub content_width: u32,
    /// True when the occupied width exceeds the requested viewport width.
    pub overflow: bool,
    /// Unused viewport width when finite maximums or zero growth prevent fill.
    pub unused_width: u32,
}

/// Invalid inputs to [`allocate_tracks`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrackAllocationError {
    /// A track has a minimum wider than its maximum.
    InvalidConstraint(usize),
    /// The requested geometry cannot be represented in `u32` logical pixels.
    ArithmeticOverflow,
}

/// Host-owned width overrides keyed by stable column identity.
///
/// A resize callback updates this state, then the host applies it to the next
/// allocation. Reordering columns does not move an override to another ID.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TrackWidthOverrides<Id: Ord> {
    widths: BTreeMap<Id, u32>,
}

impl<Id: Ord + Clone> TrackWidthOverrides<Id> {
    /// Creates an empty override map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            widths: BTreeMap::new(),
        }
    }

    /// Stores a requested width after clamping it to the track's constraints.
    ///
    /// # Errors
    /// Returns [`TrackAllocationError::InvalidConstraint`] when the range is
    /// inverted. The index is zero because this method handles one track.
    pub fn resize(
        &mut self,
        id: Id,
        requested: u32,
        track: TrackConstraint,
    ) -> Result<u32, TrackAllocationError> {
        if track.minimum > track.maximum {
            return Err(TrackAllocationError::InvalidConstraint(0));
        }
        let width = requested.clamp(track.minimum, track.maximum);
        self.widths.insert(id, width);
        Ok(width)
    }

    /// Applies the host's preferred width for an ID, if present.
    #[must_use]
    pub fn apply(&self, id: &Id, mut track: TrackConstraint) -> TrackConstraint {
        if let Some(&width) = self.widths.get(id) {
            track.preferred = width;
        }
        track
    }

    /// Removes overrides for columns absent from the complete column model.
    pub fn retain_existing(&mut self, ids: impl IntoIterator<Item = Id>) {
        let existing: BTreeSet<Id> = ids.into_iter().collect();
        self.widths.retain(|id, _| existing.contains(id));
    }
}

fn grow(widths: &mut [u64], tracks: &[TrackConstraint], mut remaining: u64) {
    while remaining > 0 {
        let eligible: Vec<usize> = tracks
            .iter()
            .enumerate()
            .filter_map(|(index, track)| {
                (track.grow > 0 && widths[index] < u64::from(track.maximum)).then_some(index)
            })
            .collect();
        if eligible.is_empty() {
            break;
        }
        let total_weight: u128 = eligible
            .iter()
            .map(|&index| u128::from(tracks[index].grow))
            .sum();
        let mut consumed = 0;
        for &index in &eligible {
            let capacity = u64::from(tracks[index].maximum) - widths[index];
            let share = u64::try_from(
                u128::from(remaining) * u128::from(tracks[index].grow) / total_weight,
            )
            .expect("weighted share cannot exceed remaining u64 pixels");
            let granted = share.min(capacity);
            widths[index] += granted;
            consumed += granted;
        }
        remaining -= consumed;
        if consumed == 0 {
            // Give indivisible pixels to the trailing eligible tracks so the
            // same input always has the same remainder placement.
            for &index in eligible.iter().rev() {
                if remaining == 0 {
                    break;
                }
                let granted = remaining.min(u64::from(tracks[index].maximum) - widths[index]);
                widths[index] += granted;
                remaining -= granted;
            }
        }
    }
}

fn shrink(widths: &mut [u64], tracks: &[TrackConstraint], mut deficit: u64) {
    while deficit > 0 {
        let eligible: Vec<usize> = tracks
            .iter()
            .enumerate()
            .filter_map(|(index, track)| {
                (widths[index] > u64::from(track.minimum)).then_some(index)
            })
            .collect();
        if eligible.is_empty() {
            break;
        }
        let total_capacity: u64 = eligible
            .iter()
            .map(|&index| widths[index] - u64::from(tracks[index].minimum))
            .sum();
        let mut consumed = 0;
        for &index in &eligible {
            let capacity = widths[index] - u64::from(tracks[index].minimum);
            let share = u64::try_from(
                u128::from(deficit) * u128::from(capacity) / u128::from(total_capacity),
            )
            .expect("capacity share cannot exceed the u64 deficit");
            let removed = share.min(capacity);
            widths[index] -= removed;
            consumed += removed;
        }
        deficit -= consumed;
        if consumed == 0 {
            for &index in eligible.iter().rev() {
                if deficit == 0 {
                    break;
                }
                let removed = deficit.min(widths[index] - u64::from(tracks[index].minimum));
                widths[index] -= removed;
                deficit -= removed;
            }
        }
    }
}

/// Allocates tracks within a viewport, respecting minimum, maximum and growth.
///
/// Padding is applied at both edges. Minimum overflow is reported instead of
/// shrinking a track below its minimum. A maximum-bound layout may leave unused
/// viewport space; `content_width` always reports the width actually occupied.
/// All widths and offsets are integer logical pixels, with deterministic
/// trailing-track remainder assignment.
///
/// # Errors
/// Returns [`TrackAllocationError::InvalidConstraint`] for an inverted range,
/// or [`TrackAllocationError::ArithmeticOverflow`] if the result exceeds `u32`.
pub fn allocate_tracks(
    viewport_width: u32,
    horizontal_padding: u32,
    gap: u32,
    tracks: &[TrackConstraint],
) -> Result<TrackAllocation, TrackAllocationError> {
    for (index, track) in tracks.iter().enumerate() {
        if track.minimum > track.maximum {
            return Err(TrackAllocationError::InvalidConstraint(index));
        }
    }
    let gap_count = u64::try_from(tracks.len().saturating_sub(1))
        .map_err(|_| TrackAllocationError::ArithmeticOverflow)?;
    let chrome = u64::from(horizontal_padding)
        .checked_mul(2)
        .and_then(|padding| {
            u64::from(gap)
                .checked_mul(gap_count)
                .and_then(|gaps| padding.checked_add(gaps))
        })
        .ok_or(TrackAllocationError::ArithmeticOverflow)?;
    let minimum_total = tracks.iter().try_fold(0u64, |sum, track| {
        sum.checked_add(u64::from(track.minimum))
            .ok_or(TrackAllocationError::ArithmeticOverflow)
    })?;
    if chrome
        .checked_add(minimum_total)
        .is_none_or(|sum| sum > u64::from(u32::MAX))
    {
        return Err(TrackAllocationError::ArithmeticOverflow);
    }
    let available = u64::from(viewport_width).saturating_sub(chrome);
    let target = available.max(minimum_total);
    let mut widths: Vec<u64> = tracks
        .iter()
        .map(|track| u64::from(track.preferred.clamp(track.minimum, track.maximum)))
        .collect();
    let preferred_total = widths.iter().try_fold(0u64, |sum, width| {
        sum.checked_add(*width)
            .ok_or(TrackAllocationError::ArithmeticOverflow)
    })?;
    if preferred_total < target {
        grow(&mut widths, tracks, target - preferred_total);
    } else if preferred_total > target {
        shrink(&mut widths, tracks, preferred_total - target);
    }

    let mut cursor = u64::from(horizontal_padding);
    let mut offsets = Vec::with_capacity(widths.len());
    for width in &widths {
        offsets.push(u32::try_from(cursor).map_err(|_| TrackAllocationError::ArithmeticOverflow)?);
        cursor += width + u64::from(gap);
    }
    let content_width = chrome
        .checked_add(widths.iter().sum::<u64>())
        .ok_or(TrackAllocationError::ArithmeticOverflow)?;
    let content_width =
        u32::try_from(content_width).map_err(|_| TrackAllocationError::ArithmeticOverflow)?;
    let widths = widths
        .into_iter()
        .map(|width| u32::try_from(width).map_err(|_| TrackAllocationError::ArithmeticOverflow))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TrackAllocation {
        widths,
        offsets,
        content_width,
        overflow: content_width > viewport_width,
        unused_width: viewport_width.saturating_sub(content_width),
    })
}

#[cfg(test)]
mod override_tests {
    use super::{TrackConstraint, TrackWidthOverrides};

    #[test]
    fn overrides_follow_column_ids_through_reorder_and_removal() {
        let mut state = TrackWidthOverrides::new();
        let track = TrackConstraint {
            preferred: 180,
            minimum: 120,
            maximum: 300,
            grow: 1,
        };
        assert_eq!(state.resize("name", 500, track).unwrap(), 300);
        assert_eq!(state.apply(&"status", track).preferred, 180);
        assert_eq!(state.apply(&"name", track).preferred, 300);
        state.retain_existing(["status", "name"]);
        assert_eq!(state.apply(&"name", track).preferred, 300);
        state.retain_existing(["status"]);
        assert_eq!(state.apply(&"name", track).preferred, 180);
    }
}
