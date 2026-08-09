//! Deterministic PNG comparison and diff generation for Atlas UI.

use std::{env, path::Path, process::ExitCode};

fn main() -> ExitCode {
    match compare() {
        Ok(passed) => {
            if passed {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn compare() -> Result<bool, String> {
    let mut arguments = env::args().skip(1);
    let baseline_path = arguments.next().ok_or("missing baseline path")?;
    let actual_path = arguments.next().ok_or("missing actual path")?;
    let diff_path = arguments.next().ok_or("missing diff path")?;
    let threshold = arguments
        .next()
        .map(|value| value.parse::<f64>().map_err(|_| "invalid threshold"))
        .transpose()?
        .unwrap_or(0.002);
    if arguments.next().is_some() {
        return Err("too many arguments".into());
    }

    let baseline = image::open(&baseline_path)
        .map_err(|error| format!("cannot open baseline {baseline_path}: {error}"))?
        .into_rgba8();
    let actual = image::open(&actual_path)
        .map_err(|error| format!("cannot open actual {actual_path}: {error}"))?
        .into_rgba8();
    if baseline.dimensions() != actual.dimensions() {
        return Err(format!(
            "invalid comparison: baseline is {}x{}, actual is {}x{}",
            baseline.width(),
            baseline.height(),
            actual.width(),
            actual.height()
        ));
    }

    let mut absolute_sum = 0_f64;
    let mut changed_pixels = 0_f64;
    let mut diff = image::RgbaImage::new(actual.width(), actual.height());
    for (x, y, baseline_pixel) in baseline.enumerate_pixels() {
        let actual_pixel = actual.get_pixel(x, y);
        let mut maximum = 0_u8;
        for channel in 0..3 {
            let difference = baseline_pixel[channel].abs_diff(actual_pixel[channel]);
            absolute_sum += f64::from(difference);
            maximum = maximum.max(difference);
        }
        if maximum > 8 {
            changed_pixels += 1.0;
        }
        diff.put_pixel(x, y, image::Rgba([maximum, 0, 0, 255]));
    }
    diff.save(Path::new(&diff_path))
        .map_err(|error| format!("cannot write diff {diff_path}: {error}"))?;

    let pixels = f64::from(actual.width()) * f64::from(actual.height());
    let mean_absolute_difference = absolute_sum / (pixels * 3.0 * 255.0);
    let changed_pixel_ratio = changed_pixels / pixels;
    let passed = mean_absolute_difference <= threshold;
    println!(
        "{{\"passed\":{passed},\"width\":{},\"height\":{},\"mean_absolute_difference\":{mean_absolute_difference:.8},\"changed_pixel_ratio\":{changed_pixel_ratio:.8},\"threshold\":{threshold:.8}}}",
        actual.width(),
        actual.height()
    );
    Ok(passed)
}
