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
