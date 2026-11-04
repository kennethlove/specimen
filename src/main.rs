use std::process::ExitCode;

mod dashboard;

struct Claim {
    id: String,
    sentence: String,
    check: String,
    scope: Vec<String>,
    tags: Vec<String>,
}

// Consumes `scope:` / `tags:` markers on a line. Returns the line with the
// marker spans removed (claim lines carry markers after `check:`) and false
// when a marker had no bracketed list (caller warns).
fn apply_markers(line: &str, claim: &mut Claim, path: &str, line_no: usize) -> (String, bool) {
    let mut work = line.trim_start();
    let mut cleaned = String::new();
    let mut ok = true;
    loop {
        let scope_at = work.find("scope:");
        let tags_at = work.find("tags:");
        let (at, key) = match (scope_at, tags_at) {
            (Some(s), Some(t)) if s < t => (s, "scope:"),
            (Some(_), Some(t)) => (t, "tags:"),
            (Some(s), None) => (s, "scope:"),
            (None, Some(t)) => (t, "tags:"),
            (None, None) => break,
        };
        cleaned.push_str(&work[..at]);
        let after = work[at + key.len()..].trim_start();
        let next = [after.find("scope:"), after.find("tags:")]
            .into_iter()
            .flatten()
            .min();
        let (value, rest) = match next {
            Some(p) => (after[..p].trim(), &after[p..]),
            None => (after.trim(), ""),
        };
        if let Some(inner) = value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
            let elements: Vec<String> = inner
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            if key == "scope:" {
                claim.scope = elements;
            } else {
                claim.tags = elements;
            }
        } else {
            eprintln!("warning: {path}:{line_no}: bad marker for {key}");
            ok = false;
        }
        work = rest;
    }
    cleaned.push_str(work);
    (cleaned, ok)
}

// Returns Err with a ready-to-print message (without the "error: " prefix).
fn parse_claims(path: &str) -> Result<Vec<Claim>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let mut claims = Vec::new();
    let mut seen: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for (i, line) in text.lines().enumerate() {
        if let Some(rest) = line.strip_prefix("- [") {
            let Some(end) = rest.find(']') else {
                continue;
            };
            let id = &rest[..end];
            if let Some(first) = seen.insert(id, i + 1) {
                return Err(format!(
                    "{path}:{}: duplicate claim {id}, first defined at line {first}",
                    i + 1
                ));
            }
            let mut claim = Claim {
                id: id.to_string(),
                sentence: String::new(),
                check: String::new(),
                scope: Vec::new(),
                tags: Vec::new(),
            };
            let (body, _) = apply_markers(rest[end + 1..].trim_start(), &mut claim, path, i + 1);
            let (sentence, check) = match body.split_once(" check: ") {
                Some((s, c)) => (s.trim_end().to_string(), c.trim().to_string()),
                None => {
                    eprintln!("warning: {path}:{}: claim {id} has no check", i + 1);
                    (body.trim_end().to_string(), String::new())
                }
            };
            claim.sentence = sentence;
            claim.check = check;
            claims.push(claim);
        } else if (line.contains("scope:") || line.contains("tags:"))
            && let Some(claim) = claims.last_mut()
        {
            apply_markers(line, claim, path, i + 1);
        }
    }
    Ok(claims)
}

// Runs one check string via sh -c; returns its exit code.
fn run_one(check: &str) -> i32 {
    match std::process::Command::new("sh")
        .arg("-c")
        .arg(check)
        .status()
    {
        Ok(s) => s.code().unwrap_or(-1),
        Err(_) => 127,
    }
}

fn run_checks(path: &str) -> ExitCode {
    let claims = match parse_claims(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let mut results = Vec::new();
    let mut failed = false;
    for claim in &claims {
        if claim.check.is_empty() {
            results.push(serde_json::json!({
                "id": claim.id,
                "status": "skipped",
                "exit_code": null,
            }));
            continue;
        }
        let code = run_one(&claim.check);
        if code != 0 {
            failed = true;
        }
        results.push(serde_json::json!({
            "id": claim.id,
            "status": if code == 0 { "pass" } else { "fail" },
            "exit_code": code,
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&results).expect("results serialize")
    );
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let sub = args.get(1).map(String::as_str);
    let path = args.get(2).map(String::as_str);
    match (sub, path) {
        (Some("--version" | "-V"), _) => {
            println!("specimen {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        (Some("parse"), Some(path)) => {
            let claims = match parse_claims(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::from(1);
                }
            };
            let json: Vec<serde_json::Value> = claims
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "id": c.id,
                        "sentence": c.sentence,
                        "check": c.check,
                        "scope": c.scope,
                        "tags": c.tags,
                    })
                })
                .collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&json).expect("claims serialize")
            );
            ExitCode::SUCCESS
        }
        (Some("check"), Some(path)) => run_checks(path),
        (Some("dashboard"), Some(path)) => match args.get(3) {
            Some(out_dir) => dashboard::run(path, out_dir),
            None => {
                eprintln!("usage: specimen dashboard <spec.md> <out-dir>");
                ExitCode::from(2)
            }
        },
        _ => {
            eprintln!("usage: specimen parse <path>");
            eprintln!("usage: specimen check <path>");
            eprintln!("usage: specimen dashboard <spec.md> <out-dir>");
            ExitCode::from(2)
        }
    }
}
