use std::process::Command;

fn run_with_home(args: &[&str], home: &std::path::Path) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_specimen"))
        .args(args)
        .env("HOME", home)
        .output()
        .expect("run specimen");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn fresh_home(tag: &str) -> std::path::PathBuf {
    let home = std::env::temp_dir().join(format!("specimen-home-{tag}"));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(home.join(".config")).expect("home dirs");
    home
}

#[test]
fn config_default_fields_when_missing() {
    let home = fresh_home("defaults");
    let out = std::env::temp_dir().join("specimen-dash-defaults");
    let _ = std::fs::remove_dir_all(&out);
    let (o, e, c) = run_with_home(
        &[
            "dashboard",
            "tests/fixtures/dash-spec.md",
            out.to_str().expect("tmpdir"),
        ],
        &home,
    );
    assert_eq!(c, 0, "stdout: {o} stderr: {e}");
    let page = std::fs::read_to_string(out.join("index.html")).expect("index.html");
    assert!(page.contains("<th>check</th>"), "default columns: {page}");
    assert!(page.contains("<th>status</th>"), "status always: {page}");
}

#[test]
fn config_fields_selects_columns() {
    let home = fresh_home("fields");
    std::fs::write(
        home.join(".config/specimen.toml"),
        "fields = [\"sentence\", \"status\"]\n",
    )
    .expect("write config");
    let out = std::env::temp_dir().join("specimen-dash-fields");
    let _ = std::fs::remove_dir_all(&out);
    let (o, e, c) = run_with_home(
        &[
            "dashboard",
            "tests/fixtures/dash-spec.md",
            out.to_str().expect("tmpdir"),
        ],
        &home,
    );
    assert_eq!(c, 0, "stdout: {o} stderr: {e}");
    let page = std::fs::read_to_string(out.join("index.html")).expect("index.html");
    assert!(page.contains("MUST render"), "sentence kept: {page}");
    assert!(page.contains("<th>status</th>"), "status always: {page}");
    assert!(page.contains("<th>id</th>"), "id always: {page}");
    assert!(!page.contains("<th>check</th>"), "check dropped: {page}");
}

#[test]
fn config_invalid_toml_exits_1_no_render() {
    let home = fresh_home("invalid");
    std::fs::write(home.join(".config/specimen.toml"), "fields = [\n").expect("write config");
    let out = std::env::temp_dir().join("specimen-dash-invalid");
    let _ = std::fs::remove_dir_all(&out);
    let (_o, e, c) = run_with_home(
        &[
            "dashboard",
            "tests/fixtures/dash-spec.md",
            out.to_str().expect("tmpdir"),
        ],
        &home,
    );
    assert_eq!(c, 1, "stderr: {e}");
    assert!(e.contains("specimen.toml"), "names config path: {e}");
    assert!(
        !out.join("index.html").exists(),
        "no render on invalid config"
    );
}
