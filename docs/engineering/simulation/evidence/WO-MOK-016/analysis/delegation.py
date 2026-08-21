"""VER-MOK-016 oracle 8: the delegation equality, the divergence, and the draw count per run.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/delegation.py \
        <90-cell-capture-dir> <30-cell-social-capture-dir>

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

WHAT THIS READER IS FOR
-----------------------
`VER-MOK-016` retains "the per-observation comparison of `social` against `individual` at every
opportunity where no living Mokiterion is perceived and no attack is unanswered, carrying both
proposals and the shared stream's position either side, together with the total draw count per
run under each source at matched seeds". Three clauses, and the released streams answer them to
three different depths, so the reader says which is which rather than presenting one verdict.

  * **The precondition is stream-visible, exactly.** Neither half of it looks visible at first:
    an observation's perceived-Mokiterion list is never emitted, and neither is the suffered
    record on its own. But rule 12 drives `fear` from *that same observation's* list -- one
    observation per opportunity, read before the decision and carried past it -- and the
    `survival_changed` event prints `fear` either side of the update. So the pair decodes the
    driver: up by `FEAR_INCREASE` (or saturated at `100`) means company was perceived, down by
    `FEAR_DECREASE` (or floored at `0`) means it was not. The equal case is not ambiguous
    either, because it can only arise at `100` with company or at `0` without it. And rule 17
    renders the trace's `suffered` field only when the record is non-empty, so its absence is
    the empty record. Both halves therefore come out of the released bytes, and the second half
    can be checked against a count another reader produced from a different property.

  * **Both proposals at one observation are visible only up to the parting.** The two sources
    part inside the first tick at every matched pair, and past the parting they are two worlds
    rather than two sources, so a per-opportunity comparison between them is not defined there.
    What the streams do carry is the agreement before the parting, the parting itself with both
    proposals, and -- because the proposals name the branch that took them -- the stream's
    position either side of it. The equality over whole runs is the crate's own assertion, which
    consults both sources on one observation; this reader measures the population that assertion
    runs over and does not restate it.

  * **The total draw count per run is nowhere reported, and is recovered here.** The engine
    prints no stream position, and no test records a whole-run count: `shared_stream_draws` is
    read only for post-initialization counts. It is reconstructed from the stream instead, by an
    accounting that the stream itself pins. Rule 16's regeneration draws a class and then
    coordinates until it finds a free cell, and it *prints* the class and the coordinate it
    chose. So at every `food_regenerated` event the reader replays the draw from each candidate
    stream position and keeps the positions that reproduce what the engine printed. Between two
    such events the only other consumer is the decision source, at no more than one draw per
    opportunity, and the opportunities are countable -- one `survival_changed` event each. The
    count in between is therefore solved for rather than assumed, and the solution is checked
    again at the next event.

WHAT VALIDATES THE ACCOUNTING
-----------------------------
  * `baseline` draws exactly one value per opportunity, unconditionally: rule 4 is a uniform
    selection over the valid-action list. So its recovered decision count must equal its
    opportunity count exactly, at every pin. That is a control on the method and not a property
    of `baseline`: if the solver were wrong, this is where it would show.
  * the regeneration total must decompose as three values per addition plus two per cell refused
    as occupied, so a recovered total that is not of that shape is caught arithmetically.
  * every cell is digested and compared against its retained manifest row, so the bytes read
    here are the bytes the packet retains.
  * the initialization replay is copied from `analysis/entropy.py` and re-checked against
    `INITIALIZATION_DRAWS`, so the two readers cannot disagree about where a run starts.
  * the trace setting is a further control: the two cells of a `(seed, density, source)` triple
    differ only in `--trace-actions`, so their recovered totals must be equal.
  * the decoded record census must equal `post/branches.md`'s reconstructed branch-1 count, cell
    for cell. Two readers, two properties, one number.

WHAT IS DELIBERATELY NOT DONE
-----------------------------
No temporary build. The figures below are computed from bytes this packet retains, for the
reason `post/branches.md` §1 states: a number produced by a build that no longer exists is not
re-derivable by a verifier. No test is added either, because the workspace test census is fixed
at `test-census-reconciliation.md`'s figures and an added test would move them.
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

GAMMA = 0x9E37_79B9_7F4A_7C15
MIX_1 = 0xBF58_476D_1CE4_E5B9
MIX_2 = 0x94D0_49BB_1331_11EB
MASK = (1 << 64) - 1

WORLD_SIZE = 128
TERRITORY_HEIGHT = 64
CELLS_PER_TERRITORY = WORLD_SIZE * TERRITORY_HEIGHT

CLASSES = ("low", "medium", "high")
SEEDS = (0, 1, 42, 123, 777)
DENSITIES = (("0.15", 15), ("0.75", 75), ("1.50", 150))
SOURCES = ("baseline", "reference", "individual", "social")

# `mokiterions-core/src/simulation.rs`, transcribed.
FEAR_INCREASE = 10
FEAR_DECREASE = 5
ATTRIBUTE_MAX = 100
REGENERATION_INTERVAL = 10
REGENERATION_YIELD = 2

# `INITIALIZATION_DRAWS`, transcribed. Reproduced by the replay below, never used as input.
RECORDED_DRAWS = {
    "0.15": (72, 72, 72, 72, 72),
    "0.75": (270, 268, 268, 268, 268),
    "1.50": (516, 516, 516, 514, 516),
}

# `post/branches.md` §2's branch-1 column, transcribed. That reader reconstructed rule 26's
# branch for every decision from positions, traits and geometry; branch 1 is the branch a
# non-empty suffered record takes. This one counts a rendered field. The two must agree.
BRANCH_1 = {
    ("0.15", 0): 4,
    ("0.15", 1): 4,
    ("0.15", 42): 11,
    ("0.15", 123): 3,
    ("0.15", 777): 13,
    ("0.75", 0): 7,
    ("0.75", 1): 12,
    ("0.75", 42): 15,
    ("0.75", 123): 18,
    ("0.75", 777): 7,
    ("1.50", 0): 4,
    ("1.50", 1): 9,
    ("1.50", 42): 6,
    ("1.50", 123): 12,
    ("1.50", 777): 10,
}

# `post/branches.md` §2's branch-6 column and total column, transcribed. Branch 6 is the only
# branch of rule 26 that draws, so under `social` that column *is* the decision source's draw
# count -- which section 10 checks against the count this reader recovers from the stream.
BRANCH_6 = {
    ("0.15", 0): 1_726,
    ("0.15", 1): 1_312,
    ("0.15", 42): 1_243,
    ("0.15", 123): 1_082,
    ("0.15", 777): 1_365,
    ("0.75", 0): 3_197,
    ("0.75", 1): 3_076,
    ("0.75", 42): 3_102,
    ("0.75", 123): 3_057,
    ("0.75", 777): 3_061,
    ("1.50", 0): 3_468,
    ("1.50", 1): 3_198,
    ("1.50", 42): 3_221,
    ("1.50", 123): 2_934,
    ("1.50", 777): 2_933,
}
DECISIONS = {
    ("0.15", 0): 4_408,
    ("0.15", 1): 2_507,
    ("0.15", 42): 2_668,
    ("0.15", 123): 2_881,
    ("0.15", 777): 2_498,
    ("0.75", 0): 10_847,
    ("0.75", 1): 10_019,
    ("0.75", 42): 9_279,
    ("0.75", 123): 9_021,
    ("0.75", 777): 11_009,
    ("1.50", 0): 11_009,
    ("1.50", 1): 10_018,
    ("1.50", 42): 11_009,
    ("1.50", 123): 11_008,
    ("1.50", 777): 10_020,
}

# `post/branches.md` §2's totals, transcribed, for the widened form's population.
BRANCH_3_TOTAL = 58_441
BRANCH_6_TOTAL = 37_975
DECISIONS_TOTAL = 118_201

# Verbs no rule-19 case can propose, so no agreed opportunity may carry one.
TARGETED = ("approach", "avoid", "attack", "threaten", "fight", "retreat", "surrender")

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


class SplitMix64:
    """`mokiterions-core/src/simulation.rs`'s generator, reimplemented.

    Constructed from an absolute state rather than from a seed, because that is what resuming a
    run mid-stream needs: `next_u64` adds `GAMMA` before mixing, so a stream that has produced
    `k` values from seed `s` stands at `s + k * GAMMA` and continues from there.
    """

    def __init__(self, state):
        self.state = state & MASK
        self.draws = 0

    def next_u64(self):
        self.state = (self.state + GAMMA) & MASK
        value = self.state
        value = ((value ^ (value >> 30)) * MIX_1) & MASK
        value = ((value ^ (value >> 27)) * MIX_2) & MASK
        self.draws += 1
        return value ^ (value >> 31)

    def choose_index(self, bound):
        threshold = ((-bound) & MASK) % bound
        while True:
            value = self.next_u64()
            if value >= threshold:
                return value % bound


def state_at(seed, draws):
    return (seed + draws * GAMMA) & MASK


def resources_per_territory(hundredths):
    return hundredths * CELLS_PER_TERRITORY // 10_000


def initialize(seed, hundredths):
    """`Simulation::new`'s placement, replayed. Copied from `analysis/entropy.py`."""
    per_territory = resources_per_territory(hundredths)
    entropy = SplitMix64(seed)
    foods = []
    for territory in ("A", "B"):
        for index in range(per_territory):
            occupied = {food[1] for food in foods if food[2] == territory}
            while True:
                x = entropy.choose_index(WORLD_SIZE)
                local_y = entropy.choose_index(TERRITORY_HEIGHT)
                y = local_y if territory == "A" else local_y + TERRITORY_HEIGHT
                if (x, y) not in occupied:
                    break
            foods.append((CLASSES[index % 3], (x, y), territory))

    taken = []
    for number in range(1, 13):
        territory = "A" if number <= 6 else "B"
        while True:
            x = entropy.choose_index(WORLD_SIZE)
            local_y = entropy.choose_index(TERRITORY_HEIGHT)
            y = local_y if territory == "A" else local_y + TERRITORY_HEIGHT
            if (x, y) not in taken:
                break
        taken.append((x, y))

    return entropy.draws


