#!/usr/bin/env python3
"""Phase D of VER-MOK-019: the static and architecture checks T1 to T9, and the security checks.

`ADR-MOK-008`'s substance is that no amendment is required. T1 to T8 are the checkable form of that
claim, and this phase is where it becomes falsifiable rather than asserted. The security bullets are
here too, because both are measurements over the tree and the instruments rather than over a sweep.

Run from anywhere; every path is absolute.
"""

from __future__ import annotations

import ast
import glob
import hashlib
import json
import os
import re
import secrets
import shutil
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from oracle import (  # noqa: E402
    CLASSIFIER,
    DRIVER,
    RELEASE,
    REPO,
    WORK,
    Report,
    classifier,
    driver,
    floats_in,
    read_jsonl,
    run,
    sha256_bytes,
)

OUT = f"{WORK}/phase-d"

# The base commit. `ADR-MOK-008`'s measurement was taken here and it is `main`'s tip before this
# chain's two governance commits, so it is the tree T1 and T2 mean by "unchanged from the base".
BASE = "c90edc9"

INSTRUMENTS = (DRIVER, CLASSIFIER)
INSTRUMENT_FILES = (
    DRIVER,
    CLASSIFIER,
    f"{REPO}/scripts/test_run_simulation_batch.py",
    f"{REPO}/scripts/test_classify_simulation_runs.py",
)

# The paths rule 1's layout enumerates, which is what T1's structural census measures.
CENSUS_PATHS = (
    "Cargo.toml",
    "Cargo.lock",
    "mokiterions-core",
    "mokiterions-tui",
)
PRODUCT_PATTERNS = (
    re.compile(r"^mokiterions-core/src/"),
    re.compile(r"^mokiterions-tui/src/"),
    re.compile(r"^Cargo\.toml$"),
    re.compile(r"^mokiterions-core/Cargo\.toml$"),
    re.compile(r"^mokiterions-tui/Cargo\.toml$"),
    re.compile(r"^Cargo\.lock$"),
)


def git(arguments, cwd=REPO):
    return run(["git"] + list(arguments), cwd=cwd)


def keep(name, text):
    os.makedirs(OUT, exist_ok=True)
    with open(f"{OUT}/{name}", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text if text.endswith("\n") else text + "\n")


def source_of(path):
    with open(path, "r", encoding="utf-8") as handle:
        return handle.read()


_RETAINED_CACHE = []


def retained_outputs():
    """Every retained batch output this contract has produced, for the pattern checks.

    The phase directories also hold record streams and, for B9, a stream with one byte altered, so a
    file is admitted only when it parses and its records are a retained output's three kinds. A
    stream is not retained output and the altered one is not valid JSON at all.
    """
    if _RETAINED_CACHE:
        return _RETAINED_CACHE
    paths = [f"{WORK}/default-sweep.jsonl"]
    for phase in ("phase-a", "phase-b", "phase-c"):
        paths.extend(sorted(glob.glob(f"{WORK}/{phase}/*.jsonl")))
    for path in paths:
        if not os.path.isfile(path):
            continue
        try:
            records = read_jsonl(path)
        except (ValueError, UnicodeDecodeError):
            continue
        if records and all(r.get("record") in ("sweep", "cell", "batch") for r in records):
            _RETAINED_CACHE.append(path)
    return _RETAINED_CACHE


# ==================================================================================================
# T1 and T2: the package layout, and the product's bytes
# ==================================================================================================


