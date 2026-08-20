#!/usr/bin/env python3
"""Tests for `check_declared_dependencies.py`, the shared implementation of rule 8.4.

Repository-owned, absent from `.engineering-harness.lock`. Two kinds of test live here and the
distinction matters to `VER-MOK-014`:

* **Reading tests** parse the real `SPEC-MOK-002` and `SPEC-MOK-003` out of this checkout. They
  fail if an amendment renames a heading, changes a column or writes a *Features* cell the
  convention cannot read — which is the failure mode a hand-copied declaration would hide.
* **Injection tests** hand the checks a declaration and a resolved graph that disagree, so the
  refusals are demonstrated rather than asserted to exist. `VER-MOK-014` acceptance scenarios
  2, 3, 4 and 10 are discharged here: a passing check proves nothing about a check that cannot
  fail, and today's graph matches its declaration, so a real mismatch has to be constructed.

The reading tests assert today's declared values — one entry, `ratatui 0.30.2`, the per-target
counts, the build-script table's size, and the two disclosures with the acceptance the technical
owner recorded against them on 2026-08-20 as `VER-MOK-014` manual assessment 6. **An approved
amendment to a declared set therefore edits this file too**, and that is the one place in this
repository where
that is true: neither workflow and no part of `check_declared_dependencies.py` names a declared
crate, version or figure, so both placements follow an amendment with no edit. These assertions
are not a second declaration — nothing reads them to decide whether a graph conforms — but an
admission that left them untouched would fail the suite, which is the intended reminder rather
than an obstacle. `VER-MOK-014`'s *What an admission must retain* list is the checklist; this
file is a line in it.

Run with:
    python -m unittest discover -s scripts -p "test_*.py"
"""

from __future__ import annotations

import io
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from check_declared_dependencies import (  # noqa: E402
    Crate,
    Declaration,
    Disclosure,
    Entry,
    ManifestEntry,
    Refusal,
    Report,
    cargo_failure,
    check_build_scripts,
    check_counts,
    check_declared_equality,
    check_engine_closure,
    check_features,
    check_prohibited_names,
    manifest_entries,
    name_tokens,
    parse_features,
    parse_target_labels,
    parse_yes_no,
    printable,
    read_declaration,
    read_tables,
    scan_names,
    specification_path,
)

ROOT = Path(__file__).resolve().parents[1]

WINDOWS = "x86_64-pc-windows-msvc"
LINUX = "x86_64-unknown-linux-gnu"
MACOS = "aarch64-apple-darwin"
TARGETS = (WINDOWS, LINUX, MACOS)

MEMBERS = {"Mokiterions", "mokiterions-tui"}


def crate(name: str, version: str, features: str = "") -> Crate:
    return Crate(
        name=name,
        version=version,
        features=frozenset(token for token in features.split(",") if token),
    )


def entry(
    name: str,
    version: str,
    features_cell: str = "`default-features = false`.",
    build_script: bool = False,
) -> Entry:
    return Entry(
        crate=name,
        version=version,
        features=parse_features(features_cell),
        build_script=build_script,
        admitted_by="a test",
    )


def manifest_entry(
    name: str,
    version: str | None,
    default_features: bool = False,
    features: tuple[str, ...] = (),
) -> ManifestEntry:
    return ManifestEntry(
        crate=name,
        version=version,
        default_features=default_features,
        features=features,
        tables=frozenset({"dependencies"}),
        inherited=False,
    )


def declaration(
    package: str = "mokiterions-tui",
    document: str = "SPEC-MOK-003",
    entries: dict[str, Entry] | None = None,
    disclosures: dict[str, Disclosure] | None = None,
    targets: tuple[str, ...] = TARGETS,
    external_counts: dict[str, int] | None = None,
    build_script_counts: dict[str, int] | None = None,
    build_script_crates: dict[str, frozenset[str]] | None = None,
) -> Declaration:
    return Declaration(
        package=package,
        document=document,
        entries=entries or {},
        disclosures=disclosures or {},
        targets=targets,
        external_counts=external_counts or {},
        build_script_counts=build_script_counts or {},
        build_script_crates=build_script_crates or {},
    )


