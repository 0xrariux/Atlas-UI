use crate::{Result, util};
use chrono::Utc;
use serde_json::{Value, json};
use std::{collections::BTreeMap, env, fs, path::Path, time::Instant};
#[allow(clippy::too_many_lines)]
pub fn run(root: &Path) -> Result {
    let manifest = util::read_json(&root.join("screenshots/scenarios.json"))?;
    let budgets = util::read_json(&root.join("scripts/render-performance-budgets.json"))?;
    let cargo = env::var("CARGO_BIN").unwrap_or_else(|_| "cargo".into());
    util::run(
        root,
        &cargo,
        &["build", "-p", "atlas-ui-gallery"],
        None,
        false,
    )?;
    let gallery = root.join("target/debug/atlas-ui-gallery");
    let temp = env::temp_dir().join(format!("atlas-render-performance-{}", std::process::id()));
    fs::create_dir_all(&temp)?;
    let outcome = (|| -> Result<Value> {
        let scenarios = manifest["scenarios"]
            .as_array()
            .ok_or("scenarios missing")?;
        let mut results = Vec::new();
        for budget in budgets["render_budgets"]
            .as_array()
            .ok_or("budgets missing")?
        {
            let id = budget["id"].as_str().unwrap();
            let scenario = scenarios
                .iter()
                .find(|s| s["id"] == budget["scenario"])
                .ok_or("budget scenario missing")?;
            let output = temp.join(format!("{id}.png"));
            let mut envs: BTreeMap<String, String> = env::vars().collect();
            for (k, v) in [
                ("ATLAS_UI_GALLERY_CAPTURE", output.display().to_string()),
                (
                    "ATLAS_UI_GALLERY_PAGE",
                    scenario["page"].as_str().unwrap().into(),
                ),
                (
                    "ATLAS_UI_GALLERY_DENSITY",
                    scenario["density"].as_str().unwrap().into(),
                ),
                ("ATLAS_UI_GALLERY_MOTION", "reduced".into()),
                (
                    "ATLAS_UI_GALLERY_TYPOGRAPHY_SCALE",
                    scenario["typography_scale"]
                        .as_str()
                        .unwrap_or("normal")
                        .into(),
                ),
                (
                    "ATLAS_UI_GALLERY_WIDTH",
                    scenario["viewport"]["width"].to_string(),
                ),
                (
                    "ATLAS_UI_GALLERY_HEIGHT",
                    scenario["viewport"]["height"].to_string(),
                ),
                ("ATLAS_UI_GALLERY_DELAY_MS", "100".into()),
                ("SLINT_BACKEND", "software".into()),
                ("SLINT_SCALE_FACTOR", "1".into()),
            ] {
                envs.insert(k.into(), v);
            }
            if scenario["theme"] == "light" {
                envs.insert("ATLAS_UI_GALLERY_LIGHT".into(), "1".into());
            }
            let warm = budget["warmup_iterations"].as_u64().unwrap();
            let samples_count = budget["sample_count"].as_u64().unwrap();
            for _ in 0..warm {
                util::run(root, gallery.to_str().unwrap(), &[], Some(&envs), true)?;
            }
            let mut samples = Vec::new();
            for _ in 0..samples_count {
                let start = Instant::now();
                util::run(root, gallery.to_str().unwrap(), &[], Some(&envs), true)?;
                samples.push(start.elapsed().as_secs_f64() * 1000.0);
                let (w, h) = util::png_size(&output)?;
                if w != u32::try_from(scenario["viewport"]["width"].as_u64().unwrap())?
                    || h != u32::try_from(scenario["viewport"]["height"].as_u64().unwrap())?
                {
                    return Err("render dimensions invalid".into());
                }
            }
            samples.sort_by(f64::total_cmp);
            let median = samples[samples.len() / 2];
            let p95 = samples[(samples.len() * 95).div_ceil(100).saturating_sub(1)];
            let limit = budget["median_limit_ms"].as_f64().unwrap();
            if median >= limit {
                return Err(format!("render median exceeded: {id}").into());
            }
            results.push(json!({"id":id,"scenario":budget["scenario"],"warmup_iterations":warm,"sample_count":samples.len(),"median_limit_ms":limit,"minimum_ms":samples[0],"median_ms":median,"p95_ms":p95,"maximum_ms":samples[samples.len()-1],"samples_ms":samples}));
        }
        Ok(
            json!({"schema_version":1,"platform_profile":budgets["platform_profile"],"renderer":"software","scale_factor":1,"compilation_included":false,"generated_at":Utc::now().to_rfc3339(),"results":results}),
        )
    })();
    fs::remove_dir_all(&temp)?;
    let report = outcome?;
    util::write_json(
        &root.join("screenshots/performance/local-render.json"),
        &report,
    )?;
    println!(
        "Render performance measured: {} scenarios.",
        report["results"].as_array().unwrap().len()
    );
    Ok(())
}
