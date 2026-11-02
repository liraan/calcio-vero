#!/bin/sh
# Gates A and B - mechanical checks from NEW-SESSION-PROMPT.md.
# Exit 0 = green (all mechanical checks pass), 1 = red.
# Run from the repo root.
#
# Usage: scripts/gates.sh [startup|closing]
#   closing (default) - full set: A1, A3, B1, B3, B6, B7 + manual B2
#   startup           - fast set: A1, A3, B1, B3, B6, B7 + manual B2
#                        (B2 is only meaningful after work has been done)
#
# Manual checks are NOT reported as "ok": they are listed as "manual" and must
# be done in the report. Reporting them as "ok" without checking would be a
# leak (D5).

set -u
MODE="${1:-closing}"
fail=0

say_ok()     { printf '  ok      %s\n' "$1"; }
say_fail()   { printf '  FAIL    %s\n' "$1"; fail=1; }
say_manual() { printf '  manual  %s\n' "$1"; }

case "$MODE" in
  startup|closing) : ;;
  *) echo "gates: unknown mode '$MODE' (expected startup or closing)" >&2; exit 2 ;;
esac

echo "Gates mode: $MODE"

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

# A2 removed (OI-15). `cargo test` runs in startup step 4 and in C2
# (closing step 1). A third run to compare the README badge with the real
# count is a documentation check, not a simulation invariant: the badge is
# declared in the closing report, where the real output is visible.

a3=$(grep -nE '[0-9]+ (tests?|crates?|ADRs?)\b|(tests?|crates?|ADRs?) [0-9]+' docs/context/NEW-SESSION-PROMPT.md 2>/dev/null | head -5)
if [ -z "$a3" ]; then
  say_ok "A3 no hardcoded counts in NEW-SESSION-PROMPT.md"
else
  say_fail "A3 hardcoded counts found in NEW-SESSION-PROMPT.md:"
  printf '%s\n' "$a3"
fi

