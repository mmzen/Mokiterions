"""VER-MOK-016: rule 18's summary per seed under each source, and REQ-MOK-060's composition ratio.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/composition.py \
        <90-cell-capture-dir> <30-cell-social-capture-dir>

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

WHAT THIS READER IS FOR
-----------------------
`VER-MOK-016` retains "rule 18's final summary per seed under each of the four sources, and the
composition ratio computed per territory per class, at both commits". Two clauses, and the second
is the one `REQ-MOK-060` is written against: "no calorie class holding more than half of any
territory's standing resources" at tick 1,000, at the default density, under `reference`,
`individual` or `social`, on every declared verification seed.

  * **The summary is already in the released bytes.** `emit_summary` prints `food_a_low` through
    `food_b_high` beside the survivor and occupancy counts, which is what `REQ-MOK-060` means by
    "measurable from the run's own output": no new event and no new instrumentation. The
    pre-change side is retained too, in `baseline/summary.txt`, so both commits are readable.
  * **The ratio is not in the bytes.** Six counts per territory are, and the requirement is
    stated as a share of a territory's standing resources, so the ratio is computed here.
  * **`REQ-MOK-060` is unimplemented at this candidate**, under the product owner's approved
    deferral of 2026-08-20, and the descope of 2026-08-21 carries it to `WO-MOK-017`. So this
    reader measures the *uncorrected* composition. That is deliberate and is the point: the
    decision `WO-MOK-017` inherits -- the ceiling's value, whether a per-class floor should
    accompany it -- is to be taken on a measured curve, and this is the curve.

WHY THE MEASUREMENT IS NOT A TRANSCRIPTION
------------------------------------------
A reader that copied twelve numbers off the last line of each cell would report whatever the
engine reported and could not fail. So the summary is **checked against the stream that produced
it**, cell by cell, and the check is available because every standing resource is accounted for
in the released records:

  * a resource arrives in `food_initialized` with its class, position and territory, or in
    `food_regenerated` with its class and position under the territory that added it;
  * it leaves in `food_consumed`, which names it and its class;
  * nothing else creates or destroys one.

So the standing composition at the end of a run is `initialized + regenerated - consumed`, per
territory per class, and it must equal the six counts rule 18 printed. `food_counts` derives each
resource's territory from its **position** rather than from a stored field, so this reader does
the same and checks the derived territory against the printed one, which is a second reading of
the same fact and would catch a resource added outside the territory that drew it.

WHAT VALIDATES THE READING
--------------------------
  * every candidate cell is digested against its row in `post/post-manifest.txt` or
    `post/social-manifest.txt`, so the bytes read here are the bytes the packet retains;
  * the reconstruction above, over all 120 candidate cells;
  * the standing set is computed twice by different means -- an arithmetic count and a set of
    surviving identifiers -- and the two must agree;
  * the trace setting must move nothing: the two cells of a `(source, density, seed)` triple
    differ only in `--trace-actions`, so their summary lines must be identical;
  * the pre-change side is compared against the candidate on all 45 shared triples, which is
    oracle 1's clause read on one line rather than on a whole stream;
  * the initial composition is measured rather than assumed to be the "balanced initial third"
    `SPEC-MOK-001` rule 5 describes, and the per-territory total is checked against the
    resource count the density resolves to.

WHAT IS DELIBERATELY NOT DONE
-----------------------------
No temporary build and no new test, for the reasons `post/branches.md` §1 and
`post/test-census-reconciliation.md` state: a figure from a build that no longer exists is not
re-derivable, and an added test would move a fixed census.

**The ceiling breach is not a check.** `REQ-MOK-060` is unimplemented by an approved deferral, so
a reader that failed on the breach would be reporting the deferral as a defect. The breach is
measured, printed per cell, and left where it belongs -- with the product owner, in
`WO-MOK-017`'s scope. Every other property here is a check, and this reader exits non-zero if any
of them fires.

THE PRE-CHANGE SIDE NEEDS NO CAPTURE DIRECTORY
----------------------------------------------
It is read entirely from bytes this packet retains: `baseline/summary.txt` for the final
composition of all 90 cells, and `baseline/init/<cell>.txt` for the initial composition of the
same 90. A verifier reproducing this file needs the two candidate captures and nothing else.
"""

import hashlib
import re
import sys
import textwrap
from collections import Counter
from pathlib import Path

# LF regardless of platform, so a plain redirect reproduces the retained file byte for byte:
# `.gitattributes` pins this evidence tree `-text`, so a CRLF here would be committed as one.
sys.stdout.reconfigure(encoding="utf-8", newline="\n")

HERE = Path(__file__).resolve().parent
PACKET = HERE.parent

WORLD_SIZE = 128
TERRITORY_HEIGHT = 64
CELLS_PER_TERRITORY = WORLD_SIZE * TERRITORY_HEIGHT

CLASSES = ("low", "medium", "high")
TERRITORIES = ("A", "B")
SEEDS = (0, 1, 42, 123, 777)
DENSITIES = (("0.15", 15), ("0.75", 75), ("1.50", 150))
SOURCES = ("baseline", "reference", "individual", "social")

# `mokiterions-core/src/simulation.rs`: the default density, 0.75%, which resolves to 61
# resources per territory. `REQ-MOK-060`'s trigger is the default density and no other.
DEFAULT_DENSITY = "0.75"
POPULATION = 12

# `REQ-MOK-060`, ratified at one half by the product owner against 60% and 40%.
CEILING = 0.5

# The three sources the requirement names. `baseline` is outside it, deliberately: it is the
# source this initiative holds byte-identical, so it cannot be given a new obligation.
BOUND_SOURCES = ("reference", "individual", "social")

# `REQ-MOK-014` binds `reference` and `REQ-MOK-034` binds the trait-aware source, both at eight
# survivors of twelve at the default density; `REQ-MOK-058`'s floor was ratified at five on the
# measured curve on 2026-08-20. Transcribed for the comparison in section 10, decided nowhere.
FLOORS = {"reference": 8, "individual": 8, "social": 5}

# `SPEC-MOK-001` rule 5's recorded measurement, which this candidate is expected to reproduce:
# "high class at 45 of 61 resources in a territory by tick 1,000, against a balanced initial
# third". Section 7 is where it is looked for rather than assumed.
RULE_5_HIGH = 45
RULE_5_TOTAL = 61

