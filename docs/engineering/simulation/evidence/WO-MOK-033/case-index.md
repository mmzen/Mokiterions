# Case index

Every case executed under `VER-MOK-019`, read from each phase's own `result.json`. A case is
one assertion over measured figures; the figures themselves are in the `detail` field of that
file and in the phase's retained text.

## Phase A: the real engine end to end

`phase-a/result.json` -- 19 cases, 0 failed.

- B1 real-engine conformance
- P1 one row per completed cell
- P7/T9 integers only in retained output
- B9 hashed as written
- B9 one altered byte changes the digest
- B12 independent attempted counts
- B13 independent effective counts (real half)
- B14 lethality is not a narrowing
- P6 attempted bounds effective
- B16 the seven cross-checks pass
- B7 the digest reproduces from both profiles
- B2 transparency to the engine
- B18 tracing independence
- B10 --keep-stream changes no row
- B8 the stream is gone
- B8/P5 peak disk is one stream
- B27/P2 batch determinism under --jobs 1 and --jobs 4
- P2 a row is a function of its cell, across build profiles
- Q1 exactly one class, traceable to its clause

## Phase B: failure, refusal and disagreement

`phase-b/result.json` -- 16 cases, 0 failed.

- B17 the seven cross-checks bite
- B13 an ineffective conflict counts as attempted and not as effective
- B14 a lethal attack reports 1, 1 and 1
- B15 zero is stated, not omitted
- B19 incompleteness is per-cell
- B20 three failure stages, distinguishable
- B21 the engine's own words, byte for byte
- B22 nothing is fabricated
- B26 partial rows survive
- Q16 the distribution carries the incompleteness
- Scenario 4 an axis value lost at one end
- B23 the four exit statuses
- B5 the llm refusal
- B24 total failure
- B25 unwritable output
- Q17 no sweep record

## Phase C: options, classes and revisability

`phase-c/result.json` -- 22 cases, 0 failed.

- B4 the declared default
- B4 a given axis leaves the other three defaulted
- B3 cell enumeration and order
- B11 no ambient value in a cell record
- B6 nine usage errors
- Q2 each predicate fires, and the order carries meaning
- Q8 class counts sum to the row count
- Q11 an unobserved class is stated as zero
- Q10 attempted stays distinct
- Q4 a missing fact
- Q13 two engines are two experiments
- Q14 small and empty groups
- Q12 the sweep travels
- Q9 both axes from the same rows
- Q15 classifier determinism
- P3 classification is a function of the row
- Q6 revisability, by digest
- Scenario 3 a threshold is revised after publication
- Q3 the residual, and its removal
- P4 rows are immutable
- Q7 rows alone
- Q5 no judgement in a row

## Phase D: static, architecture, security and privacy

`phase-d/result.json` -- 15 cases, 0 failed.

- T1 no third package
- T1 the structural census is unchanged
- T2 no product file is touched
- T3 the schema is still 3
- T3 an existing bound record-stream digest reproduces
- T4 declared dependency sets unchanged
- T5 standard library only
- T6 the instruments are not targets
- T7 nothing writes into governance
- T8 no retreat field is synthesised
- T9 integers only
- No credential, and no path by which one could arrive
- No provider is reachable
- Rule 4.4 passes no connector, credential or live selection
- Lint, format and the product's own gates

## Phase E: performance and interruption

`phase-e/result.json` -- 6 cases, 0 failed.

- The default sweep's cost is measured and reported
- P5 peak disk is one stream, over the default sweep
- B3, B27 and P2 hold under --jobs 4 on the default sweep
- Scanning cost is linear in stream bytes
- A long run, and the driver's memory high-water
- Interruption

## Phase F: acceptance scenarios and the famine measurement

`phase-f/result.json` -- 6 cases, 0 failed.

- The default sweep and both distributions are produced and retained
- Scenario 1: the distribution is stated, legible, and agrees with the oracle
- Scenario 2: density moves the class distribution, and the seed is re-measured at twenty
- Scenario 5: a chosen row is reproduced from its coordinates alone
- Scenario 6: the instrument surfaces the threat inertness as a figure
- The famine measurement is taken across the sweep and at density 0.10

84 cases, 0 failed.