# A4 (2026-11-01): crates and ADRs cited in the 4 main docs must match the
# filesystem. Tests are NOT checked here: the real count comes from
# `cargo test` in C2 (closing step 1), and the README badge is the declared
# value, not a second source. Checking the badge against itself is circular.
# Excluded: lines with "as of" (historical).
a4_real_crates=$(ls crates/ 2>/dev/null | grep -c '^pcfm-')
a4_real_adrs=$(ls docs/adr/*.md 2>/dev/null | grep -v '000-template' | wc -l | tr -d ' ')
a4_bad=""
for a4_f in README.md specs/_index.md STRUCTURE.md CONTRIBUTING.md; do
  [ -f "$a4_f" ] || continue
  for a4_p in crates ADRs; do
    case "$a4_p" in
      crates) a4_n=$a4_real_crates ;;
      ADRs) a4_n=$a4_real_adrs ;;
    esac
    a4_hit=$(grep -nE "[0-9]+ ${a4_p}" "$a4_f" 2>/dev/null | grep -v 'as of' | grep -vE "(^|[^0-9])${a4_n} ${a4_p}" | head -2)
    [ -n "$a4_hit" ] && a4_bad="${a4_bad}${a4_f}(${a4_p}=${a4_n}): ${a4_hit}; "
  done
done
if [ -z "$a4_bad" ]; then
  say_ok "A4 crates and ADRs in docs match reality (crates=$a4_real_crates, ADRs=$a4_real_adrs); tests verified in C2"
else
  say_fail "A4 numbers diverge: $a4_bad"
fi

# A4-coherence (2026-11-02): every `N tests` declared in the 4 main docs
# must be the same number. A4 above checks crates and ADRs against the
# filesystem; tests are not checked there (the real count comes from C2,
# and the badge is declared). Coherence is different: the docs must not
# contradict each other. Excluded: lines with "as of" (historical).
a4c_values=""
a4c_bad=""
for a4c_f in README.md specs/_index.md STRUCTURE.md CONTRIBUTING.md; do
  [ -f "$a4c_f" ] || continue
  a4c_hit=$(grep -nE '[0-9]+ tests?' "$a4c_f" 2>/dev/null | grep -v 'as of')
  [ -z "$a4c_hit" ] && continue
  while IFS= read -r a4c_line; do
    a4c_n=$(printf '%s\n' "$a4c_line" | grep -oE '[0-9]+ tests?' | head -1 | grep -oE '[0-9]+')
    [ -z "$a4c_n" ] && continue
    a4c_values="${a4c_values} ${a4c_n}"
  done <<< "$a4c_hit"
done
# A4-coherence extended: the README badge `tests-N_passing` declares a
# number too. It must match the docs. Pattern: the badge has the number
# after `tests-`, not before.
a4c_badge=$(grep -oE 'badge/tests-[0-9]+_passing' README.md 2>/dev/null | grep -oE '[0-9]+' | head -1)
if [ -n "$a4c_badge" ]; then
  a4c_values="${a4c_values} ${a4c_badge}"
fi
a4c_uniq=$(printf '%s\n' "$a4c_values" | grep -oE '[0-9]+' | sort -u | tr '\n' ' ' | sed 's/ $//')
a4c_count=$(printf '%s\n' "$a4c_uniq" | tr ' ' '\n' | grep -c .)
if [ "$a4c_count" -le 1 ]; then
  say_ok "A4-coherence tests count in docs (tests=$a4c_uniq)"
else
  say_fail "A4-coherence tests count diverges in docs: $a4c_uniq"
fi

# ---------- Gate B: fresh-session simulation (mechanical subset) ----------
echo "Gate B - fresh-session simulation (mechanical subset)"

SS=docs/context/SESSION-STATE.md

# B1 (2026-11-02): state check added. Prefer [next], else [open]; [done]
# never counts. Previous version found the first numeric ID regardless
# of state, so "first open step" was imprecise (it returned 11, a
# deferred step).
first_line=$(awk -F'|' 'NF>=7 && $2 ~ /\[next\]/ && $3 ~ /^[[:space:]]*([0-9]+[ab]?|[A-Z]([.][0-9]+)?)[[:space:]]*$/ {print; exit}' "$SS" 2>/dev/null)
kind="next"
if [ -z "$first_line" ]; then
  first_line=$(awk -F'|' 'NF>=7 && $2 ~ /\[open\]/ && $3 ~ /^[[:space:]]*([0-9]+[ab]?|[A-Z]([.][0-9]+)?)[[:space:]]*$/ {print; exit}' "$SS" 2>/dev/null)
  kind="open"
fi
if [ -z "$first_line" ]; then
  say_fail "B1 no open/next step found in the Plan"
else
  step=$(echo "$first_line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/,"",$3); print $3}')
  action=$(echo "$first_line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/,"",$6); print $6}')
  if [ -n "$action" ]; then
    say_ok "B1 first $kind step ($step) has a Prima azione"
  else
    say_fail "B1 first $kind step ($step) has an empty Prima azione"
  fi
fi

say_manual "B2 Plan vs Detail order - check in the report"

oi_count=$(grep -cE '^\| OI-[0-9]+ ' "$SS" 2>/dev/null || echo 0)
if [ "$oi_count" -gt 0 ]; then
  say_ok "B3 OI table present ($oi_count items)"
else
  say_fail "B3 no OI table in SESSION-STATE.md"
fi


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
# E4b (2026-11-02): before a push, the bookmark update is the last commit.
# Mechanised: if there are commits ahead of the upstream, HEAD must touch
# docs/context/SESSION-STATE.md. This failed twice on 2026-11-02: code
# pushed, bookmark not updated first.
if git rev-parse --abbrev-ref '@{u}' >/dev/null 2>&1; then
  e4b_ahead=$(git rev-list --count '@{u}..HEAD' 2>/dev/null || echo 0)
  if [ "${e4b_ahead:-0}" -gt 0 ]; then
    if git show --name-only --format='' HEAD 2>/dev/null | grep -qx 'docs/context/SESSION-STATE.md'; then
      say_ok "E4b bookmark updated before push (HEAD touches SESSION-STATE.md)"
    else
      say_fail "E4b bookmark not updated: HEAD ($(git rev-parse --short HEAD)) does not touch SESSION-STATE.md, $e4b_ahead unpushed commit(s)"
    fi
  else
    say_ok "E4b no unpushed commits"
  fi
else
  say_ok "E4b no upstream configured (skipped)"
fi

if [ "$fail" -eq 0 ]; then
  if [ "$MODE" = "closing" ]; then
    echo "gates: GREEN (closing; manual: B2 - do it in the closing report)"
  else
    echo "gates: GREEN (startup; manual: B2)"
  fi
  exit 0
else
  echo "gates: RED - fix before pushing"
  exit 1
fi
