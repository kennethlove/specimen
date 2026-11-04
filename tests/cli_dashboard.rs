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
fn dashboard_empty_state_guides_reader() {
    let dir = std::env::temp_dir().join("specimen-dash-empty");
    let _ = std::fs::remove_dir_all(&dir);
    let (o, e, c) = run(&[
        "dashboard",
        "tests/fixtures/dash-empty.md",
        dir.to_str().expect("tmpdir"),
    ]);
    assert_eq!(c, 0, "stdout: {o} stderr: {e}");
    let page = std::fs::read_to_string(dir.join("index.html")).expect("index.html");
    assert!(page.contains("dash-empty"), "project name: {page}");
    assert!(
        page.to_lowercase().contains("no claims yet"),
        "empty message: {page}"
    );
    assert!(page.contains("Getting started"), "guide: {page}");
}

#[test]
fn dashboard_failure_writes_page_with_fix_hint() {
    let dir = std::env::temp_dir().join("specimen-dash-err");
    let _ = std::fs::remove_dir_all(&dir);
    let (o, e, c) = run(&[
        "dashboard",
        "tests/fixtures/does-not-exist.md",
        dir.to_str().expect("tmpdir"),
    ]);
    assert_eq!(c, 1, "stdout: {o} stderr: {e}");
    let page = std::fs::read_to_string(dir.join("index.html")).expect("index.html");
    assert!(page.contains("does-not-exist"), "project name: {page}");
    assert!(page.contains("cannot read"), "error text: {page}");
    assert!(page.contains("fix"), "fix hint: {page}");
}

#[test]
fn dashboard_human_claim_labeled() {
    let dir = std::env::temp_dir().join("specimen-dash-human");
    let _ = std::fs::remove_dir_all(&dir);
    run(&[
        "dashboard",
        "tests/fixtures/dash-spec.md",
        dir.to_str().expect("tmpdir"),
    ]);
    let page = std::fs::read_to_string(dir.join("index.html")).expect("index.html");
    assert!(page.contains("human"), "human label: {page}");
    assert!(page.contains("human: 1"), "human count: {page}");
}

#[test]
fn dashboard_escapes_claim_text() {
    let dir = std::env::temp_dir().join("specimen-dash-escape");
    let _ = std::fs::remove_dir_all(&dir);
    run(&[
        "dashboard",
        "tests/fixtures/dash-escape.md",
        dir.to_str().expect("tmpdir"),
    ]);
    let page = std::fs::read_to_string(dir.join("index.html")).expect("index.html");
    assert!(page.contains("&lt;b&gt;"), "escaped markup: {page}");
    assert!(!page.contains("<b>"), "no raw tag: {page}");
    assert!(page.contains("&amp;"), "escaped ampersand: {page}");
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
    assert!(page.contains("passed: 2"), "passed count: {page}");
    assert!(page.contains("failed: 1"), "failed count: {page}");
}

#[test]
fn dashboard_usage_shows_open_flag() {
    let (_o, e, c) = run(&["dashboard", "tests/fixtures/dash-spec.md"]);
    assert_eq!(c, 2, "stderr: {e}");
    assert!(e.contains("[--open]"), "stderr: {e}");
}

#[test]
fn dashboard_open_flag_parses_without_browser() {
    // Bad spec + --open: the flag must parse (not usage) and the run must
    // fail before any browser open, so the suite never launches one.
    let dir = std::env::temp_dir().join("specimen-dash-open-bad");
    let _ = std::fs::remove_dir_all(&dir);
    let (_o, e, c) = run(&[
        "dashboard",
        "tests/fixtures/does-not-exist.md",
        dir.to_str().expect("tmpdir"),
        "--open",
    ]);
    assert_eq!(c, 1, "stderr: {e}");
}
