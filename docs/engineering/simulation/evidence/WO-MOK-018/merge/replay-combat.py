"""VER-MOK-012 oracle 6, extended for rules 22 to 26: the metrics and run records reconciled against
a replay of a run that carries combat.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/merge/replay-combat.py \
        <capture-dir> <output-file>

## Why this file exists at all

`analysis/replay.py` is the retained oracle and it is **not modified**: it is imported here by path,
and everything this file does is add three arms to one method. The retained file stays byte-identical
because `VER-MOK-012`'s record and its digests are bound to it.

The retained oracle is written from `SPEC-MOK-001` rules 1 to 15 and knows nothing of rules 22 to 26,
which did not exist when it was written. Rule 12 makes `survival_changed` the statement of a
Mokiterion's end-of-tick attributes, and on a run with no combat that is true. On a `social` run it is
not: a Mokiterion's health can fall after its own survival update, inside another Mokiterion's turn,
and its satiety can move in a surrender it is not the subject of. So the retained replay's last-known
attributes are stale at the tick boundary and it reports the metrics record as wrong when the metrics
record is right.

Run unextended over the thirty `social` cells it reports **304 findings**, in four kinds:

    124  health sum      the target's damage, not applied
     74  health min      the same, at the extremum
     64  satiety sum     a surrender's transfer and discard, not applied
     42  run record MXX died_at None against replay N

The first three are this instrument's gap and are closed below. **The fourth is not.** It survives the
extension, it is a defect in the product, and `merge/README.md` reports it: `resolve_attack` sets the
target's `alive` to `false` and emits `agent_died`, but never sets its `died_at`, so the run record
states a `deaths` count that its own roster does not account for. Every one of the 42 is a Mokiterion
an attack killed, no attack-killed Mokiterion anywhere in the capture carries a `died_at`, and no
Mokiterion that died any other way lacks one.

## What the three arms do, and where each comes from

Each arm reads the authoritative `to` value out of the event rather than recomputing it from the
`from` value and the delta. That is deliberate and it is the same choice rule 12's arm already makes:
this oracle reconciles two *streams*, and an arm that recomputed would be checking the engine's
arithmetic against itself instead of checking the metrics record against the events. The damage, the
increase and the transferred figures are therefore read and reported, not trusted -- they are checked
against the pair they sit beside, which is a claim the event makes about itself and which the metrics
record cannot corroborate.

* **Rule 22, `attack_resolved`.** The target's `health` becomes `target_health.to` and the striker's
  `energy` becomes `striker_energy.to`. `target_died` is checked against `target_health.to == 0`,
  because a verdict that disagreed with the health it reports beside it would be a defect no other
  check here would see. The death itself arrives as rule 13's own `agent_died` event, which the
  retained arm already handles; this arm does not anticipate it.
* **Rule 23, `threat_resolved`.** The target's `fear` becomes `target_fear.to`, and the `increase` is
  checked against the pair: rule 23 reports the *effective* increase, so `to - from` is what it must
  be, which is `0` where the target already stood at the maximum.
* **Rule 24, `surrender_resolved`.** The subject's `satiety` becomes `subject_satiety.to` and the
  recipient's becomes `recipient_satiety.to`. `transferred` is checked against the recipient's gain
  and `discarded` against the remainder the subject lost, which is the one place the two halves of
  rule 24's arithmetic are held against each other.

Rules 25 and 26 need no arm. Rule 25's suffered-attack window is an observation input and moves no
attribute a metrics record reports, and rule 26 is a decision source: it changes which actions are
proposed and nothing about how a resolved action is recorded.
"""

import importlib.util
import io
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RETAINED = os.path.join(HERE, os.pardir, 'analysis', 'replay.py')