def territory_of(position):
    return "A" if position[1] < TERRITORY_HEIGHT else "B"


def free_coordinate(stream, territory, occupied, cap=4096):
    """`choose_free_coordinate`, replayed. Returns the coordinate, or `None` past the cap."""
    for _ in range(cap):
        x = stream.choose_index(WORLD_SIZE)
        local_y = stream.choose_index(TERRITORY_HEIGHT)
        y = local_y if territory == "A" else local_y + TERRITORY_HEIGHT
        if (x, y) not in occupied:
            return (x, y)
    return None


EVENT = re.compile(r"^tick=(\d+) subject=(\S+) event=(\w+) result=(.*)$")
FOOD_INIT = re.compile(r"^class:(\w+),position:(\d+):(\d+),territory:(\w)$")
FOOD_REGEN = re.compile(r"^food:(F\d+),class:(\w+),position:(\d+):(\d+)$")
FOOD_EATEN = re.compile(r"^food:(F\d+),")
FEAR = re.compile(r"fear:(\d+)->(\d+)$")
TRACE = re.compile(r"^proposal:([^,]*)(?:,target:(M\d+))?,status:(\w+),")


def read_cell(path):
    """One capture cell as an ordered event list. Unparsed lines are a failure, not a skip."""
    events = []
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line:
            continue
        match = EVENT.match(line)
        if not match:
            # Rule 18's final summary is the one line that is not an event record.
            if line.startswith("summary "):
                continue
            failures.append(f"{path.name}:{number} is neither an event nor a summary: {line[:60]}")
            continue
        events.append((int(match.group(1)), match.group(2), match.group(3), match.group(4)))
    return events


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest_rows(path):
    rows = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        fields = line.split()
        if len(fields) == 5 and not line.startswith("#") and fields[4].isdigit():
            rows[fields[0]] = fields[1]
    return rows


def account(events, seed, hundredths):
    """Recover the run's stream position, winnowed at every regeneration addition.

    `candidates` holds the positions still consistent with everything printed so far, as
    `(total values drawn, values drawn by regeneration)`. It starts exact -- initialization is
    replayed -- and each `food_regenerated` event winnows it against the class and the
    coordinate the engine printed.

    **The reported position is the last addition, and it has to be.** Every candidate carries
    both components, so a set of candidates is a set of whole hypotheses and the report is the
    set rather than a single number. Reporting an *earlier* addition instead -- the last one
    whose survivors all agreed on the total, say -- would be unsound rather than merely
    conservative: the opportunities after it are worth at most one value each, but the
    additions after it are worth three values apiece and more when a cell is refused, so a
    bound of `total + opportunities remaining` would sit below the true position. Reported at
    the last addition, the remainder of the run contains no addition by construction, so the
    tail is decisions only and the bound holds.

    The two components can be open separately, because the evidence pins them separately. A
    coincidence at one addition -- a second position that happens to print the same class and
    the same coordinate -- can leave two survivors that agree on the total and disagree only on
    how much of it regeneration spent, and that disagreement never resolves afterwards because
    nothing later distinguishes it. So an exact total is reported beside an open split rather
    than discarded to avoid reporting one.
    """
    initial = initialize(seed, hundredths)
    foods = {}
    candidates = [(initial, 0)]
    last = {
        "pairs": candidates,
        "opportunities": 0,
        "additions": 0,
        "where": "initialization",
    }
    since_pin = 0
    opportunities = 0
    additions = 0
    skipped = 0
    traces = 0
    ambiguous = 0
    split_only = 0
    widest = 1
    dead_end = None
    ended = None

    for tick, subject, kind, result in events:
        if kind == "food_initialized":
            match = FOOD_INIT.match(result)
            foods[subject] = (int(match.group(2)), int(match.group(3)))
        elif kind == "food_consumed":
            del foods[FOOD_EATEN.match(result).group(1)]
        elif kind == "survival_changed":
            opportunities += 1
            since_pin += 1
        elif kind == "action_trace":
            traces += 1
        elif kind == "food_regeneration_skipped":
            skipped += 1
        elif kind == "simulation_ended":
            # `step` ends the run at the tick limit or at extinction, and which one it was is
            # the reason a short run is short. Read rather than inferred from the tick count.
            ended = (tick, result.split(":", 1)[1])
        elif kind == "food_regenerated":
            additions += 1
            match = FOOD_REGEN.match(result)
            food_id, printed_class = match.group(1), match.group(2)
            printed = (int(match.group(3)), int(match.group(4)))
            territory = subject
            occupied = {
                position for position in foods.values() if territory_of(position) == territory
            }
            survivors = set()
            for total, regen in candidates:
                for step in range(since_pin + 1):
                    stream = SplitMix64(state_at(seed, total + step))
                    if CLASSES[stream.choose_index(len(CLASSES))] != printed_class:
                        continue
                    if free_coordinate(stream, territory, occupied) != printed:
                        continue
                    survivors.add((total + step + stream.draws, regen + stream.draws))
            if not survivors:
                dead_end = (tick, territory, food_id)
                break
            widest = max(widest, len(survivors))
            candidates = sorted(survivors)
            reached = {total for total, _ in candidates}
            if len(reached) > 1:
                ambiguous += 1
            elif len(candidates) > 1:
                split_only += 1
            since_pin = 0
            last = {
                "pairs": candidates,
                "opportunities": opportunities,
                "additions": additions,
                "where": f"tick {tick}",
            }
            foods[food_id] = printed

    pairs = last["pairs"]
    return {
        "initial": initial,
        "where": last["where"],
        "totals": sorted({total for total, _ in pairs}),
        "regen": sorted({regen for _, regen in pairs}),
        "decisions": sorted({total - initial - regen for total, regen in pairs}),
        "refused": sorted({regen - 3 * last["additions"] for _, regen in pairs}),
        "candidates": pairs,
        "reached": last["opportunities"],
        "opportunities": opportunities,
        "tail": opportunities - last["opportunities"],
        "ended": ended,
        "additions": additions,
        "skipped": skipped,
        "traces": traces,
        "ambiguous": ambiguous,
        "split_only": split_only,
        "widest": widest,
        "dead_end": dead_end,
    }


