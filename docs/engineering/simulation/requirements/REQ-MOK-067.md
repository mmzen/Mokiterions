+++
id = "REQ-MOK-067"
type = "requirement"
title = "Replay a recorded model-backed run byte-identically, offline"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a simulation is run again with the same seed, the same configuration and the same retained transcript, THE SYSTEM SHALL produce byte-identical standard output, a byte-identical structured record stream and the same exit code, and SHALL make no provider call while doing so."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Replay a recorded model-backed run byte-identically, offline

## Rationale

This repository's determinism promise is `INT-MOK-001`'s: repeated runs at an identical seed give 100% identical
results, and `REQ-MOK-009` holds the entropy stream reproducible. A model provider gives no such promise. Temperature
and seed support are not documented for `gpt-5.6-luna`, and `ROADMAP.md` already recorded that temperature 0 *"is not a
bitwise determinism guarantee from any provider"* even where it is offered.

The resolution is to stop asking the provider for determinism and to record its answers instead. Then the transcript
is a **run input**, in the same class as the seed and the density: given the same inputs the run is identical, and the
engine's entropy stream is untouched. **That reframing is what leaves `REQ-MOK-009` unamended**, and it narrows the
determinism change to one sentence of `INT-MOK-001`'s success measure, where the determinand becomes seed *and*
transcript.

The practical consequence is the one that matters for evidence. A live run is a **recording** run; a verified run is a
**replay** run. Verification never needs a credential, a network or a budget, and a reader years later can reproduce a
published figure exactly from files in hand. Without this property a model-backed figure would be an event that
happened once and could not be examined again.

## Preconditions and trigger

- A transcript retained from an earlier run of the same seed, tick limit, density and tracing selection.
- No provider credential need be present, and no network need be reachable.

## Required response

1. Read decisions from the transcript in the order the run consumed them, matching each to the opportunity it belongs
   to.
2. Produce standard output byte-identical to the recorded run's, a structured record stream byte-identical to it where
   one was written, and the same exit code.
3. Make no provider call, open no socket and read no credential.

## Failure and boundary behavior

- **The transcript ends before the run does.** The replay fails and says so, naming the opportunity it could not
  satisfy. It does not silently shorten the run, and it does not substitute a rule-based proposal.
- **The transcript's recorded opportunity does not match the one the engine has reached** — a different tick,
  Mokiterion or observation. The replay fails and names the mismatch. A transcript from a different seed or density is
  detected here rather than producing a plausible wrong run.
- **The transcript is longer than the run needs.** The surplus is unused and the run's output is unaffected. A run
  that ends early through extinction leaves a tail, and that is not an error.
- **The transcript was recorded with tracing enabled and the replay disables it.** Standard output differs by
  specification, because tracing changes what is printed. Byte-identity is claimed for matched configurations only, and
  the configuration includes the tracing selection.

## Constraints

- Replay uses the same code path as recording. The engine sees one port either way, and no mode flag reaches the
  simulation rules; what changes is which stream the host connected. This is what makes the property structural rather
  than a second implementation that must be kept in agreement.
- The engine resolves no path and opens no file for the transcript. The host owns it, as `SPEC-MOK-006` rule 1.2
  already requires for the record sink.
- Byte-identity is claimed against the standard output stream, the structured record stream and the exit code. It is
  not claimed for standard error, whose content may carry platform text.
- A retained transcript is provenance once a verification record binds it. `REQ-MOK-069` fixes what it contains and
  `VER-MOK-018` fixes what is retained where.

## Acceptance examples

### Example: normal behavior

**Given** a transcript retained from a run at seed 0, 20 ticks, default density, tracing on

**When** the run is repeated from that transcript with no credential in the environment and no network available

**Then** standard output and the record stream compare equal byte for byte with the retained originals, and the exit
code is the same.

### Example: failure behavior

**Given** the same transcript

**When** the run is repeated at seed 1

**Then** the replay fails at the first opportunity whose recorded observation does not match, and reports the mismatch
rather than producing a run.

## Open decisions

None.