# Column headings for the class triples, built at the widths the rows are printed at so the two
# cannot drift apart: a heading nudged by hand is a heading that stops naming its column.
FOOD_HEAD = " ".join(f"{label:>3}" for label in ("low", "med", "hgh"))
PCT_HEAD = "  ".join(f"{label:>6}" for label in CLASSES)

EVENT = re.compile(r"^tick=(\d+) subject=(\S+) event=(\w+) result=(.*)$")
FOOD_INIT = re.compile(r"^class:(\w+),position:(\d+):(\d+),territory:(\w)$")
FOOD_REGEN = re.compile(r"^food:(F\d+),class:(\w+),position:(\d+):(\d+)$")
FOOD_EATEN = re.compile(r"^food:(F\d+),class:(\w+),")

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def rule(title):
    print()
    print(title)
    print("-" * len(title))


def para(text, bullet=False):
    """A paragraph with figures in it, wrapped here rather than by hand.

    The prose in this file is hand-wrapped, but a sentence whose figures are interpolated cannot
    be: a count that gains a digit would leave the line ragged, and a reader who re-runs this on
    another candidate should get a report that still reads as prose.
    """
    print(
        textwrap.fill(
            " ".join(text.split()),
            width=95,
            initial_indent="- " if bullet else "",
            subsequent_indent="  " if bullet else "",
        )
    )


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest_rows(path):
    rows = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        fields = line.split()
        if len(fields) == 5 and not line.startswith("#") and fields[4].isdigit():
            rows[fields[0]] = fields[1]
    return rows


def territory_of(position):
    """`Position::territory`, transcribed. The engine's own summary derives it this way."""
    return "A" if position[1] < TERRITORY_HEIGHT else "B"


def resources_per_territory(hundredths):
    return hundredths * CELLS_PER_TERRITORY // 10_000


def parse_summary(line):
    """Rule 18's final line as a field map. Every value in it is an integer but `reason`."""
    fields = {}
    for token in line.split()[1:]:
        key, _, value = token.partition("=")
        fields[key] = int(value) if value.isdigit() else value
    return fields


def standing_of(fields):
    """The six food counts of a summary line, keyed the way this reader keys a composition."""
    return {
        (territory, klass): fields[f"food_{territory.lower()}_{klass}"]
        for territory in TERRITORIES
        for klass in CLASSES
    }


def read_cell(path):
    """One capture cell as an ordered event list and its one summary line.

    An unparsed line is a failure rather than a skip, and so is a cell whose summary is missing,
    doubled, or not the last line it prints: every figure below is read off that line, so its
    position in the stream is part of what is being measured.
    """
    events = []
    summaries = []
    last = None
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line:
            continue
        last = number
        if line.startswith("summary "):
            summaries.append((number, line))
            continue
        match = EVENT.match(line)
        if not match:
            failures.append(f"{path.name}:{number} is neither an event nor a summary: {line[:60]}")
            continue
        events.append((int(match.group(1)), match.group(2), match.group(3), match.group(4)))
    if not check(len(summaries) == 1, f"{path.name} prints {len(summaries)} summary lines"):
        return events, None
    number, line = summaries[0]
    check(number == last, f"{path.name}'s summary is at line {number} and not last, at {last}")
    return events, line


def compose(events, name):
    """The standing composition per territory per class, reconstructed from the stream.

    `initialized + regenerated - consumed`, which is exhaustive because nothing else in the
    released records creates or destroys a resource. Two independent paths are kept: a running
    count per `(territory, class)`, and the set of identifiers never consumed. They must agree,
    which is what catches a resource counted twice or a consumption of something never born.
    """
    born = {}
    initial = Counter()
    added = Counter()
    eaten = Counter()
    alive = set()
    skipped = 0
    for tick, subject, kind, result in events:
        if kind == "food_initialized":
            match = FOOD_INIT.match(result)
            if not check(match is not None, f"{name}: unparsed food_initialized {result[:50]}"):
                continue
            klass, x, y, printed = match.group(1), int(match.group(2)), int(match.group(3)), match.group(4)
            territory = territory_of((x, y))
            check(
                territory == printed,
                f"{name}: {subject} is printed in territory {printed} and lies in {territory}",
            )
            born[subject] = (territory, klass)
            initial[(territory, klass)] += 1
            alive.add(subject)
        elif kind == "food_regenerated":
            match = FOOD_REGEN.match(result)
            if not check(match is not None, f"{name}: unparsed food_regenerated {result[:50]}"):
                continue
            food, klass = match.group(1), match.group(2)
            territory = territory_of((int(match.group(3)), int(match.group(4))))
            check(
                territory == subject,
                f"{name}: territory {subject} added {food} into territory {territory}",
            )
            check(food not in born, f"{name}: {food} is added twice")
            born[food] = (territory, klass)
            added[(territory, klass)] += 1
            alive.add(food)
        elif kind == "food_consumed":
            match = FOOD_EATEN.match(result)
            if not check(match is not None, f"{name}: unparsed food_consumed {result[:50]}"):
                continue
            food, klass = match.group(1), match.group(2)
            if not check(food in born, f"{name}: {food} is consumed and was never placed"):
                continue
            territory, born_class = born[food]
            check(
                klass == born_class,
                f"{name}: {food} was placed {born_class} and consumed {klass}",
            )
            check(food in alive, f"{name}: {food} is consumed twice")
            alive.discard(food)
            eaten[(territory, klass)] += 1
        elif kind == "food_regeneration_skipped":
            skipped += 1

    arithmetic = {
        key: initial[key] + added[key] - eaten[key]
        for key in ((t, c) for t in TERRITORIES for c in CLASSES)
    }
    surviving = Counter(born[food] for food in alive)
    check(
        arithmetic == {key: surviving[key] for key in arithmetic},
        f"{name}: the two standing counts disagree",
    )
    return {
        "initial": {key: initial[key] for key in arithmetic},
        "added": {key: added[key] for key in arithmetic},
        "eaten": {key: eaten[key] for key in arithmetic},
        "final": arithmetic,
        "skipped": skipped,
    }


def shares(standing, territory):
    """A territory's class shares, and the class holding the largest of them."""
    counts = {klass: standing[(territory, klass)] for klass in CLASSES}
    total = sum(counts.values())
    if total == 0:
        return counts, total, None, 0.0
    widest = max(CLASSES, key=lambda klass: counts[klass])
    return counts, total, widest, counts[widest] / total


def pct(part, whole):
    return "    -" if whole == 0 else f"{100 * part / whole:5.1f}%"


if len(sys.argv) != 3:
    sys.exit(__doc__)
