//! Reusable validation helpers for Atlas UI tests and capture tooling.

use std::{
    hint::black_box,
    time::{Duration, Instant},
};

/// Reproducible protocol for a local performance assertion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerformanceBudget {
    /// Untimed iterations used to warm caches and allocator paths.
    pub warmup_iterations: usize,
    /// Number of timed samples used to calculate distribution statistics.
    pub sample_count: usize,
    /// Stable regression threshold applied to the sample median.
    pub median_limit: Duration,
}

/// Distribution summary produced by [`measure_performance`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerformanceSummary {
    /// Fastest observed sample.
    pub minimum: Duration,
    /// Median sample used as the pass/fail signal.
    pub median: Duration,
    /// Nearest-rank 95th percentile retained as an outlier diagnostic.
    pub p95: Duration,
    /// Slowest observed sample.
    pub maximum: Duration,
    /// Threshold applied to `median`.
    pub median_limit: Duration,
    /// Number of timed samples in the distribution.
    pub sample_count: usize,
}

impl PerformanceSummary {
    /// Returns whether the stable median remains within its declared budget.
    #[must_use]
    pub fn is_within_budget(self) -> bool {
        self.median < self.median_limit
    }
}

/// Measures an operation after untimed preparation and warm-up.
///
/// Preparation is deliberately outside the timer so callers can rebuild or
/// clone deterministic inputs without silently changing the operation budget.
/// The median is the regression signal; p95 and extrema remain diagnostics.
///
/// # Panics
///
/// Panics when `sample_count` is zero because no distribution can be produced.
pub fn measure_performance<Input, Output, Prepare, Operation>(
    budget: PerformanceBudget,
    mut prepare: Prepare,
    mut operation: Operation,
) -> PerformanceSummary
where
    Prepare: FnMut() -> Input,
    Operation: FnMut(Input) -> Output,
{
    assert!(
        budget.sample_count > 0,
        "performance sampling requires at least one sample"
    );

    for _ in 0..budget.warmup_iterations {
        let input = prepare();
        black_box(operation(input));
    }

    let mut samples = Vec::with_capacity(budget.sample_count);
    for _ in 0..budget.sample_count {
        let input = prepare();
        let started = Instant::now();
        black_box(operation(input));
        samples.push(started.elapsed());
    }
    samples.sort_unstable();
    let median = samples[samples.len() / 2];
    let p95_index = (samples.len() * 95).div_ceil(100).saturating_sub(1);

    PerformanceSummary {
        minimum: samples[0],
        median,
        p95: samples[p95_index],
        maximum: samples[samples.len() - 1],
        median_limit: budget.median_limit,
        sample_count: samples.len(),
    }
}

/// Metadata required to compare two visual scenarios meaningfully.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioIdentity<'a> {
    /// Stable scenario identifier.
    pub id: &'a str,
    /// Fixture identifier shared by reference and result.
    pub fixture: &'a str,
    /// Theme identifier.
    pub theme: &'a str,
    /// Density identifier.
    pub density: &'a str,
    /// Logical viewport width.
    pub width: u32,
    /// Logical viewport height.
    pub height: u32,
}

impl ScenarioIdentity<'_> {
    /// Validates fields needed before a visual diff can be meaningful.
    ///
    /// # Errors
    ///
    /// Returns an error when identifiers are empty or viewport dimensions are
    /// zero.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.id.is_empty() || self.fixture.is_empty() {
            return Err("scenario and fixture identifiers must not be empty");
        }
        if self.width == 0 || self.height == 0 {
            return Err("viewport dimensions must be non-zero");
        }
        Ok(())
    }
}

/// Creates stable row identifiers for large-model and virtualization fixtures.
#[must_use]
pub fn large_row_fixture(count: usize) -> Vec<String> {
    (0..count).map(|index| format!("row-{index:08}")).collect()
}

#[cfg(test)]
mod tests {
    use super::{PerformanceBudget, ScenarioIdentity, large_row_fixture, measure_performance};
    use std::time::Duration;

    #[test]
    fn rejects_an_empty_visual_fixture() {
        let scenario = ScenarioIdentity {
            id: "button/default",
            fixture: "",
            theme: "dark",
            density: "normal",
            width: 320,
            height: 180,
        };
        assert!(scenario.validate().is_err());
    }

    #[test]
    fn builds_ten_thousand_stable_rows_within_the_debug_budget() {
        let summary = measure_performance(
            PerformanceBudget {
                warmup_iterations: 1,
                sample_count: 7,
                median_limit: Duration::from_millis(100),
            },
            || 10_000,
            large_row_fixture,
        );
        eprintln!("atlas-performance row-fixture-10000 {summary:?}");
        assert!(
            summary.is_within_budget(),
            "row fixture performance: {summary:?}"
        );
        let rows = large_row_fixture(10_000);
        assert_eq!(rows.first().map(String::as_str), Some("row-00000000"));
        assert_eq!(rows.last().map(String::as_str), Some("row-00009999"));
    }

    #[test]
    fn performance_summary_reports_an_ordered_distribution() {
        let summary = measure_performance(
            PerformanceBudget {
                warmup_iterations: 1,
                sample_count: 5,
                median_limit: Duration::from_secs(1),
            },
            || (),
            |()| 42,
        );
        assert_eq!(summary.sample_count, 5);
        assert!(summary.minimum <= summary.median);
        assert!(summary.median <= summary.p95);
        assert!(summary.p95 <= summary.maximum);
        assert!(summary.is_within_budget());
    }
}
