"""VER-MOK-012's long-horizon oracle: what a 10,000-tick run completes with, and what it retains.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/horizon.py <capture-dir> \
        --against docs/engineering/simulation/evidence/WO-MOK-012/post/social-manifest.txt
    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/horizon.py <capture-dir> \
        --control <name>

`<capture-dir>` is one produced by `../capture-horizon.sh`: ten `<cell>.txt` streams and ten
`<cell>.exit` exit codes. Exits `0` when every check holds and `1` on the first failure list.
`--against` adds the provenance check and is where the capture is tied to the released one.

The contract asks for three things of the long run -- that it **completes**, that it shows no
**unbounded growth in retained state**, and that its **composition and survivor figures** are
captured. This reader answers all three from the stream alone.

Completion is the cheap part: the exit code, the `simulation_ended` line, the summary line, and that
nothing follows them. The other two are the reasons this file exists.

**Retained state is measured, not asserted.** Four collections are retained across ticks -- the
Mokiterions, the food, each Mokiterion's suffered-attack record, and the previous tick's decision
snapshots. Three of the four are visible in the stream and are reconstructed here tick by tick:

  * the population, from the twelve `agent_initialized` lines and every `agent_died` line. It may
    only fall, no identifier may be initialized twice or die twice, and **no line may name a dead
    Mokiterion afterwards** -- rule 13's finality, checked over every tick rather than in a
    constructed state. The curve is then read a second way, from the `survival_changed` line every
    living Mokiterion emits once per tick: the subjects of each tick's lines must be exactly those
    alive at its start, which is a per-tick check where the summary's survivor count is one number
    at the end;
  * the food collection, from `food_initialized`, `food_consumed` and `food_regenerated`, per
    territory and per class. It may never exceed the per-territory capacity, and every
    `food_regeneration_skipped` line carries the engine's own count for that territory, which is
    checked against the reader's independently maintained one. The six food fields of the summary
    line are checked against the reader's final per-class counts;
  * the suffered-attack record, from the `suffered` field of `action_trace`, which a trace-off
    stream never shows. Its length is bounded by the number of *other* Mokiterions, because rule 25
    closes the window at the sufferer's own opportunity and each of the other eleven has exactly one
    opportunity inside it.

The fourth collection, and the event vector the text host leaves absent, are not visible in any
stream. They are measured instead by peak working set, in `analysis/peak.py`, and the two
measurements are reported apart because they are of different things.

**The tick ladder is the growth measurement's other half.** The same seed at 1,000, 2,000, 5,000,
10,000 and 20,000 ticks: each shorter run's body must be a prefix of the longer one's, so the tick
limit decides where a run stops and nothing else about it. Nothing here reads the implementation.
"""

import hashlib
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")

# The world SPEC-MOK-001 fixes, and the capture's own density.
AGENTS = 12
PER_TERRITORY = 61
CLASSES = ("low", "medium", "high")
TERRITORIES = ("A", "B")
SPLIT = 63
NEWLINE = b"\n"

# Rule 25's window closes at the sufferer's own next decision opportunity, so each of the other
# eleven Mokiterions has exactly one opportunity inside it and may add exactly one entry.
WINDOW_BOUND = AGENTS - 1

# Every living Mokiterion takes exactly one decision opportunity per tick, and `survival_changed` is
# emitted once for each, in identifier order. `DECISION_ORDER` is that order's direction; reversing
# it is a control.
DECISION_ORDER = 1

# The one exception, and it is the stream's rather than an allowance: a Mokiterion killed earlier in
# the same tick never reaches its own opportunity. "Earlier" is what the identifier order means, so
# the forfeit falls exactly on the victims a *lower* identifier killed in that tick. Reversing this
# comparison is a control, because it is the direction that makes the exception a rule rather than
# an excuse -- a victim its killer acted after must still be found deciding.
STRIKE_ORDER = 1

# Each control names one global and the single value it is given. Nothing else moves.
CONTROLS = {
    "population": ("AGENTS", 13),
    "capacity": ("PER_TERRITORY", 60),
    "window": ("WINDOW_BOUND", 0),
    "split": ("SPLIT", 62),
    "order": ("DECISION_ORDER", -1),
    "strike-order": ("STRIKE_ORDER", -1),
    "ladder": ("LADDER_DIRECTION", -1),
    "projection": ("PROJECT_TRACE", False),
    "provenance": ("PROVENANCE_TICKS", "t2000"),
}

