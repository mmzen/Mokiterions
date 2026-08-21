# Identifier sweep and re-derivation against a moved `master` — WO-MOK-012

Two facts had to be established before any of this work order's text could be written: that `WO-MOK-012` was free, and
that the captures and line numbers this work order relies on still describe the tree. Both are recorded here with the
commands that produced them.

## 1. Why a local maximum is not the free number

The identifier space is shared across branches and sessions, and other agents clone and work on this repository
concurrently. The highest identifier in one worktree is therefore not evidence that the next one is unused: another
ref may already carry it, and a number claimed twice is a collision that only surfaces at merge.

So the sweep covers **every remote head**, not `master`, and it covers **file contents as well as filenames**, because
an identifier can be reserved in a roadmap or a work order's prose before its own file exists.

### All remote heads at the time of the sweep

```
  docs/license ca159aca30c9a2a4dbccdbf72ff543510134a611
  documentation/directory-structure 0e20f404e751f3bd4ce2226ff009abe37061dd02
  documentation/readme-screenshot e2fb9df920cdcdf02332c0ac8f3e186fa88bdce6
  feature/help-output-options-block 3002a848049784bb032e58db289330f1eeba3020
  feature/library-target-and-test-placement b688676cb11e4c7eb79eee66b7e69faaf2787f53
  feature/package-layout-and-test-tiers 8b1dbeed021657a2c33ba0a56816b128c10c6b49
  feature/phase-1-5-observer-definition 2fad84290869687e195243f4d88ed968d24f34f9
  feature/phase-1-definition 2c8af36aa0f51192102b1e212f89d6de25c4cb65
  feature/phase-2-5-naming 1013470e221aa817a469f0fd6402b264ed90b18e
  feature/phase-2-individuality 8c29830625b81028a996cb80771e524e35969e7c
  feature/release-ci e1d6f6e4e5176fe389b8caf34975daebb5460caa
  feature/simulation-rules-guide 163aaa65a8453de1f713f9e0f683f0a81f370284
  feature/wo-mok-005-layout-axes a53712c08e5e4697cb775cd202ab732b522fa63d
  feature/wo-mok-007-roster-bands 75e3598577883a1fdef0c2da8232686a752f9c81
  governance/vrec-mok-002-verified 3e8f5bdb06ad44f3b5887998cdb7ffa3b573a2c1
  governance/vrec-mok-003-verified 87be21fd6156969b072c92901eb8773587a26ef2
  governance/vrec-mok-004-verified a1bc5b0e216e309434df04b830254c9733418b36
  governance/vrec-mok-007-candidate a7ddf745e329c988755642a1a01e8594d0e4f55a
  governance/vrec-mok-011-verified 89e00eccb0276c6722758793bf6264a8afcf5446
  governance/wo-mok-005-implemented 218b71d64330f298a22e5a7b38e331f6b1d3a06d
  governance/wo-mok-006-implemented 026d9f381fa92a1ec9c62d1d547ab075f40678b2
  governance/wo-mok-010-post-release-facts adc4ce9b1db866fcdbf3e450f737d9fb7ed368fe
  master ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3
  release/0.1 755db7297aa993f00d42f9c9794584b5d061f03d
```

Count: 24 refs.

### Highest identifier of each type, across every ref

Filenames in each ref's tree, taking the maximum over all refs:

```
  WO    WO-MOK-011
  VER   VER-MOK-011
  VREC  VREC-MOK-011
  RLS   RLS-MOK-001
  INT   INT-MOK-008
  CAP   CAP-MOK-008
  REQ   REQ-MOK-041
  SPEC  SPEC-MOK-005
  ARCH  ARCH-MOK-002
  ADR   ADR-MOK-004
```

### Contents of every ref, searched for any candidate identifier

```
git grep -ohE "(WO|VER|VREC)-MOK-01[2-9]|(INT|CAP)-MOK-0(09|1[0-9])|REQ-MOK-04[2-9]|SPEC-MOK-00[6-9]|ADR-MOK-00[5-9]" <every ref>
```

Output: **empty**. No ref mentions any of these identifiers in any file.

**Conclusion: `WO-MOK-012` is free.** `WO-MOK-011` is the highest work order that exists anywhere, in a filename or in
prose, and nothing reserves `012`. The same sweep shows `VER-MOK-012`, `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042`
onward and `SPEC-MOK-006` free, which is what the second chain of decision 12 will draw from — re-swept at that time
rather than trusted from this record, because these refs move.

## 2. `master` moved during preparation

The branch this work was prepared on was cut from `master` at `dec1b95`. During preparation `master` advanced:

```
  ff3a155 Merge pull request #30 from mmzen/governance/vrec-mok-011-verified
  89e00ec gov: set WO-MOK-011 to implemented
  dac9bac Merge pull request #29 from mmzen/documentation/directory-structure
  965fe67 gov: transition VREC-MOK-011 from ready to verified
  0e20f40 directory structure
```

That is why every figure was re-derived rather than carried. An oracle figure describes the tree it was taken on; when
`master` moves, it may describe a tree that no longer exists. Two diffs decide whether the captures survive.

### Did the four amended artifacts change?

```
git diff --stat dec1b95..origin/master -- SPEC-MOK-002.md SPEC-MOK-003.md SPEC-MOK-004.md ARCH-MOK-001.md VER-MOK-005.md
```

Output: **empty** (0 lines). All five are byte-identical across the range, so the eleven OUTSTANDING rows and
every line number cited in this work order still resolve.

### Did the observer or engine source change?

```
git diff --stat dec1b95..origin/master -- mokiterions-tui/ mokiterions-core/
```

Output: **empty** (0 lines). No source file changed, so the ten captures under `assessment-material/` still
describe the running observer and did not need to be retaken.

**Had either diff been non-empty, the captures would have been retaken before use.** They were not carried on the
assumption that nothing relevant had moved; the assumption was checked, and this file is the check.

The branch was then reset onto `ff3a155` before any file was written, so this work order's diff is against current
`master` rather than against a five-commit-stale base.