def case_t1_t2(report):
    _, members_raw, _ = git(["show", f"{BASE}:Cargo.toml"])
    current = source_of(f"{REPO}/Cargo.toml")
    declared = re.search(r"members\s*=\s*\[([^\]]*)\]", current)
    names = re.findall(r'"([^"]+)"', declared.group(1)) if declared else []

    manifests = sorted(
        path.replace("\\", "/")
        for path in glob.glob(f"{REPO}/**/Cargo.toml", recursive=True)
        if "/target/" not in path.replace("\\", "/")
    )
    relative = [path[len(REPO) + 1:] for path in manifests]

    report.record(
        "T1 no third package",
        names == ["mokiterions-core", "mokiterions-tui"]
        and current == members_raw
        and relative == ["Cargo.toml", "mokiterions-core/Cargo.toml", "mokiterions-tui/Cargo.toml"],
        f"workspace members {names}; the root manifest is byte-identical to {BASE}'s; "
        f"{len(relative)} Cargo.toml files exist in the tree and they are {relative}",
    )

    # The structural census: every path rule 1's layout names, with its blob digest, at both
    # revisions. Comparing blob digests rather than a `git diff` name list means a path that moved
    # and a path whose content changed are both caught, and the worktree is pinned to HEAD first so
    # that an uncommitted edit cannot hide behind a clean tree comparison.
    census = {}
    for revision in (BASE, "HEAD"):
        status, listing, _ = git(["ls-tree", "-r", revision, "--"] + list(CENSUS_PATHS))
        rows = []
        for line in listing.splitlines():
            if not line.strip():
                continue
            meta, path = line.split("\t", 1)
            rows.append((path, meta.split()[2]))
        census[revision] = sorted(rows)
    _, dirty, _ = git(["status", "--porcelain", "--"] + list(CENSUS_PATHS))

    layout = {
        "engine src": sorted(
            p for p, _ in census["HEAD"] if p.startswith("mokiterions-core/src/")),
        "engine tests": sorted(
            p for p, _ in census["HEAD"] if p.startswith("mokiterions-core/tests/")),
        "observer src": sorted(
            p for p, _ in census["HEAD"] if p.startswith("mokiterions-tui/src/")),
        "observer tests": sorted(
            p for p, _ in census["HEAD"] if p.startswith("mokiterions-tui/tests/")),
    }
    keep("t1-census.txt", "\n".join(
        [f"census of SPEC-MOK-004 rule 1's layout, path and blob digest", f"base {BASE}", ""]
        + [f"{digest}  {path}" for path, digest in census["HEAD"]]
        + ["", "counts at HEAD"]
        + [f"  {name}: {len(paths)}" for name, paths in layout.items()]
        + ["", "worktree status over these paths: " + (dirty.strip() or "clean")]
    ))
    report.record(
        "T1 the structural census is unchanged",
        census[BASE] == census["HEAD"] and not dirty.strip(),
        f"{len(census['HEAD'])} paths under rule 1's layout, blob for blob identical to {BASE}, "
        f"with the worktree clean over all of them: engine src {len(layout['engine src'])} files, "
        f"engine tests {len(layout['engine tests'])}, observer src {len(layout['observer src'])}, "
        f"observer tests {len(layout['observer tests'])}",
        note=(
            "Rule 1's layout sketch says the engine's tests are five files and the observer's eight. "
            "The tree carries more, and every arrival is recorded in that specification's own "
            "amendment record, which took the engine's and the observer's public-tier target counts "
            "to nine each under SPEC-MOK-002 rule 8's closing sentence. The divergence between the "
            "sketch and the tree predates this work order by six commits and is not caused by it; "
            "what T1 measures is identity with the base, which holds."
        ),
    )

    _, changed, _ = git(["diff", "--name-only", BASE])
    _, untracked, _ = git(["ls-files", "--others", "--exclude-standard"])
    touched = sorted(set(changed.split()) | set(untracked.split()))
    offenders = [path for path in touched if any(p.search(path) for p in PRODUCT_PATTERNS)]
    keep("t2-touched.txt", "\n".join(
        [f"git diff --name-only {BASE}, plus untracked files", ""] + touched))
    report.record(
        "T2 no product file is touched",
        not offenders,
        f"{len(touched)} paths differ from {BASE} or are untracked -- {touched} -- and none is "
        f"under either package's src/, either Cargo.toml, the root Cargo.toml or Cargo.lock; "
        f"offenders {offenders or 'none'}",
    )


# ==================================================================================================
# T3: the schema, and an existing retained record-stream digest
# ==================================================================================================


TRANSCRIPT = "mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl"
# The two record-stream digests WO-MOK-025's evidence binds, at
# docs/engineering/simulation/evidence/WO-MOK-025/candidate/replay-identity.txt:132.
BOUND_STREAMS = {
    "traceon sink": ("467e02feddd3468574e7df77d2881ac0f21b54fea10265e54dfa0d8ef2874557", 139638),
    "traceoff sink": ("15c22de2e82e53cf271e789877d88dd7cafbe35dfaf76118a6b76326e67e06ad", 77820),
}
BOUND_STDOUT = {
    "traceon": ("e35cc3a107a2abaa1885a1c2e423fc98e842438b2800f66deb2066d83c035772", 78315),
    "traceoff": ("a9c1da0d05378c83964dd7cdc09776a627c94c934a98ce8b88b07380319932b5", 39543),
}