# The trace-on stream is compared against the trace-off stream of the same cell with its
# `action_trace` lines removed. Comparing it unprojected is a control.
PROJECT_TRACE = True

# Which run is compared against which in the tick ladder, and in which direction. The shorter body
# is a prefix of the longer; reversing the direction is a control.
LADDER_DIRECTION = 1
LADDER = ("t1000", "t2000", "t5000", "t10000", "t20000")

# The released `social` capture is 1,000 ticks, so the ladder's first rung is one of its cells under
# a longer name and must reproduce it digest for digest. Comparing a different rung is a control.
PROVENANCE_TICKS = "t1000"


def territory_of(y: int) -> str:
    """`SPEC-MOK-001`'s split: A is `y <= SPLIT`, B is above it.

    A regenerated food's line carries its position and not its territory, so the reader derives
    the territory the same way the world does. Moving the split by one is a control, because the
    engine's own at-capacity count is what the derivation is checked against.
    """
    return "A" if y <= SPLIT else "B"


def parse(line: str):
    """`tick=<n> subject=<s> event=<e> result=<k>:<v>,...` into `(tick, subject, event, fields)`.

    Returns `None` for the summary line, which has a grammar of its own. Raises on anything else,
    because a line shape this reader does not recognize is a finding and not something to skip.
    """
    if line.startswith("summary "):
        return None
    head, _, result = line.partition(" result=")
    parts = head.split(" ")
    if len(parts) != 3 or not parts[0].startswith("tick="):
        raise ValueError(f"unrecognized line: {line}")
    tick = int(parts[0][len("tick=") :])
    subject = parts[1][len("subject=") :]
    event = parts[2][len("event=") :]
    fields = {}
    for item in result.split(","):
        key, _, value = item.partition(":")
        fields.setdefault(key, value)
    return tick, subject, event, fields


def summary_fields(line: str) -> dict:
    """The summary line's `key=value` pairs, rule 18's own order preserved."""
    fields = {}
    for item in line.split(" ")[1:]:
        key, _, value = item.partition("=")
        fields[key] = value
    return fields


def position_of(field: str) -> tuple:
    """`position:57:105` reaches this as the value `57`, so the y is read from the raw line."""
    x, _, y = field.partition(":")
    return int(x), int(y)