def opportunities_of(events):
    """Per opportunity: the proposal, its target, the record's presence, and rule 12's driver.

    Keyed by `(tick, subject)`. The trace is emitted before the record is cleared and the
    survival event immediately after, so the two lines of one opportunity are its two halves.
    """
    seen = {}
    order = []
    anomalies = []
    for tick, subject, kind, result in events:
        if kind == "action_trace":
            match = TRACE.match(result)
            if not match:
                anomalies.append(f"unparsed trace at tick {tick} {subject}")
                continue
            seen[(tick, subject)] = {
                "proposal": match.group(1),
                "target": match.group(2),
                "record": ",suffered:" in result,
                "status": match.group(3),
            }
            order.append((tick, subject))
        elif kind == "survival_changed":
            match = FEAR.search(result)
            before, after = int(match.group(1)), int(match.group(2))
            if after > before:
                company = True
                if after != before + FEAR_INCREASE and after != ATTRIBUTE_MAX:
                    anomalies.append(f"fear rose {before}->{after} at tick {tick} {subject}")
            elif after < before:
                company = False
                if after != before - FEAR_DECREASE and after != 0:
                    anomalies.append(f"fear fell {before}->{after} at tick {tick} {subject}")
            else:
                company = before == ATTRIBUTE_MAX
                if before not in (0, ATTRIBUTE_MAX):
                    anomalies.append(f"fear held at {before} at tick {tick} {subject}")
            entry = seen.setdefault((tick, subject), {})
            entry["company"] = company
            if "proposal" not in entry:
                order.append((tick, subject))
    return seen, order, anomalies


def rule(title):
    print()
    print(title)
    print("-" * len(title))


def para(text, bullet=False):
    """A paragraph with figures in it, wrapped here rather than by hand.

    The prose in this file is hand-wrapped, but a sentence whose figures are interpolated
    cannot be: a count that gains a digit would leave the line ragged, and a reader who
    re-runs this on another candidate should get a report that still reads as prose.
    """
    print(
        textwrap.fill(
            " ".join(text.split()),
            width=95,
            initial_indent="- " if bullet else "",
            subsequent_indent="  " if bullet else "",
        )
    )


if len(sys.argv) != 3:
    sys.exit(__doc__)
MATRIX = Path(sys.argv[1])
SOCIAL = Path(sys.argv[2])


def cell_path(seed, source, density, trace):
    root = SOCIAL if source == "social" else MATRIX
    return root / f"seed{seed}-{source}-d{density}-trace{trace}.txt"


TITLE = "VER-MOK-016 oracle 8: the delegation equality, the divergence, and the draw count"
print(TITLE)
print("=" * len(TITLE))
print()
print("Work order   WO-MOK-016 (Phase 3.1, encounters)")
print('Retains      "the per-observation comparison of social against individual at every')
print("             opportunity where no living Mokiterion is perceived and no attack is")
print("             unanswered, carrying both proposals and the shared stream's position either")
print("             side, together with the total draw count per run under each source at")
print('             matched seeds"')
print("Authority    VER-MOK-016 oracle 8, in both the narrow and the widened form, and oracle")
print("             8's stated consequence that a social run takes fewer draws than an")
print("             individual run at the same seed; REQ-MOK-057")
print("Candidate    7c4aef3967406c05d80da963695898b77f5329e9   (the 90-cell matrix)")
print("             59d61b915630fd55f04bcdbb346aa22cdbfdfff6   (the 30 social cells)")
print("Reader       analysis/delegation.py")
print("Date         2026-08-21")
print()
print("Oracle 8 states its equality per observation and never per run, and section 6 measures")
print("why: the two sources part inside the first tick at all fifteen matched pairs, and at")
print("every one of them the parting is where oracle 8 says it will be. Sections 3 to 5 decode")
print("the precondition from the released bytes and check the decode against a count another")
print("reader produced differently; section 6 carries both proposals and the stream's position")
print("either side of the parting; sections 7 to 12 recover the draw total per run, which")
print("nothing in the release reports, and check the recovery against two counts that were")
print("arrived at without it; section 13 says what none of it settles.")

# ---------------------------------------------------------------------------------------------
rule("1. The cells this reads, and the digests that say which")

manifests = {
    "matrix": manifest_rows(PACKET / "post" / "post-manifest.txt"),
    "social": manifest_rows(PACKET / "post" / "social-manifest.txt"),
}
cells = []
for source in SOURCES:
    for density, hundredths in DENSITIES:
        for seed in SEEDS:
            for trace in ("on", "off"):
                cells.append((seed, source, density, hundredths, trace))

missing = [c for c in cells if not cell_path(c[0], c[1], c[2], c[4]).exists()]
check(not missing, f"{len(missing)} cells absent from the capture directories")
matched = 0
for seed, source, density, hundredths, trace in cells:
    path = cell_path(seed, source, density, trace)
    if not path.exists():
        continue
    rows = manifests["social" if source == "social" else "matrix"]
    expected = rows.get(path.stem)
    if check(expected is not None, f"{path.stem} has no manifest row"):
        if check(digest(path) == expected, f"{path.stem} does not match its manifest digest"):
            matched += 1

