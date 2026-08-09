use crate::{Result, util};
use regex::Regex;
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

fn facade_symbols(source: &str) -> Result<BTreeSet<String>> {
    let exports = Regex::new(r#"(?s)export\s*\{(.*?)\}\s*from\s*\"[^\"]+\"\s*;"#)?;
    Ok(exports
        .captures_iter(source)
        .flat_map(|capture| {
            capture[1]
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect())
}

fn string_set(value: &Value, field: &str) -> Result<BTreeSet<String>> {
    value[field]
        .as_array()
        .ok_or_else(|| format!("manifest field `{field}` is missing"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("manifest field `{field}` contains a non-string").into())
        })
        .collect::<Result<BTreeSet<_>>>()
}

fn require_equal<T: PartialEq + std::fmt::Debug>(actual: &T, expected: &T, label: &str) -> Result {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label} drift: expected {expected:?}, found {actual:?}").into())
    }
}

fn validate_source_declarations(root: &Path, manifest: &Value) -> Result {
    let stable_path = root.join(
        manifest["facades"]["stable"]
            .as_str()
            .ok_or("stable facade missing")?,
    );
    let preview_path = root.join(
        manifest["facades"]["preview"]
            .as_str()
            .ok_or("preview facade missing")?,
    );
    let stable = facade_symbols(&fs::read_to_string(stable_path)?)?;
    let preview = facade_symbols(&fs::read_to_string(preview_path)?)?;

    let expected_stable = string_set(manifest, "stable_components")?
        .into_iter()
        .chain(string_set(manifest, "stable_globals")?)
        .chain(string_set(manifest, "stable_types")?)
        .collect::<BTreeSet<_>>();
    require_equal(&stable, &expected_stable, "stable facade")?;

    let expected_preview = string_set(manifest, "preview_components")?;
    let documented_preview = manifest["api"]["globals"]
        .as_array()
        .into_iter()
        .flatten()
        .chain(manifest["api"]["types"].as_array().into_iter().flatten())
        .filter(|entry| entry["maturity"] == "preview")
        .filter_map(|entry| entry["name"].as_str().map(str::to_owned));
    let expected_preview = expected_preview
        .into_iter()
        .chain(documented_preview)
        .collect::<BTreeSet<_>>();
    require_equal(&preview, &expected_preview, "preview facade")?;

    for (group, declaration_kind) in [
        ("components", "component"),
        ("globals", "global"),
        ("types", "type"),
    ] {
        for entry in manifest["api"][group]
            .as_array()
            .ok_or_else(|| format!("manifest API group `{group}` missing"))?
        {
            let name = entry["name"].as_str().ok_or("API symbol without name")?;
            let source_path = entry["source"]
                .as_str()
                .ok_or("API symbol without source")?;
            let source = fs::read_to_string(root.join(source_path))?;
            let declaration = if declaration_kind == "type" {
                format!("export {} {name}", entry["kind"].as_str().unwrap_or("enum"))
            } else {
                format!("export {declaration_kind} {name}")
            };
            if !source.contains(&declaration) {
                return Err(
                    format!("manifest symbol `{name}` is absent from {source_path}").into(),
                );
            }

            for property in entry["properties"].as_array().into_iter().flatten() {
                let declared_in = property["declared_in"].as_str().unwrap_or(name);
                if declared_in != name {
                    continue;
                }
                let property_name = property["name"].as_str().ok_or("property without name")?;
                let pattern = format!(
                    r"(?m)\b(?:in-out|in|out)\s+property\s*<[^>]+>\s*{}\b",
                    regex::escape(property_name)
                );
                if !Regex::new(&pattern)?.is_match(&source) {
                    return Err(format!(
                        "manifest property `{name}.{property_name}` is absent from {source_path}"
                    )
                    .into());
                }
            }
            for callback in entry["callbacks"].as_array().into_iter().flatten() {
                let declared_in = callback["declared_in"].as_str().unwrap_or(name);
                if declared_in != name {
                    continue;
                }
                let callback_name = callback["name"].as_str().ok_or("callback without name")?;
                let pattern = format!(r"(?m)\bcallback\s+{}\s*\(", regex::escape(callback_name));
                if !Regex::new(&pattern)?.is_match(&source) {
                    return Err(format!(
                        "manifest callback `{name}.{callback_name}` is absent from {source_path}"
                    )
                    .into());
                }
            }
        }
    }
    println!("Agent manifest declarations match the Slint source facades.");
    Ok(())
}

pub fn run(root: &Path, args: &[String]) -> Result {
    let path = root.join("docs/atlas-ui-agent-manifest.json");
    let mut manifest = util::read_json(&path)?;
    let cargo = fs::read_to_string(root.join("Cargo.toml"))?;
    let version = Regex::new(r#"(?s)\[workspace\.package\].*?version\s*=\s*"([^"]+)""#)?
        .captures(&cargo)
        .and_then(|c| c.get(1))
        .ok_or("workspace version missing")?
        .as_str();
    let slint = Regex::new(r#"(?m)^slint\s*=\s*"=([^"]+)""#)?
        .captures(&cargo)
        .and_then(|c| c.get(1))
        .ok_or("Slint pin missing")?
        .as_str();
    manifest["generated_by"] = Value::String("atlas-ui-tooling generate-agent-manifest".into());
    manifest["version"] = Value::String(version.into());
    manifest["slint_version"] = Value::String(slint.into());
    let scenarios = util::read_json(&root.join("screenshots/scenarios.json"))?;
    manifest["visual_validation"]["scenario_count"] = Value::from(
        scenarios["scenarios"]
            .as_array()
            .ok_or("invalid scenarios")?
            .len(),
    );
    validate_source_declarations(root, &manifest)?;
    let rendered = format!("{}\n", serde_json::to_string_pretty(&manifest)?);
    if args.iter().any(|a| a == "--check") {
        // Git may materialize text files with CRLF on Windows. Manifest
        // freshness concerns JSON content, not the checkout's line endings.
        if fs::read_to_string(&path)?.replace("\r\n", "\n") != rendered {
            return Err("agent manifest is stale; run `cargo run -p atlas-ui-tooling -- generate-agent-manifest`".into());
        }
    } else {
        fs::write(&path, rendered)?;
        println!("Generated {}.", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::facade_symbols;
    use std::collections::BTreeSet;

    #[test]
    fn parses_multiline_facade_exports_deterministically() {
        let source = r#"
            export { Beta, Alpha } from "one.slint";
            export {
                Gamma,
            } from "two.slint";
        "#;
        assert_eq!(
            facade_symbols(source).unwrap(),
            BTreeSet::from(["Alpha".into(), "Beta".into(), "Gamma".into()])
        );
    }

    #[test]
    fn ignores_component_declarations_and_comments() {
        let source = "// export { Nope }\nexport component Example inherits Rectangle {}";
        assert!(facade_symbols(source).unwrap().is_empty());
    }
}
