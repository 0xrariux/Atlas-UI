use crate::{Result, util};
use regex::Regex;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

#[derive(Clone)]
struct ComponentDeclaration {
    source_line: u64,
    inherits: String,
    properties: BTreeSet<String>,
    callbacks: BTreeSet<String>,
}

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

fn line_number(source: &str, byte_offset: usize) -> u64 {
    source[..byte_offset]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count() as u64
        + 1
}

fn component_declaration(source: &str, name: &str) -> Result<ComponentDeclaration> {
    let declaration = Regex::new(&format!(
        r"(?m)^[ \t]*export\s+component\s+{}\s+inherits\s+([A-Za-z_][A-Za-z0-9_-]*)\s*\{{",
        regex::escape(name)
    ))?;
    let captures = declaration
        .captures(source)
        .ok_or_else(|| format!("component declaration `{name}` is missing"))?;
    let whole = captures
        .get(0)
        .ok_or("component declaration match missing")?;
    let open = source[whole.start()..whole.end()]
        .rfind('{')
        .map(|offset| whole.start() + offset)
        .ok_or("component declaration has no body")?;
    let mut depth = 0_u32;
    let mut close = None;
    for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1).ok_or("unbalanced component body")?;
                if depth == 0 {
                    close = Some(open + offset);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close.ok_or_else(|| format!("component declaration `{name}` is unclosed"))?;
    let body = &source[open + 1..close];
    let property =
        Regex::new(r"\b(?:in-out|in|out)\s+property\s*<[^>]+>\s*([A-Za-z_][A-Za-z0-9_-]*)\b")?;
    let callback = Regex::new(r"\bcallback\s+([A-Za-z_][A-Za-z0-9_-]*)\s*\(")?;
    Ok(ComponentDeclaration {
        source_line: line_number(source, whole.start()),
        inherits: captures[1].to_owned(),
        properties: property
            .captures_iter(body)
            .map(|capture| capture[1].to_owned())
            .collect(),
        callbacks: callback
            .captures_iter(body)
            .map(|capture| capture[1].to_owned())
            .collect(),
    })
}

fn direct_signature(
    entry: &Value,
    group: &str,
    declared: &BTreeSet<String>,
    label: &str,
) -> Result<Vec<Value>> {
    let members = entry[group]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|member| {
            member["name"]
                .as_str()
                .is_some_and(|name| declared.contains(name))
        })
        .cloned()
        .collect::<Vec<_>>();
    let documented = members
        .iter()
        .filter_map(|member| member["name"].as_str().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    require_equal(&documented, declared, label)?;
    Ok(members)
}

fn effective_component(
    name: &str,
    entries: &BTreeMap<String, Value>,
    declarations: &BTreeMap<String, ComponentDeclaration>,
    cache: &mut BTreeMap<String, Value>,
    visiting: &mut BTreeSet<String>,
) -> Result<Value> {
    if let Some(entry) = cache.get(name) {
        return Ok(entry.clone());
    }
    if !visiting.insert(name.to_owned()) {
        return Err(format!("public component inheritance cycle at `{name}`").into());
    }
    let mut entry = entries
        .get(name)
        .cloned()
        .ok_or_else(|| format!("manifest component `{name}` is missing"))?;
    let declaration = declarations
        .get(name)
        .ok_or_else(|| format!("source declaration metadata for `{name}` is missing"))?;
    entry["source_line"] = Value::from(declaration.source_line);
    entry["inherits"] = Value::String(declaration.inherits.clone());

    let direct_properties = direct_signature(
        &entry,
        "properties",
        &declaration.properties,
        &format!("direct property signature for `{name}`"),
    )?;
    let direct_callbacks = direct_signature(
        &entry,
        "callbacks",
        &declaration.callbacks,
        &format!("direct callback signature for `{name}`"),
    )?;

    let mut properties = Vec::new();
    let mut callbacks = Vec::new();
    if entries.contains_key(&declaration.inherits) {
        let base = effective_component(
            &declaration.inherits,
            entries,
            declarations,
            cache,
            visiting,
        )?;
        for mut property in base["properties"].as_array().into_iter().flatten().cloned() {
            let inherited_name = property["name"].as_str().unwrap_or_default();
            if !declaration.properties.contains(inherited_name) {
                property["inherited"] = Value::Bool(true);
                properties.push(property);
            }
        }
        for mut callback in base["callbacks"].as_array().into_iter().flatten().cloned() {
            let inherited_name = callback["name"].as_str().unwrap_or_default();
            if !declaration.callbacks.contains(inherited_name) {
                callback["inherited"] = Value::Bool(true);
                callbacks.push(callback);
            }
        }
    }
    for mut property in direct_properties {
        property["declared_in"] = Value::String(name.to_owned());
        property["inherited"] = Value::Bool(false);
        properties.push(property);
    }
    for mut callback in direct_callbacks {
        callback["declared_in"] = Value::String(name.to_owned());
        callback["inherited"] = Value::Bool(false);
        callbacks.push(callback);
    }

    entry["inputs_without_explicit_default"] = Value::Array(
        properties
            .iter()
            .filter(|property| {
                matches!(property["direction"].as_str(), Some("in" | "in-out"))
                    && property["default"].is_null()
            })
            .filter_map(|property| {
                property["name"]
                    .as_str()
                    .map(|name| Value::String(name.into()))
            })
            .collect(),
    );
    entry["properties"] = Value::Array(properties);
    entry["callbacks"] = Value::Array(callbacks);
    visiting.remove(name);
    cache.insert(name.to_owned(), entry.clone());
    Ok(entry)
}