print(f"    cells read                    {len(cells) - len(missing)}")
print(f"    manifest rows they match      {matched}")
print(f"    90-cell three-source matrix   post/post-manifest.txt, {len(manifests['matrix'])} rows")
print(f"    30 social cells               post/social-manifest.txt, {len(manifests['social'])} rows")
print()
print("Every cell is digested and compared against its row in `post/post-manifest.txt` or")
print("`post/social-manifest.txt`, so the bytes measured below are the bytes the packet retains.")
print("The two capture directories are arguments and are deliberately not printed: a path is")
print("this machine's and the digests are not, and this file has to be reproducible from the")
print("retained manifests on a verifier's machine rather than only on the author's.")

# ---------------------------------------------------------------------------------------------
rule("2. The initialization replay, checked against the suite's recorded counts")

for density, hundredths in DENSITIES:
    for index, seed in enumerate(SEEDS):
        drawn = initialize(seed, hundredths)
        check(
            drawn == RECORDED_DRAWS[density][index],
            f"initialization at seed {seed}, density {density} replays {drawn}",
        )
print("    density seed  replayed  INITIALIZATION_DRAWS  state after initialization")
for density, hundredths in DENSITIES:
    for index, seed in enumerate(SEEDS):
        drawn = initialize(seed, hundredths)
        print(
            f"    {density:>7} {seed:>4}  {drawn:>8}  {RECORDED_DRAWS[density][index]:>20}"
            f"  0x{state_at(seed, drawn):016X}"
        )
print()
print("The placement replay is copied from `analysis/entropy.py`, and it is checked here rather")
print("than trusted: the two readers must agree about where a run starts, because everything")
print("below counts forward from it. The state column is `entropy.txt` §3's, recomputed.")

# ---------------------------------------------------------------------------------------------
rule("3. Rule 12's fear pair decodes the precondition's first half")

print("`apply_survival` takes rule 12's driver from the same observation the decision was taken")
print("on -- `run_tick` reads `perceived_company` before consulting the source and carries it --")
print("and `survival_changed` prints `fear` either side of the update. So the pair says what the")
print("observation held:")
print()
print(f"    company perceived        fear rises by {FEAR_INCREASE}, or holds at {ATTRIBUTE_MAX}")
print(f"    no company perceived     fear falls by {FEAR_DECREASE}, or holds at 0")
print()
print("The equal case is not an ambiguity. `fear` is unchanged only where the update saturates,")
print("and the two saturations are at opposite ends: unchanged at 100 is company, unchanged at 0")
print("is none. Rule 23's threat increase of 30 cannot confuse this, because the pair is read")
print("inside one call and any threat this tick landed before it, in the value on the left.")
print()
print("The decode is also exhaustive over opportunities, which is what makes a census from it a")
print("census and not a sample. `apply_survival` emits unconditionally and `run_tick` calls it")
print("for every agent it consults, with no path between the two, so there is exactly one")
print("`survival_changed` record per opportunity -- and in a traced cell exactly one")
print("`action_trace` beside it. Section 12 is where those two counts are checked against each")
print("other in all 120 cells.")
print()
print("The record's half needs no reconstruction: rule 17 renders the trace's `suffered` field")
print("only when the record is non-empty, so its absence is the empty record.")

# ---------------------------------------------------------------------------------------------
rule("4. The precondition census, fifteen traced social cells, whole runs")

census = {}
for density, hundredths in DENSITIES:
    for seed in SEEDS:
        events = read_cell(cell_path(seed, "social", density, "on"))
        seen, order, anomalies = opportunities_of(events)
        check(not anomalies, f"seed {seed} d{density}: {len(anomalies)} fear anomalies")
        entries = [seen[key] for key in order if "proposal" in seen[key]]
        paired = [key for key in order if "proposal" in seen[key] and "company" in seen[key]]
        check(
            len(paired) == len(entries) == len(order),
            f"seed {seed} d{density}: trace and survival records do not pair one to one",
        )
        holds = [e for e in entries if not e["company"] and not e["record"]]
        census[(seed, density)] = {
            "opportunities": len(entries),
            "company": sum(1 for e in entries if e["company"]),
            "record": sum(1 for e in entries if e["record"]),
            "holds": len(holds),
            "targeted_under_precondition": sum(1 for e in holds if e["target"]),
            "proposals": Counter(e["proposal"].split(":")[0] for e in holds),
        }

print("    density seed  opportunities  company  record  precondition holds  targeted there")
for density, _ in DENSITIES:
    for seed in SEEDS:
        row = census[(seed, density)]
        print(
            f"    {density:>7} {seed:>4}  {row['opportunities']:>13}  {row['company']:>7}"
            f"  {row['record']:>6}  {row['holds']:>18}  {row['targeted_under_precondition']:>14}"
        )
totals = {
    key: sum(census[c][key] for c in census)
    for key in ("opportunities", "company", "record", "holds", "targeted_under_precondition")
}
print(
    f"    {'all':>7} {'':>4}  {totals['opportunities']:>13}  {totals['company']:>7}"
    f"  {totals['record']:>6}  {totals['holds']:>18}  {totals['targeted_under_precondition']:>14}"
)
check(
    totals["targeted_under_precondition"] == 0,
    f"{totals['targeted_under_precondition']} targeted proposals under the precondition",
)
check(totals["holds"] > 0, "the precondition never held")
check(
    totals["opportunities"] == DECISIONS_TOTAL,
    f"{totals['opportunities']} opportunities against branches.md's {DECISIONS_TOTAL} decisions",
)
for density, _ in DENSITIES:
    for seed in SEEDS:
        counted = census[(seed, density)]["opportunities"]
        check(
            counted == DECISIONS[(density, seed)],
            f"seed {seed} d{density}: {counted} opportunities against branches.md's"
            f" {DECISIONS[(density, seed)]} decisions",
        )
check(
    totals["holds"] + totals["company"] == totals["opportunities"],
    "an opportunity carried a suffered record without company, so the two counts do not close",
)
share = 100.0 * totals["holds"] / totals["opportunities"]
print()
print(f"The narrow form's precondition holds at {share:.1f}% of all opportunities in the fifteen")
print("traced `social` cells, so the equality oracle 8 buys is asserted over a population that")
print("is most of the matrix rather than a corner of it. The proposals at those opportunities")
print("are rule 19's vocabulary and nothing else -- no targeted verb appears at any of them,")
print("which is what delegation to rule 19 predicts and is checked above rather than assumed:")
print()
kinds = Counter()
for key in census:
    kinds.update(census[key]["proposals"])
for kind, count in sorted(kinds.items(), key=lambda item: -item[1]):
    print(f"    {kind:<10} {count:>8}")
print()
print("**The widened form's population is larger and is not decoded here.** Oracle 8's amendment")
print("of 2026-08-20 widened the equality to any observation where a tolerated resource is")
print("perceived and the record is empty, whether or not a Mokiterion is perceived. That")
print("precondition is a predicate over reconstructed positions and the acting Mokiterion's own")
print("waste tolerance, not a rendered field, so this reader does not decode it. It is already")
print("measured: rule 26's branch 3 is reached only where a tolerated resource is perceived and")
print("branch 1 did not pre-empt, so `post/branches.md` §2's branch-3 count is a population the")
print("widened form covers, with thirteen checks against the engine behind it.")
print()
print(f"    branch 3, fifteen traced cells    {BRANCH_3_TOTAL:>7} of {DECISIONS_TOTAL} decisions")
print(f"    narrow precondition, same cells   {totals['holds']:>7} of {totals['opportunities']}")

