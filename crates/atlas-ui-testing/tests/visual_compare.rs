//! End-to-end checks for visual comparison refusal and success paths.

use std::{fs, process::Command};

fn temporary_file(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("atlas-ui-{}-{name}", std::process::id()))
}

#[test]
fn refuses_images_with_different_dimensions() {
    let baseline = temporary_file("baseline-2x2.png");
    let actual = temporary_file("actual-3x2.png");
    let diff = temporary_file("invalid-diff.png");
    image::RgbaImage::new(2, 2)
        .save(&baseline)
        .expect("baseline");
    image::RgbaImage::new(3, 2).save(&actual).expect("actual");

    let result = Command::new(env!("CARGO_BIN_EXE_visual_compare"))
        .args([&baseline, &actual, &diff])
        .output()
        .expect("run comparator");
    assert_eq!(result.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&result.stderr).contains("invalid comparison"));

    let _ = fs::remove_file(baseline);
    let _ = fs::remove_file(actual);
}

#[test]
fn accepts_identical_images_and_writes_a_diff() {
    let baseline = temporary_file("baseline-identical.png");
    let actual = temporary_file("actual-identical.png");
    let diff = temporary_file("identical-diff.png");
    let image = image::RgbaImage::from_pixel(2, 2, image::Rgba([10, 20, 30, 255]));
    image.save(&baseline).expect("baseline");
    image.save(&actual).expect("actual");

    let result = Command::new(env!("CARGO_BIN_EXE_visual_compare"))
        .args([&baseline, &actual, &diff])
        .output()
        .expect("run comparator");
    assert!(result.status.success());
    assert!(diff.is_file());

    let _ = fs::remove_file(baseline);
    let _ = fs::remove_file(actual);
    let _ = fs::remove_file(diff);
}
