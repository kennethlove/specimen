#!/bin/sh
# RED/GREEN gate for e02s01 (TDD_VERIFY_CMD): fails while
# skills/conformance/SKILL.md is missing or incomplete.
set -e
test -f skills/conformance/SKILL.md
grep -q 'specimen check' skills/conformance/SKILL.md
grep -q 'pass/fail' skills/conformance/SKILL.md
grep -q 'SKIPPED' skills/conformance/SKILL.md
grep -q 'cargo build' skills/conformance/SKILL.md
grep -q 'reviewer' skills/conformance/SKILL.md
