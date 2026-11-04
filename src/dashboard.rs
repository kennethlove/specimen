use std::process::ExitCode;

// Claim text may contain HTML metacharacters; never let it mark up the page.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub(crate) fn run(path: &str, out_dir: &str) -> ExitCode {
    let claims = match crate::parse_claims(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    if let Err(e) = std::fs::create_dir_all(out_dir) {
        eprintln!("error: cannot create {out_dir}: {e}");
        return ExitCode::from(1);
    }

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut rows = String::new();
    for c in &claims {
        let status = if c.check.is_empty() {
            "skipped"
        } else if crate::run_one(&c.check) == 0 {
            passed += 1;
            "pass"
        } else {
            failed += 1;
            "fail"
        };
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            esc(&c.id),
            esc(&c.sentence),
            esc(&c.check),
            status
        ));
    }

    let project = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(path);
    let total = claims.len();
    let index = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{project} — conformance</title>\n</head>\n<body>\n<h1>{project}</h1>\n\
         <p>claims: {total} | passed: {passed} | failed: {failed}</p>\n\
         <table>\n<tr><th>id</th><th>claim</th><th>check</th><th>status</th></tr>\n\
         {rows}</table>\n</body>\n</html>\n",
        project = esc(project),
        total = total,
        passed = passed,
        failed = failed,
        rows = rows
    );
    let index_path = format!("{out_dir}/index.html");
    if let Err(e) = std::fs::write(&index_path, &index) {
        eprintln!("error: cannot write {index_path}: {e}");
        return ExitCode::from(1);
    }
    println!(
        "wrote {index_path}: {total} claims {passed} passed {failed} failed"
    );
    ExitCode::SUCCESS
}
