#!/bin/sh
set -e
test -f skills/grill/SKILL.md
grep -q 'callers' skills/grill/SKILL.md
grep -q 'check:' skills/grill/SKILL.md
grep -q 'scope:' skills/grill/SKILL.md
grep -q 'specs/' skills/grill/SKILL.md
echo 'grill e03s01 verifies: ok'
