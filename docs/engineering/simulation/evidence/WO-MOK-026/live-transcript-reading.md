# VER-MOK-018 cases L4, L5, L6, L12, L13, L14, L15a, L17 and P1: the reading over the live
# transcripts, and one FAIL retained exactly as the instrument printed it
#
# WO-MOK-026's required verification asks for L16 and L17 "re-run against a live transcript
# rather than a stub's", and for P1 re-run against the live transcript. This is that reading.
# The instrument is scripts/check_transcript_reading.py, unchanged: WO-MOK-025 built it from
# SPEC-MOK-007 rule 7.3 and SPEC-MOK-001 rule 21 alone, sharing no code with the engine, so an
# agreement here is two readings of a rule agreeing rather than one reading checked against
# itself. Its own 36-test self-test passes at this candidate:
#
#     python scripts/test_check_transcript_reading.py   ->  Ran 36 tests OK
#
# candidate: 6e9ca13ba70ec46696113bb742f45d78d602d41e
# read on:   2026-08-29
#
# Three transcripts are read: the accepted live run, the rejected attempt 1 retained beside it,
# and WO-MOK-025's synthetic fixture re-read at this candidate so that the two can be compared
# rather than described. Nothing in the instrument was changed for any of them.
#
# THE HEADLINE, BEFORE THE OUTPUT: L17 FAILS on both live transcripts and passes on the
# synthetic one. The measurement of why is below the three readings, and it is not a
# floating-point value in the transcript. It is the model identifier gpt-5.6-luna, whose
# version number is a digit-dot-digit sequence inside a string. The instrument is right about
# what it saw and it is not mine to relax; the finding is escalated rather than resolved.
#

================================================================================================
THE ACCEPTED LIVE RUN  --  exit 1
================================================================================================

transcript: docs/engineering/simulation/evidence/WO-MOK-026/live-run-transcript.jsonl
records: 515 (12 prefix, 503 exchange)
requests examined: 503

  PASS  L15a the layout holds  shared block 5385 bytes, identical across all 12 prefix records; actor blocks 39 to 40 bytes (12 Mokiterions); every exchange names its own prefix and its digest
  PASS  L4 the enumeration is complete  all 503 requests agree with the independent enumerator
  PASS  L6 no unsatisfiable offer  no offer among 503 requests names a target whose precondition is unmet
  PASS  L5 the enumeration is not the core list  367 of 503 requests enumerate a targeted action; of the 367 whose observation carries a perceived Mokiterion, 0 have a set equal to the core-proposal list. Under the case's literal wording the figure is 136 of 503, every one of them a request with nothing to target; see this program's LIMITS
  PASS  L12 one Mokiterion per request  every line of all 503 observations is one of rule 6.1's forms, carrying the acting Mokiterion's own attributes or another's identifier, direction and distance
  PASS  L13 no aggregate and no derived value  no aggregate vocabulary, no figure on a list heading and no response text in any of 503 requests; nothing from another exchange has a line to occupy, which is how the closed grammar above discharges the clause
  PASS  L14 self-contained requests  12 Mokiterions acted more than once; each keeps its prefix and its digest, differs in the observation, and holds no part of its own other request; no record carries a conversation or session identifier
  FAIL  L17 the transcript's constraints  record 13.response: a decimal fraction, '5.6'; record 14.response: a decimal fraction, '5.6'; record 15.response: a decimal fraction, '5.6'; record 16.response: a decimal fraction, '5.6'

FAIL: 1 check(s) did not hold

================================================================================================
THE REJECTED ATTEMPT 1  --  exit 1
================================================================================================

transcript: docs/engineering/simulation/evidence/WO-MOK-026/attempt-1/live-run-transcript.jsonl
records: 579 (12 prefix, 567 exchange)
requests examined: 567

  PASS  L15a the layout holds  shared block 5385 bytes, identical across all 12 prefix records; actor blocks 39 to 40 bytes (12 Mokiterions); every exchange names its own prefix and its digest
  PASS  L4 the enumeration is complete  all 567 requests agree with the independent enumerator
  PASS  L6 no unsatisfiable offer  no offer among 567 requests names a target whose precondition is unmet
  PASS  L5 the enumeration is not the core list  473 of 567 requests enumerate a targeted action; of the 473 whose observation carries a perceived Mokiterion, 0 have a set equal to the core-proposal list. Under the case's literal wording the figure is 94 of 567, every one of them a request with nothing to target; see this program's LIMITS
  PASS  L12 one Mokiterion per request  every line of all 567 observations is one of rule 6.1's forms, carrying the acting Mokiterion's own attributes or another's identifier, direction and distance
  PASS  L13 no aggregate and no derived value  no aggregate vocabulary, no figure on a list heading and no response text in any of 567 requests; nothing from another exchange has a line to occupy, which is how the closed grammar above discharges the clause
  PASS  L14 self-contained requests  12 Mokiterions acted more than once; each keeps its prefix and its digest, differs in the observation, and holds no part of its own other request; no record carries a conversation or session identifier
  FAIL  L17 the transcript's constraints  record 13.response: a decimal fraction, '5.6'; record 14.response: a decimal fraction, '5.6'; record 15.response: a decimal fraction, '5.6'; record 16.response: a decimal fraction, '5.6'

FAIL: 1 check(s) did not hold

================================================================================================
WO-MOK-025'S SYNTHETIC FIXTURE, RE-READ  --  exit 0
================================================================================================