fn synchronize_declaration_metadata(root: &Path, manifest: &mut Value) -> Result<()> {
    let component_entries = manifest["api"]["components"]
        .as_array()
        .ok_or("manifest component API missing")?
        .clone();
    let mut entries = BTreeMap::new();
    let mut declarations = BTreeMap::new();
    for entry in &component_entries {
        let name = entry["name"].as_str().ok_or("component without name")?;
        let source_path = entry["source"].as_str().ok_or("component without source")?;
        let source = fs::read_to_string(root.join(source_path))?;
        entries.insert(name.to_owned(), entry.clone());
        declarations.insert(name.to_owned(), component_declaration(&source, name)?);
    }
    let mut cache = BTreeMap::new();
    let mut visiting = BTreeSet::new();
    manifest["api"]["components"] = Value::Array(
        component_entries
            .iter()
            .map(|entry| {
                effective_component(
                    entry["name"].as_str().ok_or("component without name")?,
                    &entries,
                    &declarations,
                    &mut cache,
                    &mut visiting,
                )
            })
            .collect::<Result<Vec<_>>>()?,
    );

    for (group, fallback_kind) in [("types", "type"), ("globals", "global")] {
        for entry in manifest["api"][group]
            .as_array_mut()
            .ok_or_else(|| format!("manifest API group `{group}` missing"))?
        {
            let name = entry["name"].as_str().ok_or("API symbol without name")?;
            let source_path = entry["source"]
                .as_str()
                .ok_or("API symbol without source")?;
            let source = fs::read_to_string(root.join(source_path))?;
            let kind = entry["kind"].as_str().unwrap_or(fallback_kind);
            let declaration = Regex::new(&format!(
                r"(?m)^[ \t]*export\s+{}\s+{}\b",
                regex::escape(kind),
                regex::escape(name)
            ))?;
            let location = declaration
                .find(&source)
                .ok_or_else(|| format!("{kind} declaration `{name}` is missing"))?;
            entry["source_line"] = Value::from(line_number(&source, location.start()));
        }
    }
    Ok(())
}

fn synchronize_counts(root: &Path, manifest: &mut Value) -> Result<()> {
    let stable = facade_symbols(&fs::read_to_string(
        root.join(
            manifest["facades"]["stable"]
                .as_str()
                .ok_or("stable facade missing")?,
        ),
    )?)?;
    let preview = facade_symbols(&fs::read_to_string(
        root.join(
            manifest["facades"]["preview"]
                .as_str()
                .ok_or("preview facade missing")?,
        ),
    )?)?;
    manifest["counts"]["components"] = Value::from(
        manifest["api"]["components"]
            .as_array()
            .ok_or("manifest component API missing")?
            .len(),
    );
    manifest["counts"]["stable_components"] = Value::from(
        manifest["stable_components"]
            .as_array()
            .ok_or("stable component list missing")?
            .len(),
    );
    manifest["counts"]["preview_components"] = Value::from(
        manifest["preview_components"]
            .as_array()
            .ok_or("preview component list missing")?
            .len(),
    );
    manifest["counts"]["stable_symbols"] = Value::from(stable.len());
    let compatibility_count = manifest["preview_compatibility_exports"]
        .as_array()
        .ok_or("preview compatibility export list missing")?
        .len();
    manifest["counts"]["preview_symbols"] =
        Value::from(preview.len().saturating_sub(compatibility_count));
    let icon_registry = util::read_json(&root.join("crates/atlas-ui-icons/icons.registry.json"))?;
    manifest["counts"]["registered_icons"] = Value::from(
        icon_registry["icons"]
            .as_array()
            .ok_or("icon registry entries missing")?
            .len(),
    );
    Ok(())
}