# ---------------------------------------------------------------------------------------------
rule("5. The decode's three checks, each from a direction the decode does not use")

targeted_without_cause = 0
resolutions_without_company = 0
for density, _ in DENSITIES:
    for seed in SEEDS:
        events = read_cell(cell_path(seed, "social", density, "on"))
        seen, _, _ = opportunities_of(events)
        for key, entry in seen.items():
            if entry.get("target") and not (entry.get("company") or entry.get("record")):
                targeted_without_cause += 1
        for tick, subject, kind, _ in events:
            if kind in ("attack_resolved", "threat_resolved", "surrender_resolved"):
                entry = seen.get((tick, subject), {})
                if not entry.get("company"):
                    resolutions_without_company += 1

check(targeted_without_cause == 0, f"{targeted_without_cause} targeted proposals without cause")
check(
    resolutions_without_company == 0,
    f"{resolutions_without_company} resolutions where the decode denies company",
)
print("A targeted proposal can only come from a branch that requires perceived company or a")
print("standing record, and a resolution can only follow a targeted proposal, so both must land")
print("at opportunities the fear pair agrees about:")
print()
print(f"    targeted proposals with neither company nor record   {targeted_without_cause}")
print(f"    resolutions where the decode denies company          {resolutions_without_company}")
print()
print("The third check is the strongest, because the number it is checked against is another")
print("reader's. `post/branches.md` reconstructed rule 26's branch for every decision from")
print("positions, traits and geometry, and branch 1 is the branch a non-empty record takes. This")
print("reader counts a rendered field. Two properties, two readers, one number, and they agree")
print("cell for cell:")
print()
print("    density seed  `,suffered:` rendered  branches.md branch 1  agree")
agreeing = 0
for density, _ in DENSITIES:
    for seed in SEEDS:
        mine, theirs = census[(seed, density)]["record"], BRANCH_1[(density, seed)]
        ok = mine == theirs
        agreeing += 1 if ok else 0
        check(ok, f"seed {seed} d{density}: record census {mine} against branch 1 {theirs}")
        print(f"    {density:>7} {seed:>4}  {mine:>21}  {theirs:>20}  {'yes' if ok else 'NO'}")
print(f"    {'all':>7} {'':>4}  {totals['record']:>21}  {sum(BRANCH_1.values()):>20}")
print()
print(f"    cells where the two readers agree   {agreeing} of 15")

# ---------------------------------------------------------------------------------------------
rule("6. The parting, all fifteen matched pairs, with the stream's position either side")

partings = []
for density, hundredths in DENSITIES:
    for seed in SEEDS:
        social_seen, social_order, _ = opportunities_of(
            read_cell(cell_path(seed, "social", density, "on"))
        )
        other_seen, other_order, _ = opportunities_of(
            read_cell(cell_path(seed, "individual", density, "on"))
        )
        agreed = []
        parting = None
        for left, right in zip(social_order, other_order):
            if left != right:
                parting = (left, {"proposal": "(the opportunity itself moved)"}, {})
                break
            a, b = social_seen[left], other_seen[right]
            if (a["proposal"], a["target"]) != (b["proposal"], b["target"]):
                parting = (left, a, b)
                break
            agreed.append(a)
        partings.append(
            {
                "seed": seed,
                "density": density,
                "initial": initialize(seed, hundredths),
                "agreed": agreed,
                "parting": parting,
                "seen": social_seen,
            }
        )

check(all(row["parting"] is not None for row in partings), "a matched pair never parted")

print("    density seed  agreed  tick  subject  social proposal    individual proposal")
for row in partings:
    if row["parting"] is None:
        continue
    (tick, subject), a, b = row["parting"]
    left = a["proposal"] + (f" -> {a['target']}" if a.get("target") else "")
    right = b.get("proposal", "") + (f" -> {b['target']}" if b.get("target") else "")
    print(
        f"    {row['density']:>7} {row['seed']:>4}  {len(row['agreed']):>6}  {tick:>4}"
        f"  {subject:>7}  {left:<17}  {right}"
    )

branch_five = 0
individual_moved = 0
engaged = 0
for row in partings:
    if row["parting"] is None:
        continue
    key, a, b = row["parting"]
    if a["proposal"] in ("approach", "avoid"):
        branch_five += 1
    if b.get("proposal", "").startswith("move:"):
        individual_moved += 1
    if row["seen"][key].get("company") or row["seen"][key].get("record"):
        engaged += 1
check(branch_five == 15, f"only {branch_five} partings had social taking a distance branch")
check(individual_moved == 15, f"only {individual_moved} partings had individual proposing a move")
check(engaged == 15, f"only {engaged} partings were at an engaged opportunity")
print()
print(f"    partings where social proposed approach or avoid     {branch_five} of 15")
print(f"    partings where individual proposed a plain move      {individual_moved} of 15")
print(f"    partings at an opportunity with company or a record  {engaged} of 15")
print()
print("**This is exactly where oracle 8 says the parting will be.** Oracle 8 has the two sources")
print('diverging "at the first opportunity where one Mokiterion perceives another and no')
print('tolerated resource is perceived". Both halves are readable off the two proposals.')
print("`approach` and `avoid` are rule 26 branch 5's verbs and no rule-19 case proposes either,")
print("so social's proposal says a Mokiterion was perceived; and branch 5 is reached only past")
print("branch 3, so the same proposal says no tolerated resource was perceived. Individual, on")
print("the identical observation, fell through rule 19's cases 1 to 3 to case 4's random step,")
print("which is the plain move it printed. Fifteen pairs, fifteen partings, every one of that")
print("shape, and none of it asserted anywhere else in the packet.")
print()
print("**The stream's position either side follows from the same two proposals.** Branch 5 draws")
print("nothing -- `entropy.txt` §7 shows each of branches 1 to 5 returning before the source's")
print("single `entropy` use -- and rule 19's case 4 draws exactly once. So at the parting the")
print("social stream stands still and the individual stream advances by one value. The position")
print("it advances *from* is the post-initialization state plus one value for each earlier")
print("opportunity that drew, and among those only a plain move can have drawn:")
print()
print("    density seed  agreed  of them plain moves  the stream's position before the parting")
exact = 0
for row in partings:
    if row["parting"] is None:
        continue
    moves = sum(1 for e in row["agreed"] if e["proposal"].startswith("move:"))
    others = Counter(
        e["proposal"].split(":")[0] for e in row["agreed"] if not e["proposal"].startswith("move:")
    )
    check(
        not any(verb in TARGETED for verb in others),
        f"seed {row['seed']} d{row['density']}: an agreed opportunity carried a targeted verb",
    )
    base = state_at(row["seed"], row["initial"])
    if moves == 0:
        exact += 1
        shown = f"0x{base:016X}                        exact"
    else:
        shown = (
            f"0x{base:016X} .. 0x{state_at(row['seed'], row['initial'] + moves):016X}"
            f"   {moves + 1} candidates"
        )
    print(f"    {row['density']:>7} {row['seed']:>4}  {len(row['agreed']):>6}  {moves:>19}  {shown}")
