"""VER-MOK-012: the failure captures, taken at the process boundary.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-019/analysis/capture-failures.py \
        <binary> <capture-dir> <output-file>

`VER-MOK-012` requires five failure captures -- sink not creatable, write failure mid-run, flush
failure, run-record write failure, and the reserved-spelling rejection -- "each with its standard
error, exit code, and the destination's state afterwards", plus the overwrite capture. The
corresponding `mokiterions-core/tests/records.rs` tests already pass; what those tests cannot show is
the process's own behaviour as an operator would meet it, which is what `REQ-MOK-046`'s matrix rows
are about. So every case below runs the shipped binary and records what came back.

## How a write is made to fail without changing the product

Three of the five failures are I/O errors from the platform, and a capture of them is worth nothing if
the product had to be modified to produce them: that would be a rehearsal of a failure, not an
observation of one. So the fault is external. A second process opens the destination and takes an
exclusive **byte-range lock** over a span of it; the engine's own writes into that span then fail with
the platform's `ERROR_LOCK_VIOLATION`. No engine code is compiled differently, no test double is
substituted, and the binary is the one `cargo build` produced.

Where the lock is placed decides which failure is observed, and the diagnostics tell them apart:

  * `runtime error: record sink: <reason>` carries no path. It can only be the engine's, because the
    engine is not told the destination's name -- `ARCH-MOK-001` keeps the filesystem out of the
    library, so the library cannot name a file. Its presence means a write the *engine* issued failed.
  * `runtime error: record sink <path>: <reason>` carries the path, so it is the host's, from the
    closing flush at `mokiterions-core/src/main.rs:101`.
  * The text stream is written before the record (`mokiterions-core/src/simulation.rs:1862`), so a
    complete text stream -- summary line included -- alongside a failed record write places the
    failure at the run record and nowhere earlier.

A lock over the stream's last byte is therefore a flush failure and nothing else: every write the
engine issued had already left the buffer. A lock from the run record's first byte to the end is a
run-record write failure when the host's buffer happens to fill inside the run record, which is why
the tick count below is one that was measured to do that rather than one chosen for looking round.

## What the lock cannot show

A locked destination has to exist before the lock is taken, so in every locked case the process did
not create the destination, and rule 13.4 then forbids removing it. The two removal outcomes are
captured separately, through a failure of the text stream, which needs no fault injection at all: the
parent closes its end of the pipe.
"""

import io
import msvcrt
import os
import subprocess
import sys

TICKS_THAT_FILL_THE_BUFFER_INSIDE_THE_RUN_RECORD = 200


def run(binary, arguments, close_stdout=False):
    """One invocation. Returns (exit code, stdout bytes, stderr text)."""
    if not close_stdout:
        completed = subprocess.run([binary, *arguments], capture_output=True)
        return completed.returncode, completed.stdout, completed.stderr.decode('utf-8', 'replace')

    # The child's reader goes away before it has written its stream, so the text stream fails
    # partway through. This is the same fault `a_failed_run_removes_the_destination_it_created`
    # uses, at the same place, and it needs nothing injected into the product.
    process = subprocess.Popen(
        [binary, *arguments], stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )
    process.stdout.close()
    # Read the diagnostic stream directly rather than through `communicate`, which would try to
    # drain the handle this line has just closed.
    error = process.stderr.read()
    process.stderr.close()
    process.wait()
    return process.returncode, b'', error.decode('utf-8', 'replace')


def run_against_a_lock(binary, arguments, destination, offset, length):
    """One invocation whose destination has a span of it locked by this process."""
    with io.open(destination, 'wb') as handle:
        handle.write(b'.' * (offset + length))
    handle = io.open(destination, 'r+b')
    handle.seek(offset)
    msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, length)
    try:
        return run(binary, arguments)
    finally:
        handle.seek(offset)
        msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, length)
        handle.close()