MATRIX = Path(sys.argv[1])
SOCIAL = Path(sys.argv[2])


def cell_name(seed, source, density, trace):
    return f"seed{seed}-{source}-d{density}-trace{trace}"


def cell_path(seed, source, density, trace):
    root = SOCIAL if source == "social" else MATRIX
    return root / f"{cell_name(seed, source, density, trace)}.txt"


TITLE = "VER-MOK-016: rule 18's summary per source, and REQ-MOK-060's composition per territory"
print(TITLE)
print("=" * len(TITLE))
print()
print("Work order   WO-MOK-016 (Phase 3.1, encounters)")
print('Retains      "rule 18\'s final summary per seed under each of the four sources, and the')
print('             composition ratio computed per territory per class, at both commits"')
print("Authority    REQ-MOK-060, whose ceiling is one half per territory at the default density")
print("             under reference, individual and social; VER-MOK-016 oracle 4's clause that")
print("             the composition is read from the run's own final summary")
print("Candidate    7c4aef3967406c05d80da963695898b77f5329e9   (the 90-cell matrix)")
print("             59d61b915630fd55f04bcdbb346aa22cdbfdfff6   (the 30 social cells)")
print("Pre-change   39662d13abd08e3410648d1c59ad38384f8ad2d2   (baseline/summary.txt, init/)")
print("Reader       analysis/composition.py")
print("Date         2026-08-21")
print()
print("REQ-MOK-060 is unimplemented at this candidate under the approved deferral of 2026-08-20")
print("and was carried to WO-MOK-017 by the descope of 2026-08-21, so what follows measures the")
print("uncorrected composition. That is the point rather than a gap: the ceiling's value and the")
print("question of a per-class floor beside it are to be decided on a measured curve, and this is")
print("the curve. Sections 2 and 3 establish that the summary is the engine's own accounting and")
print("check it against the stream; sections 4 and 5 are the retention clause's two halves;")
print("section 6 evaluates the ceiling exactly as REQ-MOK-060 states it; sections 7 to 10 are the")
print("mechanism, the excluded source, the reason the ceiling is per territory, and the survivor")
print("figures the same line carries; section 11 says what none of it settles.")

# ---------------------------------------------------------------------------------------------
rule("1. The cells this reads, and the digests that say which")

manifests = {
    "matrix": manifest_rows(PACKET / "post" / "post-manifest.txt"),
    "social": manifest_rows(PACKET / "post" / "social-manifest.txt"),
}

triples = [
    (source, density, hundredths, seed)
    for source in SOURCES
    for density, hundredths in DENSITIES
    for seed in SEEDS
]

missing = [
    (s, d, t) for s, d, _, t in triples for trace in ("on", "off")
    if not cell_path(t, s, d, trace).exists()
]
check(not missing, f"{len(missing)} cells absent from the capture directories")

matched = 0
cells = {}
for source, density, hundredths, seed in triples:
    for trace in ("on", "off"):
        name = cell_name(seed, source, density, trace)
        path = cell_path(seed, source, density, trace)
        if not path.exists():
            continue
        rows = manifests["social" if source == "social" else "matrix"]
        expected = rows.get(name)
        if check(expected is not None, f"{name} has no manifest row"):
            if check(digest(path) == expected, f"{name} does not match its manifest digest"):
                matched += 1
        events, summary = read_cell(path)
        cells[name] = {
            "summary": parse_summary(summary) if summary else None,
            "line": summary,
            "stream": compose(events, name),
        }

pre_lines = {}
for line in (PACKET / "baseline" / "summary.txt").read_text(encoding="utf-8").splitlines():
    if not line or line.startswith("#"):
        continue
    name, _, rest = line.partition(" ")
    pre_lines[name] = rest.strip()

pre_initial = {}
for path in sorted((PACKET / "baseline" / "init").glob("*.txt")):
    counts = Counter()
    for line in path.read_text(encoding="utf-8").splitlines():
        match = EVENT.match(line)
        if not match or match.group(3) != "food_initialized":
            continue
        placed = FOOD_INIT.match(match.group(4))
        if check(placed is not None, f"{path.name}: unparsed food_initialized"):
            counts[(placed.group(4), placed.group(1))] += 1
    pre_initial[path.stem] = counts

print(f"    candidate cells read          {len(cells)}")
print(f"    manifest rows they match      {matched}")
print(f"    90-cell three-source matrix   post/post-manifest.txt, {len(manifests['matrix'])} rows")
print(f"    30 social cells               post/social-manifest.txt, {len(manifests['social'])} rows")
print(f"    pre-change summary lines      baseline/summary.txt, {len(pre_lines)} rows")
print(f"    pre-change initial blocks     baseline/init/, {len(pre_initial)} files")
print()
para(
    """Every candidate cell is digested and compared against its manifest row, so the bytes
    measured below are the bytes the packet retains. The pre-change side needs no capture
    directory at all: its final composition is `baseline/summary.txt`, retained whole, and its
    initial composition is in `baseline/init/`, which holds every line up to and including the
    last `tick=0` line of all ninety cells. The two capture directories are arguments and are
    deliberately not printed, because a path is this machine's and a digest is not."""
)
check(len(pre_lines) == 90, f"baseline/summary.txt holds {len(pre_lines)} rows and not 90")
check(len(pre_initial) == 90, f"baseline/init/ holds {len(pre_initial)} files and not 90")

# ---------------------------------------------------------------------------------------------
rule("2. Rule 18's summary line, and the four things it has to agree with")

print("    summary reason=<reason> ticks=<n> survivors=<n> deaths=<n> territory_a=<n>")
print("            territory_b=<n> food_a_low=<n> food_a_medium=<n> food_a_high=<n>")
print("            food_b_low=<n> food_b_medium=<n> food_b_high=<n>")
print()
para(
    """`emit_summary` prints twelve fields and `food_counts` computes six of them by walking the
    standing resources and bucketing each by `food.position.territory()` and by class. That is
    what `REQ-MOK-060` means when it says the composition is measurable from the run's own
    output. What the line does not carry is a ratio: the requirement is stated as a share of a
    territory's standing resources, and a share of six counts is computed here, in section 5."""
)
print()

