<!-- story: e04s03 -->

STORY KEY: e04s03
TITLE:     Field config
TYPE:      Story
PARENT:    e04
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-04
MATURITY:  3
SIZE:      S

### 1. Business narrative [draft]

Everyone wants to see a different slice of a claim: the
reviewer wants statuses, the author wants checks, the auditor
wants tags. Hardcoding the columns makes every future request a
code change. This story puts column selection in
`~/.config/specimen.toml` — the author's chosen path — so adding
or dropping displayed fields is an edit, not a release. Missing
config means defaults; broken config means loud failure.

### 2. Value statement [draft]

As a dashboard user, I want displayed fields selectable in a
config file, so that my dashboard shows my columns without
touching code.

### 3. Actors and permissions [draft]

- Dashboard user (internal) — edits `~/.config/specimen.toml`.
- Coding agent (system) — loads config at render time.

### 4. Trigger and preconditions [draft]

Trigger: `specimen dashboard` runs.
Preconditions: e04s01 render path exists; `HOME` resolvable for
config lookup.

### 5. Main flow and business logic [draft]

1. Resolve `~/.config/specimen.toml`.
2. Missing file → use default field set (id, sentence, check,
   status); proceed silently.
3. File present → parse TOML; key `fields` = ordered list of
   field names; rendered rows show exactly those fields in that
   order, plus claim id and status always (never hide which row
   failed — status is structural).
4. File present but invalid (parse error, unknown field name) →
   print the error to stderr with the config path, exit 1, no
   render. Fail loudly (author answer 2).

Interruption point: none.

### 6. Alternative flows and exceptions [draft]

6a. Config file absent — defaults, exit unaffected.
6b. `fields = []` — empty list is invalid (a dashboard with no
    columns is meaningless) → error, exit 1.
6c. Unknown field name → error listing valid names, exit 1.
6d. Unreadable file (permissions) → same loud path as 6b-c:
    stderr, exit 1.

### 7. Interface elements [draft]

Context: existing render from e04s01.
Static elements: config file (user-owned TOML).
Dynamic elements: column set and order.

### 8. Domain model [draft]

Entities touched: Claim, Check, Tag, Dashboard page.
Entities created: Field config (ordered field selection).
Relationships: config constrains rendering, never content —
claims parsed are unchanged.

### 9. Integrations and boundaries [draft]

- TOML file at `~/.config/specimen.toml` (perennial, direction:
  in) — read once per run.
- Render pipeline (perennial, direction: in) — field list
  applied at row build.

### 10. Background processes [draft]

Not applicable.

### 11. Notifications [draft]

Not applicable — errors via stderr.

### 12. Audit and logging [draft]

Audited entity: render inputs. Fields: config path used,
resolved field list. Recorded: stdout summary line appends
`fields=<n>`.

### 13. Solution variabilities [draft]

- Valid field names may grow; unknown-name error lists the
  current set (self-describing).
- Config path fixed by the author: `~/.config/specimen.toml`.

### 14. Quality attributes *NFR* [draft]

- No config file ever required for a working dashboard
  (defaults path is the zero-setup path).
- Invalid config never renders a partial dashboard (fail
  loudly).

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: reads only the user's own config path; no env-var
  path override in MVP.
- Data class: none.
- Controls: TOML parsed by the `toml` crate — no eval, no
  string templating from config values (field names are matched
  against an allowlist, never interpolated as code).

### 16. UX and accessibility *NFR* [draft]

- Not applicable — column mechanics, not visual design.
- i18n: English only (error strings).

### 17. Acceptance criteria [draft]

```gherkin
Scenario: No config file (6a)
  Given no ~/.config/specimen.toml
  When  the dashboard runs
  Then  default fields render
  And   the exit code is 0

Scenario: Config selects fields
  Given a config with fields = ["sentence", "status"]
  When  the dashboard runs
  Then  rows show sentence and status only
  And   claim id and status remain present

Scenario: Invalid TOML (4)
  Given a config that does not parse
  When  the dashboard runs
  Then  stderr names the config path and the error
  And   no index.html is written
  And   the exit code is 1

Scenario: Empty fields list (6b)
  Given fields = []
  When  the dashboard runs
  Then  stderr reports the empty field list as invalid
  And   the exit code is 1

Scenario: Unknown field name (6c)
  Given fields = ["sentance"]
  When  the dashboard runs
  Then  stderr lists valid field names
  And   the exit code is 1
```

### 18. Out of scope [draft]

- Config-driven colors, layout, or guide text.
- Per-project config (user-level path only).
- Agent JSON interface (post-MVP).

### 19. Open questions [draft]

- Whether `fields` supports group aliases (e.g. `all`) — owner:
  kenneth, needed by: second config user.

### 20. References [draft]

- specs/product/conformance-dashboard.md (C8 + config decision
  `~/.config/specimen.toml`).
- author batch answers: fail loudly, fields via config.