class Run:
    """One captured stream, reconstructed. Every figure below is read from the stream."""

    def __init__(self, directory: str, cell: str):
        self.cell = cell
        self.path = os.path.join(directory, f"{cell}.txt")
        with open(self.path, "rb") as handle:
            raw = handle.read()
        self.bytes = len(raw)
        self.lines = raw.count(NEWLINE)
        self.digest = hashlib.sha256(raw).hexdigest()
        self.body = raw
        with open(os.path.join(directory, f"{cell}.exit"), "r", encoding="utf-8") as handle:
            self.exit_code = int(handle.read().strip())
        self.failures: list[str] = []
        self._read(raw.decode("utf-8").splitlines())

    def _read(self, lines: list[str]) -> None:
        self.summary = summary_fields(lines[-1]) if lines[-1].startswith("summary ") else None
        self.ended = None
        self.last_tick = 0

        living: set[str] = set()
        died: dict[str, int] = {}
        initialized: set[str] = set()
        self.population_series: list[int] = []
        self.after_death: list[str] = []

        # id -> (territory, class); a consumed id leaves the collection and never returns.
        food: dict[str, tuple] = {}
        self.food_high = {name: 0 for name in TERRITORIES}
        self.food_low = {name: None for name in TERRITORIES}
        self.food_ids = 0
        self.id_high = 0
        self.id_regressions = 0
        self.capacity_lines = 0
        self.consumed = 0
        self.regenerated = 0
        self.skipped = 0

        self.suffered_lines = 0
        self.suffered_widest = 0
        self.trace_lines = 0

        # tick -> the subjects that took a decision opportunity in it, in the order they took it.
        # One line per living Mokiterion per tick, so this is the population curve counted a second
        # way. tick -> the (striker, victim) pairs of that tick's lethal strikes, which is what says
        # which of the start-of-tick population never reached its opportunity.
        self.opportunities: dict[int, list] = {}
        self.lethal: dict[int, list] = {}
        self.lost: list[str] = []

        # Rule 13 is one death path, so the stream reports no cause. `attack_resolved`'s own
        # `target_died` field is what makes a combat death recoverable, and the remainder is
        # rule 12's starvation.
        self.combat_deaths = 0
        self.death_ticks: list[int] = []

        def live(name: str) -> int:
            return sum(1 for value in food.values() if value[0] == name)

        for number, line in enumerate(lines, start=1):
            if not line:
                continue
            parsed = parse(line)
            if parsed is None:
                continue
            tick, subject, event, fields = parsed
            if self.ended is not None:
                self.failures.append(f"{number}: {event} follows simulation_ended")
            self.last_tick = max(self.last_tick, tick)

            if event == "agent_initialized":
                if subject in initialized:
                    self.failures.append(f"{number}: {subject} initialized twice")
                initialized.add(subject)
                living.add(subject)
            elif event == "agent_died":
                if subject in died:
                    self.failures.append(f"{number}: {subject} died twice")
                died[subject] = tick
                living.discard(subject)
                self.population_series.append(len(living))
                self.death_ticks.append(tick)
            elif event == "attack_resolved":
                if fields.get("target_died") == "yes":
                    self.combat_deaths += 1
                    self.lethal.setdefault(tick, []).append((subject, fields["target"]))
            elif event == "food_initialized":
                food[subject] = (fields["territory"], fields["class"])
                self.food_ids += 1
                self.id_high = self._id(subject, number)
            elif event == "food_consumed":
                identifier = fields["food"]
                if identifier not in food:
                    self.failures.append(f"{number}: consumed {identifier}, not in the collection")
                else:
                    del food[identifier]
                self.consumed += 1
            elif event == "food_regenerated":
                identifier = fields["food"]
                _, y = position_of(fields["position"])
                name = territory_of(y)
                if identifier in food:
                    self.failures.append(f"{number}: regenerated {identifier}, already held")
                food[identifier] = (name, fields["class"])
                self.regenerated += 1
                self.food_ids += 1
                value = self._id(identifier, number)
                if value <= self.id_high:
                    self.id_regressions += 1
                self.id_high = max(self.id_high, value)
                held = live(name)
                self.food_high[name] = max(self.food_high[name], held)
                if held > PER_TERRITORY:
                    self.failures.append(f"{number}: territory {name} holds {held} food")
            elif event == "food_regeneration_skipped":
                self.skipped += 1
                self.capacity_lines += 1
                held = live(subject)
                stated = int(fields["count"])
                if held != stated:
                    self.failures.append(
                        f"{number}: territory {subject} at capacity states {stated}, holds {held}"
                    )
                if stated != PER_TERRITORY:
                    self.failures.append(f"{number}: capacity stated as {stated}")
            elif event == "survival_changed":
                seen = self.opportunities.setdefault(tick, [])
                if subject in seen:
                    self.failures.append(f"{number}: {subject} decides twice in tick {tick}")
                seen.append(subject)
            elif event == "action_trace":
                self.trace_lines += 1
                if "suffered" in line:
                    entries = line.rpartition(",suffered:")[2].split(";")
                    self.suffered_lines += 1
                    self.suffered_widest = max(self.suffered_widest, len(entries))
                    if len(entries) > WINDOW_BOUND:
                        self.failures.append(f"{number}: {len(entries)} entries in one window")
            elif event == "simulation_ended":
                self.ended = (tick, fields["reason"])

            if subject in died and event != "agent_died" and tick > died[subject]:
                self.after_death.append(f"{number}: {subject} acts after dying")

            # The high and low water marks of each territory's collection, tracked after
            # initialization: during tick 0 territory B is empty while A is being filled, and a
            # minimum of zero there would be an artifact of the fill order rather than a figure.
            if tick > 0 and event.startswith("food_"):
                for name in TERRITORIES:
                    held = live(name)
                    self.food_high[name] = max(self.food_high[name], held)
                    if self.food_low[name] is None or held < self.food_low[name]:
                        self.food_low[name] = held

        self.failures.extend(self.after_death)
        self.living = sorted(living)
        self.deaths = len(died)
        self.died_at = died
        self.initialized = sorted(initialized)
        self.opportunity_total = sum(len(value) for value in self.opportunities.values())
        self.final_food = {
            (name, kind): sum(1 for value in food.values() if value == (name, kind))
            for name in TERRITORIES
            for kind in CLASSES
        }
        self._reconcile()

    def _id(self, identifier: str, number: int) -> int:
        if not identifier.startswith("F"):
            self.failures.append(f"{number}: food identifier {identifier}")
            return 0
        return int(identifier[1:])

    def _reconcile(self) -> None:
        """The summary line against what the reader counted, field by field."""
        if self.summary is None:
            self.failures.append("no summary line")
            return
        if self.ended is None:
            self.failures.append("no simulation_ended line")
            return
        expected = {
            "reason": self.ended[1],
            "ticks": str(self.ended[0]),
            "survivors": str(len(self.living)),
            "deaths": str(self.deaths),
        }
        for name in TERRITORIES:
            for kind in CLASSES:
                expected[f"food_{name.lower()}_{kind}"] = str(self.final_food[(name, kind)])
        for key, value in expected.items():
            if self.summary.get(key) != value:
                self.failures.append(
                    f"summary {key}={self.summary.get(key)}, the stream gives {value}"
                )
        if len(self.living) + self.deaths != AGENTS:
            self.failures.append(
                f"{len(self.living)} survivors and {self.deaths} deaths is not {AGENTS}"
            )
        by_territory = int(self.summary["territory_a"]) + int(self.summary["territory_b"])
        if by_territory != len(self.living):
            self.failures.append(
                f"territory_a + territory_b is {by_territory}, survivors is {len(self.living)}"
            )
        if self.id_regressions:
            self.failures.append(f"{self.id_regressions} food identifiers reused or out of order")
        # The engine's food counter is monotone and never reused, so the highest identifier issued
        # is the number issued. This is the one retained scalar that grows with the run, and it is
        # a counter rather than a collection: nothing holds an entry per identifier.
        if self.id_high != self.food_ids:
            self.failures.append(
                f"the highest food identifier is {self.id_high} and {self.food_ids} were issued"
            )
        if self.exit_code != 0:
            self.failures.append(f"exit {self.exit_code}")
        series = self.population_series
        if any(later > earlier for earlier, later in zip(series, series[1:])):
            self.failures.append("the population rose")
        if self.combat_deaths > self.deaths:
            self.failures.append(
                f"{self.combat_deaths} combat deaths and {self.deaths} agent_died lines"
            )
        self._opportunities()

    def _opportunities(self) -> None:
        """Every living Mokiterion decides once per tick, checked tick by tick.

        The summary line's survivor count is one number at the end of the run. This is the same
        population read at every tick of it: the subjects of tick `t`'s `survival_changed` lines
        must be exactly those alive at its start, less the one exception the stream itself shows --
        a Mokiterion a lower identifier killed in that tick, which never reaches its own
        opportunity. No summary field can confirm this, and it fails if a tick is skipped, if a
        dead Mokiterion decides, or if the reconstructed curve and the stream's own
        per-opportunity lines disagree anywhere.

        A missing tick is a failure rather than an absence: the run reached `last_tick`, so every
        tick from 1 to it had a population to act.
        """
        self.lost = []
        found_here = 0
        for tick in range(1, self.last_tick + 1):
            expected = {
                name
                for name in self.initialized
                if name not in self.died_at or self.died_at[name] >= tick
            }
            # Identifiers are `M` and two digits, so comparing them as text is comparing them as
            # numbers; a run that named a Mokiterion any other way fails the width check below.
            forfeited = {
                victim
                for striker, victim in self.lethal.get(tick, ())
                if (striker < victim) == (STRIKE_ORDER == 1)
            }
            for striker, victim in self.lethal.get(tick, ()):
                if len(striker) != len(victim):
                    self.failures.append(f"tick {tick}: {striker} and {victim} differ in width")
            expected -= forfeited
            self.lost.extend(f"{victim} in tick {tick}" for victim in sorted(forfeited))
            found = self.opportunities.get(tick, [])
            if set(found) != expected:
                self.failures.append(
                    f"tick {tick}: {len(found)} decided, {len(expected)} were alive and able"
                )
                found_here += 1
            elif found != sorted(found, reverse=DECISION_ORDER != 1):
                self.failures.append(f"tick {tick}: decided out of identifier order")
                found_here += 1
            # Ten thousand ticks can disagree ten thousand times, and a control that perturbs this
            # check does. The cut is on this method's own count so that a control elsewhere, which
            # leaves every tick agreeing, does not collect a truncation notice it did not earn.
            if found_here > 40:
                self.failures.append("further opportunity disagreements not listed")
                return
        # A trace-on stream emits one `action_trace` per opportunity, so its count is the same
        # figure reached a third way. Trace-off streams emit none and are not compared.
        if self.trace_lines and self.trace_lines != self.opportunity_total:
            self.failures.append(
                f"{self.trace_lines} action_trace lines and {self.opportunity_total} opportunities"
            )

    @property
    def reason(self) -> str:
        return self.ended[1] if self.ended else "--"

    @property
    def ticks(self) -> int:
        return self.ended[0] if self.ended else -1


