use std::process::Command;

fn rt_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rt"))
}

#[test]
fn renders_a_small_ppm_image_to_standard_output() {
    let output = rt_command()
        .args(["--scene", "all", "--width", "10", "--height", "8"])
        .output()
        .expect("failed to run the rt binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut lines = stdout.lines();
    assert_eq!(lines.next(), Some("P3"));
    assert_eq!(lines.next(), Some("10 8"));
    assert_eq!(lines.next(), Some("255"));
    assert_eq!(lines.count(), 10 * 8);
}

#[test]
fn prints_help_without_rendering() {
    let output = rt_command().arg("--help").output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage: rt"));
    assert!(!stdout.starts_with("P3"));
}

#[test]
fn reports_an_error_and_exits_nonzero_for_an_unknown_scene() {
    let output = rt_command()
        .args(["--scene", "does-not-exist"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("does-not-exist"));
}