def refusals(function, *arguments, **keywords) -> list[str]:
    report = Report()
    function(report, *arguments, **keywords)
    return report.refusals


class TestTableReading(unittest.TestCase):
    def write(self, text: str) -> Path:
        path = Path(self.enterContext(tempfile.TemporaryDirectory())) / "doc.md"
        path.write_text(textwrap.dedent(text), encoding="utf-8")
        return path

    def test_a_table_carries_the_heading_it_sits_under(self) -> None:
        tables = read_tables(
            self.write(
                """
                ## Declared dependency set

                | Crate | Version |
                |---|---|
                | `ratatui` | `0.30.2` |

                ## Security and privacy properties

                | Crate | Version |
                |---|---|
                | `other` | `1.0.0` |
                """
            )
        )
        self.assertEqual(
            [table.heading for table in tables],
            ["Declared dependency set", "Security and privacy properties"],
        )
        self.assertEqual(tables[0].rows, (("`ratatui`", "`0.30.2`"),))

    def test_a_bold_paragraph_does_not_change_the_heading(self) -> None:
        """The disclosure table sits under a bold lead-in, not a subheading."""
        tables = read_tables(
            self.write(
                """
                ## Declared dependency set

                **Disclosed transitive capabilities.** Two crates.

                | Crate | Version |
                |---|---|
                | `mio` | `1.2.2` |
                """
            )
        )
        self.assertEqual(tables[0].heading, "Declared dependency set")

    def test_a_table_with_no_row_is_read_and_not_skipped(self) -> None:
        """`SPEC-MOK-002` rule 13's declared set is empty today, and empty is a valid answer."""
        tables = read_tables(
            self.write(
                """
                ### Rule 13 - Declared dependency set

                | Crate | Version | Features | Build script | Admitted by |
                |---|---|---|---|---|
                """
            )
        )
        self.assertEqual(len(tables), 1)
        self.assertEqual(tables[0].rows, ())

    def test_a_pipe_line_without_a_separator_is_not_a_table(self) -> None:
        tables = read_tables(self.write("| not | a | table |\nprose follows\n"))
        self.assertEqual(tables, [])


class TestFeatureCellReading(unittest.TestCase):
    """`SPEC-MOK-002` rule 13 fixes this reading; these tests hold the program to it."""

    def test_the_observers_real_cell(self) -> None:
        features = parse_features(
            "`default-features = false`, plus `crossterm`, `layout-cache`, `underline-color`. "
            "`std` is implied by those and is not declared in the manifest. `serde` is off, and "
            "no feature enabling networking, an asynchronous runtime or serialization is on."
        )
        self.assertFalse(features.default_features)
        self.assertEqual(features.declared, {"crossterm", "layout-cache", "underline-color"})
        self.assertEqual(features.implied, {"std"})
        self.assertEqual(features.prohibited, {"serde"})
        self.assertEqual(
            features.expected, {"crossterm", "layout-cache", "std", "underline-color"}
        )

    def test_default_features_stay_on_when_the_switch_is_absent(self) -> None:
        self.assertTrue(parse_features("`serde`.").default_features)

    def test_a_cell_naming_a_feature_present_and_off_refuses(self) -> None:
        with self.assertRaises(Refusal) as raised:
            parse_features("Plus `serde`. `serde` is off.")
        self.assertIn("both present and off", str(raised.exception))

    def test_an_empty_cell_declares_nothing(self) -> None:
        features = parse_features("")
        self.assertEqual(features.expected, frozenset())
        self.assertTrue(features.default_features)