def cells(directory: str) -> list[str]:
    names = sorted(
        name[: -len(".txt")] for name in os.listdir(directory) if name.endswith(".txt")
    )
    if not names:
        raise SystemExit(f"no streams in {directory}")
    return names


def ladder_failures(runs: dict) -> list[str]:
    """Each shorter body is a prefix of the longer one, in `LADDER_DIRECTION`'s order."""
    found = []
    ordered = [f"seed0-social-d0.75-{step}-traceoff" for step in LADDER]
    present = [name for name in ordered if name in runs]
    for shorter, longer in zip(present, present[1:]):
        first, second = (shorter, longer) if LADDER_DIRECTION == 1 else (longer, shorter)
        body = trim(runs[first].body)
        if not runs[second].body.startswith(body):
            found.append(f"{first} body is not a prefix of {second}")
    return found


def projected(raw: bytes) -> bytes:
    """A trace-on stream without its `action_trace` lines.

    `SPEC-MOK-001` rule 7 fixes that trace configuration "never changes entropy consumption or
    simulation state". The strong form of that claim is available at no cost here: removing the
    added lines must give back the trace-off stream of the same world, byte for byte, over every
    tick of the longest run rather than over a constructed encounter.
    """
    if not PROJECT_TRACE:
        return raw
    return b"".join(
        line + NEWLINE for line in raw.split(NEWLINE) if b" event=action_trace " not in line
    ).removesuffix(NEWLINE)


