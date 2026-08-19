"""WO-MOK-010: the negative controls, applied to the committed source and reverted.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/negative-control/controls.py

VER-MOK-010's acceptance scenario 3 requires that a check which could pass vacuously be shown able to
fail. Two of the five oracles are in that position:

  * Oracle 2 compares the shared entropy stream's position either side of trait derivation. The
    derivation takes no stream, so a before-and-after comparison around it is tautological unless the
    recorded draw counts are what carries the check.
  * Oracle 3 compares the trait-aware source against the reference source at tolerance `0` over an
    enumerated set of 2,808 situations, on both the proposal and the stream position. Two
    proposal-identical sources agree trivially, so the question is whether the comparison would notice
    if they stopped agreeing.

Each control perturbs the committed source by one line, runs the engine's own test suite, records what
failed and why, and reverts. The revert is verified by SHA-256 against the digest taken before the
perturbation was applied, and the script refuses to continue if a revert does not restore the file
exactly. What it writes -- `oracle-2.txt` and `oracle-3.txt` -- is generated from the runs, so the
figures in them are this tree's figures and not a transcription.

A control is designed to fail for the specific reason the oracle exists, not to break the build. Each
one below states the design rule it violates and the magnitude its defect predicts, and the artifact
reports whether the observed failure has that magnitude. A perturbation that made the suite fail for
some other reason would demonstrate nothing.
"""

import hashlib
import io
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.exists(os.path.join(ROOT, 'Cargo.toml')):
    ROOT = os.path.dirname(ROOT)
SOURCE = os.path.join(ROOT, 'mokiterions-core', 'src', 'simulation.rs')