closed = 0
for name, cell in sorted(cells.items()):
    fields = cell["summary"]
    if fields is None:
        continue
    closed += 1
    check(
        fields["survivors"] + fields["deaths"] == POPULATION,
        f"{name}: survivors {fields['survivors']} and deaths {fields['deaths']} are not {POPULATION}",
    )
    check(
        fields["territory_a"] + fields["territory_b"] == fields["survivors"],
        f"{name}: the two territory counts do not sum to survivors",
    )
    check(
        fields["reason"] in ("extinction", "tick_limit"),
        f"{name}: unrecognized termination reason {fields['reason']}",
    )
    check(
        (fields["reason"] == "extinction") == (fields["survivors"] == 0),
        f"{name}: reason {fields['reason']} against {fields['survivors']} survivors",
    )

print(f"    cells with exactly one summary, printed last   {closed} of {len(cells)}")
print(f"    cells where survivors + deaths == {POPULATION}            {closed} of {len(cells)}")
print("    cells where the two territory counts sum to survivors, and the termination reason")
print(f"    agrees with the survivor count                {closed} of {len(cells)}")

# ---------------------------------------------------------------------------------------------
rule("3. The summary checked against the stream that produced it")

para(
    """A reader that copied twelve numbers off the last line of a cell would report whatever the
    engine reported and could not fail. So the six food counts are reconstructed from the
    records instead. A resource arrives in `food_initialized` with class, position and
    territory, or in `food_regenerated` with class and position under the territory that added
    it; it leaves in `food_consumed`, which names it and its class; nothing else creates or
    destroys one. The standing composition is therefore `initialized + regenerated - consumed`
    per territory per class, and it must equal what rule 18 printed."""
)
print()

agreeing = 0
for name, cell in sorted(cells.items()):
    if cell["summary"] is None:
        continue
    reconstructed = cell["stream"]["final"]
    printed = standing_of(cell["summary"])
    if check(reconstructed == printed, f"{name}: the reconstruction disagrees with the summary"):
        agreeing += 1

placements = sum(
    sum(cell["stream"]["initial"].values()) + sum(cell["stream"]["added"].values())
    for cell in cells.values()
)
consumptions = sum(sum(cell["stream"]["eaten"].values()) for cell in cells.values())
print(f"    cells where the reconstruction reproduces all six counts   {agreeing} of {len(cells)}")
print("    resources placed, each with its territory derived from its position and checked")
print(f"    against the printed one                                    {placements:,}")
print("    consumptions, each checked against the class the resource was placed with, and")
print(f"    against the resource still standing                        {consumptions:,}")
print()
para(
    """`food_counts` derives a resource's territory from its position rather than from a stored
    field, so this reader does the same and then checks the derived territory against the one
    the record printed -- `territory:A` on a placement, the emitting territory on a
    regeneration. That is the check that would catch a resource added outside the territory
    that drew for it, which `choose_free_coordinate`'s bounds are what prevent. The standing
    set is also counted two ways, arithmetically and as the set of identifiers never consumed,
    and the two must agree."""
)

# ---------------------------------------------------------------------------------------------
rule("4. Rule 18's final summary per seed under each of the four sources, at both commits")

print(
    f"    {'source':<11} {'dens':>4} {'seed':>5}  {'reason':<11}{'ticks':>5} {'surv':>5}"
    f" {'dead':>5} {'A':>2} {'B':>2}  territory A   territory B"
)
print(" " * 64 + FOOD_HEAD + "   " + FOOD_HEAD)
parity = 0
compared = 0
identical = 0
for source, density, hundredths, seed in triples:
    off = cells.get(cell_name(seed, source, density, "off"))
    on = cells.get(cell_name(seed, source, density, "on"))
    if off is None or off["summary"] is None:
        continue
    if on is not None and on["line"] is not None:
        if check(
            on["line"] == off["line"],
            f"{source} d{density} seed {seed}: the trace setting moved the summary",
        ):
            parity += 1
    f = off["summary"]
    a = " ".join(f"{f[f'food_a_{k}']:>3}" for k in CLASSES)
    b = " ".join(f"{f[f'food_b_{k}']:>3}" for k in CLASSES)
    print(
        f"    {source:<11} {density:>4} {seed:>5}  {f['reason']:<11}"
        f"{f['ticks']:>5} {f['survivors']:>5} {f['deaths']:>5} "
        f"{f['territory_a']:>2} {f['territory_b']:>2}  {a}   {b}"
    )
    pre = pre_lines.get(cell_name(seed, source, density, "off"))
    if pre is not None:
        compared += 1
        if pre == off["line"]:
            identical += 1

print()
print(f"    triples where the traced and untraced cells print the same summary   {parity} of 60")
print(f"    triples the pre-change capture also holds                            {compared} of 60")
print(f"    of those, triples whose summary is identical at both commits        {identical} of {compared}")
print()
para(
    f"""**The summary does not move between the two commits, on any of the {compared} triples the
    pre-change capture holds.** That is not a null result to be passed over: it is
    `REQ-MOK-060` being unimplemented, read on one line. The requirement's permitted mechanism
    is the sources' waste condition, no source's waste condition was changed, and so no run's
    standing composition changed either. `post/byte-identity.txt` measures the same thing over
    whole streams and at greater strength; this is the clause of the retention item that asks
    for it summary by summary."""
)
para(
    """The 30 `social` cells have no pre-change side and cannot have one: the source does not
    exist at the baseline commit, so a fourth policy value would have failed 30 runs with a
    configuration error. `capture.sh` deliberately takes three sources and `capture-social.sh`
    is the separate capture. For `social`, "at both commits" is answered by one commit and the
    reason, which is what this file states rather than leaving a column blank."""
)
check(compared == 45, f"the pre-change capture holds {compared} of the 45 shared triples")
check(identical == compared, f"{compared - identical} shared triples moved between the commits")

# ---------------------------------------------------------------------------------------------
rule("5. The composition ratio per territory per class, at both commits")

print(" " * 41 + "territory A" + " " * 25 + "territory B")
print(
    f"    {'source':<11} {'dens':>4} {'seed':>5}  {'standing':>11}  {PCT_HEAD}"
    f"  {'standing':>11}  {PCT_HEAD}"
)
ratios = {}
for source, density, hundredths, seed in triples:
    cell = cells.get(cell_name(seed, source, density, "off"))
    if cell is None or cell["summary"] is None:
        continue
    standing = standing_of(cell["summary"])
    row = {}
    for territory in TERRITORIES:
        counts, total, widest, share = shares(standing, territory)
        row[territory] = (counts, total, widest, share)
    ratios[(source, density, seed)] = row
    a_counts, a_total, _, _ = row["A"]
    b_counts, b_total, _, _ = row["B"]
    print(
        f"    {source:<11} {density:>4} {seed:>5}  {a_total:>11}  "
        + "  ".join(pct(a_counts[k], a_total) for k in CLASSES)
        + f"  {b_total:>11}  "
        + "  ".join(pct(b_counts[k], b_total) for k in CLASSES)
    )