def trace_failures(runs: dict) -> list[str]:
    """Each trace-on cell against the trace-off cell of the same seed and tick count."""
    found = []
    for name in sorted(runs):
        if not name.endswith("-traceon"):
            continue
        partner = name[: -len("-traceon")] + "-traceoff"
        if partner not in runs:
            found.append(f"{name} has no trace-off partner")
            continue
        if projected(runs[name].body) != runs[partner].body:
            found.append(f"{name} projected is not {partner}")
    return found


def trim(raw: bytes) -> bytes:
    """A stream without its `simulation_ended` and `summary` lines: the part two runs share."""
    return raw.rsplit(b"\n", 3)[0] + b"\n"


def manifest_rows(path: str) -> dict:
    """`<cell> <sha256> <bytes> <lines> <exit>`, the shape `baseline/manifest.py` writes."""
    rows = {}
    with open(path, encoding="utf-8", newline="") as handle:
        for line in handle:
            parts = line.split()
            if len(parts) == 5 and len(parts[1]) == 64:
                rows[parts[0]] = (parts[1], int(parts[2]), int(parts[3]), parts[4])
    if not rows:
        raise SystemExit(f"no digest rows in {path}")
    return rows


def provenance_failures(runs: dict, path: str) -> tuple:
    """The ladder's 1,000-tick rung against the released capture's own digest.

    `VER-MOK-012`'s released `social` capture is 1,000 ticks at these seeds and densities, so a
    horizon cell at the same tick count is the same cell under a longer name. It must reproduce it
    byte for byte, and if it does not, the binary this capture was taken with is not the binary the
    released figures describe -- which would make every figure in this file provenance-less rather
    than merely different.
    """
    rows = manifest_rows(path)
    found, checked = [], []
    for name in sorted(runs):
        if f"-{PROVENANCE_TICKS}-" not in name:
            continue
        released = name.replace(f"-{PROVENANCE_TICKS}", "")
        if released not in rows:
            found.append(f"{released} is not in {os.path.basename(path)}")
            continue
        digest = rows[released][0]
        agrees = digest == runs[name].digest
        checked.append((name, released, agrees))
        if not agrees:
            found.append(f"{name} does not reproduce {released}")
    if not checked and not found:
        found.append(f"no cell at {PROVENANCE_TICKS} to compare against the manifest")
    return found, checked


