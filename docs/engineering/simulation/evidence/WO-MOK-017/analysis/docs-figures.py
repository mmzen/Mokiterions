"""WO-MOK-017: every figure this work order wrote into a reader-facing document, against its source.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/docs-figures.py \
        <pre-capture-dir> <post-capture-dir> <out-of-matrix-dir>

Writes its report to stdout and exits `0` when every claim it can check holds, non-zero otherwise.

WHAT THIS READER IS FOR
-----------------------
`WO-MOK-017`'s *In scope* names two reader-facing documents, `SIMULATION_RULES.md` and
`docs/ROADMAP.md`, and the change moves most of the numbers in them. A figure in a document that no
evidence file accounts for is the same defect this work order was raised to correct, one document
further out: `SPEC-MOK-001` rule 5 carried a wrong condition for weeks because nothing measured it,
and `SIMULATION_RULES.md` section 11 carried a decider the project had already stopped shipping
because nothing re-measured it when `7d744bb` landed.

So every figure this work order wrote, moved or deliberately left standing is listed here with:

  * the sentence it appears in, checked to be present in the document as written;
  * where it comes from -- this work order's declared matrix, a run outside that matrix, a sibling
    evidence file in this packet, or an earlier packet that this work order did not re-measure;
  * and, for everything derivable from a capture, the figure recomputed here from the streams.

FOUR KINDS OF CLAIM, AND WHAT EACH IS WORTH
-------------------------------------------
`matrix`   recomputed from the 120-cell captures at both commits. The strongest kind: the document's
           number is checked against the stream, not against another document.
`outside`  recomputed from a run outside the declared matrix. The document prints the command; this
           reader runs its output through the same arithmetic. Reproducible in one step by anybody,
           and not retained, for the reason `analysis/retain.py` gives about capture size.
`cited`    measured by a named sibling file in this packet, which this reader checks says what the
           document says rather than recomputing it. Avoids two readers of the same stream
           disagreeing about one figure.
`history`  measured on a world the engine no longer implements, and NOT re-measured. Every one of
           these is required to be flagged as history in the document itself, and the flag is part
           of the claim: a stale figure presented as current would fail here even though its
           arithmetic is nobody's to check.

WHAT IT DOES NOT CHECK
----------------------
It checks that a quoted sentence is present and that its digits agree with the streams. It does not
check English. Where a document spells a figure in words -- "fifty-six of the sixty-three strikes" --
the quote and the measured number are printed side by side and the reader confirms the spelling by
reading them, which is the one link in this file a script does not close.

It is also not a verification record and settles no obligation. `REQ-MOK-060`'s ceiling is
`post/composition.txt`, the floors are `post/survivors.txt`, and both are cited here rather than
re-derived.
"""

import hashlib
import io
import os
import re
import sys