class TestSmallReaders(unittest.TestCase):
    def test_target_cells(self) -> None:
        labels = ("Windows", "Linux", "macOS")
        self.assertEqual(parse_target_labels("all three", labels), set(labels))
        self.assertEqual(parse_target_labels("Linux, macOS", labels), {"Linux", "macOS"})
        self.assertEqual(parse_target_labels("Windows", labels), {"Windows"})
        self.assertEqual(
            parse_target_labels(f"`{LINUX}`, `{MACOS}`", labels), {"Linux", "macOS"}
        )

    def test_an_unreadable_target_cell_refuses(self) -> None:
        with self.assertRaises(Refusal):
            parse_target_labels("sometimes", ("Windows", "Linux", "macOS"))

    def test_build_script_cells(self) -> None:
        self.assertTrue(parse_yes_no("yes", "c"))
        self.assertFalse(parse_yes_no("no", "c"))
        self.assertTrue(parse_yes_no("**yes**", "c"))
        with self.assertRaises(Refusal) as raised:
            parse_yes_no("probably", "c")
        self.assertIn("enumerated and not", str(raised.exception))

    def test_a_name_is_tokenised_on_both_separators(self) -> None:
        self.assertEqual(
            name_tokens("signal-hook-mio"), {"signal-hook-mio", "signal", "hook", "mio"}
        )
        self.assertEqual(name_tokens("parking_lot_core"), {"parking_lot_core", "parking", "lot", "core"})

    def test_the_scan_matches_a_token_inside_a_compound_name(self) -> None:
        """`signal-hook-mio` is a hit on the `mio` token, which is why it is disclosed."""
        hits = scan_names([crate("signal-hook-mio", "0.2.5"), crate("ratatui", "0.30.2")], ("mio",))
        self.assertEqual([hit.name for hit in hits], ["signal-hook-mio"])

    def test_a_stale_lockfile_and_an_empty_cache_are_told_apart(self) -> None:
        """Both stop cargo before any comparison, and the fix for one is useless for the other."""
        stale = cargo_failure(
            ["tree", "-p", "mokiterions-tui"],
            "error: cannot update the lock file C:\\x\\Cargo.lock because --locked was passed to "
            "prevent this\nIf you want to try to update, run without --locked",
        )
        self.assertIn("the lockfile does not describe these manifests", str(stale))
        self.assertIn("Fetching will not help", str(stale))

        unfetched = cargo_failure(
            ["tree", "-p", "mokiterions-tui"],
            "error: no matching package named `ratatui` found\nperhaps you meant to run without "
            "--offline",
        )
        self.assertIn("cargo refused to resolve offline", str(unfetched))
        self.assertIn("cargo fetch --locked", str(unfetched))

        other = cargo_failure(["tree", "-p", "nope"], "error: package ID specification is invalid")
        self.assertIn("`cargo tree -p nope` failed", str(other))

    def test_printable_folds_only_what_the_stream_cannot_take(self) -> None:
        line = "mokiterions-tui — declared by SPEC-MOK-003"
        utf8 = io.TextIOWrapper(io.BytesIO(), encoding="utf-8")
        ascii_stream = io.TextIOWrapper(io.BytesIO(), encoding="ascii")
        self.assertEqual(printable(line, utf8), line)
        self.assertEqual(printable(line, ascii_stream), "mokiterions-tui -- declared by SPEC-MOK-003")


