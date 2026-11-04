use std::process::ExitCode;

// WHY: maud over hand-built `format!` strings — compile-time HTML macros
// with automatic escaping, which replaces the old `esc()` helper. User
// chose maud for the dashboard templates (m01220).
use maud::{DOCTYPE, html};

fn project_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}

// WHY: `toml` crate (added on e04s03) over hand-rolled key=value parsing —
// stdlib has no TOML reader and the config format is TOML by author decision
// (`~/.config/specimen.toml`). Field names are matched against this
// allowlist, never evaluated as code.
const VALID_FIELDS: [&str; 4] = ["id", "sentence", "check", "status"];
const DEFAULT_FIELDS: [&str; 4] = ["id", "sentence", "check", "status"];

fn default_fields() -> Vec<String> {
    DEFAULT_FIELDS.iter().map(|s| s.to_string()).collect()
}

// Loads the ordered display field list from ~/.config/specimen.toml.
// Missing file or missing HOME -> defaults (silent). Present file -> strict:
// any problem is an Err the caller turns into exit 1 with no render.
fn load_fields() -> Result<Vec<String>, String> {
    let Some(home) = std::env::var_os("HOME") else {
        return Ok(default_fields());
    };
    let path = std::path::PathBuf::from(home).join(".config/specimen.toml");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(default_fields()),
        Err(e) => return Err(format!("cannot read {}: {e}", path.display())),
    };
    // toml 1.x: Value::from_str parses a bare value, Table parses a document.
    let cfg: toml::Table = text
        .parse()
        .map_err(|e| format!("invalid config {}: {e}", path.display()))?;
    let Some(list) = cfg.get("fields").and_then(toml::Value::as_array) else {
        return Err(format!("{}: missing `fields` list", path.display()));
    };
    let mut fields = Vec::new();
    for value in list {
        let Some(name) = value.as_str() else {
            return Err(format!("{}: field entries must be strings", path.display()));
        };
        if !VALID_FIELDS.contains(&name) {
            return Err(format!(
                "{}: unknown field {name}; valid fields: {}",
                path.display(),
                VALID_FIELDS.join(", ")
            ));
        }
        if !fields.iter().any(|f| f == name) {
            fields.push(name.to_string());
        }
    }
    if fields.is_empty() {
        return Err(format!("{}: `fields` is empty", path.display()));
    }
    // Structural: every row must show which claim and whether it passed.
    if !fields.iter().any(|f| f == "id") {
        fields.insert(0, "id".to_string());
    }
    if !fields.iter().any(|f| f == "status") {
        fields.push("status".to_string());
    }
    Ok(fields)
}

fn header_name(field: &str) -> &str {
    if field == "sentence" { "claim" } else { field }
}

fn failure_page(project: &str, error: &str) -> String {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title { (project) " — conformance" }
            }
            body {
                h1 { (project) }
                p class="error" { (error) }
                p { "Fix: verify the spec path and claim syntax, then run the command again." }
            }
        }
    }
    .into_string()
}

struct Report {
    statuses: Vec<&'static str>,
    passed: usize,
    failed: usize,
    human: usize,
}

// Runs every claim check sequentially (document order) and records results.
fn evaluate(claims: &[crate::Claim]) -> Report {
    let mut report = Report {
        statuses: Vec::with_capacity(claims.len()),
        passed: 0,
        failed: 0,
        human: 0,
    };
    for c in claims {
        report.statuses.push(if c.check.is_empty() {
            "skipped"
        } else if crate::run_one(&c.check) == 0 {
            report.passed += 1;
            "pass"
        } else {
            report.failed += 1;
            "fail"
        });
        if c.tags.iter().any(|t| t == "human") {
            report.human += 1;
        }
    }
    report
}

// One wording for page and stdout — decided 2026-11-04 (state.yaml).
fn summary_line(total: usize, passed: usize, failed: usize, human: usize) -> String {
    let human_part = if human > 0 {
        format!(" | human: {human}")
    } else {
        String::new()
    };
    format!("claims: {total} | passed: {passed} | failed: {failed}{human_part}")
}