print()
para(
    """The ratio is per territory because the two territories regenerate independently under
    rules 14 to 16, which is `REQ-MOK-060`'s stated reason for not averaging them. Every row
    above is the candidate's, and by section 4 it is the pre-change commit's too wherever the
    pre-change capture holds the triple. A `-` is a territory holding nothing, where a share is
    undefined rather than zero."""
)

# ---------------------------------------------------------------------------------------------
rule("6. REQ-MOK-060 evaluated exactly as it is stated")

para(
    """The requirement: "WHEN a simulation runs to 1,000 ticks at the default resource density
    under the reference, trait-aware or social decision source, THE SYSTEM SHALL leave no
    calorie class holding more than half of any territory's standing resources, on every
    declared verification seed." So the population is the default density only, the three named
    sources only, and the five declared seeds -- and the trigger requires the run to reach
    1,000 ticks, which is checked below rather than assumed."""
)
print()
print(
    f"    {'source':<11} {'seed':>5}  {'reached':>7}  {'territory':>9}  {'standing':>8}"
    f"  {'widest class':>12}  {'share':>6}  against one half"
)
bound = []
breaches = []
runs = []
detail = {}
for source in BOUND_SOURCES:
    for seed in SEEDS:
        cell = cells.get(cell_name(seed, source, DEFAULT_DENSITY, "off"))
        if cell is None or cell["summary"] is None:
            continue
        fields = cell["summary"]
        reached = fields["reason"] == "tick_limit" and fields["ticks"] == 1_000
        check(reached, f"{source} d{DEFAULT_DENSITY} seed {seed} did not reach tick 1,000")
        standing = standing_of(fields)
        breaching = 0
        for territory in TERRITORIES:
            counts, total, widest, share = shares(standing, territory)
            verdict = "met" if share <= CEILING else "BREACHED"
            bound.append((source, seed, territory, widest, share, verdict))
            detail[(source, seed, territory)] = (counts, total)
            if verdict == "BREACHED":
                breaches.append((source, seed, territory, widest, share))
                breaching += 1
            print(
                f"    {source:<11} {seed:>5}  {'yes' if reached else 'no':>7}  {territory:>9}  "
                f"{total:>8}  {widest:>12}  {100 * share:5.1f}%  {verdict}"
            )
        runs.append((source, seed, breaching))

met = [row for row in bound if row[5] == "met"]
not_high = [row for row in bound if row[3] != "high"]
breach_high = [row for row in breaches if row[3] == "high"]
breaching_runs = [row for row in runs if row[2]]
clean_runs = [row for row in runs if not row[2]]
widest_share = max(share for *_, share, _ in bound) if bound else 0.0
narrowest = min(share for *_, share, _ in bound) if bound else 0.0
print()
print(f"    territory evaluations, 3 sources x 5 seeds x 2 territories   {len(bound)}")
print(f"    evaluations breaching the ceiling of one half                 {len(breaches)}")
print(f"    of those, evaluations where the class over the line is high   {len(breach_high)}")
print(f"    evaluations meeting the ceiling                               {len(met)}")
print(f"    runs carrying at least one breaching territory                {len(breaching_runs)} of {len(runs)}")
print(f"    the widest class's share, at its lowest and highest           {100 * narrowest:.1f}% and {100 * widest_share:.1f}%")
print()
para(
    f"""**{len(breaches)} of the {len(bound)} territory evaluations breach the ceiling, and the
    requirement is unmet under all three sources.** `REQ-MOK-060` binds "on every declared
    verification seed", so one breaching seed is enough to leave it unmet, and
    {len(breaching_runs)} of the {len(runs)} runs carry at least one breaching territory. In
    {'every one' if len(breach_high) == len(breaches) else f'{len(breach_high)}'} of the
    breaches the class over the line is `high`, and the widest class's share runs from
    {100 * narrowest:.1f}% to {100 * widest_share:.1f}%."""
)
print()
para(
    f"""The {len(met)} evaluations that meet it are worth naming rather than rounding away,
    because they are what tells the owner how far the correction has to reach:"""
)
for source, seed, territory, widest, share, _ in met:
    print(f"      {source:<11} seed {seed:<4} territory {territory}   {widest} at {100 * share:.1f}%")
print()
clean = clean_runs[0] if clean_runs else None
if clean:
    both = [row for row in bound if (row[0], row[1]) == (clean[0], clean[1])]
    para(
        f"""`{clean[0]}` at seed {clean[1]} is the one run of the {len(runs)} that meets the
        ceiling in both territories, at
        {' and '.join(f'{100 * row[4]:.1f}%' for row in both)} -- close enough to one half that it
        is not evidence of a different regime, and its
        {' and '.join(str(detail[(row[0], row[1], row[2])][1]) for row in both)} standing
        resources say the same thing another way."""
    )
if not_high:
    odd = not_high[0]
    counts, total = detail[(odd[0], odd[1], odd[2])]
    ranked = sorted(CLASSES, key=lambda klass: -counts[klass])
    print()
    para(
        f"""The remaining one is `{odd[0]}` at seed {odd[1]} in territory {odd[2]}, and it is the
        more interesting row of the {len(met)}: the only evaluation in the whole default-density
        set where the widest class is not `high`. Its `{ranked[0]}` class leads at
        {100 * counts[ranked[0]] / total:.1f}%, with `{ranked[1]}` at
        {100 * counts[ranked[1]] / total:.1f}% and `{ranked[2]}` last at
        {100 * counts[ranked[2]] / total:.1f}% -- a territory under the ceiling and still nowhere
        near the balanced third it started from. It bears on the second half of manual assessment
        5, whether a per-class *floor* is wanted beside the ceiling: a world can sit under a
        ceiling of one half and still be as lopsided as this one."""
    )