def case_t3(report):
    fresh = f"{OUT}/t3-fresh-stream.jsonl"
    os.makedirs(OUT, exist_ok=True)
    run([RELEASE, "--seed", "0", "--ticks", "20", "--policy", "reference", "--density", "0.75",
         "--events-path", fresh])
    header = read_jsonl(fresh)[0]
    _, spec_diff, _ = git(
        ["diff", BASE, "--", "docs/engineering/simulation/specifications/SPEC-MOK-006.md"])

    report.record(
        "T3 the schema is still 3",
        header.get("record") == "header" and header.get("schema") == 3 and not spec_diff.strip(),
        f"a freshly produced stream's first record is a header stating schema "
        f"{header.get('schema')!r}, and SPEC-MOK-006 is byte-identical to {BASE} "
        f"({len(spec_diff)} bytes of diff)",
    )

    # An existing retained record-stream digest, re-verified by reproducing the stream rather than by
    # re-reading the file that records it. The configuration is the one that evidence file names: a
    # transcript replay, which reaches no provider and needs no credential -- it is what
    # .github/workflows/provider-credentials.yml runs on every push.
    lines = []
    faults = []
    for label, trace in (("traceon", True), ("traceoff", False)):
        path = f"{OUT}/t3-replay-{label}.jsonl"
        argv = [RELEASE, "--policy", "llm", "--transcript-path", TRANSCRIPT,
                "--seed", "0", "--ticks", "20", "--events-path", path]
        if trace:
            argv.append("--trace-actions")
        completed = subprocess.run(argv, cwd=REPO, capture_output=True)
        with open(path, "rb") as handle:
            raw = handle.read()
        got = (sha256_bytes(raw), len(raw))
        want = BOUND_STREAMS[f"{label} sink"]
        lines.append(f"{label} sink   {got[0]}  {got[1]:>7}  recorded {want[0]}  {want[1]:>7}")
        if got != want:
            faults.append(f"{label} stream {got} != recorded {want}")

        out = (sha256_bytes(completed.stdout), len(completed.stdout))
        wanted_out = BOUND_STDOUT[label]
        lines.append(f"{label} stdout {out[0]}  {out[1]:>7}  recorded {wanted_out[0]}  "
                     f"{wanted_out[1]:>7}")
        if out != wanted_out:
            faults.append(f"{label} stdout {out} != recorded {wanted_out}")
        if completed.returncode != 0 or completed.stderr:
            faults.append(f"{label} exited {completed.returncode} with "
                          f"{len(completed.stderr)} bytes on standard error")

    keep("t3-bound-digests.txt", "\n".join(
        ["WO-MOK-025 candidate/replay-identity.txt:132's two record streams, and the two standard",
         "outputs above them, reproduced at this candidate from the committed transcript.", "",
         "cell           sha256                                                             bytes",
         ] + lines))
    report.record(
        "T3 an existing bound record-stream digest reproduces",
        not faults,
        f"both record-stream digests WO-MOK-025's evidence binds reproduce at this candidate, "
        f"with their byte counts, and so do the two standard-output digests above them; "
        f"faults {faults or 'none'}",
    )


# ==================================================================================================
# T4: declared dependency sets
# ==================================================================================================


DEPENDENCY_TABLE = re.compile(
    r"^\[(?:build-|dev-)?dependencies(?:\.[^\]]+)?\]$.*?(?=^\[|\Z)", re.M | re.S)


def case_t4(report):
    status, out, err = run([sys.executable, f"{REPO}/scripts/check_declared_dependencies.py",
                            "--root", "."])
    keep("t4-declared-dependencies.txt",
         f"$ python scripts/check_declared_dependencies.py --root .\nexit {status}\n\n{out}{err}")

    sets = {}
    for revision in (BASE, "HEAD"):
        for package in ("mokiterions-core", "mokiterions-tui"):
            _, text, _ = git(["show", f"{revision}:{package}/Cargo.toml"])
            sets[(revision, package)] = DEPENDENCY_TABLE.findall(text)
    same = all(sets[(BASE, package)] == sets[("HEAD", package)]
               for package in ("mokiterions-core", "mokiterions-tui"))
    declared = {
        package: re.findall(r"^([A-Za-z0-9_-]+)\s*=", "\n".join(sets[("HEAD", package)]), re.M)
        for package in ("mokiterions-core", "mokiterions-tui")
    }
    report.record(
        "T4 declared dependency sets unchanged",
        status == 0 and same,
        f"scripts/check_declared_dependencies.py exited {status}; every dependency table of both "
        f"package manifests is byte-identical to {BASE}'s, and the declared sets are "
        f"{ {p: v for p, v in declared.items()} }",
    )


# ==================================================================================================
# T5: standard library only
# ==================================================================================================