SEEDS = ['0', '1', '42', '123', '777']
SOURCES = ['baseline', 'reference', 'individual', 'social']
DENSITIES = ['0.15', '0.75', '1.50']
DEFAULT = '0.75'
RESTORE = {'low': 15, 'medium': 30, 'high': 50}
ALLOWANCE = {c: r * r // 100 for c, r in RESTORE.items()}

DOC_KEYS = {'SR': 'SIMULATION_RULES.md', 'RM': 'docs/ROADMAP.md'}

RE_SUMMARY = re.compile(
    r'^summary reason=(\w+) ticks=(\d+) survivors=(\d+) deaths=(\d+) territory_a=(\d+) '
    r'territory_b=(\d+) food_a_low=(\d+) food_a_medium=(\d+) food_a_high=(\d+) '
    r'food_b_low=(\d+) food_b_medium=(\d+) food_b_high=(\d+)$')
RE_FEAR = re.compile(r'fear:(\d+)->(\d+)')
RE_VERB = re.compile(r'event=action_trace result=proposal:([a-z_]+)')
RE_TRACE_FEAR = re.compile(r',fear:(\d+)')
RE_TICK = re.compile(r'^tick=(\d+) subject=(\S+) ')
RE_DIED = re.compile(r'event=attack_resolved .*target_died:(yes|no)')
RE_THREAT = re.compile(r'event=threat_resolved result=target:(M\d+),increase:(\d+),'
                       r'target_fear:(\d+)->(\d+)')
RE_EAT = re.compile(r'^tick=(\d+) subject=(M\d+) event=food_consumed '
                    r'result=food:(F\d+),class:(\w+),satiety:(\d+)->(\d+)')
RE_INIT = re.compile(r'^tick=0 subject=(M\d+) event=agent_initialized result=name:(\w+),'
                     r'.*waste_tolerance:(\d+)$')
RE_FOOD_INIT = re.compile(r'^tick=0 subject=(F\d+) event=food_initialized '
                          r'result=class:(\w+),position:(\d+:\d+),')
RE_PROPOSAL = re.compile(r'result=proposal:([a-z_]+)(?::([^,]+))?(?:,target:(\S+?))?,'
                         r'status:(\w+),detail:')
RE_TRACE_POS = re.compile(r'detail:(?:position:(\d+:\d+)|food:\S+?,position:(\d+:\d+))')
RE_TRACE_SAT = re.compile(r',satiety:(\d+),')


# --------------------------------------------------------------------------- the claim ledger

CLAIMS = []


def norm(text):
    return re.sub(r'\s+', ' ', text).strip()


def fmt(value):
    if isinstance(value, (list, tuple)):
        return ', '.join(fmt(v) for v in value)
    if isinstance(value, bool):
        return 'yes' if value else 'no'
    if isinstance(value, float):
        return '%.1f' % value
    if isinstance(value, int):
        return '{:,}'.format(value)
    return str(value)


class Docs(object):
    def __init__(self, root):
        self.text = {}
        for key, rel in DOC_KEYS.items():
            path = os.path.join(root, rel)
            with io.open(path, encoding='utf-8', newline='') as handle:
                self.text[key] = norm(handle.read())

    def carries(self, key, quote):
        return norm(quote) in self.text[key]


DOCS = None
SIBLINGS = {}


def sibling(rel):
    if rel not in SIBLINGS:
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', rel)
        with io.open(path, encoding='utf-8', newline='') as handle:
            SIBLINGS[rel] = norm(handle.read())
    return SIBLINGS[rel]


def claim(kind, section, figure, doc, quote, written=None, measured=None, source='', note=''):
    """Record one figure. `written` is the document's number, `measured` this reader's."""
    quoted = DOCS.carries(doc, quote)
    agrees = True if written is None else (written == measured)
    CLAIMS.append(dict(kind=kind, section=section, figure=figure, doc=doc, quote=norm(quote),
                       written=written, measured=measured, source=source, note=note,
                       quoted=quoted, agrees=agrees))


def cited(section, figure, doc, quote, rel, needle):
    """A figure measured by a sibling file, checked to be stated there as the document states it."""
    present = norm(needle) in sibling(rel)
    CLAIMS.append(dict(kind='cited', section=section, figure=figure, doc=doc, quote=norm(quote),
                       written=needle, measured=needle if present else 'NOT FOUND in ' + rel,
                       source=rel, note='', quoted=DOCS.carries(doc, quote), agrees=present))


def history(section, figure, doc, quote, flag, source):
    """A figure this work order did not re-measure. The document must flag it as history."""
    CLAIMS.append(dict(kind='history', section=section, figure=figure, doc=doc, quote=norm(quote),
                       written=None, measured=None, source=source, note=flag,
                       quoted=DOCS.carries(doc, quote) and DOCS.carries(doc, flag), agrees=True))


# --------------------------------------------------------------------------- reading the captures

class Cell(object):
    """The aggregates of one capture cell. Read once, streamed, nothing held line by line."""

    def __init__(self, path, wanted):
        self.path = path
        self.summary = None
        self.fear = []
        self.verbs = {}
        self.verb_fear = {}
        self.strikes = []
        self.lethal = 0
        self.threats = 0
        self.threats_moved = []
        self.eats = []
        self.traits = {}
        self.food_at = {}
        self.deaths_at = {}
        self.found = set()
        self.m05 = []
        digest = hashlib.sha256()
        self.nontrace_lines = 0
        with io.open(path, encoding='utf-8', newline='') as handle:
            for line in handle:
                bare = line.rstrip('\n')
                if 'event=action_trace' in bare:
                    self._trace(bare)
                else:
                    digest.update(bare.encode('utf-8') + b'\n')
                    self.nontrace_lines += 1
                    self._record(bare)
                if wanted and bare in wanted:
                    self.found.add(bare)
                if bare.startswith('tick=') and ' subject=M05 ' in bare:
                    tick = int(RE_TICK.match(bare).group(1))
                    if tick <= 21:
                        self.m05.append(bare)
        self.nontrace_digest = digest.hexdigest()

    def _trace(self, line):
        match = RE_VERB.search(line)
        if not match:
            return
        verb = match.group(1).split(':')[0]
        self.verbs[verb] = self.verbs.get(verb, 0) + 1
        fear = RE_TRACE_FEAR.search(line)
        if fear:
            self.verb_fear.setdefault(verb, []).append(int(fear.group(1)))

    def _record(self, line):
        if line.startswith('summary '):
            match = RE_SUMMARY.match(line)
            if match:
                self.summary = dict(
                    reason=match.group(1), ticks=int(match.group(2)),
                    survivors=int(match.group(3)), deaths=int(match.group(4)),
                    food=[int(match.group(i)) for i in range(7, 13)])
            return
        if 'event=survival_changed' in line:
            match = RE_FEAR.search(line)
            if match:
                self.fear.append((int(match.group(1)), int(match.group(2))))
            return
        if 'event=attack_resolved' in line:
            match = RE_DIED.search(line)
            if match:
                self.strikes.append(int(RE_TICK.match(line).group(1)))
                if match.group(1) == 'yes':
                    self.lethal += 1
            return
        if 'event=threat_resolved' in line:
            match = RE_THREAT.search(line)
            if match:
                self.threats += 1
                if match.group(2) != '0':
                    self.threats_moved.append(line)
            return
        if 'event=food_consumed' in line:
            match = RE_EAT.match(line)
            if match:
                self.eats.append((int(match.group(1)), match.group(2), match.group(3),
                                  match.group(4), int(match.group(5))))
            return
        if 'event=agent_initialized' in line:
            match = RE_INIT.match(line)
            if match:
                self.traits[match.group(1)] = (match.group(2), int(match.group(3)))
            return
        if 'event=food_initialized' in line:
            match = RE_FOOD_INIT.match(line)
            if match:
                self.food_at[match.group(1)] = (match.group(2), match.group(3))
            return
        if 'event=agent_died' in line:
            tick = int(RE_TICK.match(line).group(1))
            self.deaths_at[tick] = self.deaths_at.get(tick, 0) + 1


def cell_name(seed, source, density, trace):
    return 'seed%s-%s-d%s-trace%s' % (seed, source, density, trace)


def load(capture_dir, names, wanted):
    cells = {}
    for name in names:
        path = os.path.join(capture_dir, name + '.txt')
        if not os.path.isfile(path):
            raise SystemExit('missing capture cell: %s' % path)
        cells[name] = Cell(path, wanted)
    return cells


# --------------------------------------------------------------------------- the verbatim records

QUOTED_RECORDS = [
    ('SR', 'seed42-reference-d0.75-traceon',
     'tick=0 subject=F0002 event=food_initialized result=class:medium,position:82:20,territory:A',
     'section 14, the food record'),
    ('SR', 'seed42-reference-d0.75-traceon',
     'tick=0 subject=M08 event=agent_initialized result=name:Nurb,position:62:104,territory:B,'
     'health:100,satiety:100,energy:100,fear:0,waste_tolerance:40',
     'section 14, the Mokiterion record'),
    ('SR', 'seed42-reference-d0.75-traceon',
     'tick=10 subject=A event=food_regeneration_skipped result=reason:capacity,count:61',
     'section 14, the refused regeneration'),
    ('SR', 'seed42-reference-d0.75-traceon',
     'tick=40 subject=A event=food_regenerated result=food:F0129,class:medium,position:56:30',
     'section 14, the regeneration'),
    ('SR', 'seed42-reference-d0.75-traceon',
     'tick=40 subject=M12 event=survival_changed result=health:100->100,satiety:91->90,'
     'energy:71->70,fear:100->100',
     'section 14, the survival record and the fear it carries'),
    ('SR', 'seed123-social-d0.75-traceon',
     'tick=1 subject=M11 event=attack_resolved result=target:M10,damage:30,'
     'target_health:100->70,striker_energy:100->95,target_died:no',
     'section 14, the strike'),
    ('SR', 'seed123-social-d1.50-traceon',
     'tick=337 subject=M02 event=threat_resolved result=target:M04,increase:5,'
     'target_fear:95->100',
     'section 14, the threat that partly landed'),
    ('SR', 'seed123-social-d0.75-traceon',
     'tick=7 subject=M06 event=surrender_resolved result=recipient:M04,transferred:7,'
     'discarded:40,subject_satiety:94->47,recipient_satiety:93->100',
     'section 14, the surrender'),
    ('SR', 'seed42-individual-d0.75-traceon',
     'tick=381 subject=M08 event=food_consumed result=food:F0257,class:low,satiety:88->100,'
     'energy:45->50',
     'section 11, the real difference the trait makes'),
]


# --------------------------------------------------------------------------- out-of-matrix runs

OUTSIDE_RUNS = [
    ('ref-seed0-d0.70',
     'cargo run --bin Mokiterions -- --seed 0 --policy reference --density 0.70 --ticks 1000'),
    ('ref-seed0-d0.75',
     'cargo run --bin Mokiterions -- --seed 0 --policy reference --density 0.75 --ticks 1000'),
    ('ref-seed0-d1.00',
     'cargo run --bin Mokiterions -- --seed 0 --policy reference --density 1.00 --ticks 1000'),
    ('ref-seed0-d1.25',
     'cargo run --bin Mokiterions -- --seed 0 --policy reference --density 1.25 --ticks 1000'),
    ('seed42-d0.02-t200',
     'cargo run --bin Mokiterions -- --seed 42 --ticks 200 --density 0.02'),
    ('seed42-t40',
     'cargo run --bin Mokiterions -- --seed 42 --ticks 40'),
    ('ind-seed42-t400',
     'cargo run --bin Mokiterions -- --policy individual --seed 42 --ticks 400'),
]


def main():
    global DOCS
    if len(sys.argv) != 4:
        raise SystemExit(__doc__.strip().splitlines()[2].strip())
    if hasattr(sys.stdout, 'reconfigure'):
        # The packet is uniformly UTF-8 and LF; Windows text-mode stdout is neither by default.
        sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    pre_dir, post_dir, extra_dir = sys.argv[1], sys.argv[2], sys.argv[3]
    root = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                        '..', '..', '..', '..', '..', '..'))
    DOCS = Docs(root)

    wanted = set(record for _, _, record, _ in QUOTED_RECORDS)

    names = [cell_name(s, src, DEFAULT, t)
             for s in SEEDS for src in SOURCES for t in ('on', 'off')]
    names += [cell_name(s, 'social', d, t)
              for s in SEEDS for d in DENSITIES if d != DEFAULT for t in ('on', 'off')]
    pre = load(pre_dir, names, wanted)
    post = load(post_dir, names, wanted)

    out = []

    def emit(line=''):
        out.append(line)

    def survivors(cells, source, density=DEFAULT):
        return [cells[cell_name(s, source, density, 'on')].summary['survivors'] for s in SEEDS]

    def deaths(cells, source, density=DEFAULT):
        return [cells[cell_name(s, source, density, 'on')].summary['deaths'] for s in SEEDS]

    def lethal(cells, source, density=DEFAULT):
        return [cells[cell_name(s, source, density, 'on')].lethal for s in SEEDS]

    def verb_total(cells, verb, source='social', density=DEFAULT):
        return sum(cells[cell_name(s, source, density, 'on')].verbs.get(verb, 0) for s in SEEDS)

    def verb_fears(cells, verb, source='social', density=DEFAULT):
        got = []
        for s in SEEDS:
            got.extend(cells[cell_name(s, source, density, 'on')].verb_fear.get(verb, []))
        return got

    def fear_rows(cells, sources):
        rows = []
        for src in sources:
            for s in SEEDS:
                rows.extend(cells[cell_name(s, src, DEFAULT, 'on')].fear)
        return rows

    def high_shares(cells, sources):
        shares, worst_any = [], []
        for src in sources:
            for s in SEEDS:
                food = cells[cell_name(s, src, DEFAULT, 'on')].summary['food']
                for half in (food[0:3], food[3:6]):
                    total = sum(half)
                    if total:
                        shares.append(100.0 * half[2] / total)
                        worst_any.append(100.0 * max(half) / total)
        return shares, worst_any

    def refused(cells, allowance, source='individual'):
        """Meals the reference rule *of that commit* would have refused, per declared seed."""
        counts, actors = [], []
        for s in SEEDS:
            cell = cells[cell_name(s, source, DEFAULT, 'on')]
            hit, who = 0, set()
            for _, subject, _, klass, before in cell.eats:
                spill = before + RESTORE[klass] - 100
                if spill > allowance[klass]:
                    hit += 1
                    who.add(subject)
            counts.append(hit)
            actors.append(len(who))
        return counts, actors

    # ------------------------------------------------------------------ header

    emit('WO-MOK-017 documentation figures: every number written into a reader-facing document,')
    emit('against the stream, the sibling file or the earlier packet it comes from')
    emit('=' * 98)
    emit()
    emit('Work order   WO-MOK-017 (the resource composition drift)')
    emit('In scope     "`SIMULATION_RULES.md` and `docs/ROADMAP.md` brought back into agreement')
    emit('             with the corrected world"')
    emit('Documents    SIMULATION_RULES.md, docs/ROADMAP.md')
    emit('Pre-change   pre/COMMIT.txt')
    emit('Candidate    post/COMMIT.txt')
    emit('Reader       analysis/docs-figures.py')
    emit('Date         2026-08-21')
    emit()
    emit('Neither document is a governed artifact and neither is compiled by anything, which is')
    emit('exactly why their figures need a record of their own: nothing else in this repository')
    emit('would fail if one of them were wrong. `SIMULATION_RULES.md` section 11 carried a decider')
    emit('the project had stopped shipping for a day and a half because commit 7d744bb changed the')
    emit('social branch order and nobody re-measured the prose. This file is the answer to that.')
    emit()
    emit('Every claim below is one figure. `matrix` means recomputed here from the 120-cell')
    emit('captures; `outside` from a run the document itself prints the command for; `cited` from a')
    emit('named sibling file in this packet, checked to say what the document says; `history` from a')
    emit('world the engine no longer implements, not re-measured, and required to be flagged as')
    emit('history in the document itself.')
    emit()

    # ------------------------------------------------------------------ 1. internal checks

    emit()
    emit('1. What was read, and the two checks that make the reading usable')
    emit('-' * 66)
    pairs_ok = 0
    for name in names:
        if name.endswith('-traceon'):
            other = name[:-len('traceon')] + 'traceoff'
            if (pre[name].nontrace_digest == pre[other].nontrace_digest
                    and post[name].nontrace_digest == post[other].nontrace_digest):
                pairs_ok += 1
    total_pairs = len(names) // 2
    emit('    cells read at the pre-change commit                              %3d' % len(pre))
    emit('    cells read at the candidate                                      %3d' % len(post))
    emit('    triples whose traced and untraced halves carry a byte-identical')
    emit('    non-trace record stream, at both commits                       %3d of %d'
         % (pairs_ok, total_pairs))
    emit()
    emit('    summaries present, one per cell, at both commits               %3d of %d'
         % (sum(1 for c in list(pre.values()) + list(post.values()) if c.summary),
            len(pre) + len(post)))
    emit()
    emit('The 60 cells per commit are the default-density matrix under all four sources, plus the')
    emit('other two densities under `social` for the threat figure of section 6. The rest of the')
    emit('matrix is not read here: no figure in either document is taken from it, and the two files')
    emit('that do read all 120 are `post/composition.txt` and `post/divergence.txt`.')
    emit()
    emit('The first check is what lets the traced half stand for the run. Every figure below that is')
    emit('not a proposal count comes from records both halves print, and they print them identically:')
    emit('the traced half is the untraced half with `action_trace` interleaved and nothing else')
    emit('moved. A figure read off a traced cell is therefore a figure about the run, not about the')
    emit('instrumentation.')

    # ------------------------------------------------------------------ 2. survivors

    ref, ind, soc = (survivors(post, s) for s in ('reference', 'individual', 'social'))
    pre_ref, pre_ind, pre_soc = (survivors(pre, s) for s in ('reference', 'individual', 'social'))
    soc_deaths, soc_lethal = deaths(post, 'social'), lethal(post, 'social')

    claim('matrix', 2, 'survivors under the two food-only deciders, range', 'SR',
          '**8 to 10 of the 12**', [8, 10], [min(ref + ind), max(ref + ind)])
    claim('matrix', 2, 'survivors, reference, per declared seed', 'SR',
          '8, 9, 10, 9 and 9 across the five seeds', [8, 9, 10, 9, 9], ref)
    claim('matrix', 2, 'survivors, trait-aware, per declared seed', 'SR',
          '10, 8, 10, 8 and 8 under the trait-aware decider', [10, 8, 10, 8, 8], ind)
    claim('matrix', 2, 'the section 11 survivor table, trait-aware row', 'SR',
          '| survivors, trait-aware | **10** | 8 | 10 | 8 | 8 |', [10, 8, 10, 8, 8], ind)
    claim('matrix', 2, 'the section 11 survivor table, reference row', 'SR',
          '| survivors, reference | 8 | 9 | 10 | 9 | 9 |', [8, 9, 10, 9, 9], ref)
    claim('matrix', 2, 'trait-aware against reference, seed by seed', 'SR',
          'it wins on seed 0, ties on seed 42 and loses on the other three',
          [1, 1, 3], [sum(1 for a, b in zip(ind, ref) if a > b),
                      sum(1 for a, b in zip(ind, ref) if a == b),
                      sum(1 for a, b in zip(ind, ref) if a < b)])
    claim('matrix', 2, 'both food-only deciders against the floor of eight', 'SR',
          'Both deciders clear their floor of eight on every seed', 10,
          sum(1 for v in ref + ind if v >= 8))
    claim('matrix', 2, 'seeds sitting exactly on the floor', 'SR',
          'the reference decider is exactly on it on seed 0, and the trait-aware one on three of '
          'the five seeds', [1, 3],
          [sum(1 for v in ref if v == 8), sum(1 for v in ind if v == 8)])
    claim('matrix', 2, 'seed 777 under the trait-aware decider, both commits', 'SR',
          'The seed-777 run that used to keep all twelve alive', [12, 8],
          [pre_ind[4], ind[4]])
    claim('matrix', 2, 'seed 777, how many it now loses', 'SR', 'now loses four', 4, 12 - ind[4])
    claim('matrix', 2, 'survivors, social, range', 'SR',
          '**With that decider, 7 to 9 survive', [7, 9], [min(soc), max(soc)])
    claim('matrix', 2, 'survivors, social, per declared seed', 'SR',
          '**9, 7, 9, 8 and 9 survivors**', [9, 7, 9, 8, 9], soc)
    claim('matrix', 2, 'deaths under social, and how many are killings', 'SR',
          'Nine of the eighteen deaths across its five runs are caused by combat',
          [18, 9], [sum(soc_deaths), sum(soc_lethal)])
    claim('matrix', 2, 'combat deaths, social, per declared seed', 'SR',
          '**1, 2, 2, 3 and 1** combat deaths', [1, 2, 2, 3, 1], soc_lethal)
    claim('matrix', 2, 'the reproducibility example', 'SR',
          'a claim like "10 survived on seed 42"', 10, ref[2])
    claim('matrix', 2, 'survivors, social, candidate', 'RM',
          '9, 7, 9, 8 and 9 of twelve alive', [9, 7, 9, 8, 9], soc)
    claim('matrix', 2, 'combat deaths, social, candidate', 'RM',
          'with 1, 2, 2, 3 and 1 of the deaths attributable to combat', [1, 2, 2, 3, 1], soc_lethal)
    claim('matrix', 2, "survivors at WO-MOK-016's candidate, which is this packet's pre-change "
          'commit', 'RM', 'Measured at the resulting candidate: 9, 10, 9, 9 and 11 survivors',
          [9, 10, 9, 9, 11], pre_soc)
    claim('matrix', 2, 'combat deaths at the pre-change commit', 'RM',
          'combat deaths 1, 2, 2, 3 and 1', [1, 2, 2, 3, 1], lethal(pre, 'social'))
    claim('matrix', 2, 'surrender proposals at the pre-change commit', 'RM',
          '`surrender` proposed 5, 10, 8, 6 and 7 times', [5, 10, 8, 6, 7],
          [pre[cell_name(s, 'social', DEFAULT, 'on')].verbs.get('surrender', 0) for s in SEEDS])

    pre_meals = [len(pre[cell_name(s, 'social', DEFAULT, 'on')].eats) for s in SEEDS]
    claim('matrix', 2, 'meals eaten under social at the pre-change commit', 'RM',
          'meals recovered to 326–389', [326, 389], [min(pre_meals), max(pre_meals)])

    obligated = [(src, s) for src in ('reference', 'individual', 'social') for s in SEEDS]
    fewer = sum(1 for src, s in obligated
                if post[cell_name(s, src, DEFAULT, 'on')].summary['survivors']
                < pre[cell_name(s, src, DEFAULT, 'on')].summary['survivors'])
    claim('matrix', 2, 'obligated runs that lose survivors to the correction', 'RM',
          'nine of the fifteen obligated runs at the default density leave *fewer* Mokiterions '
          'alive', [9, 15], [fewer, len(obligated)])
    deltas = [a - b for a, b in zip(soc, pre_soc)]
    claim('matrix', 2, 'the cost to social, seed by seed', 'SR',
          'unchanged on two seeds and one to three lower on the other three',
          [2, 1, 3], [sum(1 for d in deltas if d == 0),
                      min(-d for d in deltas if d < 0), max(-d for d in deltas if d < 0)])
    claim('matrix', 2, "the worst seed against REQ-MOK-058's floor of five", 'RM',
          "`REQ-MOK-058`'s five is met by two", 2, min(soc) - 5)

    control = post[cell_name('42', 'baseline', DEFAULT, 'on')]
    claim('matrix', 2, 'the control case, and the larder it died beside', 'SR',
          'all twelve are dead by tick **142** — with 122 pieces of food still lying on the ground',
          [12, 142, 122],
          [control.summary['deaths'], max(control.deaths_at), sum(control.summary['food'])])
    claim('matrix', 2, 'both territories at capacity when it died', 'SR',
          'both territories completely full', [61, 61],
          [sum(control.summary['food'][0:3]), sum(control.summary['food'][3:6])])

    emit()
    emit()
    emit('2. Survivors and deaths at the default density, both commits')
    emit('-' * 61)
    emit('    source      seeds ordered 0, 1, 42, 123, 777')
    for label, before, after in (('reference', pre_ref, ref), ('individual', pre_ind, ind),
                                 ('social', pre_soc, soc)):
        emit('    %-11s pre  %-18s candidate  %s'
             % (label, fmt(before), fmt(after)))
    emit('    baseline    pre  %-18s candidate  %s'
         % (fmt(survivors(pre, 'baseline')), fmt(survivors(post, 'baseline'))))
    emit()
    emit('    the control case of section 11, seed 42: %d dead by tick %d, with %d pieces of food'
         % (control.summary['deaths'], max(control.deaths_at), sum(control.summary['food'])))
    emit('    still standing, %d in territory A and %d in territory B, both at the capacity of %d'
         % (sum(control.summary['food'][0:3]), sum(control.summary['food'][3:6]),
            sum(control.summary['food'][0:3])))
    emit()
    emit('    social deaths, candidate            %-18s total %d'
         % (fmt(soc_deaths), sum(soc_deaths)))
    emit('    of which lethal strikes             %-18s total %d'
         % (fmt(soc_lethal), sum(soc_lethal)))
    emit()
    emit('A death is counted as combat when the strike that produced it printed `target_died:yes`,')
    emit('which is `post/survivors.txt`\'s rule and not a second one invented here. That file also')
    emit('reconciles it against adjacency, cell by cell, and this reader defers to it.')

    # ------------------------------------------------------------------ 3. the action table

    verbs = ['avoid', 'approach', 'threaten', 'attack', 'surrender', 'retreat', 'fight']
    table = [(v, verb_total(pre, v), verb_total(post, v)) for v in verbs]
    written_table = {'avoid': (3842, 4268), 'approach': (1788, 1724), 'threaten': (128, 114),
                     'attack': (59, 54), 'surrender': (36, 31), 'retreat': (14, 14),
                     'fight': (9, 9)}
    quotes = {'avoid': '| avoid | 3,842 | 4,268 |', 'approach': '| approach | 1,788 | 1,724 |',
              'threaten': '| threaten | 128 | 114 |', 'attack': '| attack | 59 | 54 |',
              'surrender': '| **surrender** | 36 | **31** |', 'retreat': '| retreat | 14 | 14 |',
              'fight': '| fight | 9 | 9 |'}
    for verb, before, after in table:
        claim('matrix', 3, 'the section 11 action table, %s row' % verb, 'SR', quotes[verb],
              list(written_table[verb]), [before, after])

    strikes_pre = sum(len(pre[cell_name(s, 'social', DEFAULT, 'on')].strikes) for s in SEEDS)
    strike_ticks = [t for s in SEEDS
                    for t in post[cell_name(s, 'social', DEFAULT, 'on')].strikes]
    claim('matrix', 3, 'strikes across the five social runs', 'SR',
          'Sixty-three strikes across five complete runs', 63, len(strike_ticks))
    claim('matrix', 3, 'strikes landing inside the first fourteen turns', 'SR',
          '**fifty-six of them land inside the first fourteen turns**', 56,
          sum(1 for t in strike_ticks if t <= 14))
    claim('matrix', 3, 'strikes after tick 14, and the last of them', 'SR',
          'The other seven are scattered as far out as turn 882', [7, 882],
          [sum(1 for t in strike_ticks if t > 14), max(strike_ticks)])
    claim('matrix', 3, 'the same two figures in section 1', 'SR',
          'fifty-six of the sixty-three strikes land inside the first fourteen turns',
          [56, 63], [sum(1 for t in strike_ticks if t <= 14), len(strike_ticks)])
    claim('matrix', 3, 'the turns left after the opening', 'SR',
          'for the remaining nine hundred and eighty-six', 986, 1000 - 14)
    claim('matrix', 3, 'the same figures in section 16', 'SR',
          '63 strikes across five 1,000-turn runs, of which 56 land inside the first fourteen '
          'turns, and nine of the eighteen deaths are caused by combat',
          [63, 56, 9, 18], [len(strike_ticks), sum(1 for t in strike_ticks if t <= 14),
                            sum(soc_lethal), sum(soc_deaths)])
    claim('matrix', 3, 'avoidances against strikes, to the nearest ten', 'SR',
          '`avoid` outnumbers the strikes about seventy to one', 70,
          int(round(verb_total(post, 'avoid') / float(len(strike_ticks)), -1)))

    emit()
    emit()
    emit('3. Every targeted action the social decider proposed, both commits')
    emit('-' * 66)
    emit('    verb        pre-change   candidate     the document prints')
    for verb, before, after in table:
        emit('    %-11s %8s   %9s     %s' % (verb, fmt(before), fmt(after), quotes[verb]))
    emit()
    emit('    strikes, pre-change %d   candidate %d   (attack + fight, and equal to the count of')
    out[-1] = out[-1] % (strikes_pre, len(strike_ticks))
    emit('    `attack_resolved` records, which both verbs produce)')
    emit('    strikes at tick 14 or earlier, candidate   %d of %d'
         % (sum(1 for t in strike_ticks if t <= 14), len(strike_ticks)))
    emit('    last strike, candidate                     tick %d' % max(strike_ticks))
    emit()
    emit('The two columns of that table are the two commits of this work order, and neither is the')
    emit('set the section carried before 2026-08-21. Section 8 below accounts for that third set.')

    # ------------------------------------------------------------------ 4. the fear gate

    strike_fears = verb_fears(post, 'attack') + verb_fears(post, 'fight')
    approach_fears = verb_fears(post, 'approach')
    threat_fears = verb_fears(post, 'threaten')
    avoid_fears = verb_fears(post, 'avoid')
    claim('matrix', 4, 'the fear every strike was proposed at', 'SR',
          'Every one of the 63 strikes was proposed at a fear of 90 or lower', [63, 90],
          [len(strike_fears), max(strike_fears)])
    claim('matrix', 4, 'strikes proposed at exactly 90', 'SR', 'thirteen of them at exactly 90',
          13, sum(1 for f in strike_fears if f == 90))
    claim('matrix', 4, 'the fear every approach was proposed at', 'SR',
          'so was every one of the 1,724 approaches', [1724, 90],
          [len(approach_fears), max(approach_fears)])
    claim('matrix', 4, 'the fear every threat was proposed at', 'SR',
          'Every one of the 114 threats was proposed at exactly 100', [114, [100]],
          [len(threat_fears), sorted(set(threat_fears))])
    claim('matrix', 4, 'the fear every avoidance was proposed at', 'SR',
          'every one of the 4,268 avoidances at 95 or 100', [4268, [95, 100]],
          [len(avoid_fears), sorted(set(avoid_fears))])
    claim('matrix', 4, 'the same four distributions in the roadmap', 'RM',
          'every one of the 63 strikes was proposed at a `fear` of 90 or below, thirteen of them '
          'at exactly 90; every one of the 1,724 approaches at 90 or below; all 114 threats at '
          'exactly 100; and all 4,268 avoidances at 95 or 100',
          [63, 13, 1724, 114, 4268],
          [len(strike_fears), sum(1 for f in strike_fears if f == 90), len(approach_fears),
           len(threat_fears), len(avoid_fears)])
    claim('matrix', 4, 'turns of company needed to shut the gate', 'SR',
          'Ten turns of noticing anybody is enough to shut the gate for the rest of the run',
          10, -(-95 // 10))

    threats_all = sum(c.threats for c in list(pre.values()) + list(post.values())
                      if '-social-' in c.path)
    moved = [line for c in list(pre.values()) + list(post.values())
             if '-social-' in c.path for line in c.threats_moved]
    social_cells = sum(1 for c in list(pre.values()) + list(post.values()) if '-social-' in c.path)
    claim('matrix', 4, 'threats that moved their target\'s fear at all', 'SR',
          'across all sixty social captures `WO-MOK-017` took, at both commits, and the 2,850 '
          'threats they resolved between them, it is **the only threat that moved its target\'s '
          'fear at all.**',
          [60, 2850, 1], [social_cells, threats_all, len(set(moved))])

    emit()
    emit()
    emit('4. Where the fear gate sits, measured at the proposal rather than argued')
    emit('-' * 73)
    emit('    verb                     proposals   lowest fear   highest fear   distinct values')
    for label, values in (('attack + fight', strike_fears), ('approach', approach_fears),
                          ('threaten', threat_fears), ('avoid', avoid_fears)):
        distinct = sorted(set(values))
        shown = fmt(distinct) if len(distinct) <= 4 else '%d values' % len(distinct)
        emit('    %-22s %9d   %11d   %12d   %s'
             % (label, len(values), min(values), max(values), shown))
    emit()
    emit('    strikes proposed at exactly 90, the last rung below the gate   %d'
         % sum(1 for f in strike_fears if f == 90))
    emit('    `ENGAGEMENT_FEAR_THRESHOLD`, from the engine                   95')
    emit()
    emit('Nothing straddles 95: every engagement is proposed at 90 or below and every backing-off')
    emit('at 95 or above, with no verb appearing on both sides. That is the gate working as written,')
    emit('and it is why `docs/ROADMAP.md` records the raise to 95 as a mitigation rather than a')
    emit('repair -- which side of the line a Mokiterion falls on is decided by how many turns it has')
    emit('had company, not by anything about that company.')
    emit()
    emit('    `threat_resolved` records across every social cell, both commits   %s' % fmt(threats_all))
    emit('    social cells they come from                                        %d' % social_cells)
    emit('    of those threats, ones that moved the target\'s fear at all         %d'
         % len(set(moved)))
    for line in sorted(set(moved)):
        emit('      %s' % line)
    emit()
    emit('Every other threat lands on a target already at 100 and prints `increase:0`. The one')
    emit('exception is the record `SIMULATION_RULES.md` section 14 prints, which is why that section')
    emit('says what it is an example of.')

    # ------------------------------------------------------------------ 5. fear over creature-turns

    two = fear_rows(post, ['reference', 'individual'])
    four = fear_rows(post, SOURCES)
    two_pre = fear_rows(pre, ['reference', 'individual'])
    per_cell = []
    for src in ('reference', 'individual'):
        for s in SEEDS:
            rows = post[cell_name(s, src, DEFAULT, 'on')].fear
            per_cell.append(100.0 * sum(1 for a, b in rows if b > 0) / len(rows))

    def at_hundred(rows):
        return int(round(100.0 * sum(1 for a, b in rows if b == 100) / len(rows)))

    claim('matrix', 5, 'creature-turns under the two fear-carrying deciders', 'SR',
          '105,445 creature-turns', 105445, len(two))
    claim('matrix', 5, 'share of them at fear 100', 'SR',
          'fear is sitting at exactly 100 on 38% of them', 38, at_hundred(two))
    claim('matrix', 5, 'share above 0, per run', 'SR', 'above 0 on 44% to 64% of them',
          [44, 64], [int(round(min(per_cell))), int(round(max(per_cell)))])
    claim('matrix', 5, 'the +5 step the table does not predict', 'SR',
          'which occurred 260 times', 260, sum(1 for a, b in two if b - a == 5))
    claim('matrix', 5, 'the same population across all four deciders', 'SR',
          'the population is 160,921 creature-turns and fear sits at 100 on 34% of them',
          [160921, 34], [len(four), at_hundred(four)])
    claim('matrix', 5, 'the pre-change figures the document replaces', 'SR',
          'gave 111,604 creature-turns, 39% and 219', [111604, 39, 219],
          [len(two_pre), at_hundred(two_pre), sum(1 for a, b in two_pre if b - a == 5)])
    claim('matrix', 5, 'the figure section 8 asks the reader not to over-read', 'SR',
          'treat the 38% as a fact about the current', 38, at_hundred(two))

    emit()
    emit()
    emit('5. Fear across creature-turns, both commits')
    emit('-' * 44)
    emit('    population                              turns      at 100   above 0   +5 steps')
    for label, rows in (('reference and individual, candidate', two),
                        ('reference and individual, pre-change', two_pre),
                        ('all four deciders, candidate', four)):
        emit('    %-38s %8s   %5d%%   %5d%%   %8d'
             % (label, fmt(len(rows)), at_hundred(rows),
                int(round(100.0 * sum(1 for a, b in rows if b > 0) / len(rows))),
                sum(1 for a, b in rows if b - a == 5)))
    emit()
    emit('    per-run share above 0, candidate, ten runs   %d%% to %d%%'
         % (int(round(min(per_cell))), int(round(max(per_cell)))))
    emit()
    emit('One creature-turn is one `survival_changed` record and the fear read is the value it ends')
    emit('at, so the population is exactly the turns lived and nothing is weighted. The population')
    emit('shrinks with the correction -- 111,604 turns become 105,445 -- because fewer Mokiterions')
    emit('reach turn 1,000, which is the same fact the survivor rows of section 2 report from the')
    emit('other side. That is why the document dates the percentage rather than stating it flatly.')
    emit()
    emit('The `+5` step is the clamp at the top of the range: a Mokiterion at 95 gains the usual 10')
    emit('and stops at 100. It is measured rather than derived because it is the one step size')
    emit("section 8's table does not predict.")

    # ------------------------------------------------------------------ 6. composition

    post_shares, post_any = high_shares(post, ['reference', 'individual', 'social'])
    pre_shares, pre_any = high_shares(pre, ['reference', 'individual', 'social'])
    pre_ind_shares, _ = high_shares(pre, ['individual'])
    pre_ref_shares, _ = high_shares(pre, ['reference'])

    def band(values):
        return [int(round(min(values))), int(round(max(values)))]

    claim('matrix', 6, 'the high-class share at tick 1,000, candidate', 'SR',
          'runs **33% to 54%**', [33, 54], band(post_shares))
    claim('matrix', 6, 'the same share before the correction', 'SR', 'it ran 36% to 82%',
          [36, 82], band(pre_shares))
    claim('matrix', 6, 'the worst territory of any class, candidate', 'SR',
          'no class now holds more than three fifths of any territory on any declared seed',
          True, max(post_any) <= 60.0)
    claim('matrix', 6, "WO-MOK-010's two bands, re-measured on this packet's pre-change captures",
          'SR', 'got 36% to 77% and 46% to 75%', [36, 77, 46, 75],
          band(pre_ind_shares) + band(pre_ref_shares))
    claim('matrix', 6, 'the same band in the roadmap', 'RM',
          'runs 33%–54% against a starting third', [33, 54], band(post_shares))
    claim('matrix', 6, 'both bands in the roadmap', 'RM',
          'falls from 36%–82% to 33%–54%', [36, 82, 33, 54],
          band(pre_shares) + band(post_shares))

    cited(6, 'the case REQ-MOK-060 names, at the pre-change commit', 'SR',
          'High standing at 45 of 61 in one territory, 73.8%', 'post/composition.txt',
          'reference   0.75     0           61   11.5%   14.8%   73.8%')

    emit()
    emit()
    emit('6. The composition figures, from rule 18\'s own summary line')
    emit('-' * 60)
    emit('    evaluations are territory A and territory B of each of the fifteen obligated runs')
    emit()
    emit('    population                                   high share      worst class share')
    for label, shares, anyshare in (('candidate, three bound sources', post_shares, post_any),
                                    ('pre-change, three bound sources', pre_shares, pre_any)):
        emit('    %-40s %5.1f%% to %5.1f%%   %5.1f%%'
             % (label, min(shares), max(shares), max(anyshare)))
    emit('    %-40s %5.1f%% to %5.1f%%' % ('pre-change, trait-aware only',
                                           min(pre_ind_shares), max(pre_ind_shares)))
    emit('    %-40s %5.1f%% to %5.1f%%' % ('pre-change, reference only',
                                           min(pre_ref_shares), max(pre_ref_shares)))
    emit()
    emit('    the highest single high-class share before the correction   %.1f%%' % max(pre_shares))
    emit('    the highest single share of any class after it              %.1f%%' % max(post_any))
    emit()
    emit('The share is `food_*_high` over the three counts of the same territory, taken from the')
    emit('summary line each run prints. `post/composition.txt` does not trust that line: it rebuilds')
    emit('all six counts from the records that produced them, for all 120 cells at both commits, and')
    emit('it is the file that evaluates `REQ-MOK-060`. This reader takes the printed line, because')
    emit('what it is checking is a sentence in a document and the two agree.')

    # ------------------------------------------------------------------ 7. divergence

    post_ref_counts, post_ref_actors = refused(post, ALLOWANCE)
    pre_ref_counts, pre_ref_actors = refused(pre, dict((c, 0) for c in RESTORE))
    claim('matrix', 7, 'meals the reference rule would refuse, candidate', 'SR',
          '**0 to 3 per run by 0 to 2 different creatures**',
          [0, 3, 0, 2], [min(post_ref_counts), max(post_ref_counts),
                         min(post_ref_actors), max(post_ref_actors)])
    claim('matrix', 7, 'seed 777, candidate', 'SR',
          'on seed 777 there is not one such meal in the whole thousand turns',
          0, post_ref_counts[4])
    claim('matrix', 7, 'the same count before the correction', 'SR',
          'the same count was 54 to 97 per run by 9 or 10 creatures',
          [54, 97, 9, 10], [min(pre_ref_counts), max(pre_ref_counts),
                            min(pre_ref_actors), max(pre_ref_actors)])

    seed42 = post[cell_name('42', 'individual', DEFAULT, 'on')]
    seed42_pre = pre[cell_name('42', 'individual', DEFAULT, 'on')]
    printed = [e for e in seed42.eats if e[0] == 381 and e[1] == 'M08']
    former = [e for e in seed42.eats if e[0] == 14 and e[1] == 'M08']
    former_pre = [e for e in seed42_pre.eats if e[0] == 14 and e[1] == 'M08']
    claim('matrix', 7, "the tolerance of the creature in section 11's example", 'SR',
          'whose tolerance is 40, the maximum', 40, seed42.traits['M08'][1])
    claim('matrix', 7, 'what its snack destroyed', 'SR',
          'The snack restores 15, so 3 of it were destroyed', [15, 3],
          [RESTORE[printed[0][3]], printed[0][4] + RESTORE[printed[0][3]] - 100]
          if printed else 'no such meal')
    claim('matrix', 7, 'the tolerance that would also have refused it', 'SR',
          'and so would a creature with tolerance 19', True,
          bool(printed) and (19 * RESTORE[printed[0][3]]) // 100
          < printed[0][4] + RESTORE[printed[0][3]] - 100
          <= (20 * RESTORE[printed[0][3]]) // 100)
    claim('matrix', 7, 'the meal that used to be the example, at both commits', 'SR',
          'Turn 14 used to be this example — `M08` eating a snack at satiety 87, on the same run. '
          'That meal still happens',
          [87, 87], [former_pre[0][4] if former_pre else None,
                     former[0][4] if former else None])
    claim('matrix', 7, 'and that it is no longer a difference', 'SR',
          'satiety 87 is now exactly where the reference decider accepts a snack as well', True,
          bool(former) and former[0][4] + RESTORE[former[0][3]] - 100
          <= ALLOWANCE[former[0][3]])

    emit()
    emit()
    emit('7. What the trait still changes: meals the reference rule would have refused')
    emit('-' * 76)
    emit('    trait-aware runs at the default density, one row per declared seed')
    emit()
    emit('    seed    meals   refused by the reference rule of its own commit   creatures')
    for i, seed in enumerate(SEEDS):
        emit('    %-6s %6d   %-46d %d'
             % (seed, len(post[cell_name(seed, 'individual', DEFAULT, 'on')].eats),
                post_ref_counts[i], post_ref_actors[i]))
    emit('    %-6s %6s   %-46s %s'
         % ('', '', 'pre-change: ' + fmt(pre_ref_counts), fmt(pre_ref_actors)))
    emit()
    emit('Each commit is judged by its own reference rule, which is the only comparison that means')
    emit('anything: before the correction the reference rule admitted no spill at all, so a meal')
    emit('counted here is one spilling any; after it, one spilling more than the resource\'s own')
    emit('allowance of 2, 9 or 25. The trait went from deciding 54 to 97 meals a run to deciding at')
    emit('most 3, because on a High resource the allowance of 25 already exceeds anything a')
    emit('tolerance of 40 can add.')
    emit()
    emit('That is a narrowing of the trait, not of the divergence: `post/divergence.txt` accounts')
    emit('for every one of the 90 obligated cells, including seed 777, whose two runs still part')
    emit('even though not one of its meals is a meal the reference rule would refuse. A tolerance')
    emit('also decides what is worth walking to, and a path that differs moves who is standing')
    emit('where long after it has stopped moving who eats what.')

    # ------------------------------------------------------------------ 8. the quoted records

    emit()
    emit()
    emit('8. Every record either document prints, found in the cell it is attributed to')
    emit('-' * 76)
    missing = []
    for doc, cell, record, where in QUOTED_RECORDS:
        present = record in post[cell].found
        if not present:
            missing.append((cell, record))
        claim('matrix', 8, where, doc, record, True, present, source=cell)
        emit('    %-4s %s' % ('ok' if present else 'MISS', where))
        emit('         %s' % cell)
        emit('         %s' % record)
    emit()
    emit('Nine records, each read out of the capture cell the document\'s own command selects. The')
    emit('documents pad these lines into columns for reading; the comparison is on whitespace-')
    emit('normalized text, so the padding is not part of the claim and the fields are.')
    emit()
    emit('Two are worth naming. The threat at tick 337 of `seed123-social-d1.50` is the one threat')
    emit('in the whole capture that moved a target\'s fear, per section 4. The meal at tick 381 of')
    emit('`seed42-individual-d0.75` is the one the document prints as the difference the trait still')
    emit('makes, and the meal that used to be printed in its place is measured in section 7 instead')
    emit('-- the document now describes that one in prose rather than quoting it, so there is no')
    emit('record here to match.')

    # ------------------------------------------------------------------ 9. Trok

    trok_cell = post[cell_name('42', 'reference', DEFAULT, 'on')]
    walk = []
    for line in trok_cell.m05:
        if 'event=action_trace' not in line:
            continue
        proposal = RE_PROPOSAL.search(line)
        position = RE_TRACE_POS.search(line)
        walk.append(dict(tick=int(RE_TICK.match(line).group(1)),
                         verb=proposal.group(1),
                         argument=proposal.group(2) or proposal.group(3) or '',
                         status=proposal.group(4),
                         satiety=int(RE_TRACE_SAT.search(line).group(1)),
                         at=(position.group(1) or position.group(2)) if position else '?'))
    turn = dict((row['tick'], row) for row in walk)
    tolerance = trok_cell.traits['M05'][1]
    snack, ate_at = 'F0058', 20
    eat = [e for e in trok_cell.eats if e[1] == 'M05' and e[0] == ate_at]
    food_class, food_position = trok_cell.food_at[snack]
    start = turn[13]['at']
    reach = [abs(int(a) - int(b)) for a, b in zip(food_position.split(':'), start.split(':'))]

    def steps(low, high, heading):
        return sum(1 for row in walk
                   if low <= row['tick'] <= high and row['verb'] == 'move'
                   and row['argument'] == heading and row['status'] == 'accepted')

    claim('matrix', 9, "Trok's first thirteen turns, in order", 'SR',
          'wandered: south, south, north, south, north, north, south, north, east, west, east, '
          'north, south',
          ['south', 'south', 'north', 'south', 'north', 'north', 'south', 'north', 'east', 'west',
           'east', 'north', 'south'],
          [turn[t]['argument'] for t in range(1, 14)])
    claim('matrix', 9, 'the satiety it wandered through', 'SR', '| 1–13 | wandered', [100, 88],
          [turn[1]['satiety'], turn[13]['satiety']])
    claim('matrix', 9, 'what a snack would have spilled on the first and last of those turns', 'SR',
          '15 of it on turn 1, and still 3 of it on turn 13', [15, 3],
          [turn[1]['satiety'] + RESTORE['low'] - 100, turn[13]['satiety'] + RESTORE['low'] - 100])
    claim('matrix', 9, 'the turn it set off, and from where, and toward what', 'SR',
          '| 14 | **move east**, from `85:15` toward `89:13` | 87 |',
          ['85:15', '89:13', 87, 'east'],
          [start, food_position, turn[14]['satiety'], turn[14]['argument']])
    claim('matrix', 9, 'the arithmetic that opened the walk', 'SR',
          'Now 87 + 15 = 102, spilling exactly the 2 a Low piece is allowed', [102, 2],
          [turn[14]['satiety'] + RESTORE['low'],
           turn[14]['satiety'] + RESTORE['low'] - 100])
    claim('matrix', 9, 'the distance from where it set off to the resource', 'SR',
          'six squares away', 6, sum(reach))
    claim('matrix', 9, 'the shape of the walk', 'SR',
          'four steps east until it was directly below the food, then two north',
          [4, 2], [steps(14, 19, 'east'), steps(14, 19, 'north')])
    claim('matrix', 9, 'the turn it arrived, and on what', 'SR',
          '| 19 | **move north** → `89:13` | 82 |', ['89:13', 82],
          [turn[19]['at'], turn[19]['satiety']])
    claim('matrix', 9, 'the meal, and that it wasted nothing', 'SR',
          '| 20 | **eat `F0058`** (low) | 81 → 96 |', [snack, 'low', 81, 96],
          [eat[0][2], eat[0][3], eat[0][4], eat[0][4] + RESTORE[eat[0][3]]]
          if eat else 'no such meal')
    claim('matrix', 9, 'how far short of full that left it', 'SR',
          '96 is 4 short of full, so nothing at all was wasted', 4,
          100 - (eat[0][4] + RESTORE[eat[0][3]]) if eat else None)
    claim('matrix', 9, 'the turn after, and that it is wandering again', 'SR',
          '| 21 | move east | 95 |', [95, 'move', 'east'],
          [turn[21]['satiety'], turn[21]['verb'], turn[21]['argument']])
    claim('matrix', 9, "Trok's own waste tolerance", 'SR', "Trok's own waste tolerance is 21",
          21, tolerance)
    claim('matrix', 9, 'the satiety the trait-aware decider would have set off at', 'SR',
          'would have admitted that snack one point of satiety earlier, at 88', 88,
          100 - RESTORE['low'] + (tolerance * RESTORE['low']) // 100)
    claim('matrix', 9, 'the satiety above which nothing fits at all', 'SR',
          'Above satiety 87 nothing at all fits', 87, 100 - RESTORE['low'] + ALLOWANCE['low'])
    claim('matrix', 9, 'the highest a tolerance of 40 reaches on a snack', 'SR',
          'The most tolerant creature stretches that to 91 and no further', 91,
          100 - RESTORE['low'] + (40 * RESTORE['low']) // 100)
    claim('matrix', 9, 'that the run is real output rather than an illustration', 'SR',
          'This is genuine output, not an illustration', 21, len(walk))

    emit()
    emit()
    emit('9. Trok\'s twenty-one turns, the walked example of section 14')
    emit('-' * 61)
    emit('    from %s, which is the cell of the command section 14 names'
         % os.path.basename(trok_cell.path))
    emit('    M05 is %s, waste tolerance %d, and the decider in this run ignores it'
         % (trok_cell.traits['M05'][0], tolerance))
    emit('    %s is %s class, standing at %s from turn 0, and never moves'
         % (snack, food_class, food_position))
    emit()
    emit('    turn  proposal            satiety   at        a Low snack would spill')
    for row in walk:
        spill = row['satiety'] + RESTORE['low'] - 100
        if row['verb'] == 'eat':
            reading = 'this is the meal, and the satiety is already after it'
        elif spill > ALLOWANCE['low']:
            reading = '%d, over the 2 allowed' % spill
        elif spill > 0:
            reading = '%d, within the 2 allowed' % spill
        else:
            reading = 'nothing'
        emit('    %4d  %-19s %7d   %-8s  %s'
             % (row['tick'],
                row['verb'] + (':' + row['argument'] if row['argument'] else ''),
                row['satiety'], row['at'], reading))
    emit()
    emit('    the meal at turn %d   %s, %s class, satiety %d + %d = %d, %d short of full'
         % (ate_at, snack, eat[0][3], eat[0][4], RESTORE[eat[0][3]],
            eat[0][4] + RESTORE[eat[0][3]], 100 - eat[0][4] - RESTORE[eat[0][3]]))
    emit('    reach at turn 14     %s to %s, %d east and %d north, %d squares'
         % (start, food_position, reach[0], reach[1], sum(reach)))
    emit()
    emit('The satiety column is the trace record\'s own field, which is the value at the moment the')
    emit('Mokiterion decided -- except on turn %d, where the meal has already been applied by the'
         % ate_at)
    emit('time the trace prints, and the document prints that row as `81 -> 96` for exactly that')
    emit('reason. The last column is the corrected condition\'s arithmetic on each of those turns,')
    emit('and it is the whole of the walk: the spill sits above the allowance of 2 for thirteen turns')
    emit('and reaches it on turn 14, which is the turn the Mokiterion sets off.')
    emit()
    emit('This is why the example is in the document twice over. It is a real run, and the correction')
    emit('is visible in it without a second world to compare against: nothing walks toward a resource')
    emit('it would waste more than the allowance on, and the turn the arithmetic changes is the turn')
    emit('the behavior does.')

    # ------------------------------------------------------------------ 10. outside the matrix

    extras = {}
    for name, command in OUTSIDE_RUNS:
        path = os.path.join(extra_dir, name + '.txt')
        if not os.path.isfile(path):
            raise SystemExit('missing out-of-matrix run: %s' % path)
        extras[name] = Cell(path, set())

    sweep = [(d, extras['ref-seed0-d%s' % d].summary['survivors'])
             for d in ('0.70', '0.75', '1.00', '1.25')]
    claim('outside', 10, 'the density warning, four densities on seed 0', 'SR',
          '`0.70%` leaves nine alive while the default `0.75%` leaves eight, and `1.00%` leaves '
          'seven while `1.25%` leaves twelve', [9, 8, 7, 12], [n for _, n in sweep])
    claim('outside', 10, 'the default density, against the matrix cell of the same run', 'SR',
          'Only the default `0.75%` carries a promise about survivors',
          post[cell_name('0', 'reference', DEFAULT, 'on')].summary['survivors'],
          extras['ref-seed0-d0.75'].summary['survivors'])

    deaths_at = extras['seed42-d0.02-t200'].deaths_at
    claim('outside', 10, 'the starvation clock at a density of 0.02', 'SR',
          'eleven of the twelve die together on tick 119', [11, 119],
          [deaths_at.get(119, 0), 119 if 119 in deaths_at else None])
    claim('outside', 10, 'the twelfth, which found the one resource', 'SR',
          'lasted until tick 134', 134, max(deaths_at))
    claim('outside', 10, 'the arithmetic the run confirms', 'SR', '**dies on tick 119**',
          119, 100 + 100 // 5 - 1)

    forty = extras['seed42-t40'].summary
    claim('outside', 10, 'the scoreboard of section 15, its four counts', 'SR',
          'summary reason=tick_limit ticks=40 survivors=12 deaths=0 territory_a=5 territory_b=7',
          ['tick_limit', 40, 12, 0], [forty['reason'], forty['ticks'], forty['survivors'],
                                     forty['deaths']])
    claim('outside', 10, 'the scoreboard of section 15, its six food counts', 'SR',
          'food_a_low=17 food_a_medium=22 food_a_high=20 food_b_low=18 food_b_medium=19 '
          'food_b_high=19', [17, 22, 20, 18, 19, 19], forty['food'])
    claim('outside', 10, 'the three sizes at turn 40, still roughly even', 'SR',
          'this early the three sizes are still roughly even', True,
          max(forty['food'][0:3] + forty['food'][3:6])
          - min(forty['food'][0:3] + forty['food'][3:6]) <= 5)

    t400 = [e for e in extras['ind-seed42-t400'].eats if e[0] == 381 and e[1] == 'M08']
    claim('outside', 10, 'the command section 11 prints, and the line it prints', 'SR',
          'cargo run --bin Mokiterions -- --policy individual --seed 42 --ticks 400 | grep '
          'food_consumed', [(381, 'M08', 'F0257', 'low', 88)], t400)

    emit()
    emit()
    emit('10. The figures taken outside the declared matrix')
    emit('-' * 50)
    emit('Seven runs, none of them a cell of the 120. Each is a command a reader can run, and each is')
    emit('printed in the document beside the figure it produces. They are not retained: they are one')
    emit('command each and `REQ-MOK-009` makes them reproducible, which is the same reason')
    emit('`analysis/retain.py` gives for keeping two cells of the matrix rather than 120.')
    emit()
    for name, command in OUTSIDE_RUNS:
        summary = extras[name].summary
        emit('    %s' % command)
        emit('        reason=%-11s ticks=%-5d survivors=%-3d deaths=%d'
             % (summary['reason'], summary['ticks'], summary['survivors'], summary['deaths']))
    emit()
    emit('    the density warning of section 9, seed 0 under reference')
    for density, alive in sweep:
        emit('        density %s%%   %2d alive' % (density, alive))
    emit()
    emit('    the starvation clock of section 8, seed 42 at density 0.02%')
    for tick in sorted(deaths_at):
        emit('        tick %-4d %2d died' % (tick, deaths_at[tick]))
    emit()
    emit('The density row at 0.75% is in that list on purpose, and it is the check that makes the')
    emit('other three worth reading: it leaves %d alive, which is what cell')
    out[-1] = out[-1] % extras['ref-seed0-d0.75'].summary['survivors']
    emit('`seed0-reference-d0.75` of the declared matrix reports. The binary that produced the')
    emit('out-of-matrix runs is therefore the binary that produced the matrix, and the sweep is a')
    emit('statement about density rather than about a stale build.')

    # ------------------------------------------------------------------ 11. cited figures

    cited(11, 'the corrected boundaries, per class', 'SR',
          '| Low | 15 | 2 | **87** |', 'post/eaten.txt',
          'cannot exceed 85/70/50 for low/medium/high; after it, 87/79/75')
    cited(11, 'the enumerated identity set', 'SR', 'of 4,536 possible situations was checked',
          'post/updated-tests.md', '`assert_eq!(cases, 2_808)` became `assert_eq!(cases, 4_536)`')
    cited(11, 'the suite behind every figure here', 'SR',
          'the workspace runs 268 tests green', 'post/gates.txt',
          '268 test names, 268 passed, 0 failed, 0 ignored, exit 0')
    cited(11, 'consumption across the whole matrix', 'RM',
          'matrix consumption rises from 26,136 meals to 26,924', 'post/composition.txt',
          'consumptions at the candidate 26,924')
    cited(11, 'the two carried floors of eight', 'RM',
          "`REQ-MOK-014`'s eight and `REQ-MOK-034`'s eight are met on every declared seed",
          'post/survivors.txt',
          'REQ-MOK-014 -- reference at the default density, floor 8 of 12')
    cited(11, 'the allowance term, per class', 'RM',
          '`2` for low class, `9` for medium, `25` for high', 'post/eaten.txt', '87/79/75')
    cited(11, 'the observer amendment that rests on the control case', 'SR',
          'watch the random decider starve', 'post/dead-neighbours.txt',
          'candidate   12 of 12 identifiers die on tick 119')

    emit()
    emit()
    emit('11. Figures a sibling file in this packet measures, cited rather than recomputed')
    emit('-' * 80)
    emit('One figure, one reader. Where a file in this packet already measures a number the document')
    emit('states, this reader checks that the file says it and does not recompute it: two readers of')
    emit('one stream that disagree would be a defect of this packet and not a finding about the')
    emit('world.')
    emit()
    for row in [c for c in CLAIMS if c['kind'] == 'cited']:
        emit('    %-4s %s  %s' % ('ok' if row['agrees'] and row['quoted'] else 'MISS',
                                  row['doc'], row['figure']))
        emit('         the document: %s' % row['quote'])
        emit('         %s: %s' % (row['source'], row['written']))

    # ------------------------------------------------------------------ 12. history

    history(12, 'the extinction tick at the default density', 'SR',
            'dead by tick 9,154 at the default density on seed 123',
            'were measured on the **uncorrected** world', 'evidence/WO-MOK-002/density-curve.md')
    history(12, 'the same figure in the roadmap', 'RM',
            'reached extinction at tick 9,154',
            'The tick-9,154 figure above is **not** re-measured',
            'evidence/WO-MOK-002/density-curve.md')
    history(12, 'the 10,000-turn comparison', 'SR',
            'reaching the limit on four of the five seeds with 1 to 5 survivors',
            'was measured on the uncorrected world and has not been re-run',
            'evidence/WO-MOK-010/escalation.md')
    history(12, 'the tolerance-range measurement that cut the range at 40', 'SR',
            'dropped below the required eight survivors on three of the five seeds',
            'taken on the pre-2026-08-21 world, and not re-taken since',
            'evidence/WO-MOK-010/escalation.md')
    history(12, "WO-MOK-010's two composition bands", 'SR',
            'ran 35% to 77% per territory under the trait-aware decider against 45% to 75%',
            'As measured then', 'evidence/WO-MOK-010/')
    history(12, 'the action table the section carried until 2026-08-21', 'SR',
            '6,329 avoids, three attacks, no surrenders at all',
            'That set was measured before commit `7d744bb` reordered the five steps above',
            'evidence/WO-MOK-016/')
    history(12, "the first measurement of social, in the roadmap", 'RM',
            'Measured: 6, 4, 8, 4 and 5 survivors, and **zero** combat deaths on all five seeds',
            'What the first measurement of `social` reported', 'evidence/WO-MOK-016/')
    history(12, 'the starvation the branch order caused', 'RM',
            'dropped meals eaten from 378–417 per run to 205–304',
            'Both followed from one coupling', 'evidence/WO-MOK-016/')
    history(12, 'the strict count of divergent moments', 'SR',
            '"Moments where two creatures facing the genuinely same situation would have chosen '
            'differently" was 3 to 10 per run',
            '**The stricter count has not been re-measured.**', 'evidence/WO-MOK-010/')

    emit()
    emit()
    emit('12. Figures this work order did not re-measure, and the flag each one carries')
    emit('-' * 78)
    emit('Nine figures in the two documents describe a world the engine no longer implements. Every')
    emit('one is kept, because deleting a measurement is not the same as superseding it, and every')
    emit('one is required here to carry a sentence in its own document saying so. The flag is part of')
    emit('the claim: a figure of this kind stated flatly would fail this reader even though there is')
    emit('no arithmetic to check.')
    emit()
    for row in [c for c in CLAIMS if c['kind'] == 'history']:
        emit('    %-4s %s  %s' % ('ok' if row['quoted'] else 'MISS', row['doc'], row['figure']))
        emit('         the figure: %s' % row['quote'])
        emit('         the flag:   %s' % row['note'])
        emit('         measured by: %s' % row['source'])
    emit()
    emit('The long horizon is the substantive one. `REQ-MOK-060` binds tick 1,000 and nothing past')
    emit('it, deliberately, so `WO-MOK-017` measured nothing past it either -- and the figures that')
    emit('made high-class accumulation look fatal over 10,000 turns are exactly the figures the')
    emit('correction is most likely to have moved. `docs/ROADMAP.md` carries re-measuring them as')
    emit('outstanding work against a long-horizon requirement that is still unstated.')

    # ------------------------------------------------------------------ the ledger

    emit()
    emit()
    emit('13. The whole claim inventory')
    emit('-' * 30)
    emit('    `q!` means the quoted sentence is not in the document as written; `v!` that the')
    emit('    document\'s figure and this reader\'s do not agree; `c!` that a cited file does not')
    emit('    carry the figure attributed to it.')
    emit()
    kinds = ('matrix', 'outside', 'cited', 'history')
    for kind in kinds:
        rows = [c for c in CLAIMS if c['kind'] == kind]
        emit('    %s -- %d claims' % (kind, len(rows)))
        for c in rows:
            if not c['quoted']:
                mark = 'q!'
            elif not c['agrees']:
                mark = 'v!' if kind != 'cited' else 'c!'
            else:
                mark = 'ok'
            emit('      %s  %s  §%-2d %s' % (mark, c['doc'], c['section'], c['figure']))
            if c['written'] is not None and kind != 'cited':
                if c['agrees']:
                    emit('              %s' % fmt(c['written']))
                else:
                    emit('              written %s   measured %s'
                         % (fmt(c['written']), fmt(c['measured'])))
            if not c['quoted']:
                emit('              MISSING QUOTE: %s' % c['quote'][:96])
        emit()

    failures = [c for c in CLAIMS if not c['quoted'] or not c['agrees']]
    emit('    claims checked                          %3d' % len(CLAIMS))
    emit('    of which recomputed from the captures   %3d'
         % sum(1 for c in CLAIMS if c['kind'] == 'matrix'))
    emit('    recomputed from a run outside them      %3d'
         % sum(1 for c in CLAIMS if c['kind'] == 'outside'))
    emit('    cited to a sibling file                 %3d'
         % sum(1 for c in CLAIMS if c['kind'] == 'cited'))
    emit('    history, flagged as such                %3d'
         % sum(1 for c in CLAIMS if c['kind'] == 'history'))
    emit('    failures                                %3d' % len(failures))
    emit()

    emit()
    emit('14. What this file does not establish')
    emit('-' * 37)
    emit('- It is not a verification record and settles no obligation. The ceiling is')
    emit('  `post/composition.txt`, the floors are `post/survivors.txt`, and both are the assurance')
    emit('  owner\'s to accept.')
    emit('- It does not check that either document is well written, well organized or complete. It')
    emit('  checks that the numbers in it are the world\'s numbers.')
    emit('- Where a document spells a figure in words, the words are not parsed. The quote and the')
    emit('  measurement are printed together and the reader closes that link by reading them.')
    emit('- The out-of-matrix runs of section 10 are not retained. They are reproducible in one')
    emit('  command each, and the density row at 0.75% is the cross-check that says the binary that')
    emit('  produced them is the binary that produced the matrix.')
    emit('- Nothing here re-measures the long horizon. Section 12 says so, and so does every')
    emit('  document sentence it lists.')
    emit()
    result = 'PASS' if not failures else 'FAIL'
    emit('RESULT: %s -- %d figures across two reader-facing documents, %d recomputed from the'
         % (result, len(CLAIMS), sum(1 for c in CLAIMS if c['kind'] in ('matrix', 'outside'))))
    emit('streams at both commits, %d cited to a sibling file, %d flagged as history and not '
         're-measured,' % (sum(1 for c in CLAIMS if c['kind'] == 'cited'),
                           sum(1 for c in CLAIMS if c['kind'] == 'history')))
    emit('and %d disagreeing with the document that states them.' % len(failures))

    print('\n'.join(out))
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
