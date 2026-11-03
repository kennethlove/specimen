use std::process::ExitCode;

struct Claim {
    id: String,
    sentence: String,
    check: String,
}

// Returns Err with a ready-to-print message (without the "error: " prefix).
fn parse_claims(path: &str) -> Result<Vec<Claim>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {path}: {e}"))?;
    let mut claims = Vec::new();
    let mut seen: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::new();
    for (i, line) in text.lines().enumerate() {
        let Some(rest) = line.strip_prefix("- [") else {
            continue;
        };
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
        });
    }
    Ok(claims)
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
        let status =
            std::process::Command::new("sh").arg("-c").arg(&claim.check).status();
        let code = match status {
            Ok(s) => s.code().unwrap_or(-1),
            Err(_) => 127,
        };
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
                        "scope": [],
                        "tags": [],
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
