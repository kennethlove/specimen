use std::process::ExitCode;

struct Claim {
    id: String,
    sentence: String,
    check: String,
    scope: Vec<String>,
    tags: Vec<String>,
}

// Consumes `scope:` / `tags:` markers on a line. Returns false when the
// line holds a marker with no bracketed list (caller warns).
fn apply_markers(line: &str, claim: &mut Claim, path: &str, line_no: usize) -> bool {
    let mut work = line.trim_start();
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
        let after = work[at + key.len()..].trim_start();
        let next = [after.find("scope:"), after.find("tags:")]
            .into_iter()
            .flatten()
            .min();
        let (value, rest) = match next {
            Some(p) => (after[..p].trim(), &after[p..]),
            None => (after.trim(), ""),
        };
        if let Some(inner) = value
            .strip_prefix('[')
            .and_then(|v| v.strip_suffix(']'))
        {
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
    ok
}

// Returns Err with a ready-to-print message (without the "error: " prefix).
fn parse_claims(path: &str) -> Result<Vec<Claim>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {path}: {e}"))?;
    let mut claims = Vec::new();
    let mut seen: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::new();
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
            let body = rest[end + 1..].trim_start();
            let (sentence, check) = match body.split_once(" check: ") {
                Some((s, c)) => (s.trim_end().to_string(), c.trim().to_string()),
                None => {
                    eprintln!("warning: {path}:{}: claim {id} has no check", i + 1);
                    (body.to_string(), String::new())
                }
            };
            claims.push(Claim {
                id: id.to_string(),
                sentence,
                check,
                scope: Vec::new(),
                tags: Vec::new(),
            });
        } else if (line.contains("scope:") || line.contains("tags:"))
            && let Some(claim) = claims.last_mut() {
                apply_markers(line, claim, path, i + 1);
            }
    }
    Ok(claims)
}

// Runs one check string via sh -c; returns its exit code.
fn run_one(check: &str) -> i32 {
    match std::process::Command::new("sh").arg("-c").arg(check).status() {
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
        _ => {
            eprintln!("usage: specimen parse <path>");
            eprintln!("usage: specimen check <path>");
            ExitCode::from(2)
        }
    }
}