def case_t5(report):
    # A test file imports the instrument it tests. That is the subject, not a dependency: both
    # instruments' own imports are checked in the same pass and neither imports the other or anything
    # outside the interpreter's standard set, so admitting the two subject names does not widen what
    # T5 constrains. Everything else must resolve to a standard-library module.
    subjects = {"run_simulation_batch", "classify_simulation_runs"}
    allowed = set(sys.stdlib_module_names) | {"__future__"} | subjects
    imports = {}
    foreign = []
    for path in INSTRUMENT_FILES:
        tree = ast.parse(source_of(path), filename=path)
        names = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                names.update(alias.name.split(".")[0] for alias in node.names)
            elif isinstance(node, ast.ImportFrom):
                if node.level:
                    names.add(f"<relative level {node.level}>")
                elif node.module:
                    names.add(node.module.split(".")[0])
        imports[os.path.basename(path)] = sorted(names)
        foreign.extend(f"{os.path.basename(path)}: {name}"
                       for name in sorted(names) if name not in allowed)
    keep("t5-imports.txt", "\n".join(
        [f"checked against sys.stdlib_module_names of {sys.version.split()[0]} "
         f"({len(sys.stdlib_module_names)} names), plus __future__ and the two subject modules", ""]
        + [f"{name}\n  " + " ".join(names) for name, names in imports.items()]))
    report.record(
        "T5 standard library only",
        not foreign,
        f"every import of all four files resolves to the running interpreter's own standard module "
        f"set: {sum(len(v) for v in imports.values())} import names over 4 files, foreign "
        f"{foreign or 'none'}",
    )


# ==================================================================================================
# T6: the instruments are not Cargo targets
# ==================================================================================================


def case_t6(report):
    references = []
    incidental = []
    for manifest in ("Cargo.toml", "mokiterions-core/Cargo.toml", "mokiterions-tui/Cargo.toml"):
        text = source_of(f"{REPO}/{manifest}")
        for number, line in enumerate(text.splitlines(), 1):
            for name in ("run_simulation_batch", "classify_simulation_runs"):
                if name in line:
                    references.append(f"{manifest}:{number} names {name!r}")
            # A pre-existing comment in each manifest points at the declared-dependency checker. It
            # is a different script, it is a comment rather than a target declaration, and it was
            # there at the base commit; recorded so that the absence of the two instruments is a
            # measurement over a manifest that does mention a script, not over one that mentions
            # none.
            if "scripts/" in line and line.lstrip().startswith("#"):
                incidental.append(f"{manifest}:{number}: {line.strip()}")

    status, meta, _ = run(["cargo", "metadata", "--no-deps", "--format-version", "1", "--locked"])
    parsed = json.loads(meta)
    targets = sorted(
        f"{package['name']} {target['kind'][0]} {target['name']} "
        f"{target['src_path'].replace(os.sep, '/')[len(REPO) + 1:]}"
        for package in parsed["packages"] for target in package["targets"]
    )
    keep("t6-target-census.txt", "\n".join(
        ["cargo metadata --no-deps --locked, one line per target", ""] + targets
        + ["", "pre-existing comment references to a script in a manifest, at the base commit too:"]
        + (incidental or ["  none"])))
    report.record(
        "T6 the instruments are not targets",
        not references and status == 0
        and not any(".py" in line or "scripts/" in line for line in targets),
        f"neither instrument is named by any of the three Cargo.toml files, and the target census "
        f"reads {len(targets)} targets, none of them a Python file; findings "
        f"{references or 'none'} (with {len(incidental)} pre-existing comment reference(s) to the "
        f"declared-dependency checker, retained above)",
        note=(
            "The census is enumerated at the candidate and shown unchanged by the byte-identity of "
            "every input that determines it -- both package manifests, the root manifest and every "
            "file under either package's src/ and tests/ -- which T1's blob-digest census "
            "establishes against the base. That is a substitution for diffing two `cargo metadata` "
            "outputs, and a stronger one: a manifest edit that left the target list identical would "
            "pass the diff and fail T1. VER-MOK-019 cites SPEC-MOK-002 rule 5 for the census; that "
            "rule records the library target's authorized public interface rather than a target "
            "list, so the enumeration is taken from cargo itself and the discrepancy is recorded "
            "rather than resolved by reinterpreting the bullet."
        ),
    )


# ==================================================================================================
# T7: nothing writes into governance
# ==================================================================================================


