use crate::{Result, util};
use chrono::Utc;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    path::Path,
};

fn identity(schema: &Value, scenario: &Value) -> Value {
    let mut value = json!({"schema_version":schema,"id":scenario["id"],"page":scenario["page"],"fixture":scenario["fixture"],"theme":scenario["theme"],"density":scenario["density"],"motion":scenario["motion"],"grid":scenario["grid"],"viewport":scenario["viewport"],"renderer":scenario["renderer"],"scale_factor":scenario["scale_factor"]});
    for key in ["theme_mode", "system_dark", "typography_scale"] {
        if !scenario[key].is_null() {
            value[key] = scenario[key].clone();
        }
    }
    value
}

#[allow(clippy::too_many_lines)]
pub fn run(root: &Path, args: &[String]) -> Result {
    let screenshots = root.join("screenshots");
    let manifest = util::read_json(&screenshots.join("scenarios.json"))?;
    let scenarios = manifest["scenarios"]
        .as_array()
        .ok_or("invalid scenarios manifest")?;
    let mut ids = HashSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().ok_or("scenario without id")?;
        if !ids.insert(id) {
            return Err(format!("duplicate scenario: {id}").into());
        }
    }
    let scenario_arg = util::arg_value(args, "--scenario");
    let approval = util::arg_value(args, "--approve-baseline");
    if let Some(id) = approval {
        let reviewer =
            util::arg_value(args, "--reviewer").ok_or("--approve-baseline requires --reviewer")?;
        let scenario = scenarios
            .iter()
            .find(|s| s["id"] == id)
            .ok_or("unknown scenario")?;
        let path = screenshots
            .join("metadata")
            .join(format!("{id}.baseline.json"));
        let mut meta = util::read_json(&path)?;
        if meta["identity"] != identity(&manifest["schema_version"], scenario) {
            return Err("scenario identity changed".into());
        }
        meta["approval"] = json!({"status":"approved","reviewer":reviewer,"note":util::arg_value(args,"--note").unwrap_or_else(||"Baseline reviewed against the declared fixture.".into()),"approved_at":Utc::now().to_rfc3339()});
        util::write_json(&path, &meta)?;
        println!("Approved baseline {id}.");
        return Ok(());
    }
    if args.iter().any(|a| a == "--validate-only") {
        println!("Capture manifest valid: {} scenarios.", scenarios.len());
        return Ok(());
    }
    let selected: Vec<&Value> = if let Some(id) = scenario_arg {
        vec![
            scenarios
                .iter()
                .find(|s| s["id"] == id)
                .ok_or("unknown scenario")?,
        ]
    } else {
        scenarios.iter().collect()
    };
    for dir in ["baselines", "results", "diffs", "metadata"] {
        fs::create_dir_all(screenshots.join(dir))?;
    }
    let cargo = env::var("CARGO_BIN").unwrap_or_else(|_| "cargo".into());
    util::run(
        root,
        &cargo,
        &[
            "build",
            "-p",
            "atlas-ui-gallery",
            "-p",
            "atlas-ui-testing",
            "--bins",
        ],
        None,
        false,
    )?;
    let extension = if cfg!(windows) { ".exe" } else { "" };
    let gallery = root
        .join("target/debug")
        .join(format!("atlas-ui-gallery{extension}"));
    let compare = root
        .join("target/debug")
        .join(format!("visual_compare{extension}"));
    let update = args.iter().any(|a| a == "--update-baselines");
    for scenario in selected {
        let id = scenario["id"].as_str().unwrap();
        let ident = identity(&manifest["schema_version"], scenario);
        let result = screenshots.join("results").join(format!("{id}.png"));
        let baseline = screenshots.join("baselines").join(format!("{id}.png"));
        let metadata = screenshots
            .join("metadata")
            .join(format!("{id}.baseline.json"));
        if !update && util::read_json(&metadata)?["identity"] != ident {
            return Err(format!("invalid comparison identity: {id}").into());
        }
        let mut envs: BTreeMap<String, String> = env::vars().collect();
        for (key, value) in [
            ("ATLAS_UI_GALLERY_CAPTURE", result.display().to_string()),
            (
                "ATLAS_UI_GALLERY_PAGE",
                scenario["page"].as_str().unwrap().into(),
            ),
            (
                "ATLAS_UI_GALLERY_DENSITY",
                scenario["density"].as_str().unwrap().into(),
            ),
            (
                "ATLAS_UI_GALLERY_MOTION",
                scenario["motion"].as_str().unwrap().into(),
            ),
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
            ("ATLAS_UI_GALLERY_DELAY_MS", "300".into()),
            (
                "SLINT_BACKEND",
                scenario["renderer"].as_str().unwrap().into(),
            ),
            ("SLINT_SCALE_FACTOR", scenario["scale_factor"].to_string()),
        ] {
            envs.insert(key.into(), value);
        }
        for key in [
            "ATLAS_UI_GALLERY_LIGHT",
            "ATLAS_UI_GALLERY_SYSTEM_THEME",
            "ATLAS_UI_GALLERY_SYSTEM_DARK",
            "ATLAS_UI_GALLERY_GRID",
        ] {
            envs.remove(key);
        }
        if scenario["theme"] == "light" {
            envs.insert("ATLAS_UI_GALLERY_LIGHT".into(), "1".into());
        }
        if scenario["theme_mode"] == "system" {
            envs.insert("ATLAS_UI_GALLERY_SYSTEM_THEME".into(), "1".into());
        }
        if scenario["system_dark"] == true {
            envs.insert("ATLAS_UI_GALLERY_SYSTEM_DARK".into(), "1".into());
        }
        if scenario["grid"] == true {
            envs.insert("ATLAS_UI_GALLERY_GRID".into(), "1".into());
        }
        util::run(root, gallery.to_str().unwrap(), &[], Some(&envs), false)?;
        let (size_w, size_h) = util::png_size(&result)?;
        let expected = (
            u32::try_from(scenario["viewport"]["width"].as_u64().unwrap())?,
            u32::try_from(scenario["viewport"]["height"].as_u64().unwrap())?,
        );
        if (size_w, size_h) != expected {
            return Err(format!("capture size mismatch: {id}").into());
        }
        util::write_json(
            &screenshots.join("results").join(format!("{id}.json")),
            &json!({"identity":ident,"image":{"width":size_w,"height":size_h},"platform":env::consts::OS,"architecture":env::consts::ARCH}),
        )?;
        if update {
            fs::copy(&result, &baseline)?;
            util::write_json(
                &metadata,
                &json!({"identity":ident,"image":{"width":size_w,"height":size_h},"platform":env::consts::OS,"architecture":env::consts::ARCH,"approval":{"status":"pending-human","reviewer":null,"note":null,"approved_at":null}}),
            )?;
            println!("Updated baseline {id} (pending human approval).");
        } else {
            let status = std::process::Command::new(&compare)
                .args([
                    baseline.as_os_str(),
                    result.as_os_str(),
                    screenshots
                        .join("diffs")
                        .join(format!("{id}.png"))
                        .as_os_str(),
                    std::ffi::OsStr::new(&scenario["threshold"].to_string()),
                ])
                .status()?;
            if !status.success() {
                return Err(format!("visual regression: {id}").into());
            }
        }
    }
    Ok(())
}
