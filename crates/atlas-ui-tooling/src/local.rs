use crate::{Result, util};
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fs, path::Path};

fn require(condition: bool, message: impl Into<String>) -> Result {
    if condition {
        Ok(())
    } else {
        Err(message.into().into())
    }
}
fn array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>> {
    value[key]
        .as_array()
        .ok_or_else(|| format!("missing array: {key}").into())
}
fn unique(values: &[Value], key: &str, context: &str) -> Result<HashSet<String>> {
    let mut ids = HashSet::new();
    for value in values {
        let id = value[key]
            .as_str()
            .ok_or_else(|| format!("{context} item without {key}"))?;
        require(
            ids.insert(id.to_owned()),
            format!("duplicate {context}: {id}"),
        )?;
    }
    Ok(ids)
}
fn workspace_versions(root: &Path, compatibility: &Value) -> Result {
    let cargo = fs::read_to_string(root.join("Cargo.toml"))?;
    let pin = compatibility["upstream"]["pinned_version"]
        .as_str()
        .ok_or("missing Slint pin")?;
    require(
        cargo.contains(&format!("slint = \"={pin}\"")),
        "Slint dependency drift",
    )?;
    require(
        cargo.contains(&format!("slint-build = \"={pin}\"")),
        "slint-build dependency drift",
    )?;
    let rust = compatibility["atlas"]["rust_version"]
        .as_str()
        .ok_or("missing Rust version")?;
    require(
        cargo.contains(&format!("rust-version = \"{rust}\"")),
        "Rust version drift",
    )?;
    Ok(())
}
fn compatibility(root: &Path) -> Result {
    let data = util::read_json(&root.join("ai/slint-compatibility.json"))?;
    require(data["schema_version"] == 1, "invalid compatibility schema")?;
    workspace_versions(root, &data)?;
    let caps = array(&data, "capabilities")?;
    unique(caps, "id", "capability")?;
    for cap in caps {
        let status = cap["upstream_status"].as_str().unwrap_or_default();
        let action = cap["atlas_action"].as_str().unwrap_or_default();
        require(
            ["stable", "experimental", "limited"].contains(&status),
            "invalid upstream status",
        )?;
        require(
            ["use", "wrap", "implement", "monitor"].contains(&action),
            "invalid Atlas action",
        )?;
        require(
            !array(cap, "atlas_targets")?.is_empty(),
            "capability without targets",
        )?;
        if status == "experimental" {
            require(
                ["wrap", "monitor"].contains(&action),
                "experimental capability must be wrapped or monitored",
            )?;
        }
    }
    require(
        !array(&data, "watchlist")?.is_empty(),
        "empty Slint watchlist",
    )?;
    Ok(())
}
fn visual_scenarios(root: &Path) -> Result<HashSet<String>> {
    let data = util::read_json(&root.join("screenshots/scenarios.json"))?;
    require(data["schema_version"] == 1, "invalid scenario schema")?;
    let scenarios = array(&data, "scenarios")?;
    let ids = unique(scenarios, "id", "scenario")?;
    let id_pattern = Regex::new(r"^[a-z0-9.-]+$")?;
    for s in scenarios {
        require(
            id_pattern.is_match(s["id"].as_str().unwrap()),
            "invalid scenario id",
        )?;
        require(
            ["dark", "light"].contains(&s["theme"].as_str().unwrap_or_default()),
            "invalid scenario theme",
        )?;
        require(
            ["compact", "normal", "comfortable"]
                .contains(&s["density"].as_str().unwrap_or_default()),
            "invalid density",
        )?;
        require(
            s["renderer"] == "software" && s["scale_factor"] == 1,
            "non-deterministic renderer",
        )?;
        require(
            s["viewport"]["width"].as_u64().is_some() && s["viewport"]["height"].as_u64().is_some(),
            "invalid viewport",
        )?;
    }
    Ok(ids)
}
fn tokens(root: &Path) -> Result {
    let base = root.join("crates/atlas-ui-tokens");
    let registry = util::read_json(&base.join("tokens.registry.json"))?;
    let facade =
        fs::read_to_string(base.join(registry["facade"].as_str().ok_or("token facade missing")?))?;
    let families = array(&registry, "families")?;
    unique(families, "id", "token family")?;
    for family in families {
        let file = family["file"].as_str().unwrap();
        let source = fs::read_to_string(base.join(file))?;
        for symbol in array(family, "exports")?.iter().filter_map(Value::as_str) {
            require(
                source.contains(symbol),
                format!("{file} does not declare {symbol}"),
            )?;
            require(
                facade.contains(symbol),
                format!("token facade does not export {symbol}"),
            )?;
        }
    }
    let literal = Regex::new(r"#[0-9a-fA-F]{3,8}\b|-?\b\d+(?:\.\d+)?px\b")?;
    for relative in array(&registry["literal_policy"], "forbidden_roots")?
        .iter()
        .filter_map(Value::as_str)
    {
        for file in util::files(&root.join(relative))
            .into_iter()
            .filter(|p| p.extension().is_some_and(|e| e == "slint"))
        {
            let source = fs::read_to_string(&file)?;
            require(
                !literal.is_match(&source),
                format!("forbidden visual literal: {}", file.display()),
            )?;
        }
    }
    Ok(())
}
fn public_api(root: &Path) -> Result {
    let snapshot = util::read_json(&root.join("ai/public-api.json"))?;
    let audit = util::read_json(&root.join("ai/api-surface-audit.json"))?;
    require(
        snapshot["schema_version"] == 2 && snapshot["api_status"] == "mixed-stability",
        "invalid public API schema",
    )?;
    let mut all = HashSet::new();
    for kind in ["components", "types", "globals"] {
        for symbol in array(&snapshot, kind)?.iter().filter_map(Value::as_str) {
            require(all.insert(symbol.to_owned()), "duplicate public symbol")?;
        }
    }
    let mut stable = HashSet::new();
    let mut preview = HashSet::new();
    for group in array(&audit, "groups")? {
        let target = if group["decision"] == "stable" {
            &mut stable
        } else if group["decision"] == "preview" {
            &mut preview
        } else {
            return Err("unknown API maturity".into());
        };
        for kind in ["components", "types", "globals"] {
            for symbol in array(group, kind)?.iter().filter_map(Value::as_str) {
                target.insert(symbol.to_owned());
            }
        }
    }
    require(
        stable.len() == usize::try_from(audit["summary"]["stable_symbols"].as_u64().unwrap())?,
        "stable API count drift",
    )?;
    require(
        preview.len() == usize::try_from(audit["summary"]["preview_symbols"].as_u64().unwrap())?,
        "preview API count drift",
    )?;
    require(
        stable.len() + preview.len() == all.len(),
        "API partition incomplete",
    )?;
    for (kind, set) in [("stable_facade", stable), ("preview_facade", preview)] {
        let source = fs::read_to_string(root.join(snapshot[kind].as_str().unwrap()))?;
        for symbol in set {
            require(
                Regex::new(&format!(r"\b{}\b", regex::escape(&symbol)))?.is_match(&source),
                format!("facade missing {symbol}"),
            )?;
        }
    }
    Ok(())
}
fn accessibility(root: &Path) -> Result {
    let contrast = util::read_json(&root.join("ai/accessibility-contrast.json"))?;
    require(contrast["schema_version"] == 1, "invalid contrast schema")?;
    for pair in array(&contrast, "pairs")? {
        let ratio = contrast_ratio(
            pair["foreground"].as_str().unwrap(),
            pair["background"].as_str().unwrap(),
        )?;
        require(
            ratio + 0.01 >= pair["minimum"].as_f64().unwrap(),
            format!("contrast failure: {}", pair["id"]),
        )?;
    }
    let contracts = util::read_json(&root.join("ai/accessibility-contracts.json"))?;
    require(
        contracts["manual_review_status"] == "not-required",
        "manual accessibility scope drift",
    )?;
    unique(
        array(&contracts, "contracts")?,
        "id",
        "accessibility contract",
    )?;
    for contract in array(&contracts, "contracts")? {
        let source = fs::read_to_string(root.join(contract["source"].as_str().unwrap()))?;
        for required in array(contract, "required")?
            .iter()
            .filter_map(Value::as_str)
        {
            require(
                source.contains(required),
                format!("accessibility contract missing {required}"),
            )?;
        }
    }
    Ok(())
}
fn contrast_ratio(fg: &str, bg: &str) -> Result<f64> {
    fn luminance(value: &str) -> Result<f64> {
        let raw = value.strip_prefix('#').ok_or("invalid color")?;
        require(raw.len() == 6, "invalid color length")?;
        let channel = |offset| {
            u8::from_str_radix(&raw[offset..offset + 2], 16).map(|v| {
                let c = f64::from(v) / 255.0;
                if c <= 0.04045 {
                    c / 12.92
                } else {
                    ((c + 0.055) / 1.055).powf(2.4)
                }
            })
        };
        Ok(0.2126 * channel(0)? + 0.7152 * channel(2)? + 0.0722 * channel(4)?)
    }
    let a = luminance(fg)?;
    let b = luminance(bg)?;
    Ok((a.max(b) + 0.05) / (a.min(b) + 0.05))
}
fn assets(root: &Path) -> Result {
    for (registry_path, array_key, base) in [
        (
            "crates/atlas-ui-icons/icons.registry.json",
            "icons",
            "crates/atlas-ui-icons",
        ),
        (
            "crates/atlas-ui-tokens/fonts.registry.json",
            "fonts",
            "crates/atlas-ui-tokens",
        ),
    ] {
        let registry = util::read_json(&root.join(registry_path))?;
        let entries = array(&registry, array_key)?;
        unique(entries, "id", array_key)?;
        for entry in entries {
            let path = root.join(base).join(entry["asset"].as_str().unwrap());
            let bytes = fs::read(&path)?;
            let hash = format!("{:x}", Sha256::digest(bytes));
            require(
                hash == entry["sha256"],
                format!("asset hash mismatch: {}", path.display()),
            )?;
            if let Some(license) = entry["license_file"].as_str() {
                require(
                    root.join(base).join(license).exists(),
                    "font license missing",
                )?;
            }
        }
    }
    Ok(())
}
fn performance(root: &Path, scenario_ids: &HashSet<String>) -> Result {
    let data = util::read_json(&root.join("ai/performance-budgets.json"))?;
    let mut ids = HashSet::new();
    for key in ["budgets", "render_budgets"] {
        for budget in array(&data, key)? {
            let id = budget["id"].as_str().unwrap();
            require(ids.insert(id.to_owned()), "duplicate performance budget")?;
            let samples = budget["sample_count"].as_u64().unwrap_or_default();
            require(
                samples >= 5 && samples % 2 == 1,
                "unstable performance sampling",
            )?;
            require(
                budget["median_limit_ms"].as_u64().unwrap_or_default() > 0,
                "invalid performance limit",
            )?;
            if key == "render_budgets" {
                require(
                    scenario_ids.contains(budget["scenario"].as_str().unwrap()),
                    "unknown render scenario",
                )?;
            }
        }
    }
    Ok(())
}
fn legacy(root: &Path) -> Result {
    let audit = util::read_json(&root.join("ai/legacy-audit.json"))?;
    let retirement = util::read_json(&root.join("ai/legacy-retirement.json"))?;
    let required: HashSet<_> = array(&retirement, "required_components")?
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let completed: HashSet<_> = array(&retirement, "completed_components")?
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let excluded: HashSet<_> = array(&retirement, "excluded_components")?
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let expected_required: HashSet<_> = array(&audit, "components")?
        .iter()
        .filter(|v| v["decision"] != "drop")
        .filter_map(|v| v["id"].as_str())
        .collect();
    let expected_excluded: HashSet<_> = array(&audit, "components")?
        .iter()
        .filter(|v| v["decision"] == "drop")
        .filter_map(|v| v["id"].as_str())
        .collect();
    require(required == expected_required, "legacy required set drift")?;
    require(excluded == expected_excluded, "legacy exclusions drift")?;
    let ready = completed == required;
    require(
        retirement["safe_to_delete"] == ready,
        "legacy safe-to-delete drift",
    )?;
    require(
        (retirement["status"] == "ready") == ready,
        "legacy status drift",
    )?;
    Ok(())
}
fn readiness(root: &Path) -> Result {
    let data = util::read_json(&root.join("ai/release-readiness.json"))?;
    let metadata = util::files(&root.join("screenshots/metadata"))
        .into_iter()
        .filter(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".baseline.json"))
        })
        .collect::<Vec<_>>();
    let approved = metadata
        .iter()
        .filter(|p| util::read_json(p).is_ok_and(|m| m["approval"]["status"] == "approved"))
        .count();
    let visual = array(&data, "checks")?
        .iter()
        .find(|c| c["id"] == "visual-baselines")
        .ok_or("visual readiness missing")?;
    require(
        visual["approved"] == approved && visual["required"] == metadata.len(),
        "readiness baseline count drift",
    )?;
    let blockers: Vec<_> = array(&data, "checks")?
        .iter()
        .filter(|c| c["blocking"] == true && c["status"] != "passed")
        .filter_map(|c| c["id"].as_str())
        .collect();
    require(
        blockers.len() == array(&data, "blockers")?.len(),
        "readiness blockers drift",
    )?;
    require(
        data["ready"] == blockers.is_empty(),
        "readiness ready flag drift",
    )?;
    require(
        fs::read_to_string(root.join("LICENSE"))?.starts_with("MIT License\n"),
        "MIT license missing",
    )?;
    Ok(())
}
fn component_compatibility(root: &Path, scenario_ids: &HashSet<String>) -> Result {
    let data = util::read_json(&root.join("ai/component-compatibility.json"))?;
    let aggregate =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/components.slint"))?;
    unique(array(&data, "groups")?, "id", "compatibility group")?;
    for group in array(&data, "groups")? {
        require(
            scenario_ids.contains(group["scenario"].as_str().unwrap()),
            "compatibility scenario missing",
        )?;
        for component in array(group, "components")?.iter().filter_map(Value::as_str) {
            require(
                aggregate.contains(component),
                format!("compatibility component missing: {component}"),
            )?;
        }
    }
    Ok(())
}
pub fn run(root: &Path) -> Result {
    if !root.join("ai").exists() {
        println!("Local ai directory absent; optional validations skipped.");
        return Ok(());
    }
    compatibility(root)?;
    let scenarios = visual_scenarios(root)?;
    tokens(root)?;
    public_api(root)?;
    accessibility(root)?;
    assets(root)?;
    performance(root, &scenarios)?;
    legacy(root)?;
    readiness(root)?;
    component_compatibility(root, &scenarios)?;
    println!(
        "Local Atlas contracts valid: compatibility, tokens, scenarios, API, accessibility, assets, performance, legacy, readiness, and component coverage."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::contrast_ratio;
    #[test]
    fn contrast_uses_wcag_relative_luminance() {
        assert!((contrast_ratio("#000000", "#ffffff").unwrap() - 21.0).abs() < 0.001);
        assert!(contrast_ratio("#777777", "#ffffff").unwrap() > 4.47);
    }
}