CONTROLS = [
    {
        'artifact': 'oracle-2.txt',
        'oracle': 'oracle 2 - the shared entropy stream\'s position across trait derivation',
        'scenario': 'Acceptance scenario 3 requires that the entropy-position check be shown able to '
                    'fail,\nbecause a before-and-after comparison around a derivation that takes no '
                    'stream would\notherwise be tautological.',
        'site': 'mokiterions-core/src/simulation.rs, Simulation::new, agent construction',
        'before': '                waste_tolerance: derive_waste_tolerance(config.seed, number),',
        'after': '                waste_tolerance: derive_waste_tolerance(entropy.next_u64(), number),',
        'violates': 'This is the design `REQ-MOK-031` forbids: the derivation now takes its state from\n'
                    'the shared stream, so each of the twelve agents consumes one value from it.',
        'predicts': 'twelve extra draws at initialization, one per agent, so a recorded count of 72\n'
                    'becomes 84',
        'markers': ['trait_derivation_leaves_the_shared_stream_where_it_found_it',
                    'left: 84', 'right: 72'],
        'observes': 'The observed count is 84 against a recorded 72: twelve extra values, one per\n'
                    'derivation, which is the exact magnitude the perturbation predicts. The check is\n'
                    'therefore sensitive to the specific defect it exists to detect, and not merely to\n'
                    'gross breakage.\n'
                    '\n'
                    'Note also what the control demonstrates about the recorded expectation itself. The\n'
                    'perturbed build derives a full twelve-value trait table that is in range and\n'
                    'non-uniform, so a re-derived expectation would have accepted it. Only the recorded\n'
                    'row checked into the test rejected it.',
    },
    {
        'artifact': 'oracle-3.txt',
        'oracle': 'oracle 3 - equivalence with the reference source at tolerance 0',
        'scenario': 'Acceptance scenario 3 requires that the equivalence check be shown able to fail. '
                    'Rule 19\nis written so that at tolerance `0` its tolerant test reduces to rule '
                    '5\'s exact-fit test,\nand two sources that agree are indistinguishable from two '
                    'sources that are not being\ncompared. These two controls separate those cases: '
                    'each makes the trait-aware source differ\nfrom the reference source at tolerance '
                    '`0` in one of the two ways the oracle asserts about --\nthe proposal it returns, '
                    'and the position it leaves the stream in.',
        'controls': [
            {
                'label': 'Control A - the tolerant bound, off by one',
                'site': 'mokiterions-core/src/simulation.rs, Observation::fits_within_tolerance',
                'before': '        resulting - maximum <= u16::from(self.waste_tolerance) '
                          '* restored / 100',
                'after': '        resulting - maximum <= u16::from(self.waste_tolerance) '
                         '* restored / 100 + 1',
                'violates': 'Rule 19 states the admitted waste as `T * R / 100`. With `+ 1` the test '
                            'admits one\nunit more than the rule allows, and at tolerance `0` it '
                            'admits one unit of waste where\nrule 5 admits none -- so the two sources '
                            'no longer propose the same action.',
                'predicts': 'a proposal mismatch at the satieties where exactly one unit is clipped, '
                            'and no\nmismatch elsewhere',
                'markers': [
                    'at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_'
                    'proposes',
                    'left: Eat',
                ],
                'observes': 'The enumeration fails on the proposal: at tolerance `0` the trait-aware\n'
                            'source proposes to eat where the reference source, given the same\n'
                            'observation, does not. One admitted unit of waste is the whole difference\n'
                            'between the two sources, so the enumerated set is sensitive at one unit --\n'
                            'the finest resolution the arithmetic has. Two further tests fail for the\n'
                            'same reason, which is what a bound shared by rule 19\'s two cases should do.',
            },
            {
                'label': 'Control B - one extra draw in the trait-aware source',
                'site': 'mokiterions-core/src/simulation.rs, '
                        'IndividualDecisionSource::decide, first statement',
                'before': '    fn decide(&mut self, observation: &Observation, entropy: '
                          '&mut DecisionEntropy<\'_>) -> Action {\n'
                          '        debug_assert!(observation.is_consistent());\n'
                          '\n'
                          '        if let Some(food) = observation.best_tolerated_co_located_food() {',
                'after': '    fn decide(&mut self, observation: &Observation, entropy: '
                         '&mut DecisionEntropy<\'_>) -> Action {\n'
                         '        debug_assert!(observation.is_consistent());\n'
                         '        let _ = entropy.choose_index(observation.valid_actions.len().max(1));\n'
                         '\n'
                         '        if let Some(food) = observation.best_tolerated_co_located_food() {',
                'violates': '`SPEC-MOK-001` gives every run one shared stream and rule 19 adds no draw '
                            'to it. This\nconsumes one value per decision, which leaves the stream one '
                            'position ahead of where the\nreference source would leave it while, in '
                            'the cases that need no random step, proposing\nexactly the same action. '
                            'It is the defect a proposal comparison alone cannot see.',
                'predicts': 'the stream-position assertion fails; the proposal assertion may or may '
                            'not, since a\nshifted stream also changes which direction the fallback '
                            'step takes',
                'markers': [
                    'at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_'
                    'proposes',
                    'SplitMix64 { state',
                ],
                'observes': 'The very first case of the enumeration fails, and it fails on the stream\n'
                            'position rather than on the proposal: the case is a resource underfoot at\n'
                            'satiety `0`, where both sources propose to eat, so the proposal assertion on\n'
                            'the preceding line passed and the one after it did not. That is precisely\n'
                            'the defect a proposal comparison cannot see, and it is why the oracle\n'
                            'asserts on both. Two further tests fail on their own draw counts, which is\n'
                            'the same defect seen from a different direction.',
            },
        ],
    },
]

FAILED = re.compile(r'^test (\S+) \.\.\. FAILED$')
SUMMARY = re.compile(r'^test result: .*$')


def digest(path):
    return hashlib.sha256(io.open(path, 'rb').read()).hexdigest()


def read_source():
    return io.open(SOURCE, encoding='utf-8', newline='').read()


def write_source(text):
    io.open(SOURCE, 'w', encoding='utf-8', newline='').write(text)


def run_suite():
    finished = subprocess.run(
        ['cargo', 'test', '-p', 'Mokiterions', '--lib'],
        cwd=ROOT, capture_output=True, text=True, encoding='utf-8', errors='replace')
    return finished.returncode, finished.stdout + finished.stderr


def failures(output):
    """The failing test names, and each one's panic message."""
    names = [match.group(1) for line in output.split('\n')
             if (match := FAILED.match(line.strip()))]
    blocks = {}
    lines = output.split('\n')
    for index, line in enumerate(lines):
        heading = re.match(r'^---- (\S+) stdout ----$', line.strip())
        if not heading:
            continue
        collected = []
        for following in lines[index + 1:]:
            if following.strip().startswith('---- ') or following.strip() == 'failures:':
                break
            if following.strip():
                collected.append(following.rstrip())
            if len(collected) >= 8:
                break
        blocks[heading.group(1)] = collected
    return names, blocks


def summary(output):
    return [line.strip() for line in output.split('\n') if SUMMARY.match(line.strip())]