def case_t7(report):
    forbidden = f"docs/engineering/simulation/evidence/WO-MOK-033/must-not-appear.jsonl"
    absolute = f"{REPO}/{forbidden}"
    if os.path.exists(absolute):
        os.remove(absolute)
    status, out, err = driver(["--out", forbidden, "--sources", "reference", "--densities", "0.75",
                              "--seeds", "0", "--ticks", "5"])
    written = os.path.exists(absolute)

    patterns = []
    for path in INSTRUMENTS:
        text = source_of(path)
        for pattern in ("docs/engineering", "docs\\\\engineering", "verification-records"):
            if pattern in text:
                # A refusal condition naming the path is not a write path; the distinction is which
                # side of `open` it appears on, so each occurrence is reported with its line.
                for number, line in enumerate(text.splitlines(), 1):
                    if pattern in line:
                        patterns.append(f"{os.path.basename(path)}:{number}: {line.strip()}")
    writes = [line for line in patterns if re.search(r"\bopen\(|write|mkdir|makedirs", line)]
    keep("t7-governance.txt", "\n".join(
        [f"$ python scripts/run_simulation_batch.py --out {forbidden} ...", f"exit {status}",
         f"file created: {written}", "", out, err, "",
         "occurrences of a governance path in either instrument:", ] + (patterns or ["none"])))
    report.record(
        "T7 nothing writes into governance",
        status == 2 and not written and not writes,
        f"a batch whose --out named a path under docs/engineering/ exited {status} and created "
        f"nothing; {len(patterns)} occurrence(s) of a governance path in the two instruments, all "
        f"of them refusal conditions rather than write paths, and {len(writes)} on a write call",
    )


# ==================================================================================================
# T8: no retreat field, and the vocabulary gap
# ==================================================================================================


def case_t8(report):
    retreat = re.compile(rb"retreat", re.I)
    hits = []
    scanned = 0
    for path in retained_outputs():
        with open(path, "rb") as handle:
            raw = handle.read()
        scanned += len(raw)
        if retreat.search(raw):
            hits.append(path)

    # The gap is narrower than "the word does not appear", and stating it as that would understate
    # it. `retreat` IS one of SPEC-MOK-001's seven targeted actions. What does not exist is a
    # `retreat_resolved` event kind, so an outcome the engine can produce leaves no record an
    # instrument could count. Both halves are measured: the resolved-event vocabulary is enumerated,
    # and the verb list is shown to contain the action.
    spec = f"{REPO}/docs/engineering/simulation/specifications/SPEC-MOK-001.md"
    spec_text = source_of(spec)
    vocabulary = sorted(set(re.findall(r"\b([a-z_]+_resolved)\b", spec_text)))
    verbs = sorted(set(re.findall(r"`(approach|avoid|threaten|attack|fight|retreat|surrender)`",
                                 spec_text)))
    _, spec_diff, _ = git(["diff", BASE, "--", spec[len(REPO) + 1:]])
    _, roadmap, _ = run(["git", "grep", "-n", "-i", "retreat", "HEAD", "--", "docs/ROADMAP.md"])

    # The engine's own stream, over the whole 400-cell sweep's worth of coordinates, is the second
    # half: an event kind absent from the specification could still be emitted.
    emitted = set()
    for path in sorted(glob.glob(f"{WORK}/phase-a/streams/*.jsonl")):
        for record in read_jsonl(path):
            if record.get("record") == "event":
                emitted.add(record["event"])

    keep("t8-retreat.txt", "\n".join([
        f"{len(retained_outputs())} retained outputs scanned, {scanned} bytes, "
        f"occurrences of /retreat/i: {len(hits) or 'none'}",
        "",
        f"SPEC-MOK-001's resolved-event vocabulary ({len(vocabulary)} kinds):",
        "  " + " ".join(vocabulary),
        f"  retreat_resolved present: {'retreat_resolved' in vocabulary}",
        "",
        f"SPEC-MOK-001's targeted actions, as the specification names them ({len(verbs)}):",
        "  " + " ".join(verbs),
        f"  retreat present as an action: {'retreat' in verbs}",
        "",
        f"event kinds the engine actually emitted, over {len(glob.glob(f'{WORK}/phase-a/streams/*.jsonl'))} "
        f"real streams ({len(emitted)} kinds):",
        "  " + " ".join(sorted(emitted)),
        f"  any kind naming retreat: "
        f"{[kind for kind in sorted(emitted) if 'retreat' in kind] or 'none'}",
        "",
        f"SPEC-MOK-001 against {BASE}: {len(spec_diff)} bytes of diff",
        "",
        "docs/ROADMAP.md, searched for retreat:",
        roadmap.strip() or "  no match",
    ]))
    report.record(
        "T8 no retreat field is synthesised",
        not hits and "retreat_resolved" not in vocabulary
        and not [kind for kind in emitted if "retreat" in kind],
        f"no retained byte of {len(retained_outputs())} outputs ({scanned} bytes) contains a field "
        f"or value named for retreat; SPEC-MOK-001's resolved-event vocabulary is "
        f"{vocabulary} and declares no retreat_resolved; and the engine emitted "
        f"{len(emitted)} kinds over the real streams, none of them naming retreat",
        note=(
            "The measured gap, and it is sharper than the word being absent: `retreat` is one of "
            "SPEC-MOK-001's seven targeted actions and the specification's 2026-08-20 amendment "
            "discusses when rule 26 selects it, but the resolved-event vocabulary carries no "
            "retreat_resolved kind, and none was emitted in any real stream measured. So an outcome "
            "the engine can produce leaves no record any instrument could count, and "
            "docs/ROADMAP.md's Phase 6 wording -- which speaks of what agents do when threatened -- "
            "has no measurable counterpart for that verb. Recorded as a gap rather than repaired: "
            "synthesising a retreat figure from the three kinds that do exist is exactly the "
            "fabrication rule 8.7 and case B22 forbid, and adding an event kind is an amendment to "
            "an approved specification, which WO-MOK-033 stop-and-escalate condition 6 forbids on "
            "an implementation agent's judgement."
        ),
    )