// Pure HTML assembly: empty state (Getting started guide) or claims table.
fn page_html(claims: &[crate::Claim], fields: &[String], project: &str, r: &Report) -> String {
    let summary = summary_line(claims.len(), r.passed, r.failed, r.human);
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title { (project) " — conformance" }
            }
            body {
                h1 { (project) }
                @if claims.is_empty() {
                    p { "No claims yet." }
                    section {
                        h2 { "Getting started" }
                        p {
                            "Claims live in the spec as "
                            code { "- [ID] sentence check: command" }
                            ". After editing, run "
                            code { "specimen dashboard <spec.md> <out-dir>" }
                            " again."
                        }
                    }
                } @else {
                    p { (summary) }
                    table {
                        tr { @for f in fields { th { (header_name(f)) } } }
                        @for (c, status) in claims.iter().zip(&r.statuses) {
                            tr class=(if c.tags.iter().any(|t| t == "human") {
                                "human"
                            } else {
                                ""
                            }) {
                                @for f in fields {
                                    td {
                                        @match f.as_str() {
                                            "id" => {
                                                (c.id)
                                                @if c.tags.iter().any(|t| t == "human") {
                                                    span class="human" { "human" }
                                                }
                                            }
                                            "sentence" => { (c.sentence) }
                                            "check" => { (c.check) }
                                            _ => { (status) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    .into_string()
}

// WHY: stdlib has no "open in browser" API; the per-OS launcher is the
// native platform feature (ladder rung 5). A failure warns only — the
// page is already written.
fn open_in_browser(path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let program = "open";
    #[cfg(target_os = "linux")]
    let program = "xdg-open";
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    return Err("opening a browser is unsupported on this platform".to_string());
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    std::process::Command::new(program)
        .arg(path)
        .spawn()
        .map(drop)
        .map_err(|e| e.to_string())
}

pub(crate) fn run(path: &str, rest: &[String]) -> ExitCode {
    // Flag anywhere after the positionals; exactly one out-dir required.
    let open = rest.iter().any(|a| a == "--open");
    let dirs: Vec<&str> = rest
        .iter()
        .filter(|a| a.as_str() != "--open")
        .map(String::as_str)
        .collect();
    if dirs.len() != 1 {
        eprintln!("usage: specimen dashboard <spec.md> <out-dir> [--open]");
        return ExitCode::from(2);
    }
    let out_dir = dirs[0];
    // Config is checked before anything renders: invalid config -> exit 1,
    // no page (e04s03 §6).
    let fields = match load_fields() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    let project = project_name(path);
    let claims = match crate::parse_claims(path) {
        Ok(c) => c,
        Err(e) => {
            // Fail loudly (C6): report AND write the failure page, then exit 1.
            let _ = std::fs::create_dir_all(out_dir);
            let index_path = format!("{out_dir}/index.html");
            let page = failure_page(&project, &e);
            if let Err(we) = std::fs::write(&index_path, &page) {
                eprintln!("error: cannot write {index_path}: {we}");
                return ExitCode::from(1);
            }
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    if let Err(e) = std::fs::create_dir_all(out_dir) {
        eprintln!("error: cannot create {out_dir}: {e}");
        return ExitCode::from(1);
    }

    let report = evaluate(&claims);
    let page = page_html(&claims, &fields, &project, &report);
    let index_path = format!("{out_dir}/index.html");
    if let Err(e) = std::fs::write(&index_path, &page) {
        eprintln!("error: cannot write {index_path}: {e}");
        return ExitCode::from(1);
    }
    let summary = summary_line(claims.len(), report.passed, report.failed, report.human);
    println!("wrote {index_path}: {summary}");
    if open && let Err(e) = open_in_browser(&index_path) {
        eprintln!("warning: cannot open {index_path}: {e}");
    }
    ExitCode::SUCCESS
}