print()
print(f"    pairs where the position before the parting is exact   {exact} of 15")
print()
print("Where the agreed prefix is empty or carries no plain move, the position before the")
print("parting is the post-initialization state of section 2 exactly, and the position after it")
print("is that state under `social` and that state plus GAMMA under `individual`. Where the")
print("prefix carries plain moves, each of them is either rule 19's case 3 -- a step toward a")
print("tolerated resource, no draw -- or its case 4, and the released line does not distinguish")
print("them, so the position is given as the window they span. `post/branches.md`'s")
print("reconstruction is what separates cases 3 and 4, and this file does not duplicate it.")
print()
print("The agreed prefixes carry only rule 19's own verbs, which is the equality holding for as")
print("long as it is defined to:")
print()
prefix = Counter()
for row in partings:
    prefix.update(e["proposal"].split(":")[0] for e in row["agreed"])
for kind, count in sorted(prefix.items(), key=lambda item: -item[1]):
    print(f"    {kind:<10} {count:>4}")

# ---------------------------------------------------------------------------------------------
rule("7. The draw accounting, and what pins it")

print("Rule 16 regenerates on a fixed interval and prints what it drew: the class, and the")
print("coordinate it settled on. `choose_free_coordinate` retries on an occupied cell, and the")
print("cells occupied at that moment are reconstructible -- every food's position arrives in a")
print("`food_initialized` or `food_regenerated` record and leaves in a `food_consumed` one. So a")
print("candidate stream position either reproduces the printed class and coordinate or it does")
print("not, and the accounting is pinned at every addition.")
print()
print("Between two pins the only other consumer is the decision source, at no more than one")
print("value per opportunity, and every opportunity emits one `survival_changed` record. The")
print("number of values in between is therefore bounded above by a count the stream gives, and")
print("is solved for rather than assumed. A skipped regeneration draws nothing at all --")
print("`regenerate_food` returns before its first draw -- so it pins nothing either.")
print()
print("**Section 8 reports the run's last addition, always, and never an earlier one.** That is")
print("what makes its tail a bound rather than an estimate: the opportunities after the reported")
print("point are worth at most one value each, and no addition follows to spend three or more")
print("without being counted. Reporting an earlier addition -- the last one whose survivors all")
print("agreed, say -- with the same tail would understate the position instead of widening it.")
print("Where the last addition leaves several positions standing, the report is all of them.")
print()
print(f"    regeneration interval    every {REGENERATION_INTERVAL} ticks, territories A then B")
print(f"    additions per territory  at most {REGENERATION_YIELD}, and fewer near capacity")
print("    values per addition      1 class + 2 per coordinate attempt")
print(f"    classes                  {len(CLASSES)} -- {', '.join(CLASSES)}")
print()
print("Two of the bounds are exact rather than probabilistic, and the difference matters. The")
print("coordinate bounds 128 and 64 divide 2^64, so `choose_index` never rejects for modulo bias")
print("there and a coordinate attempt costs exactly two values -- `entropy.txt` §1 derives that")
print("and gives the control. The class bound 3 does not divide 2^64: its rejection threshold is")
print("1, so a class draw could in principle cost more than one value, and so could a decision")
print("at a bound that does not divide either. Neither is assumed away. A rejection anywhere")
print("would push the true consumption past the bound the solver searches, and the pin would")
print("then find no consistent position at all -- which is the `stream lost` line of section 8")
print("reading zero, and is a detection rather than a silent wrong figure.")

# ---------------------------------------------------------------------------------------------
rule("8. The draw total per run, sixty cells, four sources at matched seeds")

accounts = {}
for seed, source, density, hundredths, trace in cells:
    path = cell_path(seed, source, density, trace)
    if not path.exists():
        continue
    accounts[(seed, source, density, trace)] = account(read_cell(path), seed, hundredths)

def band(values):
    return str(values[0]) if len(values) == 1 else f"{values[0]}-{values[-1]}"


lost = 0
for key, row in accounts.items():
    if row["dead_end"] is not None:
        lost += 1
        failures.append(f"{key} lost the stream at {row['dead_end']}")
    check(
        all(refused >= 0 and refused % 2 == 0 for refused in row["refused"]),
        f"{key} regeneration total {band(row['regen'])} is not three per addition"
        " plus two per refusal",
    )
    check(
        all(0 <= drawn <= row["reached"] for drawn in row["decisions"]),
        f"{key} recovered {band(row['decisions'])} decision draws over"
        f" {row['reached']} opportunities",
    )

print("    `initial` is initialization, replayed exactly. The rest is recovered to the run's")
print("    last regeneration addition: `total` is the stream's absolute position there in values")
print("    drawn, `regen` the part of it regeneration spent and `decisions` the part the decision")
print("    source spent. `tail` is the opportunities after that addition, each of which may or")
print("    may not have drawn, and no addition follows it, so the run's own total is `total` plus")
print("    something in `0..=tail`. A figure shown as a range is a set of whole hypotheses the")
print("    evidence does not separate: where only `regen` and `decisions` are ranged the total")
print("    beside them is still exact.")
print()
print("    source      density seed  opps  initial  decisions  regen  total  tail  last addition")
no_pin = []
for source in SOURCES:
    for density, _ in DENSITIES:
        for seed in SEEDS:
            row = accounts[(seed, source, density, "on")]
            if row["where"] == "initialization":
                no_pin.append((source, density, seed))
            print(
                f"    {source:<11} {density:>7} {seed:>4} {row['opportunities']:>5}"
                f" {row['initial']:>8} {band(row['decisions']):>10}"
                f" {band(row['regen']):>6}"
                f" {band(row['totals']):>6} {row['tail']:>5}  {row['where']}"
            )

traced = {key: row for key, row in accounts.items() if key[3] == "on"}
whole = [key for key, row in traced.items() if row["tail"] == 0 and len(row["totals"]) == 1]
split = [key for key, row in traced.items() if len(row["regen"]) == 1]
open_total = [key for key, row in traced.items() if len(row["totals"]) > 1]

# The reading below names the exact thirty cells, so it is a check and not an observation: a
# candidate that changed which cells pin at tick 1,000 would make the sentence false, and a
# sentence that has to be re-read by hand to stay true is not evidence.
dense_fed = {
    key for key in traced if key[1] != "baseline" and key[2] in ("0.75", "1.50")
}
check(
    set(whole) == dense_fed,
    "the cells exact for the whole run are not exactly the fed cells at density 0.75 and 1.50:"
    f" {sorted(dense_fed - set(whole))} missing, {sorted(set(whole) - dense_fed)} unexpected",
)
print()
print(f"    cells whose total is exact for the whole run     {len(whole)} of 60")
print(f"    cells whose decision/regeneration split is exact {len(split)} of 60")
print(f"    cells whose total is open at the last addition   {len(open_total)} of 60")
print(f"    cells reaching no addition at all                {len(no_pin)} of 60")
print(f"    cells where the stream was lost                  {lost} of 60")
print(f"    additions leaving two totals open                "
      f"{sum(row['ambiguous'] for row in traced.values())}")
