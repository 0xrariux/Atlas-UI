use crate::{Result, util};
use regex::Regex;
use std::{collections::HashSet, ffi::OsStr, fs, path::Path};

fn require(condition: bool, message: &str) -> Result {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn agent_evals(root: &Path) -> Result {
    let value = util::read_json(&root.join("evals/agent-discovery/cases.json"))?;
    let cases = value["cases"].as_array().ok_or("agent cases missing")?;
    require(
        value["schema_version"] == 1
            && value["suite"] == "atlas-agent-discovery-v1"
            && cases.len() >= 5,
        "invalid agent evaluation suite",
    )?;
    let ids: HashSet<_> = cases.iter().filter_map(|c| c["id"].as_str()).collect();
    for id in [
        "discover-rust-slint-design-system",
        "avoid-overselection-small-ui",
        "integrate-stable-settings-screen",
        "preview-disclosure",
        "published-package-installation",
    ] {
        require(ids.contains(id), "required agent case missing")?;
    }
    println!("Agent evaluation suite valid: {} cases.", cases.len());
    Ok(())
}
fn agent_kit(root: &Path) -> Result {
    let m = util::read_json(&root.join("docs/atlas-ui-agent-manifest.json"))?;
    let cargo = fs::read_to_string(root.join("Cargo.toml"))?;
    require(m["schema_version"] == 2, "invalid agent manifest schema")?;
    require(
        cargo.contains(&format!(
            "version = \"{}\"",
            m["version"].as_str().unwrap_or_default()
        )),
        "agent manifest version drift",
    )?;
    for key in ["stable", "preview", "aggregate"] {
        require(
            root.join(m["facades"][key].as_str().ok_or("facade missing")?)
                .exists(),
            "agent facade missing",
        )?;
    }
    println!("Public agent kit valid.");
    Ok(())
}
fn publication(root: &Path) -> Result {
    let ignore = fs::read_to_string(root.join(".gitignore"))?;
    for rule in [
        "/ai/",
        "/.agents/",
        "/.codex/",
        "/target/",
        "/screenshots/results/",
        "/screenshots/diffs/",
        "/screenshots/performance/",
        "/screenshots/reviews/",
        ".env",
        ".env.*",
        "*.key",
        "*.pem",
        "/release-artifacts/",
    ] {
        require(
            ignore.lines().any(|line| line == rule),
            "required ignore rule missing",
        )?;
    }
    let french = Regex::new(
        r"(?iu)[À-ÖØ-öø-ÿ]|\b(?:avec|aucun|cette|chaque|dans|depuis|doit|doivent|fichier|fichiers|français|langue|les|pour|projet|répertoire|une)\b",
    )?;
    let mut count = 0;
    for file in util::files(root) {
        let rel = file.strip_prefix(root)?.to_string_lossy();
        if rel.starts_with("ai/")
            || rel.starts_with("target/")
            || rel.starts_with(".git/")
            || rel.starts_with("screenshots/reviews/")
            || rel == "crates/atlas-ui-tooling/src/validate.rs"
        {
            continue;
        }
        if matches!(
            file.extension().and_then(|x| x.to_str()),
            Some("md" | "rs" | "slint" | "toml" | "json" | "sh" | "txt" | "yaml" | "yml")
        ) {
            let text = fs::read_to_string(&file)?;
            if french.is_match(&text) {
                return Err(format!("possible non-English text: {rel}").into());
            }
            count += 1;
        }
    }
    println!("Publication validation passed ({count} public text files).");
    Ok(())
}

fn relative_markdown_links(root: &Path) -> Result {
    let link = Regex::new(r"!?(?:\[[^\]]*\])\(([^)]+)\)")?;
    let mut checked = 0;
    for file in util::files(root) {
        let rel = file.strip_prefix(root)?;
        if file.extension() != Some(OsStr::new("md"))
            || rel.starts_with("ai")
            || rel.starts_with("target")
        {
            continue;
        }
        let text = fs::read_to_string(&file)?;
        for capture in link.captures_iter(&text) {
            let raw = capture[1].trim().trim_matches(['<', '>']);
            let target = raw.split('#').next().unwrap_or_default();
            if target.is_empty()
                || target.contains("://")
                || target.starts_with("mailto:")
                || target.starts_with("data:")
            {
                continue;
            }
            let resolved = if target.starts_with('/') {
                root.join(target.trim_start_matches('/'))
            } else {
                file.parent().unwrap_or(root).join(target)
            };
            require(
                resolved.exists(),
                &format!("broken Markdown link in {}: {raw}", rel.display()),
            )?;
            checked += 1;
        }
    }
    println!("Markdown links valid ({checked} local targets).");
    Ok(())
}

fn rust_only_tooling(root: &Path) -> Result {
    let forbidden_names = [
        "package.json",
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "bun.lock",
        "bun.lockb",
    ];
    let forbidden_extensions = ["js", "mjs", "cjs", "ts", "mts", "cts"];
    for file in util::files(root) {
        let rel = file.strip_prefix(root)?;
        if rel.starts_with("target") || rel.starts_with(".git") || rel.starts_with("ai") {
            continue;
        }
        let name = file.file_name().and_then(OsStr::to_str).unwrap_or_default();
        let extension = file.extension().and_then(OsStr::to_str).unwrap_or_default();
        require(
            !forbidden_names.contains(&name) && !forbidden_extensions.contains(&extension),
            &format!(
                "non-Rust scripting artifact is forbidden: {}",
                rel.display()
            ),
        )?;
    }
    println!("Rust-only tooling invariant valid.");
    Ok(())
}

fn publishable_packages(root: &Path) -> Result {
    let cargo = std::env::var("CARGO_BIN").unwrap_or_else(|_| "cargo".into());
    for package in [
        "atlas-ui-tokens",
        "atlas-ui-core",
        "atlas-ui-icons",
        "atlas-ui-components",
        "atlas-ui-documents",
        "atlas-ui-testing",
        "atlas-ui",
    ] {
        util::run(
            root,
            &cargo,
            &["package", "--list", "--allow-dirty", "-p", package],
            None,
            true,
        )?;
    }
    println!("Publishable package contents valid (7 crates).");
    Ok(())
}

pub fn run(root: &Path, name: &str) -> Result {
    match name {
        "agent-evals" => agent_evals(root),
        "agent-kit" => agent_kit(root),
        "publication" => publication(root),
        "links" => relative_markdown_links(root),
        "rust-only" => rust_only_tooling(root),
        "packages" => publishable_packages(root),
        "local" => crate::local::run(root),
        "all" => {
            agent_evals(root)?;
            agent_kit(root)?;
            publication(root)?;
            rust_only_tooling(root)?;
            relative_markdown_links(root)?;
            publishable_packages(root)?;
            crate::local::run(root)
        }
        _ => Err(format!("unknown validation: {name}").into()),
    }
}

pub fn quality_gate(root: &Path) -> Result {
    run(root, "all")?;
    crate::manifest::run(root, &["--check".into()])?;
    crate::capture::run(root, &["--validate-only".into()])
}

#[cfg(test)]
mod tests {
    use super::{relative_markdown_links, rust_only_tooling};
    use std::fs;

    fn fixture(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "atlas-tooling-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn rejects_javascript_artifacts() {
        let root = fixture("rust-only");
        fs::write(root.join("legacy.mjs"), "").unwrap();
        assert!(rust_only_tooling(&root).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_broken_local_markdown_links() {
        let root = fixture("links");
        fs::write(root.join("README.md"), "[missing](docs/missing.md)").unwrap();
        assert!(relative_markdown_links(&root).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn accepts_existing_and_external_markdown_links() {
        let root = fixture("valid-links");
        fs::create_dir(root.join("docs")).unwrap();
        fs::write(root.join("docs/guide.md"), "# Guide").unwrap();
        fs::write(
            root.join("README.md"),
            "[guide](docs/guide.md#usage) [web](https://example.com)",
        )
        .unwrap();
        assert!(relative_markdown_links(&root).is_ok());
        fs::remove_dir_all(root).unwrap();
    }
}
