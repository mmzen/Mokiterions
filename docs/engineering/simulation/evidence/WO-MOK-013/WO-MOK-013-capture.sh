#!/usr/bin/env bash
# Captures every dependency measurement WO-MOK-013 retains, into the directory holding this file.
#
# Run from the workspace root:
#     bash docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-capture.sh
#
# Nothing here resolves anything: every cargo invocation passes --locked, and every one after the
# fetch passes --offline as well, so the figures below describe the committed lockfile and not a
# resolution taken on the day of the capture. The script is retained so that each figure in
# SPEC-MOK-002 rule 13, SPEC-MOK-003's declared set and VER-MOK-013 is reproducible from a
# recorded command rather than trusted.
#
# The determinism capture is NOT here. It reuses evidence/WO-MOK-011/capture.sh unchanged, which
# is the point: the baseline is a retained capture and re-running it with a script written today
# would make the comparison circular. See WO-MOK-013-determinism.txt for the two commands.
set -u

# Retained evidence is UTF-8. Without this, a Python process writing to a redirect on Windows
# encodes with the console code page, and every em dash quoted out of an artifact lands as a lone
# 0x97 byte -- valid cp1252, invalid UTF-8, and unreadable to everything that reads this directory.
export PYTHONIOENCODING=utf-8

here="$(cd "$(dirname "$0")" && pwd)"
targets="x86_64-pc-windows-msvc x86_64-unknown-linux-gnu aarch64-apple-darwin"

{
  echo "# WO-MOK-013: resolved dependency graphs, per package and per target"
  echo "# $(cargo --version), $(rustc --version)"
  echo "# host $(rustc -vV | sed -n 's/^host: //p')"
  echo
  for package in Mokiterions mokiterions-tui; do
    for target in $targets; do
      edges=normal
      [ "$package" = Mokiterions ] && edges=normal,build,dev
      echo "\$ cargo tree -p $package -e $edges --locked --offline --target $target --prefix none --no-dedupe -f '{p}|{f}'"
      cargo tree -p "$package" -e "$edges" --locked --offline --target "$target" \
        --prefix none --no-dedupe -f '{p}|{f}' | sort -u
      echo
    done
  done
} > "$here/WO-MOK-013-graphs.txt"

# A crate is one (name, version) pair, not one name: `hashbrown` and `syn` are each in the
# observer's graph at two versions, so a count taken over names is two short of the declared
# figure on every target. That is the difference between 55/61/60 and 57/63/62, and it is the
# reason this section counts pairs and prints the duplicated names underneath.
external() {
  cargo tree -p mokiterions-tui -e normal --locked --offline --target "$1" \
    --prefix none --no-dedupe | sort -u | grep -v -e '^Mokiterions v' -e '^mokiterions-tui v'
}

{
  echo "# WO-MOK-013: the figures a reader might confuse, each with the question it answers"
  echo
  echo "\"How many crates does a build of the observer compile on this target?\""
  for target in $targets; do
    echo "  observer, external crates on $target: $(external "$target" | grep -c .)"
  done
  echo
  echo "\"How many distinct crates across the three targets SPEC-MOK-005 rule 10 builds?\""
  echo "  observer, union of the three: $(
    for target in $targets; do external "$target"; done | sort -u | grep -c .
  )"
  echo
  echo "\"How many could any target reach?\" -- not a build of anything, and not the declared set."
  echo "  observer, --target all: $(
    cargo tree -p mokiterions-tui -e normal --locked --offline --target all \
      --prefix none --no-dedupe | sort -u \
      | grep -v -e '^Mokiterions v' -e '^mokiterions-tui v' | grep -c .
  )"
  echo
  echo "\"How many packages does the lockfile hold?\" -- includes crates no build reaches."
  echo "  cargo metadata --locked packages: $(
    cargo metadata --locked --offline --format-version 1 \
      | python -c 'import json,sys; print(len(json.load(sys.stdin)["packages"]))'
  )"
  echo
  echo "Crates present at more than one version, per target (the pair-vs-name difference):"
  for target in $targets; do
    echo "  $target:"
    external "$target" | awk '{print $1}' | sort | uniq -d | sed 's/^/    /'
    external "$target" | grep -E "^($(external "$target" | awk '{print $1}' | sort | uniq -d | paste -sd'|' -)) v" \
      | sed 's/^/      /'
  done
  echo
  echo "Engine, crates in the graph at -e normal,build,dev (itself and nothing else):"
  for target in $targets; do
    echo "  $target: $(
      cargo tree -p Mokiterions -e normal,build,dev --locked --offline --target "$target" \
        --prefix none --no-dedupe | grep -c .
    )"
  done
} > "$here/WO-MOK-013-counts.txt"