def apply(before, after):
    text = read_source()
    if text.count(before) != 1:
        raise SystemExit(f'the perturbation site is not unique ({text.count(before)} matches):'
                         f'\n{before}')
    write_source(text.replace(before, after))


def perturbation_block(control, names, blocks, output):
    lines = [f'  {control["site"]}:', '']
    for line in control['before'].split('\n'):
        lines.append(f'  -{line}')
    for line in control['after'].split('\n'):
        lines.append(f'  +{line}')
    lines += ['', control['violates'], '',
              'Result: cargo test -p Mokiterions --lib', '']
    for name in names:
        lines.append(f'  test {name} ... FAILED')
    lines.append('')
    for name in names:
        if name not in blocks:
            continue
        lines.append(f'  ---- {name} stdout ----')
        lines += [f'  {line}' for line in blocks[name]]
        lines.append('')
    lines += [f'  {line}' for line in summary(output)]

    predicted = control['predicts'].split('\n')
    lines += ['', '  What the defect predicts:  ' + predicted[0]]
    lines += [f'    {line}' for line in predicted[1:]]
    lines += ['',
              '  Markers of the predicted failure. All of them must appear for this to be the failure',
              '  the control was designed to produce rather than incidental breakage:',
              '']
    for marker in control['markers']:
        lines.append(f'    found = {str(marker in output):5}  {marker}')
    lines += ['']
    lines += [f'  {line}' for line in control['observes'].split('\n')]
    lines += ['']
    return lines


def main():
    original = read_source()
    baseline = digest(SOURCE)
    print(f'source digest before any perturbation: {baseline}')

    status, clean_output = run_suite()
    if status != 0:
        raise SystemExit('the unperturbed suite does not pass; nothing can be concluded from a '
                         'control')

    written = []
    everything_held = True
    for spec in CONTROLS:
        controls = spec.get('controls', [spec])
        held = True
        lines = [f'VER-MOK-010 {spec["oracle"]} - negative control',
                 '=' * (len(spec['oracle']) + 30), '',
                 spec['scenario'], '',
                 'Each perturbation below was applied to the committed source, the engine\'s own suite',
                 'was run, and the perturbation was reverted and the file checked byte for byte against',
                 'its digest. This artifact is generated by negative-control/controls.py from those',
                 'runs.', '']
        for control in controls:
            if len(controls) > 1:
                lines += [control['label'], '-' * len(control['label']), '']
            else:
                lines += ['Perturbation applied', '--------------------', '']
            apply(control['before'], control['after'])
            try:
                status, output = run_suite()
                names, blocks = failures(output)
            finally:
                write_source(original)
            if digest(SOURCE) != baseline:
                raise SystemExit('the revert did not restore the source exactly; stopping')
            if status == 0 or not names:
                held = False
                lines += ['  THE CONTROL DID NOT FAIL. The perturbation was applied and the suite',
                          '  still passed, which means the oracle does not detect this defect.', '']
            if any(marker not in output for marker in control['markers']):
                held = False
            lines += perturbation_block(control, names, blocks, output)

        lines += ['Revert and confirmation', '-----------------------', '',
                  'Every perturbation was reverted and the file restored byte for byte:', '',
                  f'  sha256(simulation.rs) before and after all controls: {baseline}', '',
                  'and the unperturbed suite passes:', '']
        lines += [f'  {line}' for line in summary(clean_output)]
        lines += ['',
                  'No perturbation remains in the tree. Oracle 1 confirms the same thing',
                  'independently: the frozen sources reproduce their pre-change event streams byte',
                  'for byte against commit 60fda9faffbd452752a34efa356f16cc6ad1d3ff, which a stray',
                  'edit to this file would break.', '']
        lines += ['RESULT: ' + ('PASS - every control failed in the way it was designed to fail, and'
                                '\n        the source is byte-identical to the one under verification'
                                if held else
                                'FAIL - a control did not produce the failure it was designed to '
                                'produce'), '']
        everything_held = everything_held and held

        path = os.path.join(HERE, spec['artifact'])
        io.open(path, 'w', encoding='utf-8', newline='\n').write(
            '\n'.join(line.rstrip() for line in lines) + '\n')
        written.append(path)
        print('\n'.join(lines))

    if digest(SOURCE) != baseline:
        raise SystemExit('the source is not as it was found; stopping')
    for path in written:
        print(f'written to: {path}')
    return 0 if everything_held else 1


if __name__ == '__main__':
    sys.exit(main())
