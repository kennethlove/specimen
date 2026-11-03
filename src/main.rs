use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) != Some("parse") || args.get(2).is_none() {
        eprintln!("usage: specimen parse <path>");
        return ExitCode::from(2);
    }
    let path = &args[2];
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return ExitCode::from(1);
        }
    };

    let mut claims: Vec<serde_json::Value> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let Some(rest) = line.strip_prefix("- [") else {
            continue;
        };
        let Some(end) = rest.find(']') else {
            continue;
        };
        let id = &rest[..end];
        let body = rest[end + 1..].trim_start();
        let (sentence, check) = match body.split_once(" check: ") {
            Some((s, c)) => (s.trim_end(), c.trim()),
            None => {
                eprintln!(
                    "warning: {path}:{}: claim {id} has no check",
                    i + 1
                );
                (body, "")
            }
        };
        claims.push(serde_json::json!({
            "id": id,
            "sentence": sentence,
            "check": check,
            "scope": [],
            "tags": [],
        }));
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&claims).expect("claims serialize")
    );
    ExitCode::SUCCESS
}
