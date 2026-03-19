use std::path::Path;
use std::process::Command;

fn run_script(args: &[&str]) -> String {
    let output = Command::new("sh")
        .arg("test/full_apps/scripts/refresh_external_fixture.sh")
        .args(args)
        .output()
        .expect("running refresh_external_fixture.sh");

    assert!(
        output.status.success(),
        "script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("script stdout utf-8")
}

#[test]
fn refresh_script_is_present() {
    assert!(Path::new("test/full_apps/scripts/refresh_external_fixture.sh").is_file());
}

#[test]
fn refresh_script_lists_known_fixtures() {
    let output = run_script(&["list"]);
    assert!(output.lines().any(|line| line == "musl-stdint"));
}

#[test]
fn refresh_script_shows_fixture_metadata() {
    let output = run_script(&["show", "musl-stdint"]);
    assert!(output.contains("fixture=musl-stdint"));
    assert!(output.contains("project=musl"));
    assert!(output.contains("version=v1.2.5"));
    assert!(output.contains("target=test/full_apps/external/musl/stdint"));
}