def destination_state(path):
    """The destination's state afterwards, in the terms rules 13.3 and 13.4 are stated in."""
    if not os.path.exists(path):
        return ['absent']
    if os.path.isdir(path):
        return ['present, and still a directory: the platform never opened it for writing']
    raw = io.open(path, 'rb').read()
    if not raw:
        return ['present, 0 bytes, empty']

    # A line is complete only if a newline follows it, so the last element of the split is the
    # partial tail and is empty exactly when the stream ends cleanly.
    parts = raw.split(b'\n')
    complete, partial = parts[:-1], parts[-1]

    state = [f'present, {len(raw)} bytes, {len(complete)} complete lines']
    state.append(f'first line: {parts[0][:72].decode("utf-8", "replace")}...')

    # Rule 13.4's condition, stated as the reader would meet it: a stream reads as a complete run
    # only if its last *complete* line is the run record. Testing for the run record's presence
    # anywhere would call a truncated run record a complete run, which is the flush-failure case
    # below -- 495 bytes of run record reached the file and the rest did not.
    reads_as_a_complete_run = bool(complete) and not partial and complete[-1].startswith(
        b'{"record":"run"'
    )
    state.append(f'reads as a complete run: {"yes" if reads_as_a_complete_run else "no"}')
    if not reads_as_a_complete_run and b'{"record":"run"' in raw:
        run_record = raw[raw.rindex(b'{"record":"run"'):]
        state.append(
            f'the run record began and did not finish: {len(run_record)} of its bytes are here'
        )

    if not partial:
        state.append('ends with a newline: yes')
    else:
        state.append(
            f'ends mid-record, {len(partial)} bytes into the last line: '
            f'...{partial[-48:].decode("utf-8", "replace")}'
        )
    return state


def stream_facts(binary, directory):
    """An unlocked run's total size and its run record's offset, measured not assumed."""
    path = os.path.join(directory, 'measure.jsonl')
    code, _, error = run(
        binary,
        [
            '--seed', '0',
            '--ticks', str(TICKS_THAT_FILL_THE_BUFFER_INSIDE_THE_RUN_RECORD),
            '--events-path', path,
        ],
    )
    if code != 0:
        raise SystemExit(f'the measuring run failed with {code}: {error}')
    raw = io.open(path, 'rb').read()
    os.remove(path)
    return len(raw), raw.rindex(b'{"record":"run"')