# ==================================================================================================
# T9: integers only
# ==================================================================================================


UNQUOTED_DECIMAL = re.compile(rb":\s*-?\d+\.\d+")


def case_t9(report):
    faults = []
    files = 0
    for path in retained_outputs():
        files += 1
        with open(path, "rb") as handle:
            raw = handle.read()
        for match in UNQUOTED_DECIMAL.finditer(raw):
            faults.append(f"{os.path.basename(path)}: {match.group().decode()}")
        for record in read_jsonl(path):
            faults.extend(f"{os.path.basename(path)}: float at {where}"
                          for where in floats_in(record))

    machine = []
    for options in ([], ["--group-by", "source"], ["--group-by", "seed"],
                    ["--group-by", "source,density,seed"]):
        status, out, _ = classifier([f"{WORK}/default-sweep.jsonl", "--format", "json"] + options)
        machine.append((options, len(out)))
        for match in UNQUOTED_DECIMAL.finditer(out.encode("utf-8")):
            faults.append(f"classifier {options}: {match.group().decode()}")
        faults.extend(f"classifier {options}: float at {where}"
                      for where in floats_in(json.loads(out)))

    keep("t9-integers.txt", "\n".join(
        [f"{files} retained outputs and {len(machine)} classifier outputs scanned for a float "
         f"value and for an unquoted decimal", ""]
        + [f"classifier {options or ['(default)']}: {size} bytes" for options, size in machine]
        + ["", "findings:"] + (faults or ["none"])))
    report.record(
        "T9 integers only",
        not faults,
        f"{files} retained outputs and {len(machine)} classifier JSON outputs carry no float value "
        f"and no unquoted decimal; density is carried as the engine's own string; findings "
        f"{faults[:3] or 'none'}",
    )


# ==================================================================================================
# Lint and format
# ==================================================================================================


def case_gates(report):
    transcript = []
    faults = []

    for path in INSTRUMENT_FILES:
        status, out, err = run([sys.executable, "-m", "py_compile", path])
        transcript.append(f"$ python -m py_compile {os.path.basename(path)}\nexit {status}\n{err}")
        if status != 0:
            faults.append(f"py_compile {os.path.basename(path)} exited {status}")

    suites = {}
    for path in INSTRUMENT_FILES[2:]:
        status, out, err = run([sys.executable, path])
        transcript.append(f"$ python scripts/{os.path.basename(path)}\nexit {status}\n{out}{err}")
        suites[os.path.basename(path)] = (status, (out + err).strip().splitlines()[-1:])
        if status != 0:
            faults.append(f"{os.path.basename(path)} exited {status}")

    for label, argv in (
        ("clippy", ["cargo", "clippy", "--workspace", "--all-targets", "--all-features",
                    "--locked", "--", "-D", "warnings"]),
        ("fmt", ["cargo", "fmt", "--check"]),
        ("test", ["cargo", "test", "--locked", "--workspace"]),
    ):
        status, out, err = run(argv, timeout=3600)
        transcript.append(f"$ {' '.join(argv)}\nexit {status}\n{out}{err}")
        if status != 0:
            faults.append(f"{label} exited {status}")
        if label == "test":
            suites["cargo test"] = (status, [line for line in out.splitlines()
                                             if line.startswith("test result:")])

    keep("gates.txt", "\n\n".join(transcript))
    report.record(
        "Lint, format and the product's own gates",
        not faults,
        f"py_compile clean on all four files; the two instrument suites and cargo clippy, "
        f"cargo fmt --check and cargo test --locked --workspace all exited 0; "
        f"{ {k: v[1] for k, v in suites.items()} }; faults {faults or 'none'}",
        note=(
            "The repository declares no Python linter or formatter -- no ruff, flake8, black or "
            "setup.cfg configuration exists, and the two workflows that run Python run scripts and "
            "their test files directly. So `the repository's existing Python checks` are met by "
            "byte-compiling all four files and executing both suites, which is what those workflows "
            "do for the other scripts under scripts/. Recorded as a substitution rather than as the "
            "bullet as written."
        ),
    )