{
  echo "# WO-MOK-013: build-time code-execution surface, per ADR-MOK-006 decision 13"
  echo "# A crate is listed when any of its targets has kind custom-build in cargo metadata."
  echo "#"
  echo "# Four counts are reconstructible here and they answer four different questions. The"
  echo "# declared table records the per-target ones, because those are what a build executes."
  echo
  python - "$targets" <<'PY'
import json, subprocess, sys

def cargo(*arguments):
    return subprocess.run(["cargo", *arguments], capture_output=True, text=True, check=True).stdout

metadata = json.loads(cargo("metadata", "--locked", "--offline", "--format-version", "1"))
scripted = {
    f"{p['name']} {p['version']}"
    for p in metadata["packages"]
    for t in p["targets"]
    if "custom-build" in t["kind"]
}

def reached(package, edges, target):
    tree = cargo("tree", "-p", package, "-e", edges, "--locked", "--offline",
                 "--target", target, "--prefix", "none", "--no-dedupe")
    crates = set()
    for line in tree.splitlines():
        parts = line.split()
        if len(parts) >= 2 and parts[1].startswith("v"):
            crates.add(f"{parts[0]} {parts[1][1:]}")
    return crates

targets = sys.argv[1].split()
union = set()
for target in targets:
    hits = sorted(reached("mokiterions-tui", "normal", target) & scripted)
    union.update(hits)
    print(f"{target}: {len(hits)}")
    for hit in hits:
        print(f"    {hit}")
print(f"\nunion of the three targets: {len(union)}")
for hit in sorted(union):
    print(f"    {hit}")

everywhere = reached("mokiterions-tui", "normal", "all") & scripted
print(f"\nobserver at --target all: {len(everywhere)}")
for hit in sorted(everywhere):
    print(f"    {hit}")

print(f"\npackages in the lockfile carrying a build script: {len(scripted)}")
for hit in sorted(scripted):
    print(f"    {hit}")

anywhere = set()
for package in ("Mokiterions", "mokiterions-tui"):
    anywhere |= reached(package, "normal,build,dev", "all")
unreachable = sorted(scripted - anywhere)
print(f"\nin the lockfile, carrying a build script, reachable from neither package at")
print(f"--target all and -e normal,build,dev: {len(unreachable)}")
for hit in unreachable:
    print(f"    {hit}")
PY
  echo
  echo "The two the amendment rows single out, as cargo reports them:"
  for crate in syn@1.0.109 thiserror@1.0.69; do
    echo "  \$ cargo tree -i $crate -e normal,build,dev --locked --offline --target all"
    cargo tree -i "$crate" -e normal,build,dev --locked --offline --target all 2>&1 | sed 's/^/    /'
  done
} > "$here/WO-MOK-013-build-scripts.txt"

{
  echo "# WO-MOK-013: resolved feature sets, against SPEC-MOK-003's declaration"
  echo
  for target in $targets; do
    echo "$target:"
    cargo tree -p mokiterions-tui -e normal --locked --offline --target "$target" \
      --prefix none --no-dedupe -f '{p}|{f}' | sort -u \
      | grep -E '^(ratatui|mio|signal-hook-mio|crossterm) ' | sed 's/^/  /'
  done
} > "$here/WO-MOK-013-features.txt"