def main():
    binary, directory, output_file = sys.argv[1:4]
    binary = os.path.abspath(binary)
    os.makedirs(directory, exist_ok=True)
    size, run_record_at = stream_facts(binary, directory)

    def destination(label):
        return os.path.join(directory, f'{label}.jsonl')

    ticks = str(TICKS_THAT_FILL_THE_BUFFER_INSIDE_THE_RUN_RECORD)
    cases = []

    # ---------------------------------------------------------------------------------------
    # 1. Sink not creatable. Rule 13.2: exit 1, the platform's reason, no tick run.
    # ---------------------------------------------------------------------------------------
    a_directory = os.path.join(directory, 'a-directory')
    os.makedirs(a_directory, exist_ok=True)
    for path, why in (
        (
            os.path.join(directory, 'no-such-directory', 'records.jsonl'),
            'the parent directory does not exist',
        ),
        (
            a_directory,
            'the path names a directory the platform will not open for writing',
        ),
    ):
        arguments = ['--seed', '0', '--ticks', '50', '--events-path', path]
        code, out, error = run(binary, arguments)
        cases.append(
            (
                f'sink not creatable — {why}',
                'REQ-MOK-046 "Sink cannot be created"; SPEC-MOK-006 rule 13.2',
                arguments,
                code,
                out,
                error,
                path,
                ['no fault injected: the platform refuses the path'],
            )
        )

    # ---------------------------------------------------------------------------------------
    # 2. Write failure mid-run. The lock sits a third of the way into the stream, so the
    #    engine meets it while emitting event records and long before the run record.
    # ---------------------------------------------------------------------------------------
    mid = size // 3
    path = destination('write-failure-mid-run')
    arguments = ['--seed', '0', '--ticks', ticks, '--events-path', path]
    code, out, error = run_against_a_lock(binary, arguments, path, mid, size - mid)
    cases.append(
        (
            'write failure mid-run',
            'REQ-MOK-046 "Write fails mid-run"; SPEC-MOK-006 rules 13.3 and 13.4',
            arguments,
            code,
            out,
            error,
            path,
            [
                f'a byte-range lock over [{mid}, {size}) of a {size}-byte stream, held by a '
                'second process',
                'the engine meets the lock while writing event records',
            ],
        )
    )

    # ---------------------------------------------------------------------------------------
    # 3. Flush failure. The lock is one byte, the stream's last. Every write the engine issued
    #    has already succeeded; only the host's closing flush fails. Rule 13.3 forbids a retry.
    # ---------------------------------------------------------------------------------------
    path = destination('flush-failure')
    arguments = ['--seed', '0', '--ticks', ticks, '--events-path', path]
    code, out, error = run_against_a_lock(binary, arguments, path, size - 1, 1)
    cases.append(
        (
            'flush failure',
            'REQ-MOK-046 "Flush or close fails"; SPEC-MOK-006 rule 13.3',
            arguments,
            code,
            out,
            error,
            path,
            [
                f'a one-byte lock at offset {size - 1}, the last byte of a {size}-byte stream',
                'no engine-issued write touches it, so the failure is the closing flush and '
                'nothing else',
            ],
        )
    )

    # ---------------------------------------------------------------------------------------
    # 4. Run-record write failure. The lock begins at the run record's first byte, so every
    #    event record is written and the run record is not. Rule 8's stream then has no run
    #    record, which is rule 13.4's definition of a stream that must not survive as complete.
    # ---------------------------------------------------------------------------------------
    path = destination('run-record-write-failure')
    arguments = ['--seed', '0', '--ticks', ticks, '--events-path', path]
    code, out, error = run_against_a_lock(
        binary, arguments, path, run_record_at, size - run_record_at
    )
    cases.append(
        (
            'run-record write failure',
            'REQ-MOK-046 "Failure while writing the run record"; SPEC-MOK-006 rule 13.4',
            arguments,
            code,
            out,
            error,
            path,
            [
                f'a byte-range lock over [{run_record_at}, {size}) — the run record exactly, '
                'measured from an unlocked run of the same configuration',
                'the text stream completes, summary line included, so the failing write is the '
                "run record's",
            ],
        )
    )

    # ---------------------------------------------------------------------------------------
    # 5. Reserved-spelling rejection, and every other malformed spelling of the option.
    #    Rule 13.1: exit 2, the usage text, nothing run, no file anywhere.
    # ---------------------------------------------------------------------------------------
    reserved = destination('reserved')
    for label, arguments in (
        ('the reserved spelling "-", which does not name a file', ['--events-path', '-']),
        ('the empty path', ['--events-path', '']),
        ('no value at all', ['--events-path']),
        ('the next option taken as a value', ['--events-path', '--seed']),
        ('the option given twice', ['--events-path', reserved, '--events-path', reserved]),
    ):
        code, out, error = run(binary, ['--seed', '0', '--ticks', '50', *arguments])
        cases.append(
            (
                f'reserved-spelling rejection — {label}',
                'REQ-MOK-046 "Malformed argument"; SPEC-MOK-006 rule 13.1',
                ['--seed', '0', '--ticks', '50', *arguments],
                code,
                out,
                error,
                reserved,
                ['no fault injected: the argument itself is invalid'],
            )
        )

    # ---------------------------------------------------------------------------------------
    # 6. The overwrite capture: a prior run's file replaced, and none of it left behind.
    # ---------------------------------------------------------------------------------------
    path = destination('overwrite')
    first_arguments = ['--seed', '7', '--ticks', '400', '--events-path', path]
    code, _, _ = run(binary, first_arguments)
    if code != 0:
        raise SystemExit('the first run of the overwrite capture failed')
    before = io.open(path, 'rb').read()
    arguments = ['--seed', '1', '--ticks', '30', '--events-path', path]
    code, out, error = run(binary, arguments)
    after = io.open(path, 'rb').read()
    cases.append(
        (
            'overwrite — a prior run replaced',
            'REQ-MOK-046 "Existing destination overwritten"',
            arguments,
            code,
            out,
            error,
            path,
            [
                f'the destination held a 400-tick run of seed 7, {len(before)} bytes, before this '
                'invocation',
                f'afterwards it holds {len(after)} bytes',
                'the prior run survives in it: '
                + ('YES — the file was not replaced' if before[:200] in after else 'no'),
                'the prior run\'s seed reaches the new stream\'s header: '
                + ('YES' if b'"seed":7' in after else 'no'),
            ],
        )
    )

    # ---------------------------------------------------------------------------------------
    # 7. The two removal outcomes, from a text-stream failure. No fault injection: the reader
    #    of the child's standard output closes it.
    # ---------------------------------------------------------------------------------------
    path = destination('partial-removed')
    arguments = ['--seed', '0', '--ticks', '1000', '--trace-actions', '--events-path', path]
    code, out, error = run(binary, arguments, close_stdout=True)
    cases.append(
        (
            'partial stream removed — the process created the destination',
            'REQ-MOK-046 "Partial file removed"; SPEC-MOK-006 rule 13.4',
            arguments,
            code,
            out,
            error,
            path,
            ["the reader closes the child's standard output; the text stream fails partway"],
        )
    )

    path = destination('not-removed')
    io.open(path, 'wb').write(b"an operator's file, which this run must not delete\n")
    arguments = ['--seed', '0', '--ticks', '1000', '--trace-actions', '--events-path', path]
    code, out, error = run(binary, arguments, close_stdout=True)
    # Rule 13.4 bounds *removal*, not replacement: the option's contract requires an existing
    # destination to be replaced, so the operator's bytes are expected to be gone and the
    # operator's *file* is expected to still be there. Both halves are stated, because reporting
    # only the first would read as a defect and reporting only the second would hide a deletion.
    remaining = io.open(path, 'rb').read() if os.path.exists(path) else b''
    cases.append(
        (
            'partial stream not removed — the process did not create the destination',
            'REQ-MOK-046 "A destination the process did not create is not removed"; rule 13.4',
            arguments,
            code,
            out,
            error,
            path,
            [
                "the reader closes the child's standard output; the destination existed first",
                "the operator's earlier bytes: "
                + (
                    'still present — the destination was not replaced'
                    if b"an operator's file" in remaining
                    else 'replaced by this run, which the option requires'
                ),
                "the operator's file itself: "
                + ('removed — RULE 13.4 BREACHED' if not remaining else 'still there'),
            ],
        )
    )

    # ---------------------------------------------------------------------------------------
    # 8. A text-stream failure with no sink at all, so that rule 13.5's "distinguishable from a
    #    text-stream failure" can be read off two diagnostics rather than asserted.
    # ---------------------------------------------------------------------------------------
    code, out, error = run(
        binary, ['--seed', '0', '--ticks', '1000', '--trace-actions'], close_stdout=True
    )
    cases.append(
        (
            'a text-stream failure with no sink, for comparison',
            'REQ-MOK-046 "Diagnostics distinguishable"; SPEC-MOK-006 rule 13.5',
            ['--seed', '0', '--ticks', '1000', '--trace-actions'],
            code,
            out,
            error,
            None,
            ["no --events-path is given, so no destination exists to report on"],
        )
    )

    # ---------------------------------------------------------------------------------------
    # The write-up.
    # ---------------------------------------------------------------------------------------
    lines = [
        '# VER-MOK-012: the failure captures, taken at the process boundary',
        '#',
        f'# binary: {binary}',
        f'# capture directory: {directory}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-019/analysis/'
        'capture-failures.py <binary> <capture-dir> <output-file>',
        '#',
        '# Every case below runs the shipped binary. Where a write has to fail, the fault is a',
        '# byte-range lock held by this script over a span of the destination -- external to the',
        '# product, so no engine code is compiled differently and no test double is substituted.',
        f'# The measuring run of {ticks} ticks is {size} bytes and its run record begins at',
        f'# offset {run_record_at}; both figures are measured here, not assumed.',
        '',
    ]

    exit_codes = {}
    for title, provenance, arguments, code, out, error, path, notes in cases:
        exit_codes.setdefault(code, []).append(title)
        lines.append('=' * 100)
        lines.append(title)
        lines.append('=' * 100)
        lines.append(f'  provenance:  {provenance}')
        lines.append(f'  command:     Mokiterions {" ".join(repr(a) if a == "" else a for a in arguments)}')
        for index, note in enumerate(notes):
            lines.append(f'  fault:       {note}' if index == 0 else f'               {note}')
        lines.append(f'  exit code:   {code}')
        lines.append(f'  stdout:      {len(out)} bytes'
                     + (', carrying the summary line' if b'summary reason=' in out else ''))
        lines.append('  standard error, verbatim:')
        if error:
            for line in error.rstrip('\n').split('\n'):
                lines.append(f'    | {line}')
        else:
            lines.append('    | (empty)')
        lines.append('  the destination afterwards:')
        if path is None:
            lines.append('    no destination: the invocation names none')
        else:
            for line in destination_state(path):
                lines.append(f'    {line}')
        lines.append('')

    # Rule 13.5's "distinguishable from a text-stream failure", read off the captures. Every
    # diagnostic line above is reduced to its opening phrase, so that a reviewer can see the whole
    # vocabulary the process used rather than take the distinction on trust.
    # A diagnostic line is one that opens with `runtime error:` or `configuration error:`. Every
    # other line of a diagnostic stream is the usage text, which rule 13.1 requires and which is
    # not a diagnostic form -- classifying it as one would fill this table with the option prose.
    forms = {}
    for title, _, _, _, _, error, _, _ in cases:
        for line in error.rstrip('\n').split('\n'):
            if line.startswith('configuration error:'):
                form = 'configuration error: <what was wrong>, then the usage text'
            elif line.startswith('runtime error: record sink: '):
                form = 'runtime error: record sink: <reason>          (the engine: no path)'
            elif line.startswith('runtime error: record sink '):
                tail = line.split('record sink ', 1)[1]
                form = (
                    'runtime error: record sink <path>: not removed: … (the host: the bound on removal)'
                    if 'not removed:' in tail
                    else 'runtime error: record sink <path>: <reason>   (the host: the flush or the open)'
                )
            elif line.startswith('runtime error:'):
                form = 'runtime error: <reason>                       (not the sink: no sink is named)'
            else:
                continue
            forms.setdefault(form, set()).add(title)

    lines.append('=' * 100)
    lines.append('# rule 13.5: the diagnostic forms observed, and what each distinguishes')
    lines.append('=' * 100)
    for form in sorted(forms):
        lines.append(f'  {form}')
        for title in sorted(forms[form]):
            lines.append(f'       {title}')
    lines.append('')
    lines.append('# A sink failure names the sink; a text-stream failure does not. The comparison')
    lines.append('# case above fails its text stream with no sink at all and its diagnostic carries')
    lines.append('# no "record sink", so the two are told apart by reading the line and not by')
    lines.append('# knowing which run produced it.')
    lines.append('#')
    lines.append('# One sink failure can produce two lines, at two layers. The engine reports the')
    lines.append('# write it issued and cannot name the file, because ARCH-MOK-001 keeps the')
    lines.append('# filesystem out of the library; the host then reports its own closing flush,')
    lines.append('# with the path. They are two observations of one platform error, not two errors.')
    lines.append('')
    lines.append('=' * 100)
    lines.append('# exit codes observed, over every case above')
    lines.append('=' * 100)
    for code in sorted(exit_codes):
        lines.append(f'  {code}: {len(exit_codes[code])} cases')
        for title in exit_codes[code]:
            lines.append(f'       {title}')
    lines.append('')
    lines.append(f'# distinct exit codes: {sorted(exit_codes)}')
    lines.append('# SPEC-MOK-001 already defines 0, 1 and 2, and rule 13.6 forbids a new one.')
    lines.append('')
    lines.append('# ---- full text of the capture script, retained as VER-MOK-012 requires ----')
    lines.append('')
    lines.extend(io.open(__file__, 'r', encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(f'{len(cases)} cases, exit codes {sorted(exit_codes)}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
