#!/usr/bin/env python3
"""Compare each package's resolved dependency graph against the set declared for it.

Repository-owned, absent from `.engineering-harness.lock`, decides nothing, writes nothing.
This implements `SPEC-MOK-005` rule 8.4 — checks **8.4a** to **8.4d** — and it is the single
implementation both placements of that rule call: the release gate in
`.github/workflows/release.yml` and the pull-request gate rule 15 requires. Rule 15.5 forbids
a third source of truth about what is declared, so this program **reads the declaration and
restates none of it**: every crate, version, feature, build-script status, per-target figure
and disclosed transitive capability comes out of the specifications listed below.

    engine package `Mokiterions`   `SPEC-MOK-002` rule 13, *Declared dependency set*
    observer package `mokiterions-tui`   `SPEC-MOK-003`, *Declared dependency set*

Those two pairings are the only knowledge of the repository written into this file. They are
a pointer, not a declaration: change a version in `SPEC-MOK-003` and this program's verdict
changes with no edit here, which is the property rule 15.5 is about.

What is compared, and what is not
---------------------------------
The set is the **resolved per-package graph** at the declared features, on each target
`SPEC-MOK-005` rule 10 builds, read with `--locked` so nothing resolves that the committed
lockfile does not already fix. It is expressly **not** `Cargo.lock`'s contents: two crates in
that file resolve into no build of either package, and one of them carries a build script, so
a lockfile read would attribute a build-time code-execution surface to a package that never
executes it.

Set equality (8.4a) is over each package's **direct declared entries**, which are
target-independent. The transitive graph is not enumerated — it runs to dozens of crates and
differs per target — and is held instead by `--locked`, by the build-script table
`SPEC-MOK-003` records, and by the by-name scan. The per-target figures are in that
specification and are deliberately not repeated here: a count in a comment is a declaration
this program would then hold two copies of, and the stale copy is always the one in the code. The engine package is the exception in
the strict direction: `SPEC-MOK-002` rule 13 declares that *every* external crate in its
graph, transitive and dev-only included, is an entry, so the engine's whole graph is compared
and today that graph is the engine alone.

Registry access
---------------
Every cargo invocation passes `--locked --offline`. On a machine whose registry cache is
empty — a fresh CI runner — run `cargo fetch --locked` once before this program. That is the
one step permitted to reach a registry, and it resolves nothing: it downloads what the
lockfile already fixed. Refusing with that instruction is better than reporting a graph
difference that is really a missing cache.

Usage:
    python scripts/check_declared_dependencies.py --root .
    python scripts/check_declared_dependencies.py --root . --json
    python scripts/check_declared_dependencies.py --root . --no-engine-build
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError as exc:  # pragma: no cover - version guard
    raise SystemExit("Python 3.11 or later is required (missing tomllib).") from exc


class Refusal(Exception):
    """A reason to refuse the commit under check."""


# ---------------------------------------------------------------------------------------
# Reading a declaration out of a specification.
#
# The declarations live in Markdown tables inside approved artifacts. Parsing them is the
# whole point: a copy of the table in this file would be the second declaration rule 15.5
# forbids, and it would pass while disagreeing with the artifact it claimed to check.
# ---------------------------------------------------------------------------------------

HEADING = re.compile(r"^(#{1,6})\s+(.+?)\s*$")
SEPARATOR = re.compile(r"^\|[\s:|-]+\|$")
BACKTICKED = re.compile(r"`([^`]+)`")
NUMBER = re.compile(r"\d+")
CRATE_LINE = re.compile(r"^(?P<name>[A-Za-z0-9_.+-]+)\s+v(?P<version>[0-9][^\s]*)")

DEFAULT_FEATURES_TOKEN = "default-features = false"

DEPENDENCY_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")

DECLARED_HEADERS = ("Crate", "Version", "Features", "Build script", "Admitted by")
DISCLOSURE_HEADERS = ("Crate", "Version", "Targets", "Reached by", "Capability", "Assessment")
FIGURES_HEADERS = ("Target", "External crates", "Build-script crates")
BUILD_SCRIPT_HEADERS = ("Crate", "Version", "Targets")

UNION_ROW = "union"


@dataclass(frozen=True)
class Table:
    """One Markdown table, with the heading it sits under."""

    heading: str
    headers: tuple[str, ...]
    rows: tuple[tuple[str, ...], ...]


def read_tables(path: Path) -> list[Table]:
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as exc:
        raise Refusal(f"cannot read {path}: {exc}") from exc

    tables: list[Table] = []
    heading = ""
    index = 0
    while index < len(lines):
        line = lines[index]
        matched = HEADING.match(line)
        if matched is not None:
            heading = matched.group(2)
            index += 1
            continue
        if line.startswith("|") and index + 1 < len(lines) and SEPARATOR.match(lines[index + 1]):
            headers = split_row(line)
            rows: list[tuple[str, ...]] = []
            index += 2
            while index < len(lines) and lines[index].startswith("|"):
                rows.append(split_row(lines[index]))
                index += 1
            tables.append(Table(heading, headers, tuple(rows)))
            continue
        index += 1
    return tables


def split_row(line: str) -> tuple[str, ...]:
    stripped = line.strip()
    if stripped.startswith("|"):
        stripped = stripped[1:]
    if stripped.endswith("|"):
        stripped = stripped[:-1]
    return tuple(cell.strip() for cell in stripped.split("|"))


def find_table(
    tables: list[Table],
    document: str,
    heading_contains: str,
    headers: tuple[str, ...],
) -> Table:
    matches = [
        table
        for table in tables
        if heading_contains.lower() in table.heading.lower() and table.headers == headers
    ]
    if not matches:
        raise Refusal(
            f"{document} holds no table under a heading containing {heading_contains!r} with "
            f"the columns {' | '.join(headers)}. The declaration is read from the artifact, so "
            "a renamed heading or a changed column is a refusal rather than something to "
            "guess around."
        )
    if len(matches) > 1:
        raise Refusal(
            f"{document} holds {len(matches)} tables under {heading_contains!r} with the "
            "columns " + " | ".join(headers) + ". Which one declares the set is ambiguous."
        )
    return matches[0]


def backticked(cell: str) -> list[str]:
    return [match.group(1).strip() for match in BACKTICKED.finditer(cell)]


def one_backticked(cell: str, what: str) -> str:
    tokens = backticked(cell)
    if len(tokens) != 1:
        raise Refusal(
            f"expected exactly one backticked {what} in the cell {cell!r}, found {len(tokens)}"
        )
    return tokens[0]


def parse_count(cell: str, what: str) -> int:
    numbers = NUMBER.findall(cell)
    if len(numbers) != 1:
        raise Refusal(f"expected exactly one number for {what} in the cell {cell!r}")
    return int(numbers[0])


@dataclass(frozen=True)
class Features:
    """A *Features* cell, read as `SPEC-MOK-002` rule 13 fixes the reading."""

    default_features: bool
    declared: frozenset[str]
    implied: frozenset[str]
    prohibited: frozenset[str]

    @property
    def expected(self) -> frozenset[str]:
        """What the resolved set must equal: declared together with implied."""
        return self.declared | self.implied


def parse_features(cell: str) -> Features:
    """Read a *Features* cell mechanically, per `SPEC-MOK-002` rule 13.

    Within the cell `default-features = false` is the switch and every other backticked token
    is a feature name. A sentence containing *off* names prohibited features and nothing else;
    a sentence containing *implied* names features a declared feature activates; every
    remaining token is declared. The convention is in the specification rather than here on
    purpose — a program that guessed at the reading would be a second declaration.
    """
    default_features = True
    declared: set[str] = set()
    implied: set[str] = set()
    prohibited: set[str] = set()

    for sentence in re.split(r"(?<=\.)\s+", cell):
        tokens = [token for token in backticked(sentence)]
        if DEFAULT_FEATURES_TOKEN in tokens:
            default_features = False
            tokens = [token for token in tokens if token != DEFAULT_FEATURES_TOKEN]
        lowered = sentence.lower()
        if re.search(r"\boff\b", lowered):
            prohibited.update(tokens)
        elif re.search(r"\bimplied\b", lowered):
            implied.update(tokens)
        else:
            declared.update(tokens)

    overlap = (declared & prohibited) | (implied & prohibited)
    if overlap:
        raise Refusal(
            "a Features cell names " + ", ".join(sorted(overlap)) + " as both present and off"
        )
    return Features(default_features, frozenset(declared), frozenset(implied), frozenset(prohibited))


@dataclass(frozen=True)
class Entry:
    """One declared dependency."""

    crate: str
    version: str
    features: Features
    build_script: bool
    admitted_by: str


@dataclass(frozen=True)
class Disclosure:
    """One transitive crate whose name is in a prohibited class, per rule 8.4d."""

    crate: str
    version: str
    labels: frozenset[str]
    capability: str
    assessment: str


@dataclass
class Declaration:
    """Everything one specification declares about one package."""

    package: str
    document: str
    entries: dict[str, Entry] = field(default_factory=dict)
    disclosures: dict[str, Disclosure] = field(default_factory=dict)
    targets: tuple[str, ...] = ()
    external_counts: dict[str, int] = field(default_factory=dict)
    build_script_counts: dict[str, int] = field(default_factory=dict)
    build_script_crates: dict[str, frozenset[str]] = field(default_factory=dict)


def target_label(triple: str) -> str:
    lowered = triple.lower()
    if "windows" in lowered:
        return "Windows"
    if "linux" in lowered:
        return "Linux"
    if "darwin" in lowered or "apple" in lowered:
        return "macOS"
    raise Refusal(
        f"cannot label the target {triple!r}. The declared figures are per target and the "
        "build-script table names its targets by platform, so an unlabelled triple would "
        "make the two tables unrelatable."
    )


def parse_target_labels(cell: str, labels: tuple[str, ...]) -> frozenset[str]:
    lowered = cell.lower()
    if "all three" in lowered:
        return frozenset(labels)
    named = set()
    for part in re.split(r"[,/]| and ", cell):
        cleaned = part.strip().strip(".").strip("`").strip()
        if not cleaned:
            continue
        for label in labels:
            if cleaned.lower() == label.lower() or cleaned.lower() == label.lower().rstrip("s"):
                named.add(label)
                break
        else:
            if any(marker in cleaned.lower() for marker in ("-", "_")):
                named.add(target_label(cleaned))
            else:
                raise Refusal(f"cannot read the target cell {cell!r}: {cleaned!r} names no target")
    if not named:
        raise Refusal(f"the target cell {cell!r} names no target")
    return frozenset(named)


def read_declaration(root: Path, package: str, document: str, heading: str) -> Declaration:
    path = specification_path(root, document)
    tables = read_tables(path)
    declaration = Declaration(package=package, document=document)

    declared = find_table(tables, document, heading, DECLARED_HEADERS)
    for row in declared.rows:
        crate = one_backticked(row[0], "crate name")
        if crate in declaration.entries:
            raise Refusal(f"{document} declares {crate} twice")
        declaration.entries[crate] = Entry(
            crate=crate,
            version=one_backticked(row[1], "version"),
            features=parse_features(row[2]),
            build_script=parse_yes_no(row[3], crate),
            admitted_by=row[4],
        )

    figures = [
        table
        for table in tables
        if heading.lower() in table.heading.lower() and table.headers == FIGURES_HEADERS
    ]
    if figures:
        labels: list[str] = []
        triples: list[str] = []
        for row in figures[0].rows:
            cell = row[0]
            if UNION_ROW in cell.lower():
                declaration.external_counts[UNION_ROW] = parse_count(row[1], "union crate count")
                declaration.build_script_counts[UNION_ROW] = parse_count(
                    row[2], "union build-script count"
                )
                continue
            triple = one_backticked(cell, "target triple")
            triples.append(triple)
            labels.append(target_label(triple))
            declaration.external_counts[triple] = parse_count(row[1], f"crate count for {triple}")
            declaration.build_script_counts[triple] = parse_count(
                row[2], f"build-script count for {triple}"
            )
        declaration.targets = tuple(triples)

        build_scripts = [
            table
            for table in tables
            if heading.lower() in table.heading.lower() and table.headers == BUILD_SCRIPT_HEADERS
        ]
        if build_scripts:
            per_label: dict[str, set[str]] = {label: set() for label in labels}
            for row in build_scripts[0].rows:
                crate = one_backticked(row[0], "crate name")
                version = one_backticked(row[1], "version")
                for label in parse_target_labels(row[2], tuple(labels)):
                    per_label[label].add(f"{crate} {version}")
            declaration.build_script_crates = {
                label: frozenset(names) for label, names in per_label.items()
            }

        disclosures = [
            table
            for table in tables
            if heading.lower() in table.heading.lower() and table.headers == DISCLOSURE_HEADERS
        ]
        if disclosures:
            for row in disclosures[0].rows:
                crate = one_backticked(row[0], "crate name")
                declaration.disclosures[crate] = Disclosure(
                    crate=crate,
                    version=one_backticked(row[1], "version"),
                    labels=parse_target_labels(row[2], tuple(labels)),
                    capability=row[4],
                    assessment=row[5],
                )

    return declaration


def parse_yes_no(cell: str, crate: str) -> bool:
    cleaned = cell.strip().strip("*").strip().lower()
    if cleaned in {"yes", "**yes**"}:
        return True
    if cleaned in {"no", "**no**"}:
        return False
    raise Refusal(
        f"the *Build script* cell for {crate} reads {cell!r}; `SPEC-MOK-002` rule 13 fixes it "
        "as `yes` or `no`, because the build-time code-execution surface is enumerated and not "
        "described."
    )


def specification_path(root: Path, document: str) -> Path:
    path = root / "docs" / "engineering" / "simulation" / "specifications" / f"{document}.md"
    if not path.is_file():
        raise Refusal(f"{document} is not at {path}. The declaration cannot be read.")
    return path


# ---------------------------------------------------------------------------------------
# Reading the manifests. `[workspace.dependencies]` is checked against both declared sets,
# per `SPEC-MOK-004` rule 2 as amended.
# ---------------------------------------------------------------------------------------


@dataclass(frozen=True)
class ManifestEntry:
    crate: str
    version: str | None
    default_features: bool
    features: tuple[str, ...]
    tables: frozenset[str]
    inherited: bool


def load_manifest(path: Path) -> dict:
    try:
        with open(path, "rb") as handle:
            return tomllib.load(handle)
    except OSError as exc:
        raise Refusal(f"cannot read {path}: {exc}") from exc
    except tomllib.TOMLDecodeError as exc:
        raise Refusal(f"{path} is not valid TOML: {exc}") from exc


def workspace_members(root: Path) -> dict[str, Path]:
    manifest = load_manifest(root / "Cargo.toml")
    workspace = manifest.get("workspace")
    if not isinstance(workspace, dict):
        raise Refusal(f"{root / 'Cargo.toml'} declares no [workspace] table")
    members: dict[str, Path] = {}
    for member in workspace.get("members", []):
        directory = root / member
        package = load_manifest(directory / "Cargo.toml").get("package", {})
        name = package.get("name")
        if not name:
            raise Refusal(f"the workspace member {member} declares no package name")
        members[name] = directory
    return members


def manifest_entries(
    manifest_path: Path,
    member_directories: set[Path],
) -> tuple[dict[str, ManifestEntry], list[str]]:
    """Every external direct entry of one manifest, and the member paths that are not entries.

    A path dependency on a workspace member is not an external crate — `SPEC-MOK-003`'s
    *Component layout* clause 2 governs it — and it is returned separately so the caller can
    report it rather than silently drop it. A path dependency on anything else is a refusal:
    it is neither a declared entry nor a member, and there is no third kind.
    """
    manifest = load_manifest(manifest_path)
    directory = manifest_path.parent
    entries: dict[str, ManifestEntry] = {}
    member_paths: list[str] = []

    def absorb(table_name: str, table: dict) -> None:
        for crate, value in table.items():
            if isinstance(value, str):
                specification = {"version": value}
            elif isinstance(value, dict):
                specification = value
            else:
                raise Refusal(f"{manifest_path}: the {crate} entry is neither a version nor a table")

            path = specification.get("path")
            if path is not None:
                resolved = (directory / path).resolve()
                if resolved in member_directories:
                    member_paths.append(f"{crate} (path {path})")
                    continue
                raise Refusal(
                    f"{manifest_path}: {crate} is a path dependency on {path}, which is not a "
                    "workspace member. A declared entry is an external crate and a member is "
                    "reached by path; there is no third kind, so this is a refusal."
                )

            existing = entries.get(crate)
            tables = frozenset({table_name}) | (existing.tables if existing else frozenset())
            entries[crate] = ManifestEntry(
                crate=crate,
                version=specification.get("version"),
                default_features=bool(specification.get("default-features", True)),
                features=tuple(specification.get("features", [])),
                tables=tables,
                inherited=bool(specification.get("workspace", False)),
            )

    for table_name in DEPENDENCY_TABLES:
        table = manifest.get(table_name)
        if isinstance(table, dict):
            absorb(table_name, table)

    for platform, platform_tables in (manifest.get("target") or {}).items():
        if not isinstance(platform_tables, dict):
            continue
        for table_name in DEPENDENCY_TABLES:
            table = platform_tables.get(table_name)
            if isinstance(table, dict):
                absorb(f"target.{platform}.{table_name}", table)

    return entries, member_paths


def workspace_dependency_entries(root: Path) -> dict[str, ManifestEntry]:
    manifest = load_manifest(root / "Cargo.toml")
    table = (manifest.get("workspace") or {}).get("dependencies")
    if not isinstance(table, dict):
        return {}
    entries: dict[str, ManifestEntry] = {}
    for crate, value in table.items():
        specification = {"version": value} if isinstance(value, str) else dict(value)
        entries[crate] = ManifestEntry(
            crate=crate,
            version=specification.get("version"),
            default_features=bool(specification.get("default-features", True)),
            features=tuple(specification.get("features", [])),
            tables=frozenset({"workspace.dependencies"}),
            inherited=False,
        )
    return entries


# ---------------------------------------------------------------------------------------
# Reading the resolved graph. Cargo answers, not a manifest and not the lockfile.
# ---------------------------------------------------------------------------------------


@dataclass(frozen=True)
class Crate:
    name: str
    version: str
    features: frozenset[str]

    @property
    def identity(self) -> str:
        return f"{self.name} {self.version}"


def cargo(root: Path, arguments: list[str]) -> str:
    try:
        completed = subprocess.run(
            ["cargo", *arguments],
            cwd=str(root),
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError as exc:
        raise Refusal(f"cannot run cargo: {exc}") from exc
    if completed.returncode != 0:
        raise cargo_failure(arguments, (completed.stderr or completed.stdout).strip())
    return completed.stdout


def cargo_failure(arguments: list[str], detail: str) -> Refusal:
    """Says which of the two `--locked --offline` failures happened, because the fixes differ.

    A stale lockfile and an unpopulated cache both stop cargo before any comparison runs, and
    both mention `--locked`. Telling a reader to fetch when the lockfile is what is wrong sends
    them to a step that will succeed and change nothing.
    """
    lowered = detail.lower()
    first = detail.splitlines()[0] if detail else "cargo produced no diagnostic"
    if "cannot update the lock file" in lowered or "--locked was passed" in lowered:
        return Refusal(
            "the lockfile does not describe these manifests: "
            + first
            + ". A dependency added to a manifest without updating `Cargo.lock` cannot be "
            "compared against a declared set at all, because cargo refuses to resolve before the "
            "comparison runs. Update the lockfile in the same change as the manifest — an "
            "ordinary `cargo build` does it — and commit both, then this check has something to "
            "compare. Fetching will not help: the lockfile is what is out of date."
        )
    if "offline" in lowered or "not found in registry" in lowered:
        return Refusal(
            "cargo refused to resolve offline: "
            + first
            + ". Run `cargo fetch --locked` once first — that is the one step permitted to "
            "reach a registry, and it downloads what the lockfile already fixed. Every check "
            "here then runs with no registry access."
        )
    return Refusal(f"`cargo {' '.join(arguments)}` failed: {detail}")


def cargo_tree(root: Path, package: str, edges: str, target: str | None) -> list[Crate]:
    arguments = [
        "tree",
        "-p",
        package,
        "-e",
        edges,
        "--locked",
        "--offline",
        "--prefix",
        "none",
        "--no-dedupe",
        "-f",
        "{p}|{f}",
    ]
    if target is not None:
        arguments.extend(["--target", target])
    crates: dict[str, Crate] = {}
    for line in cargo(root, arguments).splitlines():
        line = line.strip()
        if not line:
            continue
        left, _, features = line.partition("|")
        matched = CRATE_LINE.match(left.strip())
        if matched is None:
            raise Refusal(f"cannot read the `cargo tree` line {line!r}")
        crate = Crate(
            name=matched.group("name"),
            version=matched.group("version"),
            features=frozenset(token for token in features.strip().split(",") if token),
        )
        crates[crate.identity] = crate
    return sorted(crates.values(), key=lambda crate: crate.identity)


def cargo_metadata(root: Path) -> dict:
    output = cargo(root, ["metadata", "--locked", "--offline", "--format-version", "1"])
    try:
        return json.loads(output)
    except json.JSONDecodeError as exc:
        raise Refusal(f"cannot read `cargo metadata` output: {exc}") from exc


def build_script_identities(metadata: dict) -> set[str]:
    identities: set[str] = set()
    for package in metadata.get("packages", []):
        for target in package.get("targets", []):
            if "custom-build" in target.get("kind", []):
                identities.add(f"{package['name']} {package['version']}")
    return identities


# ---------------------------------------------------------------------------------------
# The by-name prohibition scan of 8.4d.
#
# The terms come from `ADR-MOK-006` decision 4's capability classes, written from the
# decision rather than from the crates that happen to be present: a list derived from the
# current graph would pass by construction on the day it was written. The scan is by name and
# is not exhaustive over those classes, which `SPEC-MOK-005` rule 8.4d states and
# `VER-MOK-014` manual assessment 3 stands behind. Its refusal is conclusive; its pass is not.
# ---------------------------------------------------------------------------------------

PROHIBITED_TERMS: dict[str, tuple[str, ...]] = {
    "network access": (
        "reqwest", "hyper", "curl", "ureq", "isahc", "surf", "attohttpc", "http", "h2", "h3",
        "quinn", "rustls", "openssl", "tls", "native", "tungstenite", "websocket", "ws",
        "socket2", "mio", "tokio", "smoltcp", "trust", "hickory", "dns", "resolv", "url",
        "ipnet", "net", "grpc", "tonic", "prost", "zmq", "nats", "mqtt", "smtp", "ftp", "ssh",
    ),
    "asynchronous runtime": (
        "tokio", "async", "smol", "futures", "executor", "rayon", "threadpool", "actix",
        "monoio", "glommio", "compio",
    ),
    "database": (
        "sqlite", "rusqlite", "libsqlite3", "postgres", "pg", "mysql", "mariadb", "mongodb",
        "redis", "diesel", "sqlx", "seaorm", "sled", "redb", "rocksdb", "leveldb", "duckdb",
        "surrealdb", "tikv", "lmdb", "heed",
    ),
    "model provider": (
        "openai", "anthropic", "claude", "gpt", "llm", "llama", "ollama", "langchain",
        "tiktoken", "huggingface", "candle", "onnx", "torch", "tensorflow", "genai",
    ),
    "credential handling": (
        "keyring", "secret", "secrets", "credential", "credentials", "vault", "oauth", "oauth2",
        "jwt", "jsonwebtoken", "sasl", "kerberos",
    ),
    "plugin system or dependency injection": (
        "libloading", "dlopen", "inventory", "shaku", "waiter", "teloc", "di",
    ),
}

USER_INTERFACE_TERMS: tuple[str, ...] = (
    "ratatui", "crossterm", "termion", "tui", "ncurses", "pancurses", "cursive", "egui",
    "iced", "winit", "gtk", "qt", "sdl2", "druid", "slint", "dioxus", "tauri", "wgpu",
    "web", "yew", "leptos", "bevy",
)

NAME_SPLIT = re.compile(r"[-_]")


def name_tokens(name: str) -> set[str]:
    lowered = name.lower()
    return {lowered, *NAME_SPLIT.split(lowered)}


def scan_names(crates: list[Crate], terms: tuple[str, ...]) -> list[Crate]:
    wanted = set(terms)
    return [crate for crate in crates if name_tokens(crate.name) & wanted]


# ---------------------------------------------------------------------------------------
# The checks. Each returns its refusals rather than raising, so one run reports every
# mismatch it found instead of the first one.
# ---------------------------------------------------------------------------------------


@dataclass
class Report:
    lines: list[str] = field(default_factory=list)
    refusals: list[str] = field(default_factory=list)

    def note(self, line: str) -> None:
        self.lines.append(line)

    def refuse(self, line: str) -> None:
        self.refusals.append(line)


def check_declared_equality(
    report: Report,
    declaration: Declaration,
    entries: dict[str, ManifestEntry],
    member_paths: list[str],
    manifest_path: Path,
) -> None:
    """8.4a, both directions, for one package."""
    declared = set(declaration.entries)
    present = set(entries)

    for crate in sorted(present - declared):
        report.refuse(
            f"8.4a {declaration.package}: {manifest_path.name} declares {crate}, which "
            f"{declaration.document}'s declared set does not. An entry reaches a manifest by "
            "amendment to that specification, approved by the technical owner; adding one here "
            "is not an implementation act."
        )
    for crate in sorted(declared - present):
        report.refuse(
            f"8.4a {declaration.package}: {declaration.document} declares {crate}, which "
            f"{manifest_path.name} does not contain. A declared entry that has vanished from the "
            "manifest is as much a mismatch as an undeclared one — the comparison runs in both "
            "directions."
        )

    for crate in sorted(declared & present):
        entry = declaration.entries[crate]
        manifest_entry = entries[crate]
        if manifest_entry.inherited:
            report.note(
                f"  {crate}: version inherited from [workspace.dependencies]"
            )
        elif manifest_entry.version != entry.version:
            report.refuse(
                f"8.4a {declaration.package}: {crate} is declared at {entry.version} and the "
                f"manifest asks for {manifest_entry.version!r}. The *Version* column is the exact "
                "resolved version and not a range; a version change is an amendment."
            )

    for path_entry in member_paths:
        report.note(f"  not an entry: {path_entry}, a workspace member reached by path")
    report.note(
        f"  {declaration.package}: {len(declared)} declared, {len(present)} in the manifest"
    )


def check_features(
    report: Report,
    declaration: Declaration,
    entries: dict[str, ManifestEntry],
    graphs: dict[str, list[Crate]],
) -> None:
    """8.4b, over the manifest declaration and over the resolved features on every target."""
    for crate, entry in sorted(declaration.entries.items()):
        manifest_entry = entries.get(crate)
        if manifest_entry is None:
            continue
        if manifest_entry.default_features != entry.features.default_features:
            report.refuse(
                f"8.4b {declaration.package}: {crate} is declared with default-features "
                f"{'off' if not entry.features.default_features else 'on'} and the manifest has "
                f"them {'on' if manifest_entry.default_features else 'off'}."
            )
        if set(manifest_entry.features) != set(entry.features.declared):
            report.refuse(
                f"8.4b {declaration.package}: {crate} is declared with features "
                f"{sorted(entry.features.declared)} and the manifest asks for "
                f"{sorted(manifest_entry.features)}. The declared cell is the exact set the "
                "manifest states; a feature set change is an amendment."
            )

        for target, crates in sorted(graphs.items()):
            resolved = next((item for item in crates if item.name == crate), None)
            if resolved is None:
                report.refuse(
                    f"8.4b {declaration.package}: {crate} is declared and does not appear in the "
                    f"resolved graph on {target}."
                )
                continue
            if resolved.version != entry.version:
                report.refuse(
                    f"8.4b {declaration.package}: {crate} resolves to {resolved.version} on "
                    f"{target} and is declared at {entry.version}."
                )
            forbidden = resolved.features & entry.features.prohibited
            # A prohibited feature is also outside the expected set. It is reported once, under
            # the sharper of the two headings.
            unexpected = resolved.features - entry.features.expected - forbidden
            missing = entry.features.declared - resolved.features
            if forbidden:
                report.refuse(
                    f"8.4b {declaration.package}: {crate} resolves on {target} with "
                    f"{sorted(forbidden)} enabled, which the declaration records as off."
                )
            if missing:
                report.refuse(
                    f"8.4b {declaration.package}: {crate} resolves on {target} without "
                    f"{sorted(missing)}, which the declaration states the manifest asks for."
                )
            if unexpected:
                report.refuse(
                    f"8.4b {declaration.package}: {crate} resolves on {target} with "
                    f"{sorted(unexpected)} enabled, which is neither declared nor recorded as "
                    "implied. A feature enabled by unification rather than by declaration is a "
                    "mismatch: it is how a capability the declaration excludes arrives without an "
                    "amendment."
                )
            else:
                report.note(
                    f"  {crate} {resolved.version} on {target}: features "
                    f"{','.join(sorted(resolved.features)) or '(none)'}"
                )


def check_engine_closure(
    report: Report,
    declaration: Declaration,
    graphs: dict[str, list[Crate]],
    members: set[str],
) -> None:
    """`SPEC-MOK-002` rule 13's strict direction, which only the engine's declaration states."""
    declared = set(declaration.entries)
    for target, crates in sorted(graphs.items()):
        external = [crate for crate in crates if crate.name not in members]
        surplus = sorted(crate.identity for crate in external if crate.name not in declared)
        if surplus:
            report.refuse(
                f"rule 13 {declaration.package}: the resolved graph on {target} reaches "
                + ", ".join(surplus)
                + ", which the declared set does not contain. Rule 13 declares that every "
                "external crate in this package's graph — transitive and dev-only included — is "
                "an entry in its table."
            )
        node = "node" if len(crates) == 1 else "nodes"
        report.note(
            f"  {declaration.package} on {target}: {len(external)} external crates "
            f"({len(crates)} {node} including workspace members)"
        )


def check_counts(
    report: Report,
    declaration: Declaration,
    graphs: dict[str, list[Crate]],
    members: set[str],
) -> None:
    """The per-target figures the declaration records, and their union."""
    union: set[str] = set()
    for target, crates in sorted(graphs.items()):
        external = {crate.identity for crate in crates if crate.name not in members}
        union |= external
        expected = declaration.external_counts.get(target)
        if expected is not None and len(external) != expected:
            report.refuse(
                f"{declaration.document} records {expected} external crates for "
                f"{declaration.package} on {target}; the resolved graph has {len(external)}. "
                "The figure is a measurement in an approved artifact, so a difference is either "
                "a dependency change or a stale specification, and both are refusals here."
            )
        report.note(
            f"  {declaration.package} on {target}: {len(external)} external crates"
            + (f", declared {expected}" if expected is not None else ", no figure declared")
        )
    expected_union = declaration.external_counts.get(UNION_ROW)
    if expected_union is not None and len(union) != expected_union:
        report.refuse(
            f"{declaration.document} records {expected_union} external crates in the union of "
            f"the declared targets; the resolved union has {len(union)}."
        )
    if union:
        report.note(
            f"  {declaration.package} union of the declared targets: {len(union)} external crates"
            + (f", declared {expected_union}" if expected_union is not None else "")
        )


def check_build_scripts(
    report: Report,
    declaration: Declaration,
    graphs: dict[str, list[Crate]],
    members: set[str],
    with_build_script: set[str],
) -> None:
    """`ADR-MOK-006` decision 13: the build-time code-execution surface, enumerated."""
    if not declaration.build_script_crates:
        return
    for target, crates in sorted(graphs.items()):
        label = target_label(target)
        expected = declaration.build_script_crates.get(label, frozenset())
        resolved = {
            crate.identity
            for crate in crates
            if crate.name not in members and crate.identity in with_build_script
        }
        for identity in sorted(resolved - set(expected)):
            report.refuse(
                f"decision 13 {declaration.package}: {identity} carries a build script and "
                f"reaches {target}, and {declaration.document}'s build-script table does not "
                "record it for that target. A crate that acquires one is a mismatch, not an "
                "unremarked change."
            )
        for identity in sorted(set(expected) - resolved):
            report.refuse(
                f"decision 13 {declaration.package}: {declaration.document} records {identity} as "
                f"carrying a build script on {target}, and the resolved graph does not."
            )
        declared_count = declaration.build_script_counts.get(target)
        if declared_count is not None and len(resolved) != declared_count:
            report.refuse(
                f"decision 13 {declaration.package}: {declaration.document} records "
                f"{declared_count} build-script crates on {target}; the resolved graph has "
                f"{len(resolved)}."
            )
        report.note(
            f"  {declaration.package} on {target}: {len(resolved)} build-script crates "
            f"({', '.join(sorted(resolved)) or 'none'})"
        )


def check_prohibited_names(
    report: Report,
    declaration: Declaration,
    graphs: dict[str, list[Crate]],
    members: set[str],
    user_interface_admissible: bool,
) -> None:
    """8.4d, with the disclosure mechanism the rule states."""
    for target, crates in sorted(graphs.items()):
        label = target_label(target)
        external = [crate for crate in crates if crate.name not in members]

        for capability, terms in sorted(PROHIBITED_TERMS.items()):
            for crate in scan_names(external, terms):
                report.note(f"  raw hit on {target}: {crate.identity} ({capability})")
                if crate.name in declaration.entries:
                    report.refuse(
                        f"8.4d {declaration.package}: the declared entry {crate.identity} is in "
                        f"the {capability} class. `ADR-MOK-006` decision 4 makes such a crate "
                        "inadmissible however stable it is, and no disclosure can admit it."
                    )
                    continue
                disclosure = declaration.disclosures.get(crate.name)
                if disclosure is None:
                    report.refuse(
                        f"8.4d {declaration.package}: {crate.identity} is reached transitively on "
                        f"{target} and its name places it in the {capability} class, and "
                        f"{declaration.document} does not disclose it. A transitive "
                        "prohibited-class name refuses until the declaring specification records "
                        "the crate, the chain, the activating feature and what the package does "
                        "not do with it, and the technical owner judges it."
                    )
                    continue
                if disclosure.version != crate.version:
                    report.refuse(
                        f"8.4d {declaration.package}: {crate.name} is disclosed at "
                        f"{disclosure.version} and resolves at {crate.version} on {target}."
                    )
                if label not in disclosure.labels:
                    report.refuse(
                        f"8.4d {declaration.package}: {crate.identity} is reached on {target} and "
                        f"is disclosed only for {', '.join(sorted(disclosure.labels))}."
                    )

        if not user_interface_admissible:
            for crate in scan_names(external, USER_INTERFACE_TERMS):
                report.refuse(
                    f"8.4d {declaration.package}: {crate.identity} is a user-interface crate in "
                    f"this package's graph on {target}. `REQ-MOK-026` confines the user interface "
                    "to the observer package, and `ADR-MOK-006` decision 4 leaves that "
                    "prohibition exactly where it was."
                )

    for crate, disclosure in sorted(declaration.disclosures.items()):
        reached = any(
            any(item.name == crate for item in crates if item.name not in members)
            for crates in graphs.values()
        )
        if not reached:
            report.refuse(
                f"8.4d {declaration.package}: {declaration.document} discloses {crate} "
                f"{disclosure.version} and no declared target reaches it. A disclosure the graph "
                "does not need is as wrong as a missing one: it makes the scan look weighed when "
                "it was not."
            )
        # A disclosed capability is printed whether or not its assessment has been recorded. An
        # acceptance that removed the line would be indistinguishable from the disclosure never
        # having existed, which is the state rule 8.4d's table was written to prevent.
        if "outstanding" in disclosure.assessment.lower():
            report.note(
                f"  disclosed and OUTSTANDING: {crate} {disclosure.version} — "
                "the technical owner's judgement is owed (`VER-MOK-014` manual assessment 6)"
            )
        else:
            report.note(
                f"  disclosed and accepted: {crate} {disclosure.version} — "
                "the technical owner's judgement is recorded (`VER-MOK-014` manual assessment 6)"
            )


def check_engine_offline(report: Report, root: Path, package: str) -> None:
    """8.4c: the engine resolves, builds and tests from the committed lockfile, offline."""
    for arguments in (
        ["build", "-p", package, "--locked", "--offline"],
        ["test", "-p", package, "--locked", "--offline"],
    ):
        try:
            cargo(root, arguments)
        except Refusal as refusal:
            report.refuse(f"8.4c {package}: `cargo {' '.join(arguments)}` did not succeed. {refusal}")
            return
    report.note(f"  {package}: builds and tests offline from the committed lockfile")


TYPOGRAPHIC = {
    "—": "--",
    "–": "-",
    "→": "->",
    "‘": "'",
    "’": "'",
    "“": '"',
    "”": '"',
}


def printable(line: str, stream) -> str:
    """Fold the typographic characters a console cannot encode.

    The notes and refusals quote artifact prose, which uses em dashes and arrows. A Windows
    console code page cannot encode them, and a gate that raised `UnicodeEncodeError` while
    reporting a mismatch would hide the mismatch behind a crash. Folding happens only when the
    stream cannot take the character as written.
    """
    encoding = getattr(stream, "encoding", None) or "utf-8"
    try:
        line.encode(encoding)
    except (UnicodeEncodeError, LookupError):
        for character, replacement in TYPOGRAPHIC.items():
            line = line.replace(character, replacement)
        line = line.encode(encoding, errors="replace").decode(encoding, errors="replace")
    return line


def emit(line: str, stream=None) -> None:
    stream = stream if stream is not None else sys.stdout
    print(printable(line, stream), file=stream)


def git_status(root: Path) -> str | None:
    try:
        completed = subprocess.run(
            ["git", "-C", str(root), "status", "--porcelain", "--untracked-files=all"],
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout


# ---------------------------------------------------------------------------------------
# Entry point.
# ---------------------------------------------------------------------------------------

DECLARATIONS = (
    ("Mokiterions", "SPEC-MOK-002", "Declared dependency set"),
    ("mokiterions-tui", "SPEC-MOK-003", "Declared dependency set"),
)


def check(root: Path, engine_build: bool) -> Report:
    report = Report()
    members = workspace_members(root)
    member_directories = {directory.resolve() for directory in members.values()}
    member_names = set(members)

    declarations = [
        read_declaration(root, package, document, heading)
        for package, document, heading in DECLARATIONS
    ]
    targets = tuple(
        dict.fromkeys(target for declaration in declarations for target in declaration.targets)
    )
    if not targets:
        raise Refusal(
            "no declaration records a target triple, so there is nothing to resolve against. "
            "The targets are read from the declared per-target figures rather than written here."
        )
    report.note(f"targets: {', '.join(targets)}")

    metadata = cargo_metadata(root)
    with_build_script = build_script_identities(metadata)
    report.note(
        f"cargo metadata --locked reports {len(metadata.get('packages', []))} packages, which is "
        "the whole lockfile across every target and both members and is not any package's set"
    )

    workspace_table = workspace_dependency_entries(root)
    if workspace_table:
        for crate in sorted(workspace_table):
            missing = [
                declaration.document
                for declaration in declarations
                if crate not in declaration.entries
            ]
            if missing:
                report.refuse(
                    f"8.4a [workspace.dependencies] holds {crate}, which is not a declared entry "
                    f"of {', '.join(missing)}. `SPEC-MOK-004` rule 2 admits an entry there only "
                    "when the crate is declared for both packages."
                )
    else:
        report.note("[workspace.dependencies] holds no entry")

    before = git_status(root)

    for declaration in declarations:
        directory = members[declaration.package]
        manifest_path = directory / "Cargo.toml"
        report.note(f"{declaration.package} — declared by {declaration.document}")

        entries, member_paths = manifest_entries(manifest_path, member_directories)
        check_declared_equality(report, declaration, entries, member_paths, manifest_path)

        engine = declaration.package == DECLARATIONS[0][0]
        edges = "normal,build,dev" if engine else "normal"
        graphs = {
            target: cargo_tree(root, declaration.package, edges, target) for target in targets
        }

        check_features(report, declaration, entries, graphs)
        check_counts(report, declaration, graphs, member_names)
        check_build_scripts(report, declaration, graphs, member_names, with_build_script)
        check_prohibited_names(
            report, declaration, graphs, member_names, user_interface_admissible=not engine
        )
        if engine:
            check_engine_closure(report, declaration, graphs, member_names)
            if engine_build:
                check_engine_offline(report, root, declaration.package)
            else:
                report.note(
                    "  8.4c SKIPPED by --no-engine-build. This run does not establish the engine's "
                    "offline build and test, and neither CI placement may pass the flag."
                )

    after = git_status(root)
    if before is not None and after is not None and before != after:
        report.refuse(
            "the working tree changed while these checks ran. Rule 8.4 states that none of them "
            "may modify a tracked file; a check that rewrote `Cargo.lock` would have verified a "
            "different commit than the one under check."
        )
    elif before is None:
        report.note("git is unavailable, so the working-tree comparison did not run")

    return report


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument(
        "--no-engine-build",
        action="store_true",
        help="skip 8.4c, the engine's offline build and test; for a local rehearsal of the set "
        "comparison alone, never for CI",
    )
    parser.add_argument("--json", action="store_true", help="emit the report as JSON")
    arguments = parser.parse_args(argv)

    root = Path(arguments.root).resolve()
    try:
        report = check(root, engine_build=not arguments.no_engine_build)
    except Refusal as refusal:
        emit(f"REFUSED: {refusal}", sys.stderr)
        return 1

    if arguments.json:
        print(json.dumps({"notes": report.lines, "refusals": report.refusals}, indent=2))
    else:
        for line in report.lines:
            emit(line)
        for refusal in report.refusals:
            emit(f"REFUSED: {refusal}", sys.stderr)

    if report.refusals:
        emit(f"{len(report.refusals)} refusal(s).", sys.stderr)
        return 1
    emit("Every declared set matches its resolved graph. 8.4a-8.4d pass.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
