# The renumbering of this chain from `007` to `010`

This file records a governance act taken on the packet itself rather than a measurement. It is retained because a
reader who hashes the files below will find one recorded digest that no longer reproduces, and because sixteen
retained captures in this packet name a work order by an identifier that now belongs to different work.

## What happened

This work order was drafted, approved and implemented as `WO-MOK-007`, with `VER-MOK-007`, `VREC-MOK-007` and an
evidence directory at `docs/engineering/simulation/evidence/WO-MOK-007/`. While it sat unmerged on
`feature/phase-2-individuality`, three other streams of work advanced and two of them took numbers this chain was
using.

`master` advanced by ten commits, four of which created a **different** `WO-MOK-007` — *Colour the roster survival
bars by value band*, implementing `REQ-MOK-020` — together with its own `VER-MOK-007` and a `VREC-MOK-007` that was
prepared and transitioned to `verified` at commit `dfab77b`, and its own evidence directory at the same path. Three
identifiers and one directory therefore denoted two unrelated pieces of work, and `master`'s side of the collision is a
verified record. Nothing here could be merged without either overwriting that record or renaming approved artifacts,
and both are the owner's acts, not an implementation agent's.

`feature/release-ci` holds a `WO-MOK-008` — *Make the provenance footer shed fields without losing authoritative
information*, in `draft` — and a `VER-MOK-008` — *Release authorization, compliance, provenance and reserved-act
verification*, `approved` — as well as a `WO-MOK-009` at `implemented`. So `008` and `009` were taken too, by work that
has not reached `master` and is not visible in it.

**The repository owner, on 2026-08-19, chose to renumber this branch's chain to `010`.** `WO-MOK-007` becomes
`WO-MOK-010`, `VER-MOK-007` becomes `VER-MOK-010`, `VREC-MOK-007` becomes `VREC-MOK-010`, and this directory moves from
`evidence/WO-MOK-007/` to `evidence/WO-MOK-010/`. `010` is the lowest number free across every ref in the repository,
so it collides with nothing in flight. `master`'s three artifacts and its evidence directory are untouched and keep the
number they were approved under, as are `feature/release-ci`'s. The alternatives the owner declined were renumbering
`master`'s verified chain, renumbering `feature/release-ci`'s approved `VER-MOK-008`, and merging two work orders into
one.

Nothing about the work changed. No requirement, specification provision, oracle, measurement, decision or line of
executable behaviour is different because of the renumbering. Only the name is.

## What was rewritten, and what was not

The rename was applied by byte substitution of the three identifiers across every tracked file, with one exclusion.

**Rewritten — 45 files, 310 occurrences.** The three artifacts themselves; `SPEC-MOK-001` through `SPEC-MOK-004`,
`REQ-MOK-032`, `REQ-MOK-034`, `SIMULATION_RULES.md`, `../../../../ROADMAP.md` and the repository
`.gitattributes` comment; the doc comments and test comments in `mokiterions-core` and `mokiterions-tui` that cite the
work order or the verification contract; and, in this packet, the six authored prose files — `README.md`,
`completion-summary.md`, `escalation.md`, `manual-assessment.md`, `requirement-to-test-mapping.md`,
`amendment-approvals.md` — together with every retained `.py`, `.sh` and `.rs` script.

The scripts were rewritten deliberately, and for a reason worth stating: each one is retained so a figure can be
reproduced from the recorded command rather than trusted, and a script whose output paths point at a directory that no
longer exists reproduces nothing. Their rewritten occurrences are output paths and header strings.

**Not rewritten — 19 files, byte-identical at the time of the rename.** Every retained `.txt` in this packet that names
an artifact at all; `baseline/COMMIT.txt` and the two `exit-codes.txt` name none and are unaffected either way. Three of
the nineteen have since been re-taken against the merged tree, which the last section of this file records:

```
baseline/pre-manifest.txt          measurements/long-horizon.txt      observer/roster-frames.txt
baseline/rebuild-check.txt         measurements/oscillation.txt       post/additivity.txt
baseline/recapture-check.txt       measurements/proposals.txt         post/post-manifest.txt
interface-and-purity.txt           measurements/traits.txt            static-checks.txt
measurements/divergence.txt        measurements/viability.txt         test-census.txt
measurements/equivalence.txt       negative-control/oracle-2.txt
measurements/fear.txt              negative-control/oracle-3.txt
```

These are program output. Twenty-five lines across them read, when the rename was applied, `WO-MOK-007` or
`VER-MOK-007`, all of them in headers the
analysis scripts printed. Editing them would make the packet assert that a tool printed something it did not print,
which is a worse defect than a stale name: a capture that has been improved is no longer a capture. **They name this
work order as `WO-MOK-007` because that is what it was called when they were taken.** A reader who meets that
identifier in a `.txt` in this directory should read it as this work order under its former name, and should not follow
it to `master`'s `WO-MOK-007`, which is unrelated. Captures re-taken after this date name it `WO-MOK-010`, so the
packet is mixed on this point by design.

