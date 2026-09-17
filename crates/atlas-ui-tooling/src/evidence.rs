//! Reproducible coverage of public components in live Slint runtime fixtures.

use crate::{Result, util};
use regex::Regex;
use std::{collections::BTreeSet, fs, path::Path};

const PUBLISHED_COMPONENTS: usize = 97;
const PUBLISHED_RUNTIME_COVERAGE: usize = 0;
const ADDED_COMPONENTS: [&str; 6] = [
    "AtlasAutocomplete",
    "AtlasCombobox",
    "AtlasContainer",
    "AtlasPopover",
    "AtlasRadioGroup",
    "AtlasTreeView",
];

struct EvidenceCounts {
    existing: usize,
    existing_events: usize,
    new: usize,
    new_events: usize,
    covered: usize,
    event_covered: usize,
    total: usize,
    added: usize,
}

pub fn run(root: &Path, check: bool) -> Result {
    let manifest = util::read_json(&root.join("docs/atlas-ui-agent-manifest.json"))?;
    let components: BTreeSet<String> = ["stable_components", "preview_components"]
        .into_iter()
        .flat_map(|key| manifest[key].as_array().into_iter().flatten())
        .filter_map(|entry| entry.as_str().map(str::to_owned))
        .collect();
    let added: BTreeSet<&str> = ADDED_COMPONENTS.into_iter().collect();
    if components.len() != PUBLISHED_COMPONENTS + added.len()
        || !added.iter().all(|name| components.contains(*name))
    {
        return Err("component evidence baseline needs review after an API change".into());
    }

    let source = fs::read_to_string(root.join("crates/atlas-ui-testing/ui/layout-runtime.slint"))?;
    let fixture_pattern = Regex::new(r"(?m)^export component (\w+RuntimeFixture)\b")?;
    let fixtures: Vec<_> = fixture_pattern.captures_iter(&source).collect();
    let tests_dir = root.join("crates/atlas-ui-testing/tests");
    let test_sources: Vec<String> = fs::read_dir(tests_dir)?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with("_runtime.rs"))
        .map(|entry| fs::read_to_string(entry.path()))
        .collect::<std::io::Result<_>>()?;
    let mut covered = BTreeSet::new();
    let mut event_covered = BTreeSet::new();

    for (index, captures) in fixtures.iter().enumerate() {
        let whole = captures.get(0).ok_or("fixture declaration missing")?;
        let name = captures.get(1).ok_or("fixture name missing")?.as_str();
        let end = fixtures
            .get(index + 1)
            .and_then(|next| next.get(0))
            .map_or(source.len(), |next| next.start());
        let body = &source[whole.end()..end];
        let matching_tests: Vec<_> = test_sources
            .iter()
            .filter(|test| test.contains("#[test]") && test.contains(&format!("{name}::new()")))
            .collect();
        if matching_tests.is_empty() {
            continue;
        }
        let dispatches_events = matching_tests
            .iter()
            .any(|test| test.contains("dispatch_event("));
        for component in &components {
            let instance = Regex::new(&format!(r"\b{}\s*\{{", regex::escape(component)))?;
            if instance.is_match(body) {
                covered.insert(component.as_str());
                if dispatches_events {
                    event_covered.insert(component.as_str());
                }
            }
        }
    }

    let existing = covered
        .iter()
        .filter(|name| !added.contains(**name))
        .count();
    let new = covered.len() - existing;
    let existing_events = event_covered
        .iter()
        .filter(|name| !added.contains(**name))
        .count();
    let new_events = event_covered.len() - existing_events;
    let summary = format!(
        "- direct Slint runtime fixture coverage: {}/{} public components, including {}/{} from v0.1.1 ({} at the v0.1.1 tag) and {}/{} newly added preview components;",
        covered.len(),
        components.len(),
        existing,
        PUBLISHED_COMPONENTS,
        PUBLISHED_RUNTIME_COVERAGE,
        new,
        added.len()
    );
    let event_summary = format!(
        "- Slint event-dispatch fixture coverage: {}/{} public components, including {}/{} from v0.1.1 ({} at the v0.1.1 tag) and {}/{} newly added preview components;",
        event_covered.len(),
        components.len(),
        existing_events,
        PUBLISHED_COMPONENTS,
        PUBLISHED_RUNTIME_COVERAGE,
        new_events,
        added.len()
    );
    if check {
        check_table(
            root,
            &EvidenceCounts {
                existing,
                existing_events,
                new,
                new_events,
                covered: covered.len(),
                event_covered: event_covered.len(),
                total: components.len(),
                added: added.len(),
            },
        )?;
    }
    println!("{summary}");
    println!("{event_summary}");
    Ok(())
}

fn check_table(root: &Path, counts: &EvidenceCounts) -> Result {
    let evidence = fs::read_to_string(root.join("docs/COMPONENT_EVIDENCE.md"))?;
    let rows = [
        (
            "Previously published components with direct runtime fixtures",
            Some(PUBLISHED_RUNTIME_COVERAGE),
            counts.existing,
            PUBLISHED_COMPONENTS,
        ),
        (
            "Previously published components in event-dispatch fixtures",
            Some(PUBLISHED_RUNTIME_COVERAGE),
            counts.existing_events,
            PUBLISHED_COMPONENTS,
        ),
        (
            "Newly added preview components with direct runtime fixtures",
            None,
            counts.new,
            counts.added,
        ),
        (
            "Newly added preview components in event-dispatch fixtures",
            None,
            counts.new_events,
            counts.added,
        ),
        (
            "All public components with direct runtime fixtures",
            Some(PUBLISHED_RUNTIME_COVERAGE),
            counts.covered,
            counts.total,
        ),
        (
            "All public components in event-dispatch fixtures",
            Some(PUBLISHED_RUNTIME_COVERAGE),
            counts.event_covered,
            counts.total,
        ),
    ];
    if rows.iter().any(|(label, published, current, total)| {
        let old = published.map_or_else(
            || "—".to_owned(),
            |count| format!("{count}/{PUBLISHED_COMPONENTS}"),
        );
        !evidence.contains(&format!("| {label} | {old} | {current}/{total} |"))
    }) {
        return Err("component evidence table is stale".into());
    }
    Ok(())
}