transcript: mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl
records: 233 (12 prefix, 221 exchange)
requests examined: 221

  PASS  L15a the layout holds  shared block 5385 bytes, identical across all 12 prefix records; actor blocks 39 to 40 bytes (12 Mokiterions); every exchange names its own prefix and its digest
  PASS  L4 the enumeration is complete  all 221 requests agree with the independent enumerator
  PASS  L6 no unsatisfiable offer  no offer among 221 requests names a target whose precondition is unmet
  PASS  L5 the enumeration is not the core list  104 of 221 requests enumerate a targeted action; of the 104 whose observation carries a perceived Mokiterion, 0 have a set equal to the core-proposal list. Under the case's literal wording the figure is 117 of 221, every one of them a request with nothing to target; see this program's LIMITS
  PASS  L12 one Mokiterion per request  every line of all 221 observations is one of rule 6.1's forms, carrying the acting Mokiterion's own attributes or another's identifier, direction and distance
  PASS  L13 no aggregate and no derived value  no aggregate vocabulary, no figure on a list heading and no response text in any of 221 requests; nothing from another exchange has a line to occupy, which is how the closed grammar above discharges the clause
  PASS  L14 self-contained requests  12 Mokiterions acted more than once; each keeps its prefix and its digest, differs in the observation, and holds no part of its own other request; no record carries a conversation or session identifier
  PASS  L17 the transcript's constraints  no floating-point value, no timestamp and no path across 233 records and 1828 text values. The case's fourth clause is withdrawn: rule 11.4.1 replaced the closed alphabet with a round trip through `escape_transcript_text`, because blocks A to D are English prose — this transcript carries 24 character(s) outside ASCII by design — and that round trip is checked in-crate by `the_escaping_survives_the_framing_and_round_trips`, not here

PASS: VER-MOK-018 cases L4, L5, L6, L12, L13, L14, L15a and L17 hold over 1 retained transcript(s)
NOT CHECKED HERE: whether the transcript is the run it claims to be, and whether two
  transcripts of one recorded run compare equal, both of which are
  mokiterions-core/tests/replay.rs; L5's literal wording; and L17's closed-alphabet
  clause, which SPEC-MOK-007 rule 11.4.1 withdrew. On the last two this program reports
  the figure and decides nothing.

================================================================================================
The FAIL, measured
================================================================================================

L17's first clause is "no floating-point value". The instrument enforces it over every text value
as well as over every JSON number, which is the correct predicate for a JSONL transcript -- a
float written as text is still a float in the file -- and it is the predicate that produced 233
records and 1,828 text values clean at WO-MOK-025.

What it found here, measured over the whole of both files rather than the four records its message
has room to name:

  accepted live run   live-run-transcript.jsonl            515 records
      fraction-shaped strings          503
      distinct such strings            1        -- '5.6'
      fields they occur in             response -- and no other field, at any depth
      distinct surrounding text        1        -- 'model":"gpt-5.6-luna","reas'
      occurrences outside gpt-5.6-luna 0
      occurrences per response         exactly 1 in every one of the 503 exchange records
      genuine JSON floats              0

  rejected attempt 1  attempt-1/live-run-transcript.jsonl  579 records
      fraction-shaped strings          567, the same single distinct string in the same field
      genuine JSON floats              0

  synthetic fixture   transcript-seed0-ticks20-hunting.jsonl  233 records
      fraction-shaped strings          0
      genuine JSON floats              0

So the transcript carries **no floating-point value**. It carries a model identifier that contains
a version number, once per exchange, in the response the provider returned. `gpt-5.6-luna` is the
identifier the owner fixed on 2026-08-23 and SPEC-MOK-007 declares; the engine did not choose it,
does not parse it, and stores it as the opaque string rule 15.2 calls for.

Why this is escalated rather than repaired here
-----------------------------------------------

Three repairs exist and all three are somebody else's:

  1. The instrument's predicate narrows -- a fraction inside the model identifier is admitted.
     SPEC-MOK-004 rule 12 governs a change to a check's strictness, and narrowing a predicate to
     admit the one string that currently fails it is the shape of change that rule exists to
     catch. It would also be narrowed against a single observation.

  2. VER-MOK-018 case L17 gains a model-identifier exception in its own wording. That is an
     amendment to an approved verification artifact, which WO-MOK-026 stop condition 6 forbids on
     an implementation agent's judgement.

  3. Neither -- L17 is recorded as holding in substance with the reading attached, which is what
     this file and verification-cases.txt do, and the owner decides between 1 and 2.

The reading is retained as printed under 3. **Nothing was changed to make a check pass**: the
instrument is byte-identical to the one WO-MOK-025 verified, its self-test still passes 36 of 36,
and its exit status over both live transcripts is 1.

This is not stop condition 11
-----------------------------

Stop condition 11 is "the synthetic transcript from WO-MOK-025 and the live transcript disagree in
form". They do not. Every one of the other seven checks passes on all three files, the record
counts split the same way (12 prefix records and one exchange record per opportunity), the shared
block is 5,385 bytes and identical across all 12 prefix records in each, and the actor blocks are
39 to 40 bytes for 12 Mokiterions in each. The single difference is the *value* of one field: the
synthetic fixture's model is `canned-connector`, which has no digits in it, and the live runs' is
`gpt-5.6-luna`, which has two. A fixture whose model identifier happened to contain a version
number would have failed at WO-MOK-025 in exactly the same way, and the stub was faithful.

P1 and L15a, which this reading also carries
--------------------------------------------

P1 is "the shared prefix is a prefix", and the L15a line above is its measurement over the live
run: the shared block is 5,385 bytes and **byte-identical across all 12 prefix records**, the
actor blocks are one per Mokiterion, and every one of the 503 exchange records names its own
prefix and its digest. That is the same predicate WO-MOK-025 measured over 12 prefix records and
221 exchanges, re-taken over 503 exchanges that a provider actually answered.