fn validate_nonresponsive_preview(
    root: &Path,
    manifest: &Value,
    preview: &BTreeSet<String>,
) -> Result {
    let path = root.join(
        manifest["facades"]["preview_nonresponsive"]
            .as_str()
            .ok_or("non-responsive preview facade missing")?,
    );
    let actual = facade_symbols(&fs::read_to_string(path)?)?;
    let expected = string_set(manifest, "preview_nonresponsive_symbols")?;
    require_equal(&actual, &expected, "non-responsive preview facade")?;
    if !actual.is_subset(preview) {
        return Err("non-responsive preview facade contains a non-preview symbol".into());
    }
    Ok(())
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
    let compatibility_preview = manifest["preview_compatibility_exports"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned);
    let expected_preview = expected_preview
        .into_iter()
        .chain(documented_preview)
        .chain(compatibility_preview)
        .collect::<BTreeSet<_>>();
    require_equal(&preview, &expected_preview, "preview facade")?;
    validate_nonresponsive_preview(root, manifest, &preview)?;

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
    manifest["release_context"]["workspace_version"] = Value::String(version.into());
    manifest["agent_workflow"] = json!([
        "Define the product reference, target viewports, theme, density, and required states using docs/AGENT_VISUAL_WORKFLOW.md.",
        "Select a component by need using docs/AGENT_COMPONENT_INDEX.md.",
        "Prefer stable symbols and make preview dependencies explicit.",
        "Read the component signature in this manifest and confirm final details in the referenced Slint source declaration.",
        "Compile the consumer, capture the target viewports, inspect the rendered screenshots, and refine the composition."
    ]);
    manifest["documentation"]["visual_workflow"] =
        Value::String("docs/AGENT_VISUAL_WORKFLOW.md".into());
    synchronize_declaration_metadata(root, &mut manifest)?;
    synchronize_counts(root, &mut manifest)?;
    manifest["facades"]["preview_nonresponsive"] =
        Value::String("crates/atlas-ui-components/ui/preview-nonresponsive.slint".into());
    let preview_nonresponsive = facade_symbols(&fs::read_to_string(
        root.join(
            manifest["facades"]["preview_nonresponsive"]
                .as_str()
                .ok_or("non-responsive preview facade missing")?,
        ),
    )?)?;
    manifest["preview_nonresponsive_symbols"] = Value::Array(
        preview_nonresponsive
            .iter()
            .cloned()
            .map(Value::String)
            .collect(),
    );
    manifest["counts"]["preview_nonresponsive_symbols"] = Value::from(preview_nonresponsive.len());
    for group in ["components", "globals", "types"] {
        for entry in manifest["api"][group].as_array_mut().into_iter().flatten() {
            let Some(name) = entry["name"].as_str() else {
                continue;
            };
            if preview_nonresponsive.contains(name)
                && let Some(example) = entry["minimal_example"].as_str()
            {
                entry["minimal_example"] =
                    Value::String(example.replace("preview.slint", "preview-nonresponsive.slint"));
            }
        }
    }
    let scenarios = util::read_json(&root.join("screenshots/scenarios.json"))?;
    manifest["visual_validation"]["scenario_count"] = Value::from(
        scenarios["scenarios"]
            .as_array()
            .ok_or("invalid scenarios")?
            .len(),
    );
    manifest["counts"]["visual_scenarios"] =
        manifest["visual_validation"]["scenario_count"].clone();
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
    use super::{component_declaration, facade_symbols};
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

    #[test]
    fn extracts_component_location_base_and_direct_members() {
        let source = r"
// Header
export component Base inherits Rectangle {
    in property <string> label;
    callback activated(bool);
    Rectangle { background: #fff; }
}
";
        let declaration = component_declaration(source, "Base").unwrap();
        assert_eq!(declaration.source_line, 3);
        assert_eq!(declaration.inherits, "Rectangle");
        assert_eq!(declaration.properties, BTreeSet::from(["label".into()]));
        assert_eq!(declaration.callbacks, BTreeSet::from(["activated".into()]));
    }
}
