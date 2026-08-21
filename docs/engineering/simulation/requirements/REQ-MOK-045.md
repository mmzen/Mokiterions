+++
id = "REQ-MOK-045"
type = "requirement"
title = "Leave the observed run unchanged by observing it"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a structured record sink is configured, THE SYSTEM SHALL produce a standard-output text stream byte-identical to, and a per-tick entropy draw sequence identical to, the same run without a sink, and SHALL reproduce every run recorded before this capability existed byte for byte."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Leave the observed run unchanged by observing it

## Rationale

A measurement that changes what it measures is worthless twice over: the figures do not describe the unmeasured
system, and every figure gathered before the instrument existed is retired. This repository has already paid that
price once in advance and refused it — `REQ-MOK-031` forbade the trait derivation from touching the shared entropy
stream precisely because twelve extra draws at initialization would have shifted every value that followed and
retired `REQ-MOK-014`'s measured survivor floor.

The same exposure is here, in two forms. The obvious one is the text stream: the roadmap's constraint for this phase
is "preserve the existing text output so `REQ-MOK-010` remains satisfied — add, do not replace," and a structured
stream that cost the text stream one byte would break a satisfied requirement to add an unsatisfied one.

The subtler one is the entropy stream, and it is the one worth stating as a requirement. Nothing about writing
records *should* draw entropy. But the engine has one shared stream that orders resource placement, decision
fallbacks and regeneration, and a single accidental draw anywhere in the new code — a shuffle, a sampled subset, a
tie broken randomly — would silently produce a different world while every record in the stream looked entirely
plausible. The failure is invisible in the output it corrupts. That is exactly the class of defect that has to be
measured rather than reasoned about, so this requirement makes the draw count evidence.

The third clause extends the same test backwards. Every quantitative figure this repository has approved — the
density viability curve, `REQ-MOK-014`'s floor, the fifty-seed distribution that fixed `WASTE_TOLERANCE_MAX` at
`40`, the ninety-run projection check under `WO-MOK-011` — is a statement about a specific entropy sequence. If this
change moves that sequence, those numbers describe a world that no longer exists, and the cost of this phase becomes
re-deriving them. Requiring the retained captures to reproduce turns that risk into a pass-or-fail check.

## Preconditions and trigger

The trigger is any run, with or without a structured record sink configured, at any seed, density, decision source,
tick limit and tracing setting.

The requirement is a comparison, so it binds three things at once: a run with a sink against the same run without
one; a run in this build against the same run in the build preceding this capability; and any retained capture from
an earlier work order against a fresh run at the configuration that produced it.

## Required response

- With a sink configured, the bytes written to standard output are identical to the bytes the same run writes with
  no sink configured. Identical means equal as byte sequences: same records, same order, same field order, same
  separators, same line endings, same length.
- With a sink configured, the run consumes the same number of values from the shared entropy stream, in the same
  order, at the same points in the same ticks, as the same run with no sink configured. The stream's state at the
  end of each tick is the same in both.
- The exit code, the standard-error stream, and every simulation outcome — survivor count, death count, each
  Mokiterion's position, attributes and death tick, each territory's standing resources and depletion state — are
  the same with a sink and without one.
- With no sink configured, the run is indistinguishable in every observable respect from the same run in a build
  without this capability, including the bytes of standard output and standard error and the exit code.
- Every run retained as evidence before this capability existed reproduces byte for byte at the configuration that
  produced it. The cumulative counters `REQ-MOK-044` obliges the engine to retain draw no entropy and change no
  rule, no decision, no applied action and no emitted text record.
- No default changes. Absent the sink option the engine's behavior and its defaults are exactly what they were.

## Failure and boundary behavior

- A single differing byte in the text stream is a failure of this requirement, whatever its cause and however
  cosmetic it appears. There is no tolerance and no whitespace exemption.
- A single differing entropy draw is a failure, even where the run's outcome happens to be unchanged. Coincidental
  agreement is not the property; the sequence is.
- A run whose sink write fails still satisfies this requirement up to the point of failure: the text bytes written
  before the failure are the bytes the sinkless run writes. What happens after is `REQ-MOK-046`'s subject, and the
  additional standard-error diagnostic and exit code a failure produces are that requirement's, not a violation of
  this one.
- Where the sink is the same destination as standard output, this requirement is not satisfiable, because the two
  streams would interleave. The specification must therefore forbid that configuration rather than define its
  behavior.
- A retained capture that cannot be reproduced because its configuration is no longer accepted is a failure of this
  requirement, not an exemption from it. No option this capability adds may change what configurations are accepted.
- Performance is not the subject. A run may take longer with a sink configured; the requirement is about output and
  entropy, not about time.

## Constraints

- No simulation rule, survival value, resource table, density mapping, perception radius, regeneration schedule,
  decision source, default, or exit-code mapping for any case that exists today may change.
- No draw against the shared entropy stream may be added, removed, moved, or made conditional. In particular no
  record-writing path may draw, and no counter may be derived from a draw.
- No unordered collection may be iterated where the order reaches a record, a text line, or a decision. An
  iteration-order dependency is a determinism defect whether or not it manifests on the tested target.
- No wall-clock time, environment value, address value, process identifier, hostname, filesystem path or credential
  may reach any output stream or influence any behavior.
- The engine package's dependency table stays empty, so no dependency's internal ordering can perturb a run.
- The verification of this requirement must compare actual bytes and an actual draw count, not a derived summary.
  Asserting that no draw was added is not evidence that none was.

## Acceptance examples

### Example: normal behavior

**Given** each declared seed, each of the three decision sources, and tracing off and on

**When** each run is executed twice, once with a sink configured and once without

**Then** the two standard-output streams are byte-identical and the two runs' exit codes are equal.

### Example: the entropy stream is untouched

**Given** the same runs

**When** the number of values drawn from the shared entropy stream is counted per tick in each

**Then** the per-tick draw counts and their cumulative totals are equal in both, at every tick.

### Example: prior runs still reproduce

**Given** the retained run captures of `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011`, and the configurations recorded
with them

**When** each configuration is run again in this build with no sink configured

**Then** the output matches the retained capture byte for byte.

### Example: failure behavior

**Given** a run at any seed with a sink whose destination is the standard-output stream

**When** the configuration is supplied

**Then** it is rejected as an invalid configuration before the run begins, because a sink that interleaves with the
text stream cannot satisfy this requirement.

## Open decisions

None. How the draw count is observed, which seeds are declared, and how the sink destination is constrained are the
technical and assurance owners' to fix in `SPEC-MOK-006` and `VER-MOK-012`. The product decision — that measurement
must be free, and must be shown to be free rather than argued to be — is settled here and by `INT-MOK-009`
principles 1 and 8.
