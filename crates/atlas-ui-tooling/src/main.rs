//! Native maintenance tooling for Atlas UI.

mod capture;
mod local;
mod manifest;
mod performance;
mod review;
mod util;
mod validate;

use std::{env, path::PathBuf, process::ExitCode};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn root() -> Result<PathBuf> {
    Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .ok_or("tooling crate is outside the Atlas workspace")?
        .to_path_buf())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("atlas-ui-tooling: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or("missing command; use `help`")?;
    let rest = args.collect::<Vec<_>>();
    let root = root()?;
    match command.as_str() {
        "capture-scenarios" => capture::run(&root, &rest),
        "generate-agent-manifest" => manifest::run(&root, &rest),
        "measure-render-performance" => performance::run(&root),
        "review-screenshots" => review::run(&root, &rest),
        "validate" => validate::run(&root, rest.first().map_or("all", String::as_str)),
        "quality-gate" => validate::quality_gate(&root),
        "release-gate" => {
            validate::quality_gate(&root)?;
            capture::run(&root, &[])
        }
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => Err(format!("unknown command: {command}").into()),
    }
}

fn print_help() {
    println!(
        "Atlas UI native tooling\n\nCommands:\n  capture-scenarios\n  generate-agent-manifest\n  measure-render-performance\n  review-screenshots\n  validate <agent-evals|agent-kit|publication|links|rust-only|packages|local|all>\n  quality-gate\n  release-gate"
    );
}
