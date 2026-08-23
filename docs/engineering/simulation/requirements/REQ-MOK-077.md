+++
id = "REQ-MOK-077"
type = "requirement"
title = "Replay a model-backed run in either host, and run live only from the engine's binary"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "THE SYSTEM SHALL make the model-backed decision source available for replay in both the engine binary and the terminal observer, SHALL make a live run available in the engine binary only, and WHEN the observer is asked for a live run THE SYSTEM SHALL refuse before the terminal is entered and state that this host replays only."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Replay a model-backed run in either host, and run live only from the engine's binary

## Rationale

This repository has two hosts of one engine library, and they are not equally able to carry this source. The engine's
own binary target streams a whole run to standard output; the terminal observer advances one tick at a time and renders
frames. `SPEC-MOK-003` rules 6.1 and 6.2 oblige the observer to render a frame every 33 milliseconds and to poll input
every 16.

A live exchange takes an **estimated** 0.4 to 0.8 seconds, and a tick holds an **estimated** eleven decision
opportunities — 10,954 measured over a 1,000-tick run. One live tick would therefore hold the observer's loop for an
**estimated** 4 to 9 seconds, rendering nothing and accepting no key press, including the key press that quits. That
misses both budgets by two orders of magnitude, so it is not a tuning problem. The two remedies are concurrency and an
asynchronous runtime: `SPEC-MOK-007` rule 16 forbids the first and `REQ-MOK-050` the second.

**Replay has none of that cost**, so the split is not between hosts but between modes, and it lets the observer do the
thing an observer is for: watch a model-backed run unfold, tick by tick, with the roster, the map, the event log and the
inspector all working as they already do — for free, offline, from a committed transcript.

This is stated as a requirement rather than left to the specification for two reasons. It carries a **prohibition** that
no other requirement here implies: `REQ-MOK-067` says a recorded run replays byte-identically, which is silent on which
host may record. And without it, a host would offer an operator a source it cannot run — the failure mode this
requirement exists to name.

## Preconditions and trigger

- The model-backed source is selected in either host.
- For replay: a retained transcript matching the run's seed, tick limit, density and tracing selection.
- For a live run: the engine's binary target, plus `REQ-MOK-072`'s two conditions and `REQ-MOK-071`'s declared ceiling.

## Required response

1. **In the engine's binary target**: accept the source for a live run and for a replay.
2. **In the terminal observer**: accept the source for a replay, and present the run through every pane the observer
   already presents, with no pane, key binding or export behaving differently because the decisions came from a model.
3. **In the terminal observer, asked for a live run** — by a connector path, a live-mode selection or a spend ceiling —
   refuse before the terminal is entered, exit with the invalid-configuration status, and state on standard error that
   this host replays only.
4. **In either host**, refuse the source with no transcript and no live capability rather than beginning a run whose
   decisions cannot be obtained.

## Failure and boundary behavior

- **The observer is given the source and no transcript.** It refuses at start-up and names the missing transcript. It
  does not substitute another source: a substituted source would present a run under the wrong label, which is what
  `REQ-MOK-074` refuses for the fallback case and this refuses for the same reason.
- **The observer is given a connector path.** It refuses with a diagnostic. It does **not** accept the option and act on
  nothing: `SPEC-MOK-003`'s *Start-up inputs* records that outcome as a defect for `--events-path`, tracked as GitHub
  issue 40, on the ground that "an operator who passes the option and receives no file and no diagnostic is worse served
  by silence". Repeating it in the same file would be a known defect knowingly added.
- **A host offers the source and does not wire the port into the entry point it uses.** The run refuses as an invalid
  configuration rather than falling back, producing no decisions or quietly running another source. This is the failure
  this requirement is most likely to meet in practice, because it compiles and runs.
- **A replay is slower than the observer's speed setting asks for.** Reading a transcript record is not a provider call
  and costs no network; if a transcript is large enough to matter, the observer falls behind exactly as it already may on
  a fast speed setting, which `SPEC-MOK-003` rule 1 governs unchanged. No new behaviour is required here.

## Constraints

- **The observer spawns no process, reads no credential, takes no ceiling and has no live mode.** It gains the ability to
  read one committed file, and nothing else.
- **The refusal is not a fallback and not a warning.** It is an invalid configuration, exit status `2`, before the
  terminal is entered — the status and the timing `SPEC-MOK-003`'s *Start-up inputs* already fixes for every invalid
  input.
- **Both of the engine's run entry points carry the port.** The whole-run path and the single-tick path are the two
  doors the two hosts enter by, and wiring one excludes the other host entirely while every other obligation still reads
  as satisfied.
- **`SPEC-MOK-003` rules 6.1 and 6.2 are not relaxed, scoped or excepted.** This requirement is satisfied by leaving
  them intact.
- The observer's authority mapping gains the fifth source, so the provenance footer keeps naming the requirement behind
  what it presents.

## Acceptance examples

### Example: normal behavior

**Given** a transcript retained from a 20-tick run at seed 0 and the default density

**When** the observer is started with the model-backed source and that transcript, with no credential in the environment
and no network reachable

**Then** the run advances tick by tick under the operator's control, every pane presents it as it presents a `social`
run, the provenance footer names the fifth source, and the run reaches tick 20 having made no provider call.

### Example: failure behavior

**Given** the same transcript

**When** the observer is started with the model-backed source, that transcript **and** a connector path

**Then** it exits `2` before entering the terminal, writes to standard error that this host replays only and that a live
run is the engine binary's, and starts no run and no child process.

## Open decisions

None. The horizon at which a live run in the observer might become affordable is not an open decision here: it is
governed by exchange latency, which is the provider's, and by rules 6.1 and 6.2, which are the technical owner's.