# ==================================================================================================
# Security and privacy
# ==================================================================================================


ENVIRONMENT_READ = re.compile(
    r"os\.environ|getenv|putenv|environb|os\.get_exec_path|expandvars")
NETWORK_NAMES = (
    "socket", "ssl", "http", "urllib", "ftplib", "smtplib", "telnetlib", "asyncio",
    "xmlrpc", "requests", "httpx", "webbrowser", "poplib", "imaplib", "nntplib",
)
LIVE_NAMES = ("llm", "connector", "transcript", "prices", "spend", "ceiling", "api_key", "token")


def case_security(report):
    # 1. No environment read, statically.
    reads = []
    for path in INSTRUMENTS:
        for number, line in enumerate(source_of(path).splitlines(), 1):
            if ENVIRONMENT_READ.search(line):
                reads.append(f"{os.path.basename(path)}:{number}: {line.strip()}")

    # 2. No environment read, behaviourally: a decoy variable under the name a connector would read.
    #    The value announces itself, so that a reviewer scanning this packet for a credential-shaped
    #    string finds a self-describing decoy rather than a candidate.
    decoy = "decoy-not-a-credential-" + secrets.token_hex(16)
    environment = dict(os.environ)
    environment["OPENAI_API_KEY"] = decoy
    environment["MOK_DECOY_NOT_A_CREDENTIAL"] = decoy
    out_path = f"{OUT}/security-decoy.jsonl"
    status, out, err = driver(
        ["--out", out_path, "--sources", "reference,social", "--densities", "0.50",
         "--seeds", "0-1", "--ticks", "200"], env=environment)
    with open(out_path, "rb") as handle:
        retained_bytes = handle.read()
    _, distribution, _ = classifier([out_path, "--format", "json"])
    leaks = [
        where for where, blob in (
            ("the retained output", retained_bytes.decode("utf-8", "replace")),
            ("the batch's standard output", out),
            ("the batch's standard error", err),
            ("the classifier's output", distribution),
        ) if decoy in blob or "OPENAI_API_KEY" in blob or "MOK_DECOY" in blob
    ]

    # 3. No socket, statically and behaviourally.
    network = []
    for path in INSTRUMENT_FILES:
        tree = ast.parse(source_of(path), filename=path)
        for node in ast.walk(tree):
            names = []
            if isinstance(node, ast.Import):
                names = [alias.name.split(".")[0] for alias in node.names]
            elif isinstance(node, ast.ImportFrom) and node.module:
                names = [node.module.split(".")[0]]
            network.extend(f"{os.path.basename(path)}: imports {name}"
                           for name in names if name in NETWORK_NAMES)

    guarded = dict(os.environ)
    guarded["PYTHONPATH"] = f"{WORK}/nosocket"
    breaches = f"{WORK}/nosocket/sitecustomize.py.breaches"
    if os.path.exists(breaches):
        os.remove(breaches)
    guard_out = f"{OUT}/security-nosocket.jsonl"
    batch_status, batch_out, batch_err = driver(
        ["--out", guard_out, "--sources", "reference", "--densities", "0.75", "--seeds", "0-3",
         "--ticks", "200"], env=guarded)
    class_status, class_out, class_err = classifier([guard_out, "--format", "markdown"],
                                                   env=guarded)
    # Read before the probe runs. The probe deliberately trips the guard, and the guard records every
    # breach to one file, so asking whether a breach exists after the probe would always say yes.
    breached = os.path.exists(breaches)
    # The guard must be reachable at all, or a clean run proves nothing about it.
    probe_status, _, probe_err = run(
        [sys.executable, "-c", "import socket; socket.socket()"], env=guarded)
    probe_recorded = os.path.exists(breaches)

    # 4. Rule 4.4's argument vector, captured from the child's side.
    argv_log = f"{OUT}/child-argv.jsonl"
    if os.path.exists(argv_log):
        os.remove(argv_log)
    driver(["--out", f"{OUT}/security-argv.jsonl", "--binary", f"{WORK}/argvlog.bat",
            "--sources", "reference,social", "--densities", "0.75", "--seeds", "0-1",
            "--ticks", "50"])
    vectors = read_jsonl(argv_log)
    flags = sorted({token for record in vectors for token in record["argv"]
                    if token.startswith("--")})
    live = [flag for flag in flags if any(name in flag for name in LIVE_NAMES)]
    inherited = vectors[0]["environ_names"] if vectors else []
    # cmd.exe defines PROMPT for itself in every command session, so a .bat stand-in shows it whether
    # the driver passed anything or not. It is the measuring tool's artefact and is excluded by name,
    # with the exclusion stated rather than silently applied: the driver reaches the child through
    # CreateProcess, and the stand-in has to be a .bat because a Python file is not launchable.
    CMD_OWN = {"PROMPT", "__CD__"}
    added = sorted((set(inherited) - set(os.environ)) - CMD_OWN)
    excluded = sorted((set(inherited) - set(os.environ)) & CMD_OWN)

    keep("security.txt", "\n".join([
        "1. Environment reads in either instrument, statically:",
        *(reads or ["   none"]),
        "",
        "2. A decoy variable named OPENAI_API_KEY, and a second named MOK_DECOY_NOT_A_CREDENTIAL,",
        f"   set in the batch's environment. The value is a self-describing decoy; its sha256 is",
        f"   {sha256_bytes(decoy.encode())} and it is {len(decoy)} characters. The batch exited",
        f"   {status} and retained {len(retained_bytes)} bytes.",
        f"   Occurrences of the value or of either variable name in the retained output, in either",
        f"   of the batch's streams, or in the classifier's output: {leaks or 'none'}",
        "",
        "3. Network-capable imports in the two instruments and their two test files: "
        + (", ".join(network) if network else "none"),
        f"   A four-cell batch and a classifier run, both under a sitecustomize that raises on any",
        f"   socket creation: batch exit {batch_status}, classifier exit {class_status}, breach log",
        f"   {'present' if breached else 'absent'} after both.",
        f"   The guard is reachable: `import socket; socket.socket()` under the same environment",
        f"   exits {probe_status} with {probe_err.strip().splitlines()[-1:] or ['no message']}, and",
        f"   the breach log is {'present' if probe_recorded else 'absent'} after it -- so the absence",
        f"   above is the instruments not opening a socket, not the guard failing to load.",
        "",
        f"4. Rule 4.4's vector, captured by the child rather than read off the driver's source.",
        f"   {len(vectors)} invocations, option flags {flags}.",
        f"   Options matching a live-run or credential name: {live or 'none'}.",
        f"   Variables in the child's environment that are not in the operator's: {added or 'none'},",
        f"   excluding cmd.exe's own {excluded or 'none'}, which the .bat stand-in introduces.",
        "",
        "5. No live selection: no case in this contract passes --policy llm to a sweep other than",
        "   B5, which requires the refusal, and no case supplies a connector or a credential.",
        "   REQ-MOK-072 is untouched by this contract.",
    ]))

    report.record(
        "No credential, and no path by which one could arrive",
        not reads and not leaks and status == 0,
        f"neither instrument contains an environment read ({len(reads)} findings); a batch run with "
        f"a decoy variable named OPENAI_API_KEY exited {status} and the decoy appears in no retained "
        f"byte, neither of the batch's streams, and no classifier output; leaks {leaks or 'none'}",
    )
    report.record(
        "No provider is reachable",
        not network and batch_status == 0 and class_status == 0
        and not breached and probe_status != 0 and probe_recorded,
        f"no network-capable module is imported by any of the four files; a four-cell batch and a "
        f"classifier run completed under a guard that raises on any socket creation (exits "
        f"{batch_status} and {class_status}, no breach recorded), and the guard is reachable -- a "
        f"bare socket() under the same environment exits {probe_status} and does record a breach",
    )
    report.record(
        "Rule 4.4 passes no connector, credential or live selection",
        not live and not added and len(vectors) == 4,
        f"{len(vectors)} child invocations captured from the child's own side; the whole option set "
        f"is {flags}, none of it a connector, price, ceiling, transcript or credential option, and "
        f"the child's environment adds nothing to the operator's ({added or 'nothing'}, excluding "
        f"cmd.exe's own {excluded or 'none'})",
    )


# ==================================================================================================


def main():
    report = Report("phase D: static, architecture, security and privacy")
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(OUT, exist_ok=True)
    case_t1_t2(report)
    case_t3(report)
    case_t4(report)
    case_t5(report)
    case_t6(report)
    case_t7(report)
    case_t8(report)
    case_t9(report)
    case_security(report)
    case_gates(report)
    return 0 if report.write(f"{OUT}/result.json") else 1


if __name__ == "__main__":
    sys.exit(main())