class TestReadingTheRepositoryDeclarations(unittest.TestCase):
    """These parse the artifacts in this checkout. An amendment that breaks the reading fails here."""

    def test_the_engine_declares_an_empty_set(self) -> None:
        engine = read_declaration(ROOT, "Mokiterions", "SPEC-MOK-002", "Declared dependency set")
        self.assertEqual(engine.entries, {})

    def test_the_observer_declares_one_entry(self) -> None:
        observer = read_declaration(
            ROOT, "mokiterions-tui", "SPEC-MOK-003", "Declared dependency set"
        )
        self.assertEqual(list(observer.entries), ["ratatui"])
        ratatui = observer.entries["ratatui"]
        self.assertEqual(ratatui.version, "0.30.2")
        self.assertFalse(ratatui.build_script)
        self.assertEqual(
            ratatui.features.expected, {"crossterm", "layout-cache", "std", "underline-color"}
        )
        self.assertEqual(ratatui.features.prohibited, {"serde"})

    def test_the_observers_figures_and_targets(self) -> None:
        observer = read_declaration(
            ROOT, "mokiterions-tui", "SPEC-MOK-003", "Declared dependency set"
        )
        self.assertEqual(observer.targets, TARGETS)
        self.assertEqual(observer.external_counts[WINDOWS], 57)
        self.assertEqual(observer.external_counts[LINUX], 63)
        self.assertEqual(observer.external_counts[MACOS], 62)
        self.assertEqual(observer.external_counts["union"], 66)
        self.assertEqual(observer.build_script_counts[WINDOWS], 7)
        self.assertEqual(observer.build_script_counts[LINUX], 9)

    def test_the_observers_build_script_table(self) -> None:
        observer = read_declaration(
            ROOT, "mokiterions-tui", "SPEC-MOK-003", "Declared dependency set"
        )
        self.assertEqual(len(observer.build_script_crates["Windows"]), 7)
        self.assertEqual(len(observer.build_script_crates["Linux"]), 9)
        self.assertEqual(len(observer.build_script_crates["macOS"]), 9)
        self.assertIn("winapi 0.3.9", observer.build_script_crates["Windows"])
        self.assertNotIn("winapi 0.3.9", observer.build_script_crates["Linux"])
        self.assertIn("rustix 1.1.4", observer.build_script_crates["macOS"])

    def test_the_observers_two_disclosures_are_accepted(self) -> None:
        # The technical owner recorded `VER-MOK-014` manual assessment 6 on 2026-08-20 and
        # accepted both. What is asserted here is that the *Assessment* cell still carries a
        # judgement with its grounds and its limit — an acceptance stated as "accepted" and
        # nothing else would read the same as a cell nobody had filled in.
        observer = read_declaration(
            ROOT, "mokiterions-tui", "SPEC-MOK-003", "Declared dependency set"
        )
        self.assertEqual(sorted(observer.disclosures), ["mio", "signal-hook-mio"])
        mio = observer.disclosures["mio"]
        self.assertEqual(mio.version, "1.2.2")
        self.assertEqual(mio.labels, {"Linux", "macOS"})
        self.assertIn("net", mio.capability)
        self.assertNotIn("OUTSTANDING", mio.assessment)
        self.assertIn("Accepted 2026-08-20", mio.assessment)
        self.assertIn("compiled and uncalled capability", mio.assessment)
        signal_hook_mio = observer.disclosures["signal-hook-mio"]
        self.assertNotIn("OUTSTANDING", signal_hook_mio.assessment)
        self.assertIn("Accepted 2026-08-20", signal_hook_mio.assessment)

    def test_a_missing_heading_refuses_rather_than_guesses(self) -> None:
        with self.assertRaises(Refusal) as raised:
            read_declaration(ROOT, "mokiterions-tui", "SPEC-MOK-003", "Dependency inventory")
        self.assertIn("read from the artifact", str(raised.exception))

    def test_the_specification_paths_exist(self) -> None:
        for document in ("SPEC-MOK-002", "SPEC-MOK-003"):
            self.assertTrue(specification_path(ROOT, document).is_file())