print()
bound_cells = [
    cells[cell_name(seed, source, DEFAULT_DENSITY, "off")]
    for source in BOUND_SOURCES
    for seed in SEEDS
    if cell_name(seed, source, DEFAULT_DENSITY, "off") in cells
]
opening = sum(cell["stream"]["initial"][(t, "high")] for cell in bound_cells for t in TERRITORIES)
opening_all = sum(sum(cell["stream"]["initial"].values()) for cell in bound_cells)
closing = sum(cell["stream"]["final"][(t, "high")] for cell in bound_cells for t in TERRITORIES)
closing_all = sum(sum(cell["stream"]["final"].values()) for cell in bound_cells)
opening_share = opening / opening_all if opening_all else 0.0
closing_share = closing / closing_all if closing_all else 0.0
headroom = CEILING - opening_share
overshoot = closing_share - opening_share
print(f"    the high class's share of these {len(bound_cells)} runs at tick 0        {100 * opening_share:5.1f}%")
print(f"    the same share at tick 1,000                             {100 * closing_share:5.1f}%")
print(f"    the headroom a ceiling of one half leaves above tick 0    {100 * headroom:5.1f} points")
print(f"    the drift these runs actually carry                      {100 * overshoot:5.1f} points")
print()
para(
    f"""**Those last two figures are what manual assessment 5 is against.** The assessment records
    why one half was chosen over 60% and 40%: for "the `17` points of headroom above the balanced
    initial third that it leaves". Measured, the initial third is {100 * opening_share:.1f}% and
    the ceiling therefore allows {100 * headroom:.1f} points of drift -- the decision's figure,
    reproduced from the bytes. What these runs carry is {100 * overshoot:.1f} points, about
    {overshoot / headroom:.1f} times that. So the reasoning the value was chosen on holds, and
    the curve says the correction has to remove roughly half the drift rather than shave the top
    off it."""
)
print()
para(
    """This reader exits `0` with the breach on the page, and deliberately. `REQ-MOK-060` is
    unimplemented under the product owner's approved deferral of 2026-08-20 and was carried to
    `WO-MOK-017` by the descope of 2026-08-21, so a check that failed here would report an
    approved deferral as a defect. What the figures are for is the decision `WO-MOK-017`
    inherits, and they are also what makes the deferral not a blind one: the ceiling was
    ratified at one half against 60% and 40% before any of this was measured, and a correction
    that has to move a majority class from the upper sixties into the forties is a different
    piece of work from one that trims a few percent."""
)
check(len(bound) == 30, f"{len(bound)} territory evaluations and not 30")
check(len(runs) == 15, f"{len(runs)} default-density runs under the three bound sources and not 15")

print()
para(
    """The same evaluation at the pre-change commit, read from `baseline/summary.txt` rather than
    inferred from section 4's identity, because `VER-MOK-016`'s coverage row asks for the
    pre-change state as the contrast in its own right:"""
)
print()
print(
    f"    {'source':<11} {'seed':>5}  {'territory':>9}  {'standing':>8}"
    f"  {'widest class':>12}  {'share':>6}  against one half"
)
pre_bound = []
for source in ("reference", "individual"):
    for seed in SEEDS:
        line = pre_lines.get(cell_name(seed, source, DEFAULT_DENSITY, "off"))
        if line is None:
            continue
        standing = standing_of(parse_summary(line))
        for territory in TERRITORIES:
            counts, total, widest, share = shares(standing, territory)
            verdict = "met" if share <= CEILING else "BREACHED"
            pre_bound.append((source, seed, territory, widest, share, verdict))
            print(
                f"    {source:<11} {seed:>5}  {territory:>9}  "
                f"{total:>8}  {widest:>12}  {100 * share:5.1f}%  {verdict}"
            )
pre_breaches = [row for row in pre_bound if row[5] == "BREACHED"]
candidate_subset = [row for row in bound if row[0] != "social"]
print()
print(f"    pre-change territory evaluations, 2 sources x 5 seeds x 2 territories   {len(pre_bound)}")
print(f"    of those, evaluations breaching the ceiling                             {len(pre_breaches)}")
print(f"    evaluations whose verdict is the same at both commits                   "
      f"{sum(1 for a, b in zip(pre_bound, candidate_subset) if a == b)} of {len(pre_bound)}")
print()
para(
    f"""`VER-MOK-016` asks for this as "the pre-change state is reproduced as the contrast", with
    the expectation that it "puts high class above half in a territory, which is the recorded 45
    of 61 this requirement ends". It does, in {len(pre_breaches)} of the {len(pre_bound)}
    evaluations, and section 7 finds the 45 of 61 itself. What the coverage row anticipated,
    though, was a contrast: a pre-change commit above the ceiling against a candidate below it.
    Under the descope there is no such contrast, because the candidate is the pre-change state
    on this measurement -- every verdict is identical. The row is answered on its pre-change
    half and its candidate half is `WO-MOK-017`'s, which is recorded here rather than presented
    as a contrast that was not measured."""
)
check(
    pre_bound == candidate_subset,
    f"{sum(1 for a, b in zip(pre_bound, candidate_subset) if a != b)} evaluations differ between the commits",
)

# ---------------------------------------------------------------------------------------------
rule("7. The mechanism, measured: a balanced third at tick 0 and what is left at the end")

para(
    """`REQ-MOK-060`'s rationale states the mechanism: a high-class resource restores `50`
    satiety, so the non-waste condition makes it eatable only at satiety of at most `50`; high
    class is passed over more often than low or medium; what is passed over stays standing while
    regeneration keeps adding classes uniformly. The initial composition is a balanced third,
    and that is measured here rather than taken from the rationale."""
)
print()
print(f"    {'dens':>4} {'per territory':>14}  {'territory A':<11}  {'territory B':<11}  balanced")
print(f"    {'':>4} {'':>14}  {FOOD_HEAD}  {FOOD_HEAD}  within one")
for density, hundredths in DENSITIES:
    expected = resources_per_territory(hundredths)
    sample = cells[cell_name(SEEDS[0], "reference", density, "off")]["stream"]["initial"]
    a = [sample[("A", k)] for k in CLASSES]
    b = [sample[("B", k)] for k in CLASSES]
    balanced = max(a) - min(a) <= 1 and max(b) - min(b) <= 1
    check(sum(a) == expected, f"d{density}: territory A starts with {sum(a)} and not {expected}")
    check(sum(b) == expected, f"d{density}: territory B starts with {sum(b)} and not {expected}")
    check(balanced, f"d{density}: the initial split is not balanced within one")
    print(
        f"    {density:>4} {expected:>14}  {' '.join(f'{n:>3}' for n in a)}  "
        f"{' '.join(f'{n:>3}' for n in b)}  {'yes' if balanced else 'no'}"
    )
print()
para(
    """The initial split is checked at every density on one cell per density and holds; the
    per-territory total is checked against the count the density resolves to, which is
    `hundredths * 128 * 64 / 10000` truncated. Rule 14 places resources before any source has
    acted, so the initial composition is a property of the world and not of the run."""
)
print()