print(f"    additions leaving only the split open            "
      f"{sum(row['split_only'] for row in traced.values())}")
print(f"    widest candidate set at any addition             "
      f"{max(row['widest'] for row in traced.values())}")
print()
print("Four readings, and three of them are limitations rather than findings:")
print()
para(
    f"""**{len(whole)} of the sixty cells give an exact whole-run total, and they are exactly the
    thirty cells at density `0.75` or `1.50` under `reference`, `individual` and `social`.** Their
    last regeneration is at tick 1,000 and regeneration is the last thing a tick does, so nothing
    follows it to be uncertain about, and for those cells the retention item's third clause is
    discharged with no residual at all. The other thirty are the twenty sparse cells and the ten
    remaining `baseline` cells, and it is the same reason in both: regeneration is what pins the
    stream, and it is scarce where resources are scarce and where nothing eats them. Section 10
    closes the `social` cells among those thirty anyway, with an instrument that is not this
    one.""",
    bullet=True,
)
para(
    f"""**{len(no_pin)} cells reach no addition at all, and every one of them is `baseline`.**
    Regeneration is skipped while a territory is at capacity, and rule 4 selects uniformly rather
    than seeking food, so nothing is eaten, no cell is freed and nothing is added. What ends those
    runs is extinction rather than the tick limit -- `step` stops at either, and the event says
    which -- and they end early enough that the whole regeneration record of the run is a couple
    of dozen skips. For those cells this reader reports initialization and a tail the width of the
    run, and claims nothing further. They are named below, with the tick they ended at and why,
    rather than dropped.""",
    bullet=True,
)
para(
    """**An open split is not an open total.** Two positions that print the same class at the same
    coordinate agree on where the stream ends and disagree on which consumer spent which part of
    it, and nothing printed later separates them, so the split stays open for the rest of the run
    while the total does not.""",
    bullet=True,
)
para(
    f"""**An open *total* closes at the next addition, where there is one.** A position that
    survives one addition by coincidence has to survive the next one too, and the addition after
    it, so the totals converge and the cost is tail width rather than accuracy. Across the sixty
    cells {sum(row['ambiguous'] for row in traced.values())} additions left two totals open, and
    in {len(open_total)} of the sixty the ambiguous addition is the run's last, so nothing follows
    to close it. There section 9's control does more than agree with the solver: `baseline`'s
    known count is one of the hypotheses and is not the others, so it selects between them, and it
    could have matched none of them.""",
    bullet=True,
)

if no_pin:
    print()
    print("    the cells reaching no addition, named:")
    for source, density, seed in no_pin:
        row = accounts[(seed, source, density, "on")]
        tick, reason = row["ended"]
        check(
            reason == "extinction",
            f"{source} seed {seed} d{density} reached no addition and ended by {reason}",
        )
        print(
            f"      {source:<11} d{density}  seed {seed:<4}"
            f"  {row['skipped']:>3} skipped, {row['additions']} added,"
            f" {row['opportunities']:>4} opportunities, {reason} at tick {tick}"
        )

if open_total:
    print()
    print("    the cells whose total is open at the last addition, named, with every hypothesis:")
    for source in SOURCES:
        for density, _ in DENSITIES:
            for seed in SEEDS:
                if (seed, source, density, "on") not in open_total:
                    continue
                row = accounts[(seed, source, density, "on")]
                print(
                    f"      {source:<11} d{density}  seed {seed:<4}"
                    f"  {row['additions']} added, last at {row['where']}"
                )
                for total, regen in row["candidates"]:
                    print(
                        f"        total {total:>6}  regeneration {regen:>4}"
                        f"  decisions {total - row['initial'] - regen:>6}"
                    )

# ---------------------------------------------------------------------------------------------
rule("9. The control: `baseline` draws once per opportunity, and the solver finds it")

print("Rule 4 selects uniformly over the valid-action list at every opportunity,")
print("unconditionally, so `baseline`'s decision total is its opportunity count -- known without")
print("any solving. The solver is not told this. Where the two disagree, the solver is wrong.")
print()
print("    density seed  opportunities to the pin  recovered decisions  agree")
control = 0
controlled = 0
exactly = 0
for density, _ in DENSITIES:
    for seed in SEEDS:
        row = accounts[(seed, "baseline", density, "on")]
        if row["where"] == "initialization":
            continue
        controlled += 1
        known = row["reached"]
        agree = known in row["decisions"]
        control += 1 if agree else 0
        exactly += 1 if row["decisions"] == [known] else 0
        check(
            agree,
            f"baseline seed {seed} d{density}: recovered {band(row['decisions'])} against"
            f" {known} opportunities",
        )
        print(
            f"    {density:>7} {seed:>4}  {known:>23}"
            f"  {band(row['decisions']):>19}  {'yes' if agree else 'NO'}"
        )
print()
print(f"    baseline cells carrying a pin                        {controlled} of 15")
print(f"    cells where the known count is a recovered count     {control} of {controlled}")
print(f"    cells where it is the only recovered count           {exactly} of {controlled}")
print()
para(
    f"""The `baseline` cells that reach no addition are not silent: section 8 names them, and they
    carry no pin because nothing regenerated. The {controlled} that do are what the control is
    for, and it is a strong one -- {exactly} of them recover the known count with no slack at all,
    over hundreds of opportunities and dozens of additions, from a stream that reports neither of
    the two quantities."""
)
if exactly < controlled:
    print()
    para(
        f"""That leaves {controlled - exactly} of the {controlled}, which section 8 names: cells
        whose last addition left more than one hypothesis standing. The known count is one of
        those hypotheses and not the others, so there the control selects rather than merely
        agrees -- a solver wrong about the tail would have left it matching none of them -- and
        the hypothesis it selects is the position this record reads for that cell."""
    )

# ---------------------------------------------------------------------------------------------
rule("10. The second control, and the strongest: branch 6 is the social source's draw count")

print("`entropy.txt` §7 enumerates every draw site in the shipped engine and shows that the only")
print("one a `social` decision can reach is rule 19's case 4, through branch 6: each of branches")
print("1 to 5 returns before the source's single `entropy` use. So under `social` the decision")
print("source's whole-run draw count *is* rule 26's branch-6 count.")
print()
print("That count already exists, measured by an unrelated instrument. `post/branches.md`")
print("reconstructed the branch of all 118,201 decisions from positions, traits and geometry,")
print("with thirteen checks against the engine, and never once looked at the entropy stream.")
print("This reader replayed the stream and never once looked at a branch. The two numbers are")
print("the same number, and here they meet:")
print()
print("    density seed  recovered decision draws  branches.md branch 6  agrees")
inside = 0
identical = 0
narrowed = 0
for density, _ in DENSITIES:
    for seed in SEEDS:
        row = accounts[(seed, "social", density, "on")]
        low = row["decisions"][0]
        high = row["decisions"][-1] + row["tail"]
        theirs = BRANCH_6[(density, seed)]
        ok = low <= theirs <= high
        inside += 1 if ok else 0
        identical += 1 if low == high == theirs else 0
        narrowed += 1 if ok and low != high else 0
        check(
            ok,
            f"social seed {seed} d{density}: recovered {low}..{high} excludes"
            f" branch 6's {theirs}",
        )
        shown = str(low) if low == high else f"{low}..{high}"
        print(
            f"    {density:>7} {seed:>4}  {shown:>24}  {theirs:>20}"
            f"  {'yes' if ok else 'NO'}"
        )
