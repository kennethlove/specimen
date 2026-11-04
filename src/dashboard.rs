use std::process::ExitCode;

// Claim text may contain HTML metacharacters; never let it mark up the page.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn project_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}

// Guide constant for the empty state (e04s02): lives in template code, not a file.
const GUIDE: &str = "<section><h2>Getting started</h2><p>Claims live in the \
spec as <code>- [ID] sentence check: command</code>. After editing, run \
<code>specimen dashboard &lt;spec.md&gt; &lt;out-dir&gt;</code> again.</p>\
</section>\n";

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
            return Err(format!(
                "{}: field entries must be strings",
                path.display()
            ));
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
    if field == "sentence" {
        "claim"
    } else {
        field
    }
}

fn failure_page(project: &str, error: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{project} — conformance</title>\n</head>\n<body>\n\
         <h1>{project}</h1>\n<p class=\"error\">{error}</p>\n\
         <p>Fix: verify the spec path and claim syntax, then run the \
command again.</p>\n</body>\n</html>\n",
        project = esc(project),
        error = esc(error)
    )
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

// Pure HTML assembly: empty state (GUIDE) or the claims table.
fn page_html(claims: &[crate::Claim], fields: &[String], project: &str, r: &Report) -> String {
    let total = claims.len();
    let body = if total == 0 {
        format!(
            "<h1>{project}</h1>\n<p>No claims yet.</p>\n{GUIDE}",
            project = esc(project)
        )
    } else {
        let mut header = String::new();
        for f in fields {
            header.push_str(&format!("<th>{}</th>", header_name(f)));
        }
        let mut rows = String::new();
        for (c, status) in claims.iter().zip(&r.statuses) {
            let is_human = c.tags.iter().any(|t| t == "human");
            let chip = if is_human {
                " <span class=\"human\">human</span>"
            } else {
                ""
            };
            let mut cells = String::new();
            for f in fields {
                let cell = match f.as_str() {
                    "id" => format!("{}{chip}", esc(&c.id)),
                    "sentence" => esc(&c.sentence),
                    "check" => esc(&c.check),
                    _ => status.to_string(),
                };
                cells.push_str(&format!("<td>{cell}</td>"));
            }
            rows.push_str(&format!(
                "<tr{row_class}>{cells}</tr>\n",
                row_class = if is_human { " class=\"human\"" } else { "" }
            ));
        }
        let human_part = if r.human > 0 {
            format!(" | human: {}", r.human)
        } else {
            String::new()
        };
        format!(
            "<h1>{project}</h1>\n\
             <p>claims: {total} | passed: {p} | failed: {f}{human_part}</p>\n\
             <table>\n<tr>{header}</tr>\n{rows}</table>\n",
            project = esc(project),
            p = r.passed,
            f = r.failed
        )
    };
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{project} — conformance</title>\n</head>\n<body>\n{body}\
         </body>\n</html>\n",
        project = esc(project)
    )
}

pub(crate) fn run(path: &str, out_dir: &str) -> ExitCode {
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
    let human_part = if report.human > 0 {
        format!(" {} human", report.human)
    } else {
        String::new()
    };
    println!(
        "wrote {index_path}: {total} claims {passed} passed {failed} failed{human_part}",
        total = claims.len(),
        passed = report.passed,
        failed = report.failed
    );
    ExitCode::SUCCESS
}
