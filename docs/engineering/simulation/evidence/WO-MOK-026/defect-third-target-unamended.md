# Defect: the engine package declares a third target and `SPEC-MOK-002` rule 1 still forbids one

Recorded 2026-08-29 under `WO-MOK-026` item 16, while measuring `SPEC-MOK-004` rule 11's figures at candidate
`7f9e20a`. Nothing here is repaired. Every item below is a change of substance in an approved rule, and this work
order's stop-and-escalate condition 6 forbids an implementation agent amending an approved artifact on its own
judgement. It is recorded on the route the repository owner has twice chosen for exactly this shape — a conflict
between an approved rule and what the tree does, recorded with the stage's evidence and left for the owner.

It was found by measuring rather than by reading: this rule states a **test-binary count** beside its test counts, and
reconciling twenty-three to twenty-five is what exposed a target no approved rule admits.

## 1. The manifest declares three targets and rule 1 permits two

`mokiterions-core/Cargo.toml` at this candidate, and on `main` since `1046f23`:

```toml
[lib]
name = "mokiterions"
path = "src/lib.rs"

[[bin]]
name = "Mokiterions"
path = "src/main.rs"

[[bin]]
name = "canned-connector"
path = "tests/support/canned_connector.rs"
```

`SPEC-MOK-002` rule 1, unamended:

> The package name stays `Mokiterions`. It declares exactly two targets:
>
> | Target | Kind | Name | Path |
> |---|---|---|---|
> | Library | `[lib]` | `mokiterions` | `src/lib.rs` |
> | Binary | `[[bin]]` | `Mokiterions` | `src/main.rs` |
>
> No third target and no build script.

**The clause has been reaffirmed twice and never relaxed.** The 2026-08-18 amendment records that the rule read "No
third target, no second package, no workspace, no build script" and removed only the package and workspace clauses,
for `REQ-MOK-026`'s two-package split. The 2026-08-20 amendment states in terms that *"No third target and no build
script" is **unchanged***, and that `ADR-MOK-006`'s admission of third-party crates "does not open" the build-script
half. So the prohibition is not a survival of stale text: it is the standing form of the rule at both of its last two
amendments.

Measured rather than asserted: `grep -n "canned" docs/engineering/simulation/specifications/SPEC-MOK-002.md` returns
nothing, and `WO-HUP-002` — which the manifest's own comment cites as the authority for declaring the target — appears
nowhere in that specification. `WO-HUP-002` is a harness-domain work order whose in-scope amendment is
`SPEC-HUP-001` rule 11; it decides *how a fixture becomes a process* and does not reach `SPEC-MOK-002`.

**What the target is, stated fairly.** It is not a second product binary. `SPEC-MOK-007` rule 20.5's canned connector
is an offline test fixture, it declares no dependency, it ships in no release, and it is a `[[bin]]` for one reason:
a connector is a *child process*, so a fixture that is not a real executable cannot exercise the thing it exists for,
and `CARGO_BIN_EXE_canned-connector` is what gives an integration test a path to spawn. The path is `tests/support/`
and not `tests/` so that Cargo does not also discover it as an integration-test target. The reasoning is sound; what
is missing is a rule that admits it.

**`default-run` is not part of this defect.** `SPEC-MOK-004` rule 2 admits a member-level `default-run` key "if, and
only if, rule 14's command check shows that a form the operator uses no longer resolves without it", requires that the
resolution it restores be stated in the manifest as a comment, and forbids a rename as the correction. `b6d9423` meets
all three: `cargo run -p Mokiterions` stopped resolving once the package built two binaries, two repository-owned
workflows invoke it that way, the comment names both, and neither target was renamed. That key is admitted; the
`[[bin]]` above is what is not.

## 2. `SPEC-MOK-004` rule 1's own ground for standing still has gone false

`WO-MOK-026` item 16 closes:

> An earlier draft of this work order made its amendment a precondition, on the assumption that a provider package
> would need a layout entry; the connector lives outside the repository and the canned one lives in an existing test
> tree, so **no directory is added** and the rule stands unchanged.

A directory *was* added. `mokiterions-core/tests/support/` does not exist at `bce4229` and does exist at `d96cced` and
at this candidate, measured by `git ls-tree` at each. Rule 1's layout tree shows the engine package as

```text
  src/                         # lib.rs, main.rs, cli.rs, simulation.rs
  tests/                       # the engine's public tier, five files
```

and shows no subdirectory under `tests/`.

**Rule 1 is left unmoved, because this work order says so in terms — "`SPEC-MOK-004` rule 1 does not move."** What is
corrected is the *ground*, here rather than in the rule: the instruction is followed, and the premise it rests on is
recorded as false rather than repeated. Two further staleness facts in the same tree, both older than this work order
and neither this stage's: the engine's public tier reads "five files" and holds **eleven** at this candidate, and the
observer's reads "eight files" and holds **nine** since `WO-MOK-025` added `tests/replay.rs`. Rule 1's own closing
prohibitions — no third package directory, no nested workspace, no directory holding two packages' sources — are all
satisfied, and nothing above touches them. The `src/` comments are accurate at this candidate for both packages.

## 3. What repairing it would take

Recorded so that the owner's options are on the record and none is chosen here.

1. **Amend `SPEC-MOK-002` rule 1** to admit a third target of a stated kind — a test fixture that is a child process,
   under a path Cargo does not auto-discover, declaring no dependency and shipping in no release. This is the narrowest
   form and it is the one the manifest already conforms to. It has to say what makes a target admissible rather than
   naming this one, or the next fixture reopens the same question.
2. **Amend `SPEC-MOK-004` rule 1's layout tree** to show `tests/support/`, which this work order forbids and a later
   one can do. The two staleness figures above would be corrected in the same act.
3. **Remove the target** and reach the connector some other way. Every route measured at this candidate is worse: a
   plain source file is not an executable, so nothing can spawn it; a third workspace member is what `WO-MOK-026`
   forbids outright ("a test fixture inside an existing package's test tree — not a third workspace member"); and a
   script invoked through an interpreter puts a runtime this repository declares nowhere into the test path.

Options 1 and 2 are independent: the first is about what the package may declare and the second about what a diagram
says. Only the first is a live contradiction between an approved rule and the tree.