print()
print(f"    cells where branch 6's count is inside the recovered interval   {inside} of 15")
print(f"    cells where the two are one number with no interval at all      {identical} of 15")
print(f"    cells where branch 6's count resolves this reader's interval    {narrowed} of 15")
print()
print("**This is the check that makes the accounting evidence rather than arithmetic.** The")
print("solver could have been wrong in a way no internal consistency would show -- a miscounted")
print("opportunity, a draw site missed, a rejection unaccounted for -- and every one of those")
print("would move these figures away from a number computed without the stream. None of them do.")
print("It also runs the other way: the agreement is a check on `post/branches.md`'s branch-3 and")
print("branch-6 split, which is the one part of that reconstruction no rendered field")
print("distinguishes and which §1 of that file says rests on its thirteen checks. It now rests")
print("on the entropy stream too.")
print()
print("And it closes the intervals. Where this reader reports a range -- because a coincidence")
print("left the decision/regeneration split open, or because a tail is open -- branch 6's count")
print("is a single number inside it, so the run's decision draw count is that number and the")
print("regeneration figure beside it in section 8 is the total less that number less")
print("initialization.")

# ---------------------------------------------------------------------------------------------
rule("11. Oracle 8's stated consequence: social takes fewer draws than individual")

print("Oracle 8: \"Because `REQ-MOK-057`'s branches 4 and 5 pre-empt rule 19's case 4, a")
print("`social` run takes **fewer** draws than an `individual` run on the same seed.\" The two")
print("runs are different worlds past the parting, so this is a comparison of totals and not of")
print("positions -- which is how oracle 8 states it, and why. Where a tail is open the")
print("comparison is made on the whole interval, and it reads as holding only if the whole")
print("interval holds it.")
print()
print("    density seed   social total      individual total    social fewer")
holds = 0
undecided = 0
for density, _ in DENSITIES:
    for seed in SEEDS:
        left = accounts[(seed, "social", density, "on")]
        right = accounts[(seed, "individual", density, "on")]
        low, high = left["totals"][0], left["totals"][-1] + left["tail"]
        other_low = right["totals"][0]
        other_high = right["totals"][-1] + right["tail"]
        verdict = "yes" if high < other_low else ("no" if low >= other_high else "undecided")
        holds += 1 if verdict == "yes" else 0
        undecided += 1 if verdict == "undecided" else 0
        check(verdict != "no", f"seed {seed} d{density}: social drew at least as many as individual")
        print(
            f"    {density:>7} {seed:>4}  {low:>6}..{high:<6}  {other_low:>7}..{other_high:<6}"
            f"    {verdict}"
        )
print()
print(f"    matched pairs where the social total is strictly lower   {holds} of 15")
print(f"    matched pairs the open tails leave undecided             {undecided} of 15")
print(f"    matched pairs where the consequence is contradicted      {15 - holds - undecided} of 15")
check(holds > 0, "oracle 8's fewer-draws consequence is decided nowhere")
print()
print("The mechanism is section 4's census read as arithmetic. `social` spends a value only at")
print(f"branch 6, and `post/branches.md` §2 counts {BRANCH_6_TOTAL:,} of {DECISIONS_TOTAL:,}")
print("decisions there; every branch-4 and branch-5 decision is an opportunity where")
print("`individual` would have reached rule 19's case 4 and drawn. The comparison above is that")
print("same difference measured on the stream rather than counted on the branches, which is why")
print("it is worth having beside it and not instead of it.")

# ---------------------------------------------------------------------------------------------
rule("12. The control: the trace setting moves no draw")

pairs = 0
paired = 0
for source in SOURCES:
    for density, _ in DENSITIES:
        for seed in SEEDS:
            on = accounts[(seed, source, density, "on")]
            off = accounts[(seed, source, density, "off")]
            pairs += 1
            same = (
                on["totals"],
                on["reached"],
                on["opportunities"],
                on["additions"],
            ) == (
                off["totals"],
                off["reached"],
                off["opportunities"],
                off["additions"],
            )
            paired += 1 if same else 0
            check(same, f"{source} seed {seed} d{density}: the trace setting moved the accounting")
            check(
                on["traces"] == on["opportunities"],
                f"{source} seed {seed} d{density}: {on['traces']} traces over"
                f" {on['opportunities']} opportunities",
            )
            check(
                off["traces"] == 0,
                f"{source} seed {seed} d{density}: an untraced cell emitted"
                f" {off['traces']} traces",
            )
print(f"    trace pairs compared                               {pairs}")
print(f"    pairs agreeing on total, opportunities, additions  {paired}")
print()
print("`--trace-actions` adds a record and must move nothing else. The two cells of a triple are")
print("accounted independently and must reach the same position from the same opportunity count.")
print("This is also where the traced cell's `action_trace` count is checked against its")
print("`survival_changed` count and the untraced cell's against zero, which is section 3's")
print("exhaustiveness claim measured in all 120 cells rather than argued from the source.")

# ---------------------------------------------------------------------------------------------
rule("13. What this reader does not establish")

print("It does not consult the two sources on one observation. That is the crate's own")
print("assertion, and past the parting it is the only instrument that can: past the parting")
print("there is no shared observation to consult them on. Section 4 measures the population the")
print("assertion runs over and section 6 measures why the released streams cannot reach further;")
print("neither restates the assertion as a measurement of its own.")
print()
print("It does not decode the widened form's precondition. Section 4 says why, and where the")
print("population it covers is already measured.")
print()
print("It does not close every tail *by itself*. The cells named in section 8 are exact for the")
print("whole run; the rest are exact to a stated pin, with the remaining opportunities carried as")
print("a range rather than folded into the figure. Closing them from the stream alone would need")
print("a position the engine does not print, which is `entropy.txt` §8's recorded residual and")
print("not a new one. What does close them, where it reaches, is section 10: `post/branches.md`'s")
print("branch-6 count is a single number inside every `social` interval this reader leaves open,")
print("so for that source the range is a property of this instrument and not of the evidence. The")
print("`reference` and `individual` ranges at density `0.15` have no such second instrument and")
print("stay open as printed.")
print()
print("It does not separate rule 19's case 3 from its case 4 on a released line. Section 6's")
print("windows are that limitation stated where it bites, and `post/branches.md`'s")
print("reconstruction is where the separation is made.")
print()
print("It is not a verification verdict. `VER-MOK-016` is the contract and `VREC-MOK-016` the")
print("record.")

print()
print("=" * 74)
if failures:
    print(f"RESULT: FAIL -- {len(failures)} checks did not hold")
    for message in failures:
        print(f"  * {message}")
    sys.exit(1)
print("RESULT: PASS -- every check above holds")
