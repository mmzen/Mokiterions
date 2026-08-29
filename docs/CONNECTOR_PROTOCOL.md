# The Mokiterions connector protocol

> **Authority note.** This document describes a contract fixed by `engineering/simulation/specifications/SPEC-MOK-007`
> rules 8 and 10, under `WO-MOK-026`. Where this document and that specification disagree, the specification is
> authoritative and this document has a defect. It authorizes nothing.
>
> **Corrected 2026-08-29** for rules 10.3a, 10.4a and 10.4c, which moved the provider binding out of the request: the
> request no longer carries `model` or `reasoning`, and the response carries both. Every other field name is unchanged,
> and deliberately so — rule 10.4b makes these names normative *as this document already documented them*, because it
> was published before that amendment and a connector may already have been written against it.

A **connector** is a program *you* write. Mokiterions spawns it, speaks to it over pipes, and never learns anything
about what is on the other side of it.

This document is meant to be sufficient on its own. If you have to read this repository's Rust to write a working
connector, that is a bug in this document.

## What a connector is for

The simulation engine decides nothing about behaviour. At each decision opportunity it composes a **request** — a
complete description of one Mokiterion's situation and the exact set of actions it is permitted to propose — and hands
it to a decision source. Four of the five sources are deterministic rule sets compiled into the engine. The fifth,
`--policy llm`, asks a language model.

The engine does not know how to talk to a language model, and deliberately never will:

