use crate::{Result, util};
use chrono::Utc;
use regex::Regex;
use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
fn bullets(v: &Value) -> String {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(|s| format!("- {s}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}
#[allow(clippy::too_many_lines)]
pub fn run(root: &Path, args: &[String]) -> Result {
    util::run(root, "codex", &["login", "status"], None, true)?;
    let shots = root.join("screenshots");
    let reviews = shots.join("reviews");
    fs::create_dir_all(&reviews)?;
    let state_path = reviews.join("state.json");
    if args.iter().any(|a| a == "--reset") && state_path.exists() {
        fs::remove_file(&state_path)?;
    }
    let manifest = util::read_json(&shots.join("scenarios.json"))?;
    let scenarios = manifest["scenarios"]
        .as_array()
        .ok_or("scenarios missing")?;
    let contexts = util::read_json(&shots.join("review-contexts.json"))?;
    let state = if state_path.exists() {
        util::read_json(&state_path)?
    } else {
        json!({"schema_version":1,"prompt_version":"atlas-visual-review-v3","completed":[]})
    };
    let completed: HashSet<String> = state["completed"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let size = util::arg_value(args, "--batch-size")
        .unwrap_or_else(|| "4".into())
        .parse::<usize>()?;
    if !(1..=4).contains(&size) {
        return Err("batch size must be 1..=4".into());
    }
    let requested = util::arg_value(args, "--scenario");
    let mut candidates: Vec<&Value> = if let Some(id) = requested {
        vec![
            scenarios
                .iter()
                .find(|s| s["id"] == id)
                .ok_or("unknown scenario")?,
        ]
    } else if args.iter().any(|a| a == "--rerun-last-batch") {
        scenarios
            .iter()
            .filter(|s| completed.contains(s["id"].as_str().unwrap()))
            .rev()
            .take(size)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    } else {
        scenarios
            .iter()
            .filter(|s| !completed.contains(s["id"].as_str().unwrap()))
            .collect()
    };
    candidates.truncate(size);
    if candidates.is_empty() {
        println!("All visual scenarios have a Codex pre-review. Use --reset to start over.");
        return Ok(());
    }
    println!("Visual review batch ({}/4 maximum):", candidates.len());
    for s in &candidates {
        println!("- {}", s["id"].as_str().unwrap());
    }
    if args.iter().any(|a| a == "--dry-run") {
        return Ok(());
    }
    let template = fs::read_to_string(shots.join("visual-review.prompt.md"))?;
    let variable = Regex::new(r"\{\{([a-z_]+)\}\}")?;
    let mut reports = Vec::new();
    let mut done = completed;
    for scenario in candidates {
        let id = scenario["id"].as_str().unwrap();
        let context = contexts["pages"]
            .get(scenario["page"].as_str().unwrap())
            .unwrap_or(&contexts["default"]);
        let metadata =
            util::read_json(&shots.join("metadata").join(format!("{id}.baseline.json")))?;
        let mut vars = HashMap::new();
        for (k, v) in [
            ("scenario_id", id.to_owned()),
            ("family", "foundations and components".into()),
            ("theme", scenario["theme"].as_str().unwrap().into()),
            ("density", scenario["density"].as_str().unwrap().into()),
            ("viewport_width", scenario["viewport"]["width"].to_string()),
            (
                "viewport_height",
                scenario["viewport"]["height"].to_string(),
            ),
            (
                "reduced_motion",
                (scenario["motion"] == "reduced").to_string(),
            ),
            ("state", scenario["fixture"].as_str().unwrap().into()),
            ("atlas_version", env!("CARGO_PKG_VERSION").into()),
            ("slint_version", "1.17.1".into()),
            (
                "metadata_json",
                serde_json::to_string(&metadata["identity"])?,
            ),
            (
                "fixture_intent",
                context["intent"].as_str().unwrap_or_default().into(),
            ),
            (
                "intentional_traits",
                bullets(&context["intentional_traits"]),
            ),
            ("invariants", bullets(&context["invariants"])),
            ("exclusions", bullets(&context["exclusions"])),
        ] {
            vars.insert(k, v);
        }
        let prompt = variable
            .replace_all(&template, |caps: &regex::Captures| {
                vars.get(&caps[1]).cloned().unwrap_or_default()
            })
            .into_owned();
        let report_path = reviews.join(format!("{id}.json"));
        println!("Reviewing {id}...");
        let mut child = Command::new("codex")
            .current_dir(root)
            .args([
                "exec",
                "--ephemeral",
                "--sandbox",
                "read-only",
                "--skip-git-repo-check",
                "--image",
                shots
                    .join("baselines")
                    .join(format!("{id}.png"))
                    .to_str()
                    .unwrap(),
                "--output-schema",
                shots.join("visual-review.schema.json").to_str().unwrap(),
                "--output-last-message",
                report_path.to_str().unwrap(),
                "-",
            ])
            .stdin(Stdio::piped())
            .spawn()?;
        child.stdin.take().unwrap().write_all(prompt.as_bytes())?;
        if !child.wait()?.success() {
            return Err("Codex review failed".into());
        }
        let report = util::read_json(&report_path)?;
        reports.push(report);
        done.insert(id.into());
        util::write_json(
            &state_path,
            &json!({"schema_version":1,"prompt_version":"atlas-visual-review-v3","completed":scenarios.iter().filter_map(|s|s["id"].as_str()).filter(|id|done.contains(*id)).collect::<Vec<_>>(),"updated_at":Utc::now().to_rfc3339()}),
        )?;
    }
    let mut candidates = Vec::new();
    for report in &reports {
        for observation in report["observations"].as_array().into_iter().flatten() {
            if observation["classification"] == "defect"
                && observation["auto_correction_eligible"] == true
                && observation["confidence"].as_f64().unwrap_or_default() >= 0.9
                && !observation["violated_contract"]
                    .as_str()
                    .unwrap_or_default()
                    .trim()
                    .is_empty()
            {
                candidates.push((
                    report["scenario_id"].as_str().unwrap_or_default(),
                    observation,
                ));
            }
        }
    }
    let key_for = |scenario: &str, observation: &Value| {
        if observation["scope"]["kind"] == "probably-shared" {
            format!(
                "contract:{}",
                observation["violated_contract"]
                    .as_str()
                    .unwrap_or_default()
                    .trim()
                    .to_lowercase()
            )
        } else {
            format!(
                "local:{scenario}:{}",
                observation["consensus_key"].as_str().unwrap_or_default()
            )
        }
    };
    let mut consensus: HashMap<String, HashSet<String>> = HashMap::new();
    for (scenario, observation) in &candidates {
        consensus
            .entry(key_for(scenario, observation))
            .or_default()
            .insert((*scenario).into());
    }
    let actionable=candidates.iter().filter(|(scenario,o)|o["scope"]["kind"]=="local"||consensus[&key_for(scenario,o)].len()>=2).map(|(scenario,o)|json!({"scenario_id":scenario,"observation_id":o["id"],"consensus_key":o["consensus_key"],"consensus_count":consensus[&key_for(scenario,o)].len(),"probable_source_layer":o["probable_source_layer"],"target":o["scope"]["target"]})).collect::<Vec<_>>();
    let summary = json!({"schema_version":1,"prompt_version":"atlas-visual-review-v3","generated_at":Utc::now().to_rfc3339(),"scenarios":reports.iter().map(|r|json!({"scenario_id":r["scenario_id"],"verdict":r["verdict"],"score":r["score"],"priority_issues":r["priority_issues"]})).collect::<Vec<_>>(),"correction_gate":{"candidate_count":candidates.len(),"actionable_count":actionable.len(),"actionable_observations":actionable}});
    let batch = reviews.join(format!(
        "batch-{}.json",
        Utc::now().format("%Y-%m-%dT%H-%M-%S-%3fZ")
    ));
    util::write_json(&batch, &summary)?;
    println!("Batch complete: {}", batch.display());
    Ok(())
}