def load_retained():
    """The retained oracle, imported from its own file and left unmodified on disk."""
    specification = importlib.util.spec_from_file_location('replay_retained', RETAINED)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def extend(retained):
    """`Replay` with rules 22, 23 and 24 added to its event arm, and nothing else changed."""

    class ReplayWithCombat(retained.Replay):
        def event(self, record):
            super().event(record)

            tick, subject, kind, result = (
                record['tick'],
                record['subject'],
                record['event'],
                record['result'],
            )

            if kind == 'attack_resolved':
                target = result['target']
                self.attributes[target]['health'] = result['target_health']['to']
                self.attributes[subject]['energy'] = result['striker_energy']['to']
                died = result['target_health']['to'] == 0
                stated = result['target_died']
                if stated != ('yes' if died else 'no'):
                    self.note(
                        f'tick {tick}: {subject} struck {target} to health '
                        f'{result["target_health"]["to"]} and reported target_died {stated}'
                    )
                fell = result['target_health']['from'] - result['target_health']['to']
                if result['damage'] != fell and not (died and result['damage'] > fell):
                    # Rule 22 saturates at zero, so a killing blow may report more damage than the
                    # health it removed. Anything else is a disagreement.
                    self.note(
                        f'tick {tick}: {subject} reported damage {result["damage"]} against '
                        f'{target} health falling by {fell}'
                    )
            elif kind == 'threat_resolved':
                target = result['target']
                self.attributes[target]['fear'] = result['target_fear']['to']
                rose = result['target_fear']['to'] - result['target_fear']['from']
                if result['increase'] != rose:
                    self.note(
                        f'tick {tick}: {subject} reported increase {result["increase"]} against '
                        f'{target} fear rising by {rose}'
                    )
            elif kind == 'surrender_resolved':
                recipient = result['recipient']
                self.attributes[subject]['satiety'] = result['subject_satiety']['to']
                self.attributes[recipient]['satiety'] = result['recipient_satiety']['to']
                gained = result['recipient_satiety']['to'] - result['recipient_satiety']['from']
                lost = result['subject_satiety']['from'] - result['subject_satiety']['to']
                if result['transferred'] != gained:
                    self.note(
                        f'tick {tick}: {subject} reported transferred {result["transferred"]} '
                        f'against {recipient} satiety rising by {gained}'
                    )
                if result['discarded'] != lost - result['transferred']:
                    self.note(
                        f'tick {tick}: {subject} reported discarded {result["discarded"]} against '
                        f'a satiety fall of {lost} of which {result["transferred"]} transferred'
                    )

    return ReplayWithCombat


def main():
    capture_dir, output_file = sys.argv[1:3]
    retained = load_retained()
    retained.Replay = extend(retained)

    streams = sorted(name for name in os.listdir(capture_dir) if name.endswith('.jsonl'))
    if not streams:
        print(f'no record stream in {capture_dir}', file=sys.stderr)
        return 1

    rows = []
    findings = []
    for stream in streams:
        state = retained.replay(os.path.join(capture_dir, stream))
        rows.append((stream, state.ticks_reconciled, len(state.findings)))
        findings.extend(state.findings)

    total = sum(ticks for _, ticks, _ in rows)
    lines = [
        '# VER-MOK-012 oracle 6, extended for rules 22 to 24: the metrics and run records against a',
        '# replay of the events',
        '#',
        f'# capture directory: {capture_dir}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-018/merge/replay-combat.py '
        '<capture-dir> <output-file>',
        '#',
        '# The retained analysis/replay.py is imported unmodified and its Replay class is subclassed',
        '# to add three arms to one method: rule 22\'s damage and energy cost, rule 23\'s fear, and',
        '# rule 24\'s two satieties. Each arm reads the event\'s authoritative `to` value and checks',
        '# the event\'s own reported delta against the pair beside it. Nothing else is changed, and',
        '# the whole of this driver is reproduced at the foot of the file.',
        '',
    ]
    for stream, ticks, count in rows:
        verdict = 'reconciled' if count == 0 else f'{count} FINDINGS'
        lines.append(f'{stream:<42} {ticks:>5} ticks  {verdict}')

    lines.append('')
    lines.append(
        f'# {len(streams)} streams, {total} tick boundaries reconciled, {len(findings)} findings'
    )
    lines.append(f'# result: {"FAIL" if findings else "PASS"}')
    if findings:
        lines.append('')
        lines.append('# findings')
        lines.extend(findings[:400])
        if len(findings) > 400:
            lines.append(f'# ... and {len(findings) - 400} more')
    lines.append('')
    lines.append('# ---- full text of this driver ----')
    lines.append('')
    lines.extend(io.open(os.path.abspath(__file__), 'r', encoding='utf-8').read().split('\n'))
    lines.append('')
    lines.append('# ---- full text of the retained oracle, imported unmodified ----')
    lines.append('')
    lines.extend(io.open(RETAINED, 'r', encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(f'{len(streams)} streams, {total} tick boundaries reconciled, {len(findings)} findings')
    return 1 if findings else 0


if __name__ == '__main__':
    sys.exit(main())