def check(directory: str, manifest: str = "") -> tuple:
    runs = {name: Run(directory, name) for name in cells(directory)}
    failures = []
    for name in sorted(runs):
        failures.extend(f"{name}: {item}" for item in runs[name].failures)
    failures.extend(ladder_failures(runs))
    failures.extend(trace_failures(runs))
    if manifest:
        failures.extend(provenance_failures(runs, manifest)[0])
    return runs, failures


def report(directory: str, manifest: str = "") -> int:
    runs, failures = check(directory, manifest)
    ordered = sorted(runs)

    print("1. The capture, by cell")
    print("   cell  sha256(raw)  bytes  lines  exit")
    print()
    for name in ordered:
        run = runs[name]
        print(f"   {name:38} {run.digest} {run.bytes:>9} {run.lines:>7} {run.exit_code}")

    print()
    print("2. Completion")
    print(f"   {'cell':44} {'exit':>4} {'reason':>11} {'ticks':>6} {'lines':>7} {'bytes':>9}")
    for name in ordered:
        run = runs[name]
        print(
            f"   {name:44} {run.exit_code:>4} {run.reason:>11} {run.ticks:>6} "
            f"{run.lines:>7} {run.bytes:>9}"
        )

    print()
    print("3. Survivors and composition, from rule 18's own summary")
    print(f"   {'cell':44} {'surv':>4} {'died':>4} {'terr_a':>6} {'terr_b':>6} {'food A':>14} {'food B':>14}")
    for name in ordered:
        run = runs[name]
        summary = run.summary
        food_a = "/".join(summary[f"food_a_{kind}"] for kind in CLASSES)
        food_b = "/".join(summary[f"food_b_{kind}"] for kind in CLASSES)
        print(
            f"   {name:44} {summary['survivors']:>4} {summary['deaths']:>4} "
            f"{summary['territory_a']:>6} {summary['territory_b']:>6} {food_a:>14} {food_b:>14}"
        )
    print(f"   food columns are low/medium/high; capacity is {PER_TERRITORY} per territory")

    print()
    print("4. Deaths, by cause and by tick")
    print(
        f"   {'cell':44} {'died':>4} {'combat':>6} {'starved':>7} {'first':>5} {'last':>5} "
        f"{'no turn':>7}"
    )
    for name in ordered:
        run = runs[name]
        first = run.death_ticks[0] if run.death_ticks else 0
        last = run.death_ticks[-1] if run.death_ticks else 0
        print(
            f"   {name:44} {run.deaths:>4} {run.combat_deaths:>6} "
            f"{run.deaths - run.combat_deaths:>7} {first:>5} {last:>5} {len(run.lost):>7}"
        )
    print("   combat is attack_resolved's own target_died:yes; the remainder is rule 12's decay.")
    print("   'no turn' counts the victims a lower identifier killed earlier in the same tick,")
    print("   which never reached their own decision opportunity in it:")
    for name in ordered:
        run = runs[name]
        if run.lost:
            print(f"   {name:44} {', '.join(run.lost)}")

    print()
    print("5. Retained state, reconstructed tick by tick")
    print(
        f"   {'cell':44} {'pop':>4} {'A high':>6} {'B high':>6} {'A low':>5} {'B low':>5} "
        f"{'at cap':>6} {'id high':>7} {'suff':>4} {'opps':>7}"
    )
    for name in ordered:
        run = runs[name]
        print(
            f"   {name:44} {len(run.living):>4} {run.food_high['A']:>6} {run.food_high['B']:>6} "
            f"{run.food_low['A']:>5} {run.food_low['B']:>5} {run.capacity_lines:>6} "
            f"{run.id_high:>7} {run.suffered_widest:>4} {run.opportunity_total:>7}"
        )
    print("   pop is the surviving population; 'at cap' counts food_regeneration_skipped lines,")
    print("   each carrying the engine's own count, checked against the reader's; 'opps' is the")
    print("   decision opportunities taken, checked per tick against the reconstructed population")

    print()
    print("6. Food, consumed against regenerated")
    print(f"   {'cell':44} {'consumed':>8} {'regen':>8} {'skipped':>8} {'ids':>6}")
    for name in ordered:
        run = runs[name]
        print(
            f"   {name:44} {run.consumed:>8} {run.regenerated:>8} {run.skipped:>8} "
            f"{run.food_ids:>6}"
        )

    print()
    print("7. The tick ladder: the same world, five limits")
    ordered_ladder = [f"seed0-social-d0.75-{step}-traceoff" for step in LADDER]
    present = [name for name in ordered_ladder if name in runs]
    for shorter, longer in zip(present, present[1:]):
        body = trim(runs[shorter].body)
        held = runs[longer].body.startswith(body)
        print(
            f"   {shorter.rsplit('-', 2)[1]:>7} body of {body.count(NEWLINE):>6} lines "
            f"is a prefix of {longer.rsplit('-', 2)[1]:<7} {'yes' if held else 'NO'}"
        )
    if "seed0-social-d0.75-t10000-traceoff" in runs and "seed0-social-d0.75-t20000-traceoff" in runs:
        identical = (
            runs["seed0-social-d0.75-t10000-traceoff"].body
            == runs["seed0-social-d0.75-t20000-traceoff"].body
        )
        print(f"   t10000 and t20000 are byte-identical: {'yes' if identical else 'NO'}")
    for name in sorted(runs):
        if name.endswith("-traceon"):
            partner = name[: -len("-traceon")] + "-traceoff"
            held = partner in runs and projected(runs[name].body) == runs[partner].body
            print(
                f"   traceon minus its {runs[name].lines - runs[partner].lines} action_trace "
                f"lines is traceoff, byte for byte: {'yes' if held else 'NO'}"
            )

    print()
    print("8. The suffered-attack record, where the stream shows it")
    for name in ordered:
        run = runs[name]
        if run.suffered_lines:
            print(
                f"   {name:44} {run.suffered_lines:>6} traces carry it, widest "
                f"{run.suffered_widest}, bound {WINDOW_BOUND}"
            )
            print(
                f"   {'':44} {run.trace_lines:>6} traces in all, against "
                f"{run.opportunity_total} decision opportunities"
            )
    print("   trace-off streams carry no such field and are not counted")

    if manifest:
        print()
        print("9. Provenance: the ladder's shortest rung against the released capture")
        for name, released, agrees in provenance_failures(runs, manifest)[1]:
            print(f"   {name} reproduces {released}: {'yes' if agrees else 'NO'}")
        print(f"   compared against {manifest}")

    print()
    if failures:
        print(f"RESULT: FAIL -- {len(failures)} disagreements")
        for item in failures[:40]:
            print(f"  {item}")
        if len(failures) > 40:
            print(f"  ... {len(failures) - 40} more")
        return 1
    print(
        f"RESULT: PASS -- {len(ordered)} runs complete, "
        f"{sum(run.lines for run in runs.values())} lines checked, no disagreement"
    )
    return 0


def main(argv: list[str]) -> int:
    if len(argv) < 2:
        print(__doc__)
        return 2
    directory = argv[1]
    manifest = argv[argv.index("--against") + 1] if "--against" in argv else ""
    if "--control" in argv:
        name = argv[argv.index("--control") + 1]
        if name not in CONTROLS:
            print(f"unknown control {name}; have {', '.join(sorted(CONTROLS))}")
            return 2
        target, value = CONTROLS[name]
        print(f"CONTROL {name}: {target} = {value!r}")
        globals()[target] = value
        if target == "AGENTS":
            globals()["WINDOW_BOUND"] = value - 1
    return report(directory, manifest)


if __name__ == "__main__":
    sys.exit(main(sys.argv))