{
  echo "# WO-MOK-013: the by-name scan of SPEC-MOK-005 rule 8.4d, before disclosure"
  echo "#"
  echo "# The terms are read out of the checking program, so this file cannot disagree with what"
  echo "# actually ran. They were written from the prohibitions ADR-MOK-006 decision 4 preserved,"
  echo "# not from the present graph: a term list derived from the graph would pass by construction."
  echo "# The hits below are what the scan sees BEFORE SPEC-MOK-003's disclosure table is consulted."
  echo
  python - <<'PY'
import importlib.util, pathlib, sys

spec = importlib.util.spec_from_file_location(
    "check_declared_dependencies", pathlib.Path("scripts/check_declared_dependencies.py")
)
check = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = check  # dataclasses resolves cls.__module__ through sys.modules
spec.loader.exec_module(check)
root = pathlib.Path(".")

print("Prohibited capability classes and their terms:")
for capability, terms in check.PROHIBITED_TERMS.items():
    print(f"  {capability} ({len(terms)} terms)")
    print(f"    {', '.join(terms)}")
print(f"\nUser-interface terms, scanned against the engine's graph only ({len(check.USER_INTERFACE_TERMS)}):")
print(f"    {', '.join(check.USER_INTERFACE_TERMS)}")

targets = ["x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu", "aarch64-apple-darwin"]
graphs = {
    ("Mokiterions", t): check.cargo_tree(root, "Mokiterions", "normal,build,dev", t) for t in targets
}
graphs.update(
    {("mokiterions-tui", t): check.cargo_tree(root, "mokiterions-tui", "normal", t) for t in targets}
)

print("\nRaw hits, per package and target:")
any_hit = False
for (package, target), crates in graphs.items():
    for capability, terms in check.PROHIBITED_TERMS.items():
        for crate in check.scan_names(crates, terms):
            any_hit = True
            print(f"  {package} on {target}: {crate.identity} -- {capability}")
    for crate in (check.scan_names(crates, check.USER_INTERFACE_TERMS) if package == "Mokiterions" else []):
        any_hit = True
        print(f"  {package} on {target}: {crate.identity} -- user interface")
if not any_hit:
    print("  none")

print("\nWhy signal-hook-mio matches: name_tokens splits on - and _, so")
print(f"  signal-hook-mio -> {sorted(check.name_tokens('signal-hook-mio'))}")
print("  and the token `mio` is in the network-access list. Substring matching was not used;")
print("  token matching was, and the near misses below are the reason.")

every_term = {t for terms in check.PROHIBITED_TERMS.values() for t in terms} | set(
    check.USER_INTERFACE_TERMS
)
print("\nNear misses: a crate whose name contains a term as a substring but not as a token.")
seen = set()
for crates in graphs.values():
    for crate in crates:
        lowered = crate.name.lower()
        tokens = check.name_tokens(crate.name)
        inside = sorted(t for t in every_term if t in lowered and t not in tokens)
        if inside and crate.name not in seen:
            seen.add(crate.name)
            print(f"  {crate.name}: contains {', '.join(inside)}")
if not seen:
    print("  none")

print("\nPositive control: the same token rule applied to names it must and must not catch.")
print("A refusal here would be the scan working; a silence on the first four would be the bug")
print("the near-miss list above raises, since none of them is the literal term `ratatui`.")
for name in ("ratatui", "ratatui-core", "ratatui-widgets", "ratatui-crossterm", "windows-link",
             "tokio", "signal-hook", "windows-sys"):
    fake = [check.Crate(name=name, version="0.0.0", features=frozenset())]
    ui = bool(check.scan_names(fake, check.USER_INTERFACE_TERMS))
    classes = sorted(
        capability for capability, terms in check.PROHIBITED_TERMS.items()
        if check.scan_names(fake, terms)
    )
    print(f"  {name}: user interface {'HIT' if ui else 'no'}"
          f"; prohibited classes {', '.join(classes) if classes else 'none'}")

print("\nSPEC-MOK-003's disclosure table, as the checking program reads it:")
declaration = check.read_declaration(root, "mokiterions-tui", "SPEC-MOK-003", "Declared dependency set")
for name, disclosure in sorted(declaration.disclosures.items()):
    # The same two-way the checking program prints, and for the same reason: a disclosure whose
    # assessment has been recorded must still appear, or an acceptance would be indistinguishable
    # from the disclosure never having existed.
    state = "OUTSTANDING" if "outstanding" in disclosure.assessment.lower() else "accepted"
    print(f"  {name} {disclosure.version} on {', '.join(sorted(disclosure.labels))}"
          f" -- {disclosure.capability} -- assessment {state}")
    print(f"      assessment as SPEC-MOK-003 states it: {' '.join(disclosure.assessment.split())}")
print("\nResidue after disclosure: the run in WO-MOK-013-check-run.txt refuses on nothing, so every")
print("raw hit above is a disclosed row and no hit is unaccounted for. A disclosure matching no hit")
print("would refuse as well, which is what keeps the table from outliving the graph.")
PY
} > "$here/WO-MOK-013-scan.txt"

{
  echo "# WO-MOK-013: the declared-set comparison, full run"
  echo "\$ python scripts/check_declared_dependencies.py --root ."
  python scripts/check_declared_dependencies.py --root . 2>&1
  echo "exit $?"
} > "$here/WO-MOK-013-check-run.txt"

{
  echo "# WO-MOK-013: the checking program's own test suite"
  echo "\$ python -m unittest discover -s scripts -p 'test_check_declared*.py' -v"
  python -m unittest discover -s scripts -p 'test_check_declared*.py' -v 2>&1
  echo "exit $?"
} > "$here/WO-MOK-013-check-tests.txt"

echo "captured into $here"