One consequence is worth naming for whoever reproduces a figure: the retained scripts now write to
`evidence/WO-MOK-010/`, while the captures beside them were written by the same scripts pointing at
`evidence/WO-MOK-007/`. Re-running a script therefore reproduces a capture's *content* at a new path, and a byte
comparison against the retained file must be made on content, not on the header line, which will differ by three
characters.

**The raw per-cell captures are not in this packet and never were.** `baseline/capture.sh` writes 180 files to a
`baseline/streams/` directory; `VER-MOK-010`'s retention list keeps 11 of the 42 baseline cells whole in
`baseline/full/` and the rest by digest in `baseline/pre-manifest.txt`, which is what was approved. Those 180 files
exist only in the clone that produced them, untracked, and are absent from any checkout of this commit.

## The one recorded digest the rename invalidates

`negative-control/oracle-2.txt` and `negative-control/oracle-3.txt` both record

```
sha256(simulation.rs) before and after all controls: 4850384d0fec95682dadda00d87a53fbeba026474a6916a773058a46927b3671
```

and `completion-summary.md` quotes the same digest. That value is the SHA-256 of the **committed blob**, which is to say
of the file with `\n` line endings; a Windows checkout under `core.autocrlf = true` holds the same content with `\r\n`
and hashes differently, which is the same trap `README.md` records for the retained streams. Checked rather than
assumed: the blob at the pre-rename commit hashes to `4850384d…` exactly.

The rename edits eight lines of `mokiterions-core/src/simulation.rs`, so that file's blob now hashes to

```
55732ea499221e8acfd931095c8009574c18ab044c5af6bb5a8955292c30e671
```

All eight edited lines are comments — eight changed, eight added, none of them code. The complete diff is:

```
-/// It was `ATTRIBUTE_MAX` until the sweep in `evidence/WO-MOK-007/escalation.md` showed the full
+/// It was `ATTRIBUTE_MAX` until the sweep in `evidence/WO-MOK-010/escalation.md` showed the full
-// ---- WO-MOK-007: the trait, fear, and the trait-aware source -------------------------
+// ---- WO-MOK-010: the trait, fear, and the trait-aware source -------------------------
-/// The verification seed set `VER-MOK-002` declares, reused unchanged by `VER-MOK-007` so
+/// The verification seed set `VER-MOK-002` declares, reused unchanged by `VER-MOK-010` so
-/// `VER-MOK-007` requires a recorded expectation checked into the suite rather than a
+/// `VER-MOK-010` requires a recorded expectation checked into the suite rather than a
-/// negative control in `evidence/WO-MOK-007/negative-control/oracle-2.txt` shows why a
+/// negative control in `evidence/WO-MOK-010/negative-control/oracle-2.txt` shows why a
-/// These counts are what `VER-MOK-007` oracle 2 pins. Initialization places
+/// These counts are what `VER-MOK-010` oracle 2 pins. Initialization places
-/// `VER-MOK-007` oracle 2: the shared entropy stream's own position, either side of trait
+/// `VER-MOK-010` oracle 2: the shared entropy stream's own position, either side of trait
-/// `VER-MOK-007` oracle 3: at the trait's lower bound the trait-aware source proposes what
+/// `VER-MOK-010` oracle 3: at the trait's lower bound the trait-aware source proposes what
```

The claim those two controls make is *that the file was byte-identical before and after every injected control* — that
no control leaked into the tree. That claim is unaffected: it is a claim about equality across the control run, not
about a particular value, and both controls were run against the file as it stood at capture. What the recorded value
no longer does is identify the file in the tree. Recomputing it here rather than editing the captures keeps both facts
available. **The digests of every simulation stream in `baseline/`, `post/` and `measurements/` are untouched by the
rename**, because no stream contains an artifact identifier; they are the digests the manifests exist to protect, and
they still reproduce.

## What this act does not discharge

The renumbering resolves an identity collision. It does not make this packet current. Independently of it, and
recorded here so the two are not confused:

- `observer/roster-frames.txt` was captured against `SPEC-MOK-003` rule 5's tier table, which `WO-MOK-005` withdrew on
  `master`. It measures which viewports present the roster, and that set changed from four to eight. It needs
  re-deriving against the merged tree. **Since discharged — see below.**
- `test-census.txt` reconciles 169 tests to 190. The merged tree runs more than that, `master`'s band tests among them.
  **Since discharged — see below.**
- `VREC-MOK-010` is a `ready` candidate still bound to commit `4f32a9f`, which predates the merge, this renumbering and
  the reconciliation of rule 4 with `master`'s bands. It is re-captured against the merged commit, not edited into
  agreement with it.
- Five of `VER-MOK-010`'s seven manual assessments remain outstanding, and four amendment corrections await the
  technical owner — two of the four written after the merge, which also forced two further rows that change no
  provision. All seven beyond-the-list amendments are in `amendment-approvals.md` §3. The renumbering changes none of
  that.

## The three captures re-taken on 2026-08-19, two of which discharge the notes above

Each was re-taken against the merged tree rather than edited into agreement with it, and each is a fresh run of the
retained tooling. In all three cases the tooling itself had to be changed first, which is recorded here because a
retained script that no longer runs reproduces nothing and a retained script that was changed is not the one that
produced the capture beside it.