class TestManifestReading(unittest.TestCase):
    def setUp(self) -> None:
        self.directory = Path(self.enterContext(tempfile.TemporaryDirectory()))
        (self.directory / "member").mkdir()
        (self.directory / "other").mkdir()

    def write(self, text: str) -> Path:
        path = self.directory / "Cargo.toml"
        path.write_text(textwrap.dedent(text), encoding="utf-8")
        return path

    def test_a_path_dependency_on_a_member_is_not_an_entry(self) -> None:
        path = self.write(
            """
            [dependencies]
            Mokiterions = { path = "member" }
            ratatui = { version = "0.30.2", default-features = false, features = ["crossterm"] }
            """
        )
        entries, member_paths = manifest_entries(path, {(self.directory / "member").resolve()})
        self.assertEqual(list(entries), ["ratatui"])
        self.assertEqual(len(member_paths), 1)
        self.assertFalse(entries["ratatui"].default_features)
        self.assertEqual(entries["ratatui"].features, ("crossterm",))

    def test_a_path_dependency_outside_the_workspace_refuses(self) -> None:
        path = self.write(
            """
            [dependencies]
            vendored = { path = "other" }
            """
        )
        with self.assertRaises(Refusal) as raised:
            manifest_entries(path, {(self.directory / "member").resolve()})
        self.assertIn("there is no third kind", str(raised.exception))

    def test_platform_and_dev_tables_are_entries_too(self) -> None:
        path = self.write(
            """
            [dev-dependencies]
            proptest = "1.0.0"

            [target.'cfg(unix)'.dependencies]
            libc = "0.2.189"
            """
        )
        entries, _ = manifest_entries(path, set())
        self.assertEqual(sorted(entries), ["libc", "proptest"])
        self.assertEqual(entries["libc"].tables, {"target.cfg(unix).dependencies"})

    def test_an_inherited_version_is_marked_as_such(self) -> None:
        path = self.write(
            """
            [dependencies]
            ratatui = { workspace = true }
            """
        )
        entries, _ = manifest_entries(path, set())
        self.assertTrue(entries["ratatui"].inherited)


