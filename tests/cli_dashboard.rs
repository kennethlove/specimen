use std::process::Command;

fn run(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_specimen"))
        .args(args)
        .output()
        .expect("run specimen");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

#[test]
fn dashboard_usage_names_subcommand() {
    let (_o, e, c) = run(&["dashboard"]);
    assert_eq!(c, 2, "stderr: {e}");
    assert!(e.contains("usage: specimen dashboard"), "stderr: {e}");
}

#[test]
fn dashboard_renders_index() {
    let dir = std::env::temp_dir().join("specimen-dash-e2e");
    let _ = std::fs::remove_dir_all(&dir);
    let (o, _e, c) = run(&[
        "dashboard",
        "tests/fixtures/dash-spec.md",
        dir.to_str().expect("tmpdir"),
    ]);
    assert_eq!(c, 0, "stdout: {o}");
    let page = std::fs::read_to_string(dir.join("index.html")).expect("index.html");
    assert!(page.contains("dash-spec"), "project name: {page}");
    assert!(page.contains("MUST render"), "claim sentence: {page}");
    assert!(page.contains("claims:"), "summary: {page}");
}

#[test]
fn dashboard_counts_pass_and_fail() {
    let dir = std::env::temp_dir().join("specimen-dash-counts");
    let _ = std::fs::remove_dir_all(&dir);
    run(&[
        "dashboard",
        "tests/fixtures/dash-spec.md",
        dir.to_str().expect("tmpdir"),
    ]);
    let page = std::fs::read_to_string(dir.join("index.html")).expect("index.html");
    assert!(page.contains("passed: 1"), "passed count: {page}");
    assert!(page.contains("failed: 1"), "failed count: {page}");
}
