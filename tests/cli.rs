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
fn parse_emits_claim_json() {
    let (stdout, stderr, code) = run(&["parse", "tests/fixtures/smoke.md"]);
    assert_eq!(code, 0, "expected exit 0, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is JSON");
    let claims = v.as_array().expect("JSON array");
    assert_eq!(claims.len(), 1, "one claim, stdout: {stdout}");
    assert_eq!(claims[0]["id"], "C1");
    assert_eq!(claims[0]["sentence"], "The CLI SHALL parse claim lines.");
    assert_eq!(claims[0]["check"], "true");
    assert_eq!(claims[0]["scope"], serde_json::json!([]));
    assert_eq!(claims[0]["tags"], serde_json::json!([]));
}

#[test]
fn parse_warns_on_missing_check() {
    let (stdout, stderr, code) = run(&["parse", "tests/fixtures/no-check.md"]);
    assert_eq!(code, 0, "expected exit 0, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is JSON");
    let claims = v.as_array().expect("JSON array");
    assert_eq!(claims.len(), 1, "one claim, stdout: {stdout}");
    assert_eq!(claims[0]["check"], "", "empty check");
    let warn = stderr.to_lowercase();
    assert!(warn.contains("warning"), "stderr: {stderr}");
    assert!(
        stderr.contains("tests/fixtures/no-check.md:2"),
        "warning names file and line, stderr: {stderr}"
    );
}

#[test]
fn parse_rejects_duplicate_id() {
    let (stdout, stderr, code) = run(&["parse", "tests/fixtures/dup.md"]);
    assert_eq!(code, 1, "expected exit 1, stderr: {stderr}");
    assert!(stdout.is_empty(), "no JSON on stdout, got: {stdout}");
    assert!(stderr.contains("duplicate"), "stderr: {stderr}");
    assert!(stderr.contains("dup.md:3"), "stderr: {stderr}");
    assert!(stderr.contains("line 2"), "stderr: {stderr}");
}

#[test]
fn parse_missing_file() {
    let (stdout, stderr, code) = run(&["parse", "tests/fixtures/nope.md"]);
    assert_eq!(code, 1, "expected exit 1, stderr: {stderr}");
    assert!(stdout.is_empty(), "no JSON on stdout, got: {stdout}");
    assert!(stderr.contains("error"), "stderr: {stderr}");
}

#[test]
fn check_all_pass() {
    let (stdout, stderr, code) = run(&["check", "tests/fixtures/passing.md"]);
    assert_eq!(code, 0, "expected exit 0, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is JSON");
    let results = v.as_array().expect("JSON array");
    assert_eq!(results.len(), 1, "one result, stdout: {stdout}");
    assert_eq!(results[0]["id"], "C1");
    assert_eq!(results[0]["status"], "pass");
    assert_eq!(results[0]["exit_code"], 0);
}

#[test]
fn check_failure_exits_nonzero() {
    let (stdout, stderr, code) = run(&["check", "tests/fixtures/failing.md"]);
    assert_eq!(code, 1, "expected exit 1, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is JSON");
    let results = v.as_array().expect("JSON array");
    assert_eq!(results.len(), 1, "one result, stdout: {stdout}");
    assert_eq!(results[0]["id"], "C1");
    assert_eq!(results[0]["status"], "fail");
    assert_eq!(results[0]["exit_code"], 1);
}

#[test]
fn check_mixed_continues_and_skips() {
    let (stdout, stderr, code) = run(&["check", "tests/fixtures/mixed.md"]);
    assert_eq!(code, 1, "fail present, exit 1, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is JSON");
    let results = v.as_array().expect("JSON array");
    assert_eq!(results.len(), 3, "all claims reported, stdout: {stdout}");
    assert_eq!(results[0]["status"], "pass");
    assert_eq!(results[1]["status"], "fail");
    assert_eq!(results[1]["exit_code"], 1);
    assert_eq!(results[2]["status"], "skipped");
    assert!(results[2]["exit_code"].is_null(), "skipped has null code");
    assert!(
        stderr.contains("mixed.md:4"),
        "missing-check warning names line, stderr: {stderr}"
    );
}

#[test]
fn check_bad_command_records_127() {
    let (stdout, stderr, code) = run(&["check", "tests/fixtures/badcmd.md"]);
    assert_eq!(code, 1, "fail present, exit 1, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is JSON");
    let results = v.as_array().expect("JSON array");
    assert_eq!(results[0]["status"], "fail");
    assert_eq!(results[0]["exit_code"], 127);
}

#[test]
fn check_unparsable_input_exits_2() {
    let (stdout, stderr, code) = run(&["check", "tests/fixtures/dup.md"]);
    assert_eq!(code, 2, "input error, exit 2, stderr: {stderr}");
    assert!(stdout.is_empty(), "no JSON on stdout, got: {stdout}");
    assert!(stderr.contains("duplicate"), "stderr: {stderr}");
}

#[test]
fn check_missing_file_exits_2() {
    let (stdout, stderr, code) = run(&["check", "tests/fixtures/nope.md"]);
    assert_eq!(code, 2, "input error, exit 2, stderr: {stderr}");
    assert!(stdout.is_empty(), "no JSON on stdout, got: {stdout}");
    assert!(stderr.contains("error"), "stderr: {stderr}");
}

#[test]
fn check_missing_path_is_usage() {
    let (stdout, stderr, code) = run(&["check"]);
    assert_eq!(code, 2, "usage, exit 2, stderr: {stderr}");
    assert!(stdout.is_empty(), "no JSON on stdout, got: {stdout}");
    assert!(stderr.contains("usage"), "stderr: {stderr}");
}

#[test]
fn version_flag_prints_version() {
    let out = run(&["--version"]);
    assert_eq!(out.2, 0, "exit code");
    assert!(out.0.starts_with("specimen "), "stdout: {}", out.0);
}
