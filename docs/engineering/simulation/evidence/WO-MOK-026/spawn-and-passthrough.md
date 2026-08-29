# Item 13: the spawn and the pass-through — `VER-MOK-018` case `S3a`

Measured at candidate `6e9ca13` on 2026-08-29. `S3a`'s own wording is that the prohibition is "checked
statically rather than assumed", and the four claims below are three source inspections and one run.

`SPEC-MOK-006` rule 1.2 and `ADR-MOK-003` put the process boundary in one place: the engine's **binary**
target. The library target and the observer must contain none.

## 1. The spawn appears only in the engine's binary target

`mokiterions-core/src/main.rs`, every line of it that names a process, an environment read or the spawn:

```
 26://! the pass-through is `std::process::Command`'s behaviour with no environment call made on it, so
 27://! this file contains no `env::var`, no `env` builder call and no variable name. `env::args` is the
 34:use std::process::{Child, Command as Spawn, ExitCode, Stdio};
121:    let arguments: Vec<String> = env::args().skip(1).collect();
447:    let mut child = match Spawn::new(&connector)
450:        .spawn()
```

Lines 26 and 27 are the file's own documentation. **One import, one `Spawn::new`, one `.spawn()`, and one
`env::args`** — and `env::args` is the command line, not the environment. There is **no `env::var`, no `env`
builder call on the command, and no variable name anywhere in the file**, which is what makes the
pass-through the platform's default behaviour rather than a copy this file performs.

## 2. The library target contains none

The engine's library tree is three files. Counted for `process::Command`, `Command::new` and `.spawn()`:

```
mokiterions-core/src/cli.rs:        0
mokiterions-core/src/lib.rs:        0
mokiterions-core/src/simulation.rs: 0
```

And the stronger reading, over the whole tree excluding `main.rs`, for **`std::process` or `std::env` in any
form**:

```
$ grep -rn "std::process\|std::env\|process::Command\|Command::new" --include=*.rs mokiterions-core/src/ \
    | grep -v '^mokiterions-core/src/main.rs'
(no output)
```

**Not one occurrence.** The library does not spawn, does not read the environment, and does not name the
types that would let it. `src/lib.rs` states the same thing in prose at two places; this is the measurement
behind the prose.

That is what makes `ConnectorPort` a port: it takes a reader and a writer, so the process that produced them
is the caller's business and the library never learns there was one.

## 3. The observer contains none

```
$ grep -rn "process::Command\|Command::new\|std::process" --include=*.rs mokiterions-tui/src/
mokiterions-tui/src/main.rs:27:use std::process::ExitCode;
```

Ten source files, one hit, and it is `ExitCode` — a status a program returns, not a process it starts. **No
`Command::new` and no `process::Command` anywhere in `mokiterions-tui/src/`.** The observer's own test
`a_refused_live_run_enters_no_terminal_and_spawns_nothing` asserts this same absence over every `.rs` file
under that directory on every `cargo test`, so it is a property the suite defends and not only a reading
taken once.

`REQ-MOK-077`'s prohibition is therefore checked rather than assumed, in both directions: the component that
may spawn does, once; the two that may not, do not at all.

## 4. The pass-through: a synthetic credential reaches the child and appears in no produced byte

`cases/passthrough/`, exit **`0`**. The parent's environment carried
`MOKITERIONS_TEST_CREDENTIAL=sk-canned-0000-authenticates-nothing`, and the connector's script was
`credential MOKITERIONS_TEST_CREDENTIAL` — the directive that answers normally when the named variable is
present and non-empty, and refuses when it is not.

**The child got it.** Every one of the run's twelve exchange records carries `"fallback":false`, and the only
way the fixture answers at all under that directive is by finding the variable in **its own** process
environment. The refusal case — same directive, variable removed — is row 3 of `gate-matrix.md`, and it
refuses on the first exchange. So the pass-through is measured by a difference between two runs rather than
by a claim about what `Command` does.

Nothing in the host performed that read. The engine's binary makes no `env::var` call and names no variable,
per §1: the child inherited the parent's environment because that is what `std::process::Command` does when
nothing is said about it.

**And it appears in no produced byte.** Searched over every file the run produced, for the whole value and
for its first twelve characters — the shape a truncated or partially escaped leak would take:

| produced | whole value | 12-character fragment |
|---|---|---|
| `cases/passthrough/transcript.jsonl` | **0** | **0** |
| `cases/passthrough/records.jsonl` | **0** | **0** |
| `cases/passthrough/stdout.txt` | **0** | **0** |
| `cases/passthrough/stderr.txt` | **0** | **0** |

The variable's **name** appears 0 times in this run's transcript, because the connector answered rather than
refusing; in row 3, where it refused, the name appears in all twelve records and the value appears nowhere,
which is rule 13.3's distinction holding in the one run where a connector had something to say about the
credential.

## What this does not cover

The value used here is the test suite's own invention and authenticates nothing. **The live run's credential
is a different value**, held only in the operator's environment, and it appears in no file of this
repository — that is `credential-attestation.md`'s subject, and the search over the live run's own produced
bytes is recorded there. This file establishes the mechanism; that one establishes the fact for the run that
cost money.

Nor does this establish anything about what an operator's connector does with a credential once it has one.
The connector is a separate program; `docs/CONNECTOR_PROTOCOL.md` asks it to emit none, and asking is all a
document can do.