print(
    f"    {'source':<11} {'dens':>4} {'ticks reached':>15} {'initial high':>14}{'final high':>13}"
    f" {'low+medium eaten':>18} {'high eaten':>12}"
)
drift = {}
for source in SOURCES:
    for density, hundredths in DENSITIES:
        group = [
            cells[cell_name(seed, source, density, "off")]
            for seed in SEEDS
            if cell_name(seed, source, density, "off") in cells
        ]
        if not group:
            continue
        rows = [cell["stream"] for cell in group]
        ticks = sorted(cell["summary"]["ticks"] for cell in group)
        initial_high = sum(row["initial"][(t, "high")] for row in rows for t in TERRITORIES)
        initial_all = sum(sum(row["initial"].values()) for row in rows)
        final_high = sum(row["final"][(t, "high")] for row in rows for t in TERRITORIES)
        final_all = sum(sum(row["final"].values()) for row in rows)
        soft = sum(row["eaten"][(t, k)] for row in rows for t in TERRITORIES for k in ("low", "medium"))
        hard = sum(row["eaten"][(t, "high")] for row in rows for t in TERRITORIES)
        drift[(source, density)] = (
            initial_high / initial_all if initial_all else 0.0,
            final_high / final_all if final_all else 0.0,
        )
        print(
            f"    {source:<11} {density:>4} {f'{ticks[0]}-{ticks[-1]}':>15} "
            f"{pct(initial_high, initial_all):>14}{pct(final_high, final_all):>13}"
            f" {soft:>18,} {hard:>12,}"
        )
print()
para(
    """The two right-hand columns are the mechanism itself, counted over the five seeds of each
    row: what the sources ate. Every source eats far more low and medium than high, which is
    what the waste condition makes them do, and the share beside it drifts upward as the
    consequence. `baseline` is on the table for contrast and is discussed in section 8."""
)
print()
short_drift = max(abs(after - before) for (_, density), (before, after) in drift.items() if density == "0.15")
long_drift = max(abs(after - before) for (_, density), (before, after) in drift.items() if density != "0.15")
para(
    f"""**The drift is a function of how long the run lasts, and the ticks column is what shows
    it.** At `0.15` the world cannot carry the population: every non-`baseline` row ends in
    extinction well before tick 1,000 -- with one exception, `social` at seed 0, which survives
    to the limit with a single agent -- and the high share moves by at most
    {100 * short_drift:.1f} points, moving *downward* under `individual`. At the two higher
    densities every run reaches the tick limit and the same share moves by up to
    {100 * long_drift:.1f} points.
    That is the reason `REQ-MOK-060`'s trigger is a 1,000-tick run at the default density rather
    than any run: the composition it constrains is a property the world only develops when it
    lasts long enough to regenerate many times over what a starving population takes."""
)

rule_5 = [
    (source, density, seed, territory, counts, total)
    for (source, density, seed), row in sorted(ratios.items())
    for territory in TERRITORIES
    for counts, total, _, _ in [row[territory]]
    if total == RULE_5_TOTAL and counts["high"] == RULE_5_HIGH
]
print()
para(
    f"""**`SPEC-MOK-001` rule 5's recorded measurement is reproduced here.** That rule records
    "high class at {RULE_5_HIGH} of {RULE_5_TOTAL} resources in a territory by tick 1,000,
    against a balanced initial third", and that sentence is where `REQ-MOK-060` comes from. Of
    the {2 * len(ratios)} territory evaluations in this capture, just {len(rule_5)} of them
    produces those two numbers:"""
)
for source, density, seed, territory, counts, total in rule_5:
    print(f"      {source:<11} d{density} seed {seed:<4} territory {territory}   "
          f"low {counts['low']:>3}  medium {counts['medium']:>3}  high {counts['high']:>3}  of {total}")
print()
para(
    """It is looked for rather than assumed, and two things make it worth the search. It is the
    default source at the default density on the first declared seed, which is the run a spec
    author would have measured. And the pre-change capture holds that same cell with a summary
    identical to the candidate's, checked in section 4, so the figure rule 5 records is one this
    packet still retains the bytes for -- the requirement's starting point is not a remembered
    number."""
)
check(rule_5 != [], "rule 5's 45-of-61 measurement is reproduced nowhere in this capture")

# ---------------------------------------------------------------------------------------------
rule("8. `baseline` is outside the obligation, and the measurement says why it has to be")

print(
    f"    {'dens':>4} {'seed':>5}  {'reason':<11}{'ticks':>5} {'resources eaten':>16}"
    f"  final == initial"
)
unmoved = 0
baseline_rows = 0
baseline_ticks = []
worst_shift = 0
most_eaten = 0
for density, hundredths in DENSITIES:
    for seed in SEEDS:
        name = cell_name(seed, "baseline", density, "off")
        cell = cells.get(name)
        if cell is None or cell["summary"] is None:
            continue
        baseline_rows += 1
        baseline_ticks.append(cell["summary"]["ticks"])
        eaten = sum(cell["stream"]["eaten"].values())
        most_eaten = max(most_eaten, eaten)
        initial, final = cell["stream"]["initial"], cell["stream"]["final"]
        same = final == initial
        unmoved += same
        worst_shift = max(worst_shift, max(abs(final[key] - initial[key]) for key in initial))
        check(
            cell["summary"]["reason"] == "extinction",
            f"{name} ends in {cell['summary']['reason']} and not extinction",
        )
        print(
            f"    {density:>4} {seed:>5}  {cell['summary']['reason']:<11}"
            f"{cell['summary']['ticks']:>5} {eaten:>16,}  {'yes' if same else 'no'}"
        )
print()
para(
    f"""`baseline` is excluded from `REQ-MOK-060` by name, and the requirement says why: it is the
    source this initiative holds byte-identical, so it cannot be given a new obligation about the
    world it produces. The measurement adds the other half of the reason.
    `Simulation::observation` puts one `Eat` in the valid-action list for each co-located resource
    with no waste condition attached, and `BaselineDecisionSource::decide` takes a uniform index
    into that list, so
    `baseline` has no waste condition to correct. And every one of its {baseline_rows} runs ends
    in extinction between ticks {min(baseline_ticks)} and {max(baseline_ticks)}, so none of them
    reaches the 1,000-tick trigger the requirement is written against -- an obligation about the
    composition at tick 1,000 would be vacuous for this source even if it were given one."""
)
print()
para(
    f"""In {unmoved} of the {baseline_rows} cells the standing composition at the end is exactly
    the composition rule 14 placed, and where it differs the widest difference in any class is
    {worst_shift}. `baseline` eats almost nothing -- {most_eaten} resources at most across a run
    with twelve agents in it, because `eat` only enters the list when an agent is standing on a
    resource and then competes with `wait`, `sleep` and up to four moves for a uniform draw --
    and rule 15 regenerates what was eaten with a class drawn the same way rule 14 draws one. So
    a class can shift by a resource or two without any drift at all, and that is what the
    {baseline_rows - unmoved} `no` rows are: not a composition drifting, but a resource replaced
    by one of another class."""
)

