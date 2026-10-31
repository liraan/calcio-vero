#!/bin/sh
# Gates A and B - mechanical checks from NEW-SESSION-PROMPT.md closing ritual.
# Exit 0 = green (all mechanical checks pass), 1 = red.
# Run from the repo root. The pre-push hook calls this script.
#
# Manual checks (B2, B4, B5) are NOT reported as "ok": they are listed as
# "manual" and must be done in the closing report. Reporting them as "ok"
# without checking would be a leak (D5).

set -u
fail=0

say_ok()     { printf '  ok      %s\n' "$1"; }
say_fail()   { printf '  FAIL    %s\n' "$1"; fail=1; }
say_manual() { printf '  manual  %s\n' "$1"; }

# ---------- Gate A: documentation coherence ----------
echo "Gate A - documentation coherence"

a_structure=$(grep -cE '^\| 0[0-9][0-9] ' STRUCTURE.md 2>/dev/null || echo 0)
a_index=$(grep -cE '^\| \[0[0-9][0-9]\]' specs/_index.md 2>/dev/null || echo 0)
a_files=$(ls docs/adr/*.md 2>/dev/null | grep -v '000-template' | wc -l | tr -d ' ')
if [ "$a_structure" = "$a_index" ] && [ "$a_index" = "$a_files" ]; then
  say_ok "A1 ADR counts: STRUCTURE=$a_structure, _index=$a_index, files=$a_files"
else
  say_fail "A1 ADR counts diverge: STRUCTURE=$a_structure, _index=$a_index, files=$a_files"
fi

r_badge=$(grep -oE 'tests-[0-9]+_passing' README.md 2>/dev/null | grep -oE '[0-9]+' | head -1)
r_real=$(cargo test --workspace 2>/dev/null | grep 'test result' | awk -F'[ ;]' '{p+=$4} END{print p+0}')
if [ -n "$r_badge" ] && [ "$r_badge" = "$r_real" ]; then
  say_ok "A2 README badge ($r_badge) = real ($r_real)"
else
  say_fail "A2 README badge (${r_badge:-missing}) != real ($r_real)"
fi

a3=$(grep -nE '[0-9]+ (tests?|crates?|ADRs?)\b|(tests?|crates?|ADRs?) [0-9]+' docs/context/NEW-SESSION-PROMPT.md 2>/dev/null | head -5)
if [ -z "$a3" ]; then
  say_ok "A3 no hardcoded counts in NEW-SESSION-PROMPT.md"
else
  say_fail "A3 hardcoded counts found in NEW-SESSION-PROMPT.md:"
  printf '%s\n' "$a3"
fi

# ---------- Gate B: fresh-session simulation (mechanical subset) ----------
echo "Gate B - fresh-session simulation (mechanical subset)"

SS=docs/context/SESSION-STATE.md

# B1. First open step in the Plan has a Prime azioni entry.
# B1: extract the first Plan step number via field split (no emoji dependency).
first_step=$(awk -F'|' 'NF>=6 && $3 ~ /^[[:space:]]*[0-9]+[ab]?[[:space:]]*$/ {gsub(/[[:space:]]/,"",$3); print $3; exit}' "$SS" 2>/dev/null)
if [ -n "$first_step" ] && grep -qE "^\\| $first_step[[:space:]]+\\|" "$SS"; then
  say_ok "B1 first open step ($first_step) has a Prime azioni entry"
else
  say_fail "B1 first open step (${first_step:-none}) has no Prime azioni entry"
fi

# B2. Plan vs Detail order. Requires reading both and comparing. Manual.
say_manual "B2 Plan vs Detail order - check in the closing report"

# B3. OI table present.
oi_count=$(grep -cE '^\| OI-[0-9]+ ' "$SS" 2>/dev/null || echo 0)
if [ "$oi_count" -gt 0 ]; then
  say_ok "B3 OI table present ($oi_count items)"
else
  say_fail "B3 no OI table in SESSION-STATE.md"
fi

# B4. Prime azioni match reality. Requires reading. Manual.
say_manual "B4 Prime azioni match reality - check in the closing report"

# B5. Files touched vs git status. Only meaningful at closing time. Manual.
say_manual "B5 Files touched vs git status - check before the closing commit"

# B6. Proposed ADRs surfaced.
proposed=0
surfaced=0
for f in docs/adr/*.md; do
  s=$(sed -n '3p' "$f" 2>/dev/null)
  if echo "$s" | grep -q 'Status:\*\* proposed$'; then
    proposed=$((proposed + 1))
    id=$(basename "$f" | cut -d- -f1)
    if grep -q "ADR-$id" "$SS"; then surfaced=$((surfaced + 1)); fi
  fi
done
if [ "$proposed" -eq 0 ]; then
  say_ok "B6 no proposed ADR"
elif [ "$surfaced" -eq "$proposed" ]; then
  say_ok "B6 proposed ADRs surfaced ($surfaced/$proposed)"
else
  say_fail "B6 proposed ADRs surfaced ($surfaced/$proposed)"
fi

# B7. Reports in analysis/ cited in the bookmark.
uncited=0
for r in docs/context/analysis/*.md; do
  [ -e "$r" ] || continue
  base=$(basename "$r")
  if ! grep -q "$base" "$SS"; then uncited=$((uncited + 1)); fi
done
if [ "$uncited" -eq 0 ]; then
  say_ok "B7 all reports in analysis/ are cited in SESSION-STATE.md"
else
  say_fail "B7 $uncited report(s) in analysis/ not cited in SESSION-STATE.md"
fi

echo ""
if [ "$fail" -eq 0 ]; then
  echo "gates: GREEN (manual: B2, B4, B5 - do them in the closing report)"
  exit 0
else
  echo "gates: RED - fix before pushing"
  exit 1
fi