- **It performs no network I/O and holds no HTTP client.** Neither Rust package may acquire a crate for one
  (`REQ-MOK-050`, `SPEC-MOK-002` rule 13, `SPEC-MOK-003`'s declared dependency set).
- **It reads no credential.** Not from a file, not from an option, not from the environment (rule 10.5).
- **It knows no endpoint, no vendor, no model identifier and no wire format** (rule 10.4c).

Your connector supplies all of that. The engine spawns it as a child process, writes request lines to its standard
input, and reads response lines from its standard output. That is the entire interface.

```text
    Mokiterions engine                    your connector                 the provider
    ------------------                    --------------                 ------------
    composes a request
        |
        |  one JSON object per line
        +---------- stdin ------------>  reads the line
                                          adds the credential
                                          from its own environment
                                          translates to the provider's API
                                                |
                                                +----- however you like ----->
                                                <----------------------------+
                                          translates the answer back
                                          and names which model answered
        <--------- stdout ------------  one JSON object per line
        |
    validates it like any
    other source's proposal
```

## How your connector is started

**The engine starts the value of `--connector-path` directly, and passes it no arguments.** Whatever that option names
has to be something the operating system can execute on its own.

That is worth spelling out, because on Windows it excludes the obvious choice. A `.py`, `.js` or `.sh` file is not
directly executable there, and naming one produces `runtime error: connector <path>: %1 is not a valid Win32
application` — a message about the platform, not about this protocol. Two ways round it, and no others are needed:

- **Ship an executable.** A compiled binary, or on Unix a script with a `#!` line and the execute bit set.
- **Name a wrapper.** On Windows a two-line `.cmd` file that invokes the interpreter works. Begin it with `@echo off`:
  without that the shell echoes each command to standard output, and standard output is the protocol channel.

Four more facts about the child, each of which decides something a connector author would otherwise guess at:

- **It is started once per run**, before the first tick, and it lives until the run ends. It is not restarted per tick,
  per actor or per exchange, so anything expensive to set up should be set up once.
- **Only two pipes are connected**: your standard input and your standard output. **Standard error is inherited** — it
  goes to the operator's terminal, and the engine never reads it. Write diagnostics there freely.
- **Your environment and your working directory are the engine's**, unmodified. That is how the credential reaches you
  (see [The credential](#the-credential)). Do not depend on the working directory; an operator may run the engine from
  anywhere.
- **Your exit status is reported and does not fail the run.** The engine closes your standard input, waits for you, and
  writes `connector <path>: exited with status <n>` to standard error if you exit nonzero. By that point every exchange
  has already happened, so a nonzero exit is a diagnostic and not a failure. Exit `0` anyway.

## Framing

**One JSON object per line, in each direction, in the same order.** Rule 10.2.

- Every line is terminated by a single `\n`.
- No object contains a raw newline. Escape newlines inside string values as `\n`.
- Requests arrive in order. Responses must leave in the same order. There is exactly one response per request.
- The engine writes one request and waits for one response before writing the next. You do not need to handle
  pipelining, and you must not reorder.
- Encoding is UTF-8.

When the engine has no more work it closes your standard input. Read to end-of-file and exit `0`.

## The request object

Rule 10.3, with the field names fixed by rule 10.3a. Every field is present on every request.

| Field | Type | Meaning |
|---|---|---|
| `protocol` | integer | Protocol version. `1` today. Refuse a version you do not implement. |
| `tick` | integer | The simulation tick this opportunity belongs to. Informational. |
| `actor` | string | The Mokiterion's identifier, e.g. `"M03"`. Informational. |
| `prompt` | string | **The entire prompt.** Send it to the model unchanged. |
| `schema` | object | A JSON Schema describing the response `action` object. Pass it to the provider's structured-output facility (rule 8.4). |

Those five are all of them. **The request names no model and no reasoning level.** Until 2026-08-29 this table listed
`model` and `reasoning` too, and both were a defect rather than a convenience: the engine holds no provider binding, so
it had nothing to put in them. Rule 10.3a removed them.

**`prompt` is composed, ordered and complete.** It is built from four blocks — the shared rules, the actor, the
observation, and the permitted set — in an order rule 3 fixes so that the longest identical leading span is shared
between consecutive requests. That ordering exists so the provider's prompt cache can match it, and
`REQ-MOK-070` obliges the run to hold a cache ratio of 0.85 or better.

> **Do not modify the prompt.** Do not reorder it, prepend a system message before it, append instructions to it,
> template it, or strip whitespace from it. Every one of those breaks the shared prefix, collapses the cache ratio,
> and costs the operator real money. If your provider requires a system role, put the whole of `prompt` in it, or the
> whole of it in a user message — but send it as one contiguous string either way.

`schema` is provided so you do not have to construct it. It describes exactly the response grammar of the next
section.

**The request carries no credential** and never will (rule 10.3).

**The provider binding is yours in whole.** The model, the reasoning level and the endpoint are decided in your
connector, which is what `ADR-MOK-007` decision 3 fixes and what keeps the engine free of a provider binding of any
kind (rule 10.4c). Nothing here tells you which model to call. Those two values instead travel the other way: you
report on each response which model answered and at what reasoning level, and the run record names *that* rather than
naming what something asked for. Rule 8.5 fixes the level a run may use at `none` today — the repository owner's
decision, recorded in the specification rather than left to a connector to pick — but it is your connector that carries
that out and your response that shows it did.

## The response object

Rule 10.4, with the field names fixed by rule 10.4a. Exactly one of `action` or `error` is present.

### Success

```json
{"protocol":1,"action":{"verb":"eat","parameter":"F1042"},"model":"gpt-5.6-luna","reasoning":"none","usage":{"prompt":3120,"cached_prompt":2944,"output":11,"reasoning":0}}
```

| Field | Type | Meaning |
|---|---|---|
| `protocol` | integer | `1`. |
| `action.verb` | string | One of the eleven verbs below. |
| `action.parameter` | string | Present only for a verb that takes one. |
| `model` | string | **The model that answered**, as the provider identified it. |
| `reasoning` | string | The reasoning level it answered at. `"none"` today (rule 8.5). |
| `usage.prompt` | integer | Prompt tokens **as the provider reported them**. |
| `usage.cached_prompt` | integer | Cached prompt tokens, as reported. |
| `usage.output` | integer | Output tokens, as reported. |
| `usage.reasoning` | integer | Reasoning tokens, as reported. |

**`model` and `reasoning` accompany every `action`** (rules 10.4a, 10.4c), added 2026-08-29 in the same amendment that
removed them from the request. Report the identifier the provider gave back rather than the one you sent it, where the
two can differ: a provider that resolves an alias to a dated build has told you something the run record should carry.
A run record naming the wrong model is worse than one naming none, which is why the report follows the answer.

Note that **`reasoning` appears at two levels and means two different things**: the top-level `reasoning` is the level,
a string, while `usage.reasoning` is a count of reasoning tokens. Both are on the same response. Rule 8.5 fixes the
level at `none` today and `usage.reasoning` is where a run shows that it got what it asked for.

A response that carries an `action` and does not name what answered fails the grammar check exactly as a missing action
would, and the opportunity becomes a counted fallback (rules 10.4c, 9.5). An `error` response carries neither field:
there was no answer to name.

**The usage counts must be the provider's own figures, passed through unchanged** (rule 10.4). Do not estimate them,
do not round them, do not recompute them from the text. The engine computes cost and the cache ratio from these
numbers and from nothing else, and `REQ-MOK-070`'s obligation is measured from them.

If your provider reports no cached-token figure, report `cached_prompt` as `0` rather than inventing one. A run that
cannot compute the ratio is a failure to evaluate, not a pass (rule 14.5).

### Error

```json
{"protocol":1,"error":{"kind":"transport","message":"connection reset"}}
```

`kind` is one of `transport`, `provider`, `malformed` or `refused`. `message` is short, human-readable, and
**contains no credential** (rule 19.7).

A `transport` error is retried a bounded number of times, and each attempt is recorded (rule 19.5). When retries are
exhausted the run continues with a counted fallback rather than dying — a run of an estimated 10,954 exchanges that
died on its first timeout would be an instrument nobody could use.

## The action grammar

Rule 8. **The grammar is closed.** The eleven verbs are:

```text
wait      sleep     eat <food-id>        move <direction>
attack <mokiterion-id>     fight <mokiterion-id>      threaten <mokiterion-id>
retreat <mokiterion-id>    surrender <mokiterion-id>  approach <mokiterion-id>
avoid <mokiterion-id>
```

A response is well-formed only if its verb is one of the eleven **and** its parameter is one the request's permitted
set enumerated for that verb. The permitted set is inside `prompt`, block D, and it enumerates every action the engine
will accept at this opportunity — `eat` against each co-located food identifier, `move` against each valid direction,
and each targeted verb against each perceived Mokiterion whose precondition it satisfies.

Three consequences worth stating plainly:

1. **You never need knowledge the request does not contain.** Block A and block D together are sufficient to produce a
   well-formed response (rule 7.6).
2. **A verb no target satisfies is not offered.** The permitted set never contains an action the engine would reject on
   a ground it could have known about (rule 7.4).
3. **A well-formed action can still be rejected.** A move into a cell holding something the observation does not carry
   is rejected by the engine's own rules. That is an ordinary rejected proposal, it is not your error, and it is not
   counted as a fallback (rule 7.5).

**No prose.** The response carries no explanation, no confidence and no alternatives (rule 8.3). A reason nothing
consumes is output tokens spent to no effect, and a second thing a later reader might mistake for evidence about the
decision.

## The credential

**Your connector reads the credential from its own process environment, and nothing else in this system reads it at
all.** Rule 10.5.

The engine passes its own environment through to the child it spawns. That is how a credential reaches you without any
component in this repository naming, parsing, logging or storing one. Neither Rust target reads it, no command-line
option carries it, and it appears in no request object.

Your obligations:

- Read it from the environment, on your own variable name.
- Never write it to standard output, to standard error, or to any file.
- Never include it in an `error.message`.

Nothing in this repository can enforce any of that, which is exactly why it is written down.

## Two limits, stated rather than defended

**Your output is untrusted in whole.** Rule 10.7. Every response passes the grammar check and then the engine's own
validation, unchanged, exactly as a compiled-in source's proposal does — the usage counts and the success flag
included. `ADR-MOK-001` fixes model output as untrusted input, and a connector is a program the operator supplied and
not a component this repository verified.

**The spend ceiling protects against an honest connector, not a dishonest one.** Rule 10.8. Cost is computed from the
usage counts you report, so a connector that under-reports usage spends past the operator's ceiling and the run cannot
tell. This is recorded as a limit rather than defended against: you write the connector, and a containment that
assumes otherwise would be theatre. What does not depend on your behaviour is rule 13's two gates — a live run happens
only with an explicit live-mode selection *and* a credential present — and the absence of any credential in this
repository's automation.

## A worked exchange

Request, one line (wrapped here for reading only):

```json
{"protocol":1,"tick":42,"actor":"M03",
 "prompt":"You control one creature in a closed world...\n\n--- YOU ---\nid: M03\n...\n--- PERMITTED ---\nwait\nsleep\neat F1042\nmove north\nmove east\navoid M07\n",
 "schema":{"type":"object","required":["verb"],"properties":{"verb":{"enum":["wait","sleep","eat","move","attack","fight","threaten","retreat","surrender","approach","avoid"]},"parameter":{"type":"string"}}}}
```

Response, one line (wrapped here for reading only):

```json
{"protocol":1,"action":{"verb":"eat","parameter":"F1042"},
 "model":"gpt-5.6-luna","reasoning":"none",
 "usage":{"prompt":3120,"cached_prompt":2944,"output":11,"reasoning":0}}
```

That exchange contributes 3120 prompt tokens and 2944 cached to the run's totals, and its cost is computed from those
counts and the unit prices the operator declared. The cache ratio for this exchange alone is 0.943. The run record will
name `gpt-5.6-luna` at reasoning level `none` because this response said so, and for no other reason.

## Checklist for a new connector

- [ ] Is directly executable by the platform, taking no arguments — a wrapper if the implementation is a script.
- [ ] Reads one line at a time from standard input; exits `0` at end-of-file.
- [ ] Writes exactly one response line per request, in order.
- [ ] Sends `prompt` to the model **byte-for-byte unchanged**.
- [ ] Uses the provider's structured-output facility with the supplied `schema`.
- [ ] Reports the provider's own token counts, unmodified.
- [ ] Reports `cached_prompt` as `0` when the provider gives no figure, never an estimate.
- [ ] Names the model that answered and the reasoning level it answered at, on every successful response.
- [ ] Reads its credential from its own environment only.
- [ ] Emits no credential on any stream, in any message, ever.
- [ ] Returns `error` rather than crashing on a transport failure, so the run can retry.
- [ ] Writes nothing to standard output that is not a response line. Diagnostics go to standard error.

## The canned connector

This repository owns exactly one connector: a test fixture that speaks this protocol and answers from a script, with
no provider, no credential and no network. It exists so the spawn, the framing, the gates, the accounting, the ceiling
and the retry path can all be exercised for free, and it can be told to fail on command.

It is **not** a reference implementation of a real connector and it establishes nothing about yours. `VER-MOK-018`'s
case `S2` checks the canned connector and says so plainly: rule 10.6 is explicit that this specification does not
constrain, and cannot constrain, the dependency surface of a program the operator supplies.