# ---------------------------------------------------------------------------------------------
rule("9. Why the ceiling is per territory and not over the world, measured")

hidden = []
for source in BOUND_SOURCES:
    for seed in SEEDS:
        row = ratios.get((source, DEFAULT_DENSITY, seed))
        if row is None:
            continue
        world = Counter()
        for territory in TERRITORIES:
            counts, _, _, _ = row[territory]
            for klass in CLASSES:
                world[klass] += counts[klass]
        world_total = sum(world.values())
        world_share = max(world[k] for k in CLASSES) / world_total if world_total else 0.0
        per_territory = max(row[t][3] for t in TERRITORIES)
        spread = abs(row["A"][3] - row["B"][3])
        hidden.append((source, seed, world_share, per_territory, spread))

print(
    f"    {'source':<11} {'seed':>5} {'world high share':>17} {'worst territory':>23}"
    f" {'the A/B gap':>24}"
)
for source, seed, world_share, per_territory, spread in hidden:
    print(
        f"    {source:<11} {seed:>5} {100 * world_share:16.1f}% {100 * per_territory:22.1f}%"
        f" {100 * spread:23.1f}%"
    )
widest_gap = max(spread for *_, spread in hidden) if hidden else 0.0
masked = sum(1 for _, _, w, p, _ in hidden if w <= CEILING < p)
over_both = sum(1 for _, _, w, p, _ in hidden if p > CEILING and w > CEILING)
breaching = sum(1 for _, _, _, p, _ in hidden if p > CEILING)
print()
print(f"    runs where a territory is over one half                                 {breaching} of {len(hidden)}")
print(f"    of those, runs where the world average is over one half as well          {over_both}")
print(f"    runs a world average would hide, being under one half while a territory is over  {masked}")
print(f"    the widest gap between the two territories' worst shares            {100 * widest_gap:.1f}%")
print()
para(
    f"""`REQ-MOK-060` is stated per territory rather than over the world because "a world average
    would hide a single territory's drift". **At this candidate it hides nothing**, and the
    figures say why rather than leaving it at a null: {breaching} of the {len(hidden)} runs carry
    a territory over one half, and in all {over_both} of them the world average is over one half
    too, so a world-average form of the requirement would have caught the same runs. The
    per-territory form is not what makes the breach visible here."""
)
print()
para(
    f"""What the columns do show is that the clause is not idle. The two territories are far from
    equal -- the gap between their worst shares reaches {100 * widest_gap:.1f}% -- so a corrected
    world sitting just under one half on average could still carry one territory well above it,
    which is exactly the case the clause is written for. Its value shows after the correction
    rather than before it, and that is what is recorded here: a measured reason to keep the
    clause in `WO-MOK-017`, not evidence that it is doing work at this candidate."""
)

# ---------------------------------------------------------------------------------------------
rule("10. The survivor figures the same line carries, at the default density")

print(
    f"    {'source':<11} {'seed':>5} {'survivors':>10} {'deaths':>7} {'A':>2} {'B':>2}"
    f" {'floor':>6}  against the floor"
)
for source in BOUND_SOURCES:
    for seed in SEEDS:
        cell = cells.get(cell_name(seed, source, DEFAULT_DENSITY, "off"))
        if cell is None or cell["summary"] is None:
            continue
        f = cell["summary"]
        floor = FLOORS[source]
        verdict = "at or above" if f["survivors"] >= floor else "BELOW"
        print(
            f"    {source:<11} {seed:>5} {f['survivors']:>10} {f['deaths']:>7} "
            f"{f['territory_a']:>2} {f['territory_b']:>2} {floor:>6}  {verdict}"
        )
print()
para(
    """Rule 18's summary carries these on the same line as the composition, so they are reported
    here rather than reconstructed elsewhere: `REQ-MOK-014` binds `reference` at eight of twelve
    at the default density, `REQ-MOK-034` binds the trait-aware source at eight, and
    `REQ-MOK-058`'s floor was ratified at five for `social` on the measured curve of
    2026-08-20. The floors are not adjusted to fit a measurement and are not ratified here;
    `post/runs.md` is where the per-seed outcome tables live and `escalation.md` §10 is where the
    `social` floor's ratification is recorded."""
)

# ---------------------------------------------------------------------------------------------
rule("11. What this reader does not establish")

para(
    """It is not a verification verdict. `VER-MOK-016` is the contract and `VREC-MOK-016` the
    record."""
)
print()
para(
    """It does not establish that `REQ-MOK-060` is met, and measures that it is not. The
    requirement is unimplemented under an approved deferral and its ceiling, its permitted
    mechanisms and the question of a per-class floor beside it are `WO-MOK-017`'s, with
    `VER-MOK-016`'s manual assessment 5 following the requirement. Section 6 is the curve that
    decision is to be taken on, not the decision."""
)
print()
para(
    """It does not measure a *corrected* composition anywhere, because no candidate here carries
    a correction. The sentence in `VER-MOK-016` assessment 5 that speaks of "the measured
    corrected composition" has no subject at this candidate, and that is recorded as a defect of
    fit between the assessment's wording and the descope rather than worked around."""
)
print()
para(
    """It does not carry a pre-change side for `social`, for the reason section 4 states, and it
    does not compare the two commits at greater strength than one line per cell.
    `post/byte-identity.txt` is the whole-stream comparison and it is the stronger instrument."""
)
print()
para(
    """It does not attribute the drift to any particular decision inside a source. It counts what
    was eaten by class, which is consistent with the mechanism `REQ-MOK-060`'s rationale states,
    and `post/branches.md` is where proposals are classified branch by branch."""
)

# ---------------------------------------------------------------------------------------------
print()
print("=" * 74)
if failures:
    print(f"RESULT: FAIL -- {len(failures)} checks fired")
    for message in failures:
        print(f"  - {message}")
    sys.exit(1)
print("RESULT: PASS -- every check above holds")