**`observer/roster-frames.txt`, oracle 4.** `observer/frame-probe.rs` wrote each frame's tier down by calling
`Panes::tier` and `Tier::label`, which `WO-MOK-005` deleted; the probe therefore no longer compiled, and the analysis
script sized a tier column and narrated a tier table that rule 5 no longer has. The probe now takes its viewport set
from rule 5 as amended — the nine viewports the rule draws derived consequences for, plus `33x21` below the floor, which
is the same set `mokiterions-tui/tests/verification.rs` declares as its contract — and writes down, for the
below-floor frame, how many of its cells carry a character rather than a tier. `analysis/frames.py` lost the tier
column and gained the two counts the prose used to state by hand. The expectation it rebuilds is rule 4's and is
unchanged.

The measured result: **996 bar rows** rebuilt character for character across the **85 of 157 probed frames** that draw
a roster, **0 discrepancies**, and the `f` gauge at columns 36, 38–39 and 41–43 at **all eight** roster-drawing
viewports rather than the four the first capture reached. The earlier capture read 864 rows over its own smaller set.
Every figure moved upward, because the merged rule 5 presents the roster at more viewports than the tier table did;
nothing that was true of the earlier capture became false.

**`test-census.txt`.** The census is a difference between two trees, so which commit is on the before side decides what
it attributes to this work order. It was `60fda9f`, the branch point. Against the merged tree that commit would have
counted `master`'s own arrivals — five in `mokiterions-tui`'s internal tier, three in `tests/layout.rs`, two in
`tests/render.rs` — among this work order's additions. The before side is now `master`'s tip,
`7a2b502b908be03ad8e2de7c23ee3eaaf4ece048`, taken from a clean worktree at it, and `analysis/test-census.py` takes the
commit as an argument instead of naming one in its own text, so the recorded command says which comparison was made.

The measured result: **179 tests before, 200 after, 21 added, 0 removed**, and the 21 are this work order's twenty-one
and none of `master`'s. `master`'s ten now sit on the before side, where they belong: `mokiterions-tui`'s internal tier
and `tests/layout.rs` are `master`'s alone and read `+0`, and `tests/render.rs`, the one row the two work orders share,
reads `+2` — this work order's two, `master`'s two having been absorbed into the before side's ten.

**`static-checks.txt`.** This one was not on the list above, and it should have been: it quotes the suite's own pass
count, which the merged tree moves from 190 to 200, and it compares both dependency trees against a clean worktree at
the pre-change commit, which has to be the commit the branch is merged into for the comparison to mean what it says.
Re-captured, it reads **20 runners, 20 ok, 200 passed, 0 failed, 0 ignored, 0 filtered out**, `cargo fmt --check` with
zero diff lines, clippy with zero warnings over both crates re-linted, the engine's tree still one line, and the
observer's 111 lines identical to `7a2b502`'s line for line. The toolchain is the same one as before.

Its tooling had a defect the recapture exposed. `analysis/static-checks.py`'s `read` dropped the first line of every
capture file, taking it for the `### <command>` heading `capture-static.sh` writes. The two `-pre` tree files are not
written by that script — they are redirections of `cargo tree` run in another checkout, by the commands the script's own
header records — and those commands emit no heading, so the first line dropped was the root of the tree. The engine's
`-pre` graph came out at zero lines against the candidate's one, and the run reported **FAIL: the dependency graph
changed** on a graph that had not changed. The heading is now read as optional, which is what makes the recorded command
reproduce the recorded figure; the first capture's `-pre` files evidently carried a heading the recorded command does not
produce, so the figure was right and the route to it was not.

One prose figure fell rather than rose. `completion-summary.md` §11 said three of the harness inspector's warnings were
new and caused by this change. Measured against `master`'s tip with `--json` on both sides and compared finding by
finding, **two** are: `ARCH-MOK-001` predating `SPEC-MOK-001` and `SPEC-MOK-002`. The third, `ARCH-MOK-002` predating
`SPEC-MOK-003`, is present at `7a2b502` without this branch, because `master` amended `SPEC-MOK-003` itself in
`WO-MOK-005` and in its own `WO-MOK-007`. The inspector's totals move with the merged tree too, from 10 warnings and 6
informational to 12 and 8, and the validator's artifact count from 76 to 80 over 248 relations.

**`interface-and-purity.txt` did not need re-deriving, which was checked rather than assumed.** Its before side is a
worktree at `60fda9f` and it reads only `mokiterions-core/src`. `git diff --stat 60fda9f 7a2b502 -- mokiterions-core/`
is empty: `master` advanced the observer and the governance tree and did not touch the engine by one byte. The engine's
public interface, its arithmetic prohibitions and `fear`'s one writer are therefore the same on both candidate
baselines, and the capture's before side is the merged tree's before side too.

Three of the nineteen byte-identical captures listed above have consequently been re-taken. **Sixteen remain**, and the
count of lines in this packet naming this work order by its former identifier falls from twenty-five to twenty-one,
counted rather than deduced. All three re-taken captures name it `WO-MOK-010`.
