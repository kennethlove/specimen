use std::process::Command;

fn run(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_specimen"))
        .args(args)
        .output()
        .expect("specimen runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().expect("exit code"),
    )
}

// spec: e01s03-tags-scope.md §6a/c/d — markers parse into arrays
#[test]
fn tags_scope_markers_parse() {
    let (stdout, stderr, code) = run(&["parse", "tests/fixtures/tags.md"]);
    assert_eq!(code, 0, "parse succeeds, stderr: {stderr}");
    assert!(stderr.is_empty(), "no warnings expected: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("JSON");
    let claims = v.as_array().expect("array");
    assert_eq!(claims.len(), 4, "four claims, stdout: {stdout}");
    let c1 = &claims[0];
    assert_eq!(c1["scope"], serde_json::json!(["api", "cli"]));
    assert_eq!(c1["tags"], serde_json::json!(["core"]));
    // §6c bracketed value keeps internal spaces
    let c2 = &claims[1];
    assert_eq!(c2["tags"], serde_json::json!(["out of scope"]));
    // §6a empty markers are empty arrays, no warning
    let c3 = &claims[2];
    assert_eq!(c3["scope"], serde_json::json!([]));
    assert_eq!(c3["tags"], serde_json::json!([]));
    // §6d absent markers are empty arrays, no warning
    let c4 = &claims[3];
    assert_eq!(c4["scope"], serde_json::json!([]));
    assert_eq!(c4["tags"], serde_json::json!([]));
}

// spec: e01s03-tags-scope.md §6b — marker without brackets warns, yields []
#[test]
fn tags_scope_bad_marker_warns() {
    let (stdout, stderr, code) = run(&["parse", "tests/fixtures/badmarker.md"]);
    assert_eq!(code, 0, "warning only, exit 0, stderr: {stderr}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("JSON");
    let claims = v.as_array().expect("array");
    assert_eq!(
        claims[0]["scope"],
        serde_json::json!([]),
        "bad marker -> []"
    );
    assert_eq!(claims[0]["tags"], serde_json::json!([]), "no tags either");
    assert!(
        stderr.contains("badmarker.md:3") && stderr.contains("bad marker"),
        "warning names line 3, stderr: {stderr}"
    );
}