class TestSetEquality(unittest.TestCase):
    """8.4a, injected in both directions. Scenarios 2 and 3 of `VER-MOK-014`."""

    def setUp(self) -> None:
        self.declaration = declaration(entries={"ratatui": entry("ratatui", "0.30.2")})
        self.manifest = Path("mokiterions-tui/Cargo.toml")

    def test_a_matching_pair_passes(self) -> None:
        found = refusals(
            check_declared_equality,
            self.declaration,
            {"ratatui": manifest_entry("ratatui", "0.30.2")},
            [],
            self.manifest,
        )
        self.assertEqual(found, [])

    def test_an_undeclared_crate_in_the_manifest_refuses(self) -> None:
        found = refusals(
            check_declared_equality,
            self.declaration,
            {
                "ratatui": manifest_entry("ratatui", "0.30.2"),
                "serde": manifest_entry("serde", "1.0.0"),
            },
            [],
            self.manifest,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("declares serde, which SPEC-MOK-003", found[0])
        self.assertIn("not an implementation act", found[0])

    def test_a_declared_crate_absent_from_the_manifest_refuses(self) -> None:
        found = refusals(check_declared_equality, self.declaration, {}, [], self.manifest)
        self.assertEqual(len(found), 1)
        self.assertIn("both directions", found[0])

    def test_a_version_the_manifest_does_not_ask_for_refuses(self) -> None:
        found = refusals(
            check_declared_equality,
            self.declaration,
            {"ratatui": manifest_entry("ratatui", "0.30.1")},
            [],
            self.manifest,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("not a range", found[0])


class TestFeatureChecks(unittest.TestCase):
    """8.4b, injected. Scenario 4 of `VER-MOK-014`, including the negatives."""

    def setUp(self) -> None:
        self.cell = (
            "`default-features = false`, plus `crossterm`, `layout-cache`, `underline-color`. "
            "`std` is implied by those. `serde` is off."
        )
        self.declaration = declaration(
            entries={"ratatui": entry("ratatui", "0.30.2", self.cell)}
        )
        self.manifest = {
            "ratatui": manifest_entry(
                "ratatui",
                "0.30.2",
                default_features=False,
                features=("crossterm", "layout-cache", "underline-color"),
            )
        }
        self.resolved = "crossterm,layout-cache,std,underline-color"

    def graphs(self, features: str) -> dict[str, list[Crate]]:
        return {target: [crate("ratatui", "0.30.2", features)] for target in TARGETS}

    def test_the_real_feature_set_passes(self) -> None:
        self.assertEqual(
            refusals(
                check_features, self.declaration, self.manifest, self.graphs(self.resolved)
            ),
            [],
        )

    def test_a_feature_arriving_by_unification_refuses(self) -> None:
        found = refusals(
            check_features,
            self.declaration,
            self.manifest,
            self.graphs(self.resolved + ",unicode-truncation"),
        )
        self.assertTrue(all("neither declared nor recorded as implied" in line for line in found))
        self.assertEqual(len(found), len(TARGETS))

    def test_a_prohibited_feature_refuses(self) -> None:
        found = refusals(
            check_features, self.declaration, self.manifest, self.graphs(self.resolved + ",serde")
        )
        self.assertEqual(len(found), len(TARGETS))
        self.assertIn("records as off", found[0])

    def test_a_missing_declared_feature_refuses(self) -> None:
        found = refusals(
            check_features,
            self.declaration,
            self.manifest,
            self.graphs("crossterm,layout-cache,std"),
        )
        self.assertEqual(len(found), len(TARGETS))
        self.assertIn("underline-color", found[0])

    def test_default_features_left_on_in_the_manifest_refuses(self) -> None:
        manifest = {
            "ratatui": manifest_entry(
                "ratatui",
                "0.30.2",
                default_features=True,
                features=("crossterm", "layout-cache", "underline-color"),
            )
        }
        found = refusals(check_features, self.declaration, manifest, self.graphs(self.resolved))
        self.assertEqual(len(found), 1)
        self.assertIn("default-features", found[0])

    def test_a_manifest_feature_the_declaration_does_not_state_refuses(self) -> None:
        manifest = {
            "ratatui": manifest_entry(
                "ratatui", "0.30.2", default_features=False, features=("crossterm",)
            )
        }
        found = refusals(check_features, self.declaration, manifest, self.graphs(self.resolved))
        self.assertIn("is an amendment", found[0])

    def test_a_declared_crate_missing_from_the_graph_refuses(self) -> None:
        found = refusals(check_features, self.declaration, self.manifest, {LINUX: []})
        self.assertEqual(len(found), 1)
        self.assertIn("does not appear in the resolved graph", found[0])


class TestProhibitedNames(unittest.TestCase):
    """8.4d, injected. Scenarios 3 and 10 of `VER-MOK-014`."""

    def disclosure(self, labels: frozenset[str] = frozenset({"Linux", "macOS"})) -> Disclosure:
        return Disclosure(
            crate="mio",
            version="1.2.2",
            labels=labels,
            capability="Non-blocking I/O poll; `net` compiles in socket types.",
            assessment="**OUTSTANDING.** Requires the technical owner.",
        )

    def accepted_disclosure(self) -> Disclosure:
        """The observer's real position since 2026-08-20: assessment 6 recorded, accepted."""
        return Disclosure(
            crate="mio",
            version="1.2.2",
            labels=frozenset({"Linux", "macOS"}),
            capability="Non-blocking I/O poll; `net` compiles in socket types.",
            assessment=(
                "**Accepted 2026-08-20** by the repository owner acting as accountable technical "
                "owner. What is accepted is a compiled and uncalled capability."
            ),
        )

    def unix_graphs(self, *names: Crate) -> dict[str, list[Crate]]:
        return {LINUX: list(names), MACOS: list(names), WINDOWS: []}

    def test_an_outstanding_disclosure_passes_and_says_the_judgement_is_owed(self) -> None:
        report = Report()
        check_prohibited_names(
            report,
            declaration(
                entries={"ratatui": entry("ratatui", "0.30.2")},
                disclosures={"mio": self.disclosure()},
            ),
            self.unix_graphs(crate("mio", "1.2.2"), crate("ratatui", "0.30.2")),
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertEqual(report.refusals, [])
        self.assertTrue(any("raw hit" in line for line in report.lines))
        self.assertTrue(any("disclosed and OUTSTANDING" in line for line in report.lines))

    def test_an_accepted_disclosure_is_still_printed(self) -> None:
        """The observer's real position. An acceptance must not silence the disclosure.

        If the report only spoke about disclosures whose assessment is outstanding, recording
        the acceptance would delete the line — and a graph with an accepted `mio` would read
        exactly like a graph with no `mio` in it. That is the state rule 8.4d's table exists to
        prevent, so the line is printed either way and says which case it is.
        """
        report = Report()
        check_prohibited_names(
            report,
            declaration(
                entries={"ratatui": entry("ratatui", "0.30.2")},
                disclosures={"mio": self.accepted_disclosure()},
            ),
            self.unix_graphs(crate("mio", "1.2.2"), crate("ratatui", "0.30.2")),
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertEqual(report.refusals, [])
        self.assertTrue(any("raw hit" in line for line in report.lines))
        accepted = [line for line in report.lines if "disclosed and accepted" in line]
        self.assertEqual(len(accepted), 1)
        self.assertIn("mio 1.2.2", accepted[0])
        self.assertFalse(any("OUTSTANDING" in line for line in report.lines))

    def test_the_raw_hits_are_reported_before_any_disclosure_is_applied(self) -> None:
        """Scenario 10: the evidence retains what the scan saw, not what survived it."""
        report = Report()
        check_prohibited_names(
            report,
            declaration(disclosures={"mio": self.disclosure()}),
            self.unix_graphs(crate("mio", "1.2.2")),
            MEMBERS,
            user_interface_admissible=True,
        )
        raw = [line for line in report.lines if "raw hit" in line]
        self.assertEqual(len(raw), 2)
        self.assertTrue(all("mio 1.2.2" in line for line in raw))

    def test_an_undisclosed_transitive_hit_refuses(self) -> None:
        found = refusals(
            check_prohibited_names,
            declaration(),
            self.unix_graphs(crate("mio", "1.2.2")),
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertEqual(len(found), 2)
        self.assertIn("does not disclose it", found[0])

    def test_a_declared_entry_in_a_prohibited_class_refuses_with_no_disclosure_available(
        self,
    ) -> None:
        found = refusals(
            check_prohibited_names,
            declaration(
                entries={"tokio": entry("tokio", "1.0.0")},
                disclosures={
                    "tokio": Disclosure("tokio", "1.0.0", frozenset({"Linux"}), "x", "accepted")
                },
            ),
            {LINUX: [crate("tokio", "1.0.0")]},
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertTrue(any("no disclosure can admit it" in line for line in found))
        self.assertTrue(any("decision 4" in line for line in found))

    def test_a_disclosure_for_the_wrong_target_refuses(self) -> None:
        found = refusals(
            check_prohibited_names,
            declaration(disclosures={"mio": self.disclosure(frozenset({"Linux"}))}),
            self.unix_graphs(crate("mio", "1.2.2")),
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("disclosed only for Linux", found[0])

    def test_a_disclosure_at_the_wrong_version_refuses(self) -> None:
        found = refusals(
            check_prohibited_names,
            declaration(disclosures={"mio": self.disclosure()}),
            self.unix_graphs(crate("mio", "1.3.0")),
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertEqual(len(found), 2)
        self.assertIn("resolves at 1.3.0", found[0])

    def test_a_disclosure_no_target_reaches_refuses(self) -> None:
        found = refusals(
            check_prohibited_names,
            declaration(disclosures={"mio": self.disclosure()}),
            {WINDOWS: [crate("winapi", "0.3.9")]},
            MEMBERS,
            user_interface_admissible=True,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("look weighed when it was not", found[0])

    def test_a_user_interface_crate_in_the_engine_graph_refuses(self) -> None:
        found = refusals(
            check_prohibited_names,
            declaration(package="Mokiterions", document="SPEC-MOK-002"),
            {LINUX: [crate("ratatui", "0.30.2")]},
            MEMBERS,
            user_interface_admissible=False,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("REQ-MOK-026", found[0])

    def test_a_workspace_member_is_never_a_scan_hit(self) -> None:
        """`mokiterions-tui` carries the `tui` token; it is a member, not a dependency."""
        found = refusals(
            check_prohibited_names,
            declaration(package="Mokiterions", document="SPEC-MOK-002"),
            {LINUX: [crate("mokiterions-tui", "0.1.0"), crate("Mokiterions", "0.1.0")]},
            MEMBERS,
            user_interface_admissible=False,
        )
        self.assertEqual(found, [])


class TestCountsAndBuildScripts(unittest.TestCase):
    def graphs(self, *names: Crate) -> dict[str, list[Crate]]:
        return {LINUX: list(names)}

    def test_a_count_that_moved_refuses(self) -> None:
        found = refusals(
            check_counts,
            declaration(external_counts={LINUX: 2}),
            self.graphs(crate("ratatui", "0.30.2")),
            MEMBERS,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("stale specification", found[0])

    def test_a_union_that_moved_refuses(self) -> None:
        found = refusals(
            check_counts,
            declaration(external_counts={LINUX: 1, "union": 3}),
            self.graphs(crate("ratatui", "0.30.2")),
            MEMBERS,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("union", found[0])

    def test_members_do_not_count_towards_the_figure(self) -> None:
        found = refusals(
            check_counts,
            declaration(external_counts={LINUX: 1}),
            self.graphs(crate("ratatui", "0.30.2"), crate("Mokiterions", "0.1.0")),
            MEMBERS,
        )
        self.assertEqual(found, [])

    def test_a_crate_that_gained_a_build_script_refuses(self) -> None:
        found = refusals(
            check_build_scripts,
            declaration(
                build_script_crates={"Linux": frozenset({"libc 0.2.189"})},
                build_script_counts={LINUX: 1},
            ),
            self.graphs(crate("libc", "0.2.189"), crate("ratatui", "0.30.2")),
            MEMBERS,
            {"libc 0.2.189", "ratatui 0.30.2"},
        )
        self.assertTrue(any("ratatui 0.30.2 carries a build script" in line for line in found))
        self.assertTrue(any("not an unremarked change" in line for line in found))

    def test_a_recorded_build_script_the_graph_does_not_reach_refuses(self) -> None:
        found = refusals(
            check_build_scripts,
            declaration(
                build_script_crates={"Linux": frozenset({"winapi 0.3.9"})},
                build_script_counts={LINUX: 1},
            ),
            self.graphs(crate("ratatui", "0.30.2")),
            MEMBERS,
            set(),
        )
        self.assertTrue(any("and the resolved graph does not" in line for line in found))

    def test_a_declaration_with_no_build_script_table_checks_nothing(self) -> None:
        self.assertEqual(
            refusals(
                check_build_scripts,
                declaration(),
                self.graphs(crate("libc", "0.2.189")),
                MEMBERS,
                {"libc 0.2.189"},
            ),
            [],
        )


class TestEngineClosure(unittest.TestCase):
    """`SPEC-MOK-002` rule 13's strict direction: the engine's whole graph is its declared set."""

    def test_an_empty_graph_passes(self) -> None:
        found = refusals(
            check_engine_closure,
            declaration(package="Mokiterions", document="SPEC-MOK-002"),
            {LINUX: [crate("Mokiterions", "0.1.0")]},
            MEMBERS,
        )
        self.assertEqual(found, [])

    def test_any_transitive_crate_refuses_when_the_set_is_empty(self) -> None:
        found = refusals(
            check_engine_closure,
            declaration(package="Mokiterions", document="SPEC-MOK-002"),
            {LINUX: [crate("Mokiterions", "0.1.0"), crate("libc", "0.2.189")]},
            MEMBERS,
        )
        self.assertEqual(len(found), 1)
        self.assertIn("libc 0.2.189", found[0])
        self.assertIn("transitive and dev-only included", found[0])

    def test_a_transitive_crate_of_a_declared_entry_is_not_surplus(self) -> None:
        found = refusals(
            check_engine_closure,
            declaration(
                package="Mokiterions",
                document="SPEC-MOK-002",
                entries={"libc": entry("libc", "0.2.189")},
            ),
            {LINUX: [crate("Mokiterions", "0.1.0"), crate("libc", "0.2.189")]},
            MEMBERS,
        )
        self.assertEqual(found, [])


if __name__ == "__main__":
    unittest.main()
