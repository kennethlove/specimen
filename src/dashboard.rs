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

pub(crate) fn run(path: &str, out_dir: &str) -> ExitCode {
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

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut human = 0usize;
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
        let is_human = c.tags.iter().any(|t| t == "human");
        let chip = if is_human {
            human += 1;
            " <span class=\"human\">human</span>"
        } else {
            ""
        };
        rows.push_str(&format!(
            "<tr{row_class}><td>{id}{chip}</td><td>{sentence}</td>\
             <td>{check}</td><td>{status}</td></tr>\n",
            row_class = if is_human { " class=\"human\"" } else { "" },
            id = esc(&c.id),
            chip = chip,
            sentence = esc(&c.sentence),
            check = esc(&c.check),
            status = status
        ));
    }

    let total = claims.len();
    let summary = if total == 0 {
        format!(
            "<h1>{project}</h1>\n<p>No claims yet.</p>\n{GUIDE}",
            project = esc(&project)
        )
    } else {
        let human_part = if human > 0 {
            format!(" | human: {human}")
        } else {
            String::new()
        };
        format!(
            "<h1>{project}</h1>\n\
             <p>claims: {total} | passed: {passed} | failed: {failed}{human_part}</p>\n\
             <table>\n<tr><th>id</th><th>claim</th><th>check</th><th>status</th></tr>\n\
             {rows}</table>\n",
            project = esc(&project),
            rows = rows
        )
    };
    let page = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{project} — conformance</title>\n</head>\n<body>\n{summary}\
         </body>\n</html>\n",
        project = esc(&project)
    );
    let index_path = format!("{out_dir}/index.html");
    if let Err(e) = std::fs::write(&index_path, &page) {
        eprintln!("error: cannot write {index_path}: {e}");
        return ExitCode::from(1);
    }
    let human_part = if human > 0 {
        format!(" {human} human")
    } else {
        String::new()
    };
    println!(
        "wrote {index_path}: {total} claims {passed} passed {failed} failed{human_part}"
    );
    ExitCode::SUCCESS
}
