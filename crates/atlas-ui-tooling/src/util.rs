use crate::Result;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

pub fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
pub fn write_json(path: &Path, value: &Value) -> Result {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}
pub fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|index| args.get(index + 1))
        .cloned()
}
pub fn run(
    root: &Path,
    program: &str,
    args: &[&str],
    envs: Option<&BTreeMap<String, String>>,
    capture: bool,
) -> Result<Output> {
    let mut command = Command::new(program);
    command.current_dir(root).args(args);
    if let Some(envs) = envs {
        command.envs(envs);
    }
    if !capture {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }
    let output = command.output()?;
    if !output.status.success() {
        return Err(format!(
            "{program} failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(output)
}
pub fn files(root: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name();
            !(entry.depth() == 1 && name == OsStr::new("template"))
                && name != OsStr::new("node_modules")
                && name != OsStr::new("dist")
        })
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .collect()
}
pub fn png_size(path: &Path) -> Result<(u32, u32)> {
    let data = fs::read(path)?;
    if data.len() < 24 || &data[1..4] != b"PNG" {
        return Err(format!("not a PNG: {}", path.display()).into());
    }
    Ok((
        u32::from_be_bytes(data[16..20].try_into()?),
        u32::from_be_bytes(data[20..24].try_into()?),
    ))
}

#[cfg(test)]
mod tests {
    use super::{arg_value, files, png_size};
    use std::fs;

    #[test]
    fn reads_flag_values_without_consuming_other_arguments() {
        let args = vec![
            "--scenario".to_owned(),
            "foundations.dark".to_owned(),
            "--check".to_owned(),
        ];
        assert_eq!(
            arg_value(&args, "--scenario").as_deref(),
            Some("foundations.dark")
        );
        assert_eq!(arg_value(&args, "--missing"), None);
    }

    #[test]
    fn reads_png_dimensions_from_the_standard_header() {
        let path = std::env::temp_dir().join(format!("atlas-tooling-{}.png", std::process::id()));
        let mut bytes = vec![0_u8; 24];
        bytes[1..4].copy_from_slice(b"PNG");
        bytes[16..20].copy_from_slice(&720_u32.to_be_bytes());
        bytes[20..24].copy_from_slice(&800_u32.to_be_bytes());
        fs::write(&path, bytes).unwrap();
        assert_eq!(png_size(&path).unwrap(), (720, 800));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn repository_walk_skips_dependency_trees() {
        let root = std::env::temp_dir().join(format!("atlas-tooling-files-{}", std::process::id()));
        let dependencies = root.join("template/web/node_modules/package");
        let distribution = root.join("template/web/dist/assets");
        let local_template = root.join("template/web");
        fs::create_dir_all(&dependencies).unwrap();
        fs::create_dir_all(&distribution).unwrap();
        fs::write(root.join("public.md"), "public").unwrap();
        fs::write(dependencies.join("README.md"), "dependency").unwrap();
        fs::write(distribution.join("index.js"), "generated").unwrap();
        fs::write(local_template.join("package-lock.json"), "generated").unwrap();

        let walked = files(&root);
        assert_eq!(walked, [root.join("public.md")]);

        fs::remove_dir_all(root).unwrap();
    }
}
