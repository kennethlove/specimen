<!-- story: e04s02 -->

STORY KEY: e04s02
TITLE:     States and highlights
TYPE:      Story
PARENT:    e04
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-04
MATURITY:  3
SIZE:      S

### 1. Business narrative [draft]

A dashboard that renders nothing says nothing. The two moments
a reviewer needs words are the ones with no rows: nothing has
run yet, and the run itself broke. This story gives both states
real pages — an empty state that tells the reader the project is
empty and how to get started, and a failure state that names the
project, the error, and how to fix it — plus the one highlight
that changes how claims read: human claims stand out, because a
reviewer must know instantly which claims no machine will prove.

### 2. Value statement [draft]

As a reviewer, I want meaningful empty and failure pages and
human claims marked, so that every dashboard state answers "what
now?" without a second tool.

### 3. Actors and permissions [draft]

- Reviewer (internal) — reads states, acts on hints.
- Coding agent (system) — renders states, highlights rows.

### 4. Trigger and preconditions [draft]

Trigger: `specimen dashboard <spec.md> <out-dir>` produces zero
claims, or the run fails before claims can execute (unreadable
spec, parse error), or renders claims containing `[human]` tags.
Precondition: e04s01 render path exists.

### 5. Main flow and business logic [draft]

1. Empty state: zero claims parsed → write `index.html`
   containing the project name, an explicit "no claims yet"
   message, and the getting-started guide (C5). Guide text is a
   constant inside the HTML template code — no external file.
2. Failure state: the run errors before results exist → still
   write `index.html` with the project name, the error text
   verbatim, and a fix hint (how to fix), then exit 1 — fail
   loudly (C6): page exists AND the command fails.
3. Highlight: each rendered claim row carrying a `human` tag
   gets a distinct visual treatment (CSS class plus a visible
   `human` label — never color alone) (C4).
4. Summary counts include a human count line when any claim is
   tagged human.

Interruption point: none — states are pure render branches.

### 6. Alternative flows and exceptions [draft]

6a. Empty state page also exits 0 — nothing is wrong, nothing
    ran.
6b. Failure state with an unwritable out dir — stderr error,
    exit 1 (cannot both report and write).
6c. Claim tagged `[human]` and passing — still highlighted; the
    highlight marks provenance, not status.
6d. Guide text changes — edit the template constant; no config
    in this story.

### 7. Interface elements [draft]

Context: new
Static elements: empty-state block, guide text, error block,
fix hint, human label chip
Dynamic elements: error message text, human count

### 8. Domain model [draft]

Entities touched: Claim, Tag, Spec.
Entities created: Empty state (zero-claim render), Failure state
(error render), Human marker (visual flag over tagged claims).
Relationships: every page shows project name; states are
mutually exclusive branches of one run.

### 9. Integrations and boundaries [draft]

- Render pipeline from e04s01 (perennial, direction: in) — state
  branches hook before row rendering.
- Template constant (perennial, direction: in) — guide text
  source.

### 10. Background processes [draft]

Not applicable.

### 11. Notifications [draft]

Not applicable — states are in-page.

### 12. Audit and logging [draft]

Audited entity: run outcome. Fields: state (empty|failure|ok),
error text, exit code. Recorded: stderr error line + stdout
summary.

### 13. Solution variabilities [draft]

- Guide text is a constant (author decision); moving it to a
  config file is post-MVP.
- Highlight styling (CSS) may change; the `human` label text
  stays for greppability.

### 14. Quality attributes *NFR* [draft]

- Failure state never yields an empty file or a success exit
  (author: fail loudly, always).
- Human marker visible without color (text label present).

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: same as e04s01 — write only `<out-dir>`.
- Data class: none.
- Controls: error text HTML-escaped before embedding.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — structure only; visual design not built here.
- i18n: English only.
- Human label textual, not color-only (WCAG redundancy).

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Empty state (C5)
  Given a spec that parses to zero claims
  When  the command runs
  Then  index.html contains the project name
  And   an explicit no-claims-yet message
  And   a Getting started guide
  And   the exit code is 0

Scenario: Failure state (C6)
  Given a run that fails before results exist
  When  the command runs
  Then  index.html contains the project name, the error text,
    and how to fix it
  And   the exit code is 1

Scenario: Human claim highlight (C4)
  Given a spec with a claim tagged [human]
  When  the command renders it
  Then  the claim row carries a visible `human` label
  And   its summary line counts it as human

Scenario: Guide lives in the template (6d)
  Given the guide text
  When  the source is searched
  Then  the text appears in the template code, not an external
    file

Scenario: Failure plus unwritable dir (6b)
  Given a failure state and an unwritable out dir
  When  the command runs
  Then  stderr reports the write error
  And   the exit code is 1
```

### 18. Out of scope [draft]

- Config-driven fields (e04s03).
- Color palette / final visual design.
- Retry or partial-failure pages (single error page only).

### 19. Open questions [draft]

- Fix-hint wording per error kind (first: unreadable spec) —
  owner: kenneth, needed by: first render review.

### 20. References [draft]

- specs/product/conformance-dashboard.md (C4, C5, C6).
- author batch answers: fail loudly always, guide in templates.
