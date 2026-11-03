<!-- story: e01s03 -->

STORY KEY: e01s03
TITLE:     Extract tags and scope
TYPE:      Story
PARENT:    e01
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      XS

### 1. Business narrative [draft]

Claims carry classification: tags such as `test` or `invariant`
record how a claim is enforced, and the scope line records where it
applies. The e02 skill filters and groups claims by these fields, so
the parser must deliver them as arrays — empty when a claim has none,
never null and never a raw unparsed string.

This story freezes the extraction rules: bracketed lists, comma
separated, defaults to empty. It completes the JSON contract that
e01s01 started.

### 2. Value statement [draft]

As a conformance skill, I want tags and scope as arrays on every
claim, so that I can filter claims without re-parsing markdown.

### 3. Actors and permissions [draft]

- Conformance skill (system) — filters claims by tags and scope.
- Reviewer (internal) — inspects the same arrays in the JSON.

### 4. Trigger and preconditions [draft]

Trigger: `specimen parse` or `specimen check` processes a claim
line that may carry `scope:` and `tags:` markers.
Precondition: the claim line already matched the e01s01 layout.

### 5. Main flow and business logic [draft]

1. After the id and sentence, the parser looks for a `scope:`
   marker followed by a bracketed list.
2. It splits the list text on commas, trims whitespace around each
   element, and drops empty elements.
3. It repeats the same steps for a `tags:` marker.
4. Markers are consumed so they never leak into `sentence` or
   `check`.
5. Claims with no marker get an empty array for that field.

Example:

```text
- [C2] The system SHALL record outcomes. check: cargo test
  scope: [repo] tags: [test, invariant]
```

yields `"scope": ["repo"]` and `"tags": ["test", "invariant"]`.

Interruption point: N/A — parsing has no resumable state.

### 6. Alternative flows and exceptions [draft]

6a. Marker present with empty brackets (`tags: []`) — field is an
empty array, no warning.
6b. Marker present without a bracketed list — warning to stderr
naming file and line, field treated as absent, exit unaffected.
6c. Elements contain internal spaces (`[out of scope]`) — only
comma edges are trimmed; the element text is preserved.
6d. Claim has no scope or tags markers — both fields are empty
arrays, exit 0.

### 7. Interface elements [draft]

Context: new
Static elements: field markers `scope:` and `tags:` in claim lines
Dynamic elements: stderr warnings for malformed markers

### 8. Domain model [draft]

Entities touched: Claim (e01s01).
Entities changed: Claim.scope and Claim.tags become parsed arrays
on every emitted object.
Relationships: none added.

### 9. Integrations and boundaries [draft]

- Spec markdown file (perennial, direction: in) — same path
  argument as parse.
- Claims JSON (perennial, direction: out) — arrays consumed by the
  e02 skill.

### 10. Background processes [draft]

Not applicable — extraction runs inside the parse invocation.

### 11. Notifications [draft]

Not applicable — no notification channels; warnings go to stderr.

### 12. Audit and logging [draft]

Audited entity: extraction. Fields: file path, line number of each
malformed marker, resulting arrays. stderr only.

### 13. Solution variabilities [draft]

Not applicable — marker names are fixed, not configurable.

### 14. Quality attributes *NFR* [draft]

- Extraction adds no more than 20 ms to a 1000-line parse.
- Every emitted claim object always contains both `scope` and
  `tags` keys, always arrays.

### 15. Security and compliance *NFR* [draft]

- AuthN: none — local CLI, no network access.
- AuthZ: filesystem permissions of the invoking user.
- Data class: none — tags are project vocabulary, not personal
  data.
- Controls: no shell execution in this story.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — CLI, no graphical interface.
- i18n: stderr messages in English only.
- Machine contract: arrays in, arrays out; never null.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Claim with tags and scope
  Given a claim line with scope: [repo] and tags: [test, invariant]
  When  the user runs specimen parse on that file
  Then  the claim object has scope ["repo"]
  And   the claim object has tags ["test", "invariant"]

Scenario: Empty bracket list (6a)
  Given a claim line with tags: []
  When  the user runs specimen parse on that file
  Then  the claim object has tags []
  And   no warning is printed
  And   the exit code is 0

Scenario: Marker without brackets (6b)
  Given a claim line where tags: is not followed by brackets
  When  the user runs specimen parse on that file
  Then  the claim object has tags []
  And   a warning naming file and line is printed to stderr
  And   the exit code is 0

Scenario: Element with internal spaces (6c)
  Given a claim line with tags: [out of scope]
  When  the user runs specimen parse on that file
  Then  the claim object has tags ["out of scope"]

Scenario: Claim without markers (6d)
  Given a claim line with no scope or tags markers
  When  the user runs specimen parse on that file
  Then  the claim object has scope [] and tags []
  And   the exit code is 0
```

### 18. Out of scope [draft]

- Freezing the tag vocabulary itself (project decides as specs
  arrive).
- Filtering or grouping by tags (e02 skill).
- A `scope:` prose sentence form; only the bracketed list form is
  parsed.

### 19. Open questions [draft]

- Whether the scope line and the `[out-of-scope]` tag are one
  concept with two encodings — owner: kenneth, needed by: first
  real spec.

### 20. References [draft]

- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Tag, Scope line, out-of-
  scope flag).
- specs/epics/e01-checker-cli/e01s01-parse-claims.md (JSON shape).
- docs/brainstorm/spec-tool-requirements.md (tags requirement).
