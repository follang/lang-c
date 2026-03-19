use std::path::Path;
use std::process::Command;
use std::{fs, str};

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
    assert!(output.lines().any(|line| line == "zlib-header"));
    assert!(output.lines().any(|line| line == "zlib-zpipe"));
}

#[test]
fn refresh_script_shows_fixture_metadata() {
    let output = run_script(&["show", "musl-stdint"]);
    assert!(output.contains("fixture=musl-stdint"));
    assert!(output.contains("project=musl"));
    assert!(output.contains("version=v1.2.5"));
    assert!(output.contains("target=test/full_apps/external/musl/stdint"));
}

#[test]
fn refresh_script_shows_zlib_fixture_metadata() {
    let output = run_script(&["show", "zlib-header"]);
    assert!(output.contains("fixture=zlib-header"));
    assert!(output.contains("project=zlib"));
    assert!(output.contains("version=v1.3.1"));
    assert!(output.contains("target=test/full_apps/external/zlib/header"));
}

#[test]
fn refresh_script_shows_zlib_example_metadata() {
    let output = run_script(&["show", "zlib-zpipe"]);
    assert!(output.contains("fixture=zlib-zpipe"));
    assert!(output.contains("project=zlib"));
    assert!(output.contains("version=v1.3.1"));
    assert!(output.contains("target=test/full_apps/external/zlib/zpipe_example"));
}

#[test]
fn musl_license_note_is_pinned() {
    let path = Path::new("test/full_apps/licenses/musl-MIT.txt");
    assert!(path.is_file());

    let content = fs::read(path).expect("reading musl license file");
    let content = str::from_utf8(&content).expect("license utf-8");
    assert!(content.contains("Permission is hereby granted, free of charge"));
    assert!(content.contains("THE SOFTWARE IS PROVIDED \"AS IS\""));
}

#[test]
fn zlib_license_note_is_pinned() {
    let path = Path::new("test/full_apps/licenses/zlib-License.txt");
    assert!(path.is_file());

    let content = fs::read(path).expect("reading zlib license file");
    let content = str::from_utf8(&content).expect("license utf-8");
    assert!(content.contains("This software is provided 'as-is'"));
    assert!(content.contains("Permission is granted to anyone to use this software"));
}
