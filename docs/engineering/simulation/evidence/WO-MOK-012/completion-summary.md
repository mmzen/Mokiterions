# `WO-MOK-012` completion summary — the optional structured record stream

`WO-MOK-012`'s *Completion report format* fixes sixteen items and their order. This file follows that
order exactly, so a numbered heading below is that clause and not a topic chosen here.

| | |
|---|---|
| Work order | `WO-MOK-012`, Phase 4a, status `in_progress` |
| Base commit | `de33d7440c323a98ac88db3fabaf87bea48ebf4e` — oracle 1's baseline, captured before the first code change |
| Governance commit | `bb4a21491eff321cbfd14ba3ea794e34535e3033` — the three amendments and the pre-change capture, documents only, code-identical to the base |
| Branch | `feature/phase-4a-definition` |
| Date | 2026-08-20 |
| Toolchain | `cargo 1.97.1 (c980f4866 2026-06-30)`, `rustc 1.97.1 (8bab26f4f 2026-07-14)` |

**This work order is not complete and does not claim to be.** Seven oracles are measured and pass.
Eight manual assessments are prepared and **all eight are outstanding** (item 15). `VREC-MOK-012` is a
separate commit-bound record that this work order does not write and cannot self-approve (item 16).
Four defects in the approved artifacts and one amendment beyond the approved list are disclosed here,
in items 1, 7, 12 and 16.

---

## 1. What changed, file by file, and what deliberately did not

### The engine package, `mokiterions-core`

| File | Base | Now | Added | Removed | What |
|---|---|---|---|---|---|
| `src/simulation.rs` | 4,180 | 5,754 | 1,626 | 52 | The record types, the four writers, the projection, the counters, the metrics computation, and the internal-tier tests |
| `src/main.rs` | 19 | 154 | 138 | 3 | The whole sink lifetime: resolve, create and truncate, buffer, flush, close, and remove a file this process created when the run fails |
| `src/cli.rs` | 153 | 183 | 30 | 0 | `--events-path`, its two rejected spellings, and its `USAGE` entry |
| `src/lib.rs` | 79 | 97 | 22 | 4 | `execute`'s one new parameter, threaded to `run_recording` |
| `tests/records.rs` | — | 1,107 | 1,107 | 0 | New public-tier target: 17 tests over the record stream |
| `tests/cli.rs` | | | 99 | 0 | The option's parsing, its rejections, and the `USAGE`-versus-parser test extended |
| `tests/process.rs` | | | 6 | 5 | The `execute` call sites, mechanically |

1,921 lines added and 64 removed across six files, plus one new file.

**The seam is one parameter.** `execute` gains `records: Option<&mut dyn Write>` and nothing else.
`interface.txt` enumerates both revisions item for item: 49 public items, 43 public fields, 92 `pub`
lines, and **the only textual difference in the whole enumeration is that one signature line**. No
item was added, none was widened, none was removed.

**The library target performs no filesystem operation.** It takes a writer, never a name. Static
check 1 measures it: the library reaches three standard-library modules — `collections`, `fmt`, `io` —
and no filesystem, path, environment or process module. `main.rs` owns the destination's entire
lifetime, which is why `main.rs` grew from 19 lines to 154.

### The observer package, `mokiterions-tui`

**Not one line changed.** No file, no test, no manifest. The observer's ten targets run the same 127
tests they ran at the base commit, and `interface.txt` records its interface as unchanged. `SPEC-MOK-003`
is not amended. The observer does not read the record stream and does not offer the option.

### What deliberately did not change

| | Measured by |
|---|---|
| **The text stream, byte for byte** — with a sink and without, over all 90 cells | Oracle 1, three comparisons, 0 differing cells over 114,723,785 bytes per capture (item 3) |
| **The entropy draw sequence** — at every tick boundary, not only at the end | Oracle 4, 4,388 boundary rows over 30 series, 0 differing (item 6) |
| **`AgentSnapshot`, `Observation`, `ProposedAction`, `Event`, `RunSummary`, `TickOutcome`** | `interface.txt`: unchanged, field for field |
| **The exit codes** — 0, 1, 2 and no fourth | `SPEC-MOK-001` rule 13.6; measured over 14 failure captures, distinct codes `[0, 1, 2]` (item 11) |
| **Every default** | `--events-path` is absent by default and writes nothing; `USAGE`-versus-parser test extended, not rewritten |
| **Every simulated rule, constant, floor, attribute and ordering** | No `SPEC-MOK-001` rule amended; static check 5: no floating-point type, operation, cast or decimal literal in 6,037 lines of library source |
| **The dependency table** | `cargo tree -p Mokiterions`: one crate, no dependency line |

### Two things worth naming that are not in the diff

**No JSON library.** `ARCH-MOK-001` keeps the engine's dependency table empty, and the closed value
alphabet of `SPEC-MOK-006` rule 3.2 is what makes hand-written writers safe rather than merely cheap:
no value the engine can emit contains a quotation mark, a reverse solidus or a code point below
U+0020, so there is nothing to escape. Oracle 5 checks that exhaustively (item 7) and oracle 3 checks
the output with a parser this repository does not own (item 5).

**No classification, and no clock.** Static check 6 enumerates all 61 field names over the four record
kinds and finds none of the four counterexample-only names emitted. `WO-MOK-012` named an outcome label
the most tempting scope creep in this work; it is prohibited rather than deferred.

---

## 2. One full record stream, quoted

Produced by the shipped binary, quoted whole, so a reader can see the artifact this phase produced
without running anything:

```
Mokiterions --seed 42 --ticks 2 --events-path <destination>
```

165 records, 26,980 bytes, exit code 0, standard error empty. The bytes do not depend on the
destination — `retained-sink-streams.txt` proves that by running each retained cell to two
deliberately different paths and requiring byte-identical records — so the destination is elided
above and appears nowhere below. Seed 42 is a declared seed; `--ticks 2` is the short limit the clause
asks for; the density and policy are the resolved defaults, which is what the `header` record states.

The four kinds appear in rule 9.1's order: the header; tick 0's initialization events; then per tick,
that tick's events followed by its metrics record; then the run record. Tick 0 has no metrics record,
because no tick has completed, so the two metrics records carry ticks 1 and 2.

Composition: 1 `header`, 161 `event`, 2 `metrics`, 1 `run`. The events are 1 `world_initialized`, 122
`food_initialized` and 12 `agent_initialized` at tick 0, 1 `decision_source_selected` at tick 0, 24
`survival_changed` over ticks 1 and 2, and 1 `simulation_ended` at tick 2. Six of the twelve event
types appear; the other six need a longer run or `--trace-actions`, which is why the matrix of item 3
runs 1,000 ticks with tracing off and on rather than resting on a stream like this one.

```jsonl
{"record":"header","schema":1,"engine":"0.1.0","config":{"seed":42,"ticks":2,"policy":"reference","density":"0.75","trace_actions":false}}
{"record":"event","tick":0,"subject":"world","event":"world_initialized","result":{"width":128,"height":128,"territories":2}}
{"record":"event","tick":0,"subject":"F0001","event":"food_initialized","result":{"class":"low","position":{"x":21,"y":3},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0002","event":"food_initialized","result":{"class":"medium","position":{"x":82,"y":20},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0003","event":"food_initialized","result":{"class":"high","position":{"x":114,"y":6},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0004","event":"food_initialized","result":{"class":"low","position":{"x":93,"y":36},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0005","event":"food_initialized","result":{"class":"medium","position":{"x":85,"y":46},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0006","event":"food_initialized","result":{"class":"high","position":{"x":63,"y":62},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0007","event":"food_initialized","result":{"class":"low","position":{"x":102,"y":55},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0008","event":"food_initialized","result":{"class":"medium","position":{"x":92,"y":50},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0009","event":"food_initialized","result":{"class":"high","position":{"x":117,"y":29},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0010","event":"food_initialized","result":{"class":"low","position":{"x":7,"y":32},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0011","event":"food_initialized","result":{"class":"medium","position":{"x":104,"y":25},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0012","event":"food_initialized","result":{"class":"high","position":{"x":85,"y":17},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0013","event":"food_initialized","result":{"class":"low","position":{"x":80,"y":17},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0014","event":"food_initialized","result":{"class":"medium","position":{"x":53,"y":31},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0015","event":"food_initialized","result":{"class":"high","position":{"x":7,"y":3},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0016","event":"food_initialized","result":{"class":"low","position":{"x":117,"y":30},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0017","event":"food_initialized","result":{"class":"medium","position":{"x":29,"y":30},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0018","event":"food_initialized","result":{"class":"high","position":{"x":85,"y":3},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0019","event":"food_initialized","result":{"class":"low","position":{"x":17,"y":1},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0020","event":"food_initialized","result":{"class":"medium","position":{"x":64,"y":4},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0021","event":"food_initialized","result":{"class":"high","position":{"x":34,"y":57},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0022","event":"food_initialized","result":{"class":"low","position":{"x":12,"y":17},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0023","event":"food_initialized","result":{"class":"medium","position":{"x":51,"y":10},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0024","event":"food_initialized","result":{"class":"high","position":{"x":42,"y":23},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0025","event":"food_initialized","result":{"class":"low","position":{"x":110,"y":47},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0026","event":"food_initialized","result":{"class":"medium","position":{"x":65,"y":8},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0027","event":"food_initialized","result":{"class":"high","position":{"x":36,"y":27},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0028","event":"food_initialized","result":{"class":"low","position":{"x":53,"y":11},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0029","event":"food_initialized","result":{"class":"medium","position":{"x":1,"y":58},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0030","event":"food_initialized","result":{"class":"high","position":{"x":48,"y":58},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0031","event":"food_initialized","result":{"class":"low","position":{"x":22,"y":6},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0032","event":"food_initialized","result":{"class":"medium","position":{"x":70,"y":42},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0033","event":"food_initialized","result":{"class":"high","position":{"x":62,"y":51},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0034","event":"food_initialized","result":{"class":"low","position":{"x":84,"y":22},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0035","event":"food_initialized","result":{"class":"medium","position":{"x":61,"y":38},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0036","event":"food_initialized","result":{"class":"high","position":{"x":74,"y":27},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0037","event":"food_initialized","result":{"class":"low","position":{"x":114,"y":44},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0038","event":"food_initialized","result":{"class":"medium","position":{"x":2,"y":29},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0039","event":"food_initialized","result":{"class":"high","position":{"x":47,"y":14},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0040","event":"food_initialized","result":{"class":"low","position":{"x":60,"y":27},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0041","event":"food_initialized","result":{"class":"medium","position":{"x":125,"y":29},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0042","event":"food_initialized","result":{"class":"high","position":{"x":4,"y":3},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0043","event":"food_initialized","result":{"class":"low","position":{"x":83,"y":58},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0044","event":"food_initialized","result":{"class":"medium","position":{"x":90,"y":52},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0045","event":"food_initialized","result":{"class":"high","position":{"x":2,"y":26},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0046","event":"food_initialized","result":{"class":"low","position":{"x":23,"y":7},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0047","event":"food_initialized","result":{"class":"medium","position":{"x":115,"y":16},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0048","event":"food_initialized","result":{"class":"high","position":{"x":30,"y":53},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0049","event":"food_initialized","result":{"class":"low","position":{"x":119,"y":12},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0050","event":"food_initialized","result":{"class":"medium","position":{"x":75,"y":24},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0051","event":"food_initialized","result":{"class":"high","position":{"x":122,"y":63},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0052","event":"food_initialized","result":{"class":"low","position":{"x":0,"y":34},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0053","event":"food_initialized","result":{"class":"medium","position":{"x":90,"y":31},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0054","event":"food_initialized","result":{"class":"high","position":{"x":98,"y":56},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0055","event":"food_initialized","result":{"class":"low","position":{"x":67,"y":20},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0056","event":"food_initialized","result":{"class":"medium","position":{"x":105,"y":54},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0057","event":"food_initialized","result":{"class":"high","position":{"x":30,"y":29},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0058","event":"food_initialized","result":{"class":"low","position":{"x":89,"y":13},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0059","event":"food_initialized","result":{"class":"medium","position":{"x":80,"y":34},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0060","event":"food_initialized","result":{"class":"high","position":{"x":121,"y":35},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0061","event":"food_initialized","result":{"class":"low","position":{"x":78,"y":54},"territory":"A"}}
{"record":"event","tick":0,"subject":"F0062","event":"food_initialized","result":{"class":"low","position":{"x":54,"y":78},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0063","event":"food_initialized","result":{"class":"medium","position":{"x":13,"y":111},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0064","event":"food_initialized","result":{"class":"high","position":{"x":108,"y":73},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0065","event":"food_initialized","result":{"class":"low","position":{"x":5,"y":102},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0066","event":"food_initialized","result":{"class":"medium","position":{"x":119,"y":124},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0067","event":"food_initialized","result":{"class":"high","position":{"x":1,"y":85},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0068","event":"food_initialized","result":{"class":"low","position":{"x":45,"y":78},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0069","event":"food_initialized","result":{"class":"medium","position":{"x":46,"y":94},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0070","event":"food_initialized","result":{"class":"high","position":{"x":125,"y":124},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0071","event":"food_initialized","result":{"class":"low","position":{"x":122,"y":82},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0072","event":"food_initialized","result":{"class":"medium","position":{"x":88,"y":101},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0073","event":"food_initialized","result":{"class":"high","position":{"x":127,"y":66},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0074","event":"food_initialized","result":{"class":"low","position":{"x":89,"y":112},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0075","event":"food_initialized","result":{"class":"medium","position":{"x":18,"y":85},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0076","event":"food_initialized","result":{"class":"high","position":{"x":43,"y":95},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0077","event":"food_initialized","result":{"class":"low","position":{"x":111,"y":75},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0078","event":"food_initialized","result":{"class":"medium","position":{"x":28,"y":73},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0079","event":"food_initialized","result":{"class":"high","position":{"x":58,"y":65},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0080","event":"food_initialized","result":{"class":"low","position":{"x":19,"y":109},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0081","event":"food_initialized","result":{"class":"medium","position":{"x":121,"y":94},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0082","event":"food_initialized","result":{"class":"high","position":{"x":50,"y":92},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0083","event":"food_initialized","result":{"class":"low","position":{"x":57,"y":75},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0084","event":"food_initialized","result":{"class":"medium","position":{"x":67,"y":67},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0085","event":"food_initialized","result":{"class":"high","position":{"x":41,"y":95},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0086","event":"food_initialized","result":{"class":"low","position":{"x":61,"y":106},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0087","event":"food_initialized","result":{"class":"medium","position":{"x":26,"y":117},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0088","event":"food_initialized","result":{"class":"high","position":{"x":24,"y":106},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0089","event":"food_initialized","result":{"class":"low","position":{"x":84,"y":80},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0090","event":"food_initialized","result":{"class":"medium","position":{"x":69,"y":111},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0091","event":"food_initialized","result":{"class":"high","position":{"x":121,"y":103},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0092","event":"food_initialized","result":{"class":"low","position":{"x":105,"y":117},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0093","event":"food_initialized","result":{"class":"medium","position":{"x":45,"y":105},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0094","event":"food_initialized","result":{"class":"high","position":{"x":38,"y":81},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0095","event":"food_initialized","result":{"class":"low","position":{"x":36,"y":119},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0096","event":"food_initialized","result":{"class":"medium","position":{"x":40,"y":120},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0097","event":"food_initialized","result":{"class":"high","position":{"x":69,"y":78},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0098","event":"food_initialized","result":{"class":"low","position":{"x":56,"y":71},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0099","event":"food_initialized","result":{"class":"medium","position":{"x":25,"y":113},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0100","event":"food_initialized","result":{"class":"high","position":{"x":86,"y":117},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0101","event":"food_initialized","result":{"class":"low","position":{"x":24,"y":125},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0102","event":"food_initialized","result":{"class":"medium","position":{"x":126,"y":125},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0103","event":"food_initialized","result":{"class":"high","position":{"x":80,"y":81},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0104","event":"food_initialized","result":{"class":"low","position":{"x":66,"y":114},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0105","event":"food_initialized","result":{"class":"medium","position":{"x":81,"y":107},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0106","event":"food_initialized","result":{"class":"high","position":{"x":35,"y":81},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0107","event":"food_initialized","result":{"class":"low","position":{"x":83,"y":75},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0108","event":"food_initialized","result":{"class":"medium","position":{"x":97,"y":104},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0109","event":"food_initialized","result":{"class":"high","position":{"x":60,"y":85},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0110","event":"food_initialized","result":{"class":"low","position":{"x":11,"y":124},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0111","event":"food_initialized","result":{"class":"medium","position":{"x":71,"y":120},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0112","event":"food_initialized","result":{"class":"high","position":{"x":86,"y":81},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0113","event":"food_initialized","result":{"class":"low","position":{"x":46,"y":101},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0114","event":"food_initialized","result":{"class":"medium","position":{"x":71,"y":74},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0115","event":"food_initialized","result":{"class":"high","position":{"x":125,"y":82},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0116","event":"food_initialized","result":{"class":"low","position":{"x":12,"y":77},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0117","event":"food_initialized","result":{"class":"medium","position":{"x":107,"y":64},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0118","event":"food_initialized","result":{"class":"high","position":{"x":42,"y":106},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0119","event":"food_initialized","result":{"class":"low","position":{"x":64,"y":120},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0120","event":"food_initialized","result":{"class":"medium","position":{"x":116,"y":120},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0121","event":"food_initialized","result":{"class":"high","position":{"x":19,"y":85},"territory":"B"}}
{"record":"event","tick":0,"subject":"F0122","event":"food_initialized","result":{"class":"low","position":{"x":111,"y":112},"territory":"B"}}
{"record":"event","tick":0,"subject":"M01","event":"agent_initialized","result":{"name":"Zug","position":{"x":11,"y":1},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":11}}
{"record":"event","tick":0,"subject":"M02","event":"agent_initialized","result":{"name":"Krul","position":{"x":17,"y":63},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":40}}
{"record":"event","tick":0,"subject":"M03","event":"agent_initialized","result":{"name":"Quib","position":{"x":26,"y":52},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":4}}
{"record":"event","tick":0,"subject":"M04","event":"agent_initialized","result":{"name":"Sput","position":{"x":83,"y":44},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":24}}
{"record":"event","tick":0,"subject":"M05","event":"agent_initialized","result":{"name":"Trok","position":{"x":84,"y":15},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":21}}
{"record":"event","tick":0,"subject":"M06","event":"agent_initialized","result":{"name":"Womp","position":{"x":12,"y":46},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":13}}
{"record":"event","tick":0,"subject":"M07","event":"agent_initialized","result":{"name":"Hozz","position":{"x":67,"y":95},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":7}}
{"record":"event","tick":0,"subject":"M08","event":"agent_initialized","result":{"name":"Nurb","position":{"x":62,"y":104},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":40}}
{"record":"event","tick":0,"subject":"M09","event":"agent_initialized","result":{"name":"Vonk","position":{"x":64,"y":82},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":24}}
{"record":"event","tick":0,"subject":"M10","event":"agent_initialized","result":{"name":"Gorm","position":{"x":120,"y":120},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":15}}
{"record":"event","tick":0,"subject":"M11","event":"agent_initialized","result":{"name":"Xob","position":{"x":123,"y":116},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":10}}
{"record":"event","tick":0,"subject":"M12","event":"agent_initialized","result":{"name":"Drix","position":{"x":67,"y":98},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":23}}
{"record":"event","tick":0,"subject":"world","event":"decision_source_selected","result":{"source":"reference"}}
{"record":"event","tick":1,"subject":"M01","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":0}}}
{"record":"event","tick":1,"subject":"M02","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M03","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M04","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":0}}}
{"record":"event","tick":1,"subject":"M05","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":0}}}
{"record":"event","tick":1,"subject":"M06","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M07","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M08","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M09","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M10","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M11","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"event","tick":1,"subject":"M12","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":10}}}
{"record":"metrics","tick":1,"living":12,"deaths":0,"population":{"A":6,"B":6},"health":{"sum":1200,"min":100},"satiety":{"sum":1188,"min":99},"energy":{"sum":1188,"min":99},"fear":{"sum":90,"max":10},"territories":{"A":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false},"B":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false}}}
{"record":"event","tick":2,"subject":"M01","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":0,"to":0}}}
{"record":"event","tick":2,"subject":"M02","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M03","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M04","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":0,"to":0}}}
{"record":"event","tick":2,"subject":"M05","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":0,"to":0}}}
{"record":"event","tick":2,"subject":"M06","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M07","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M08","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M09","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M10","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M11","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"M12","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":99,"to":98},"energy":{"from":99,"to":98},"fear":{"from":10,"to":20}}}
{"record":"event","tick":2,"subject":"world","event":"simulation_ended","result":{"reason":"tick_limit"}}
{"record":"metrics","tick":2,"living":12,"deaths":0,"population":{"A":6,"B":6},"health":{"sum":1200,"min":100},"satiety":{"sum":1176,"min":98},"energy":{"sum":1176,"min":98},"fear":{"sum":180,"max":20},"territories":{"A":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false},"B":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false}}}
{"record":"run","reason":"tick_limit","ticks":2,"survivors":12,"deaths":0,"crossings":0,"consumed":{"low":0,"medium":0,"high":0},"regenerated":0,"regeneration_skipped":{"depleted":0,"capacity":0},"final":{"territories":{"A":{"population":6,"low":21,"medium":20,"high":20},"B":{"population":6,"low":21,"medium":20,"high":20}}},"agents":[{"id":"M01","name":"Zug","territory":"A","died_at":null},{"id":"M02","name":"Krul","territory":"A","died_at":null},{"id":"M03","name":"Quib","territory":"A","died_at":null},{"id":"M04","name":"Sput","territory":"A","died_at":null},{"id":"M05","name":"Trok","territory":"A","died_at":null},{"id":"M06","name":"Womp","territory":"A","died_at":null},{"id":"M07","name":"Hozz","territory":"B","died_at":null},{"id":"M08","name":"Nurb","territory":"B","died_at":null},{"id":"M09","name":"Vonk","territory":"B","died_at":null},{"id":"M10","name":"Gorm","territory":"B","died_at":null},{"id":"M11","name":"Xob","territory":"B","died_at":null},{"id":"M12","name":"Drix","territory":"B","died_at":null}]}
```

---

## 3. Oracle 1 — the text stream, three-way byte comparison over the whole declared matrix

Ninety cells: seeds `0 1 42 123 777` × policies `baseline reference individual` × densities
`0.15 0.75 1.50` × tracing off and on, `--ticks 1000` throughout. `VER-MOK-012` declares sixty — the
two densities `0.75` and `1.50` — and this is a superset, adding `0.15`, which is the matrix
`WO-MOK-011` captured and is what lets item 10's re-run compare cell for cell.

| Comparison | Cells | Differing cells | Differing bytes | Result |
|---|---|---|---|---|
| A: pre-change vs post-change, no sink | 90 | 0 | **0** | PASS |
| B: post-change no sink vs post-change with a sink | 90 | 0 | **0** | PASS |
| C: pre-change vs post-change with a sink | 90 | 0 | **0** | PASS |

114,723,785 bytes of standard output per capture, identical in all three. Compared columns:
`sha256(stdout)`, byte count, line count, `sha256(stderr)`, standard-error byte count, exit code.

**No projection was applied, and none was needed.** Item 10's comparison against older captures does
apply one, and it is a different comparison for a different reason. Here nothing was decoded,
normalized, newline-translated or tolerated: the three captures are byte-identical as written.
`SPEC-MOK-006` rule 11.1 admits no whitespace exemption, and a normalizing comparison would establish
a weaker property than the one required.

The sink capture additionally produced 218,464,568 bytes of records across its 90 cells, which
comparison B and C do not compare because the left-hand side has none.

---

## 4. Oracle 2 — the text stream reconstructed from the records

| | |
|---|---|
| Cells reconstructed | 90 |
| Text lines reconstructed | 905,247 |
| Cells differing from the captured standard output | **0** |
| Result | PASS |

**The reconstructor has no per-event-type branch, and that is checked rather than claimed.** The
source of `render_value`, `render_result` and `event_line` in `analysis/reconstruct.py` was searched
for each of the 12 event names the captured streams carry — `action_trace`, `agent_died`,
`agent_initialized`, `decision_source_selected`, `food_consumed`, `food_initialized`,
`food_regenerated`, `food_regeneration_skipped`, `simulation_ended`, `survival_changed`,
`territory_crossed`, `world_initialized` — and **none of them appears**. Rule 6.6's generic walk
sufficed for all 905,247 lines. That is the property that matters: a reconstructor carrying a branch
per event type would prove only that somebody transcribed the text format twice.

---

## 5. Oracle 3 — every record parsed by a parser outside this repository

```
python docs/engineering/simulation/evidence/WO-MOK-012/analysis/validate.py <capture-dir> <output-file>
```

| | |
|---|---|
| Streams parsed | 90 |
| Records parsed | 961,105 |
| Findings | **0** |
| Result | PASS |

Python's `json` module, which this repository does not own and did not write. Every record is one line
and parses standalone. **The integer-only assertion is part of the check**: every numeric value in
every record is required to parse as an integer, and no floating-point value appears anywhere in
961,105 records. Static check 5 establishes the same thing from the other side — no `f32`, no `f64`,
no float cast, no float-only operation and no decimal literal in 6,037 lines of library source — so
the property holds both in the emitted bytes and in the source that emits them.

---

## 6. Oracle 4 — the entropy draw sequence, per tick, and against the pre-change build

**Additivity.** The state a run starts from is the seed and the density and nothing else. Twelve
combinations per row — three policies × tracing off and on × sink and no sink — and one value across
all twelve is the pass condition. Fifteen rows, five declared seeds × three densities, all `all_equal=yes`.

**Per tick.** 30 series — five seeds × three policies × tracing off and on, at the default density,
150 ticks — each run twice, once with a sink and once without, compared at **every** boundary:

| | |
|---|---|
| Boundary rows compared | 4,388 |
| Rows equal | 4,388 |
| Rows differing | **0** |

Three facts the equality alone does not give, all in 30 of 30 series: the draw count never goes
backwards; `k(0) == k(1)`, so emitting the initialization events draws nothing; and boundary 1 agrees
with the additivity test's figure for the same seed and density, which is a cross-check between two
separately captured tests.

**The state is a draw counter, and that is derived rather than assumed.** `SplitMix64` advances by one
fixed odd increment per draw — `simulation.rs:877`, `wrapping_add(0x9E37_79B9_7F4A_7C15)` — and the
increment is invertible mod 2⁶⁴, so every state figure inverts to a draw count `k`. Every `k` is
required to be a non-negative integer under 2²⁰, and a state that did not come from this generator
seeded from this seed would invert to an essentially uniform 64-bit number. Construction's draws:
72 at density 0.15, 268 or 270 at 0.75, and 514 or 516 at 1.50 — the same across all twelve
combinations of a row, and varying by seed within a density because rejection sampling of occupied
cells takes a seed-dependent number of retries.

**Against the pre-change build, the figure cannot be measured and is established by consequence.**
This is stated plainly because it is the weakest step in oracle 4. `Simulation::entropy_state` is
`#[cfg(test)]` and this work order added it; the pre-change build does not have it. Adding it to the
pre-change tree would make a different build, and recapturing the baseline is what oracle 1's first
obligation forbids. So the argument runs: the state is a draw counter; every drawn value is a function
of the state before the draw; every draw the engine takes reaches the output; and oracle 1 finds no
difference in 90 cells. If the pre-change build had taken a different number of draws at any point
preceding an emitted event, the text stream would differ there.

**The residual, restated here as `entropy.txt` Part 3 requires.** That argument covers draw counts up
to the last draw whose value reaches the output. It does not cover a *trailing* draw at the very end
of a run whose result nothing consumes: such a draw would be invisible to oracle 1 and would change
the tick-1,000 figure. Two things bound it and neither closes it — no shipped call site takes a draw
and discards its value, and this work order removed no call site, which `interface.txt` measures;
and `a_record_sink_moves_no_entropy_draw_at_any_tick_boundary` closes the same question for the sink
in both directions at every boundary. **The tick-1,000 figures are established against the pre-change
build only up to a trailing draw.** That is a limitation of the evidence, not a passing check.

---

## 7. Oracle 5 — the value alphabet, each domain's size beside the specification's

```
cargo test -p Mokiterions --lib -- --exact --nocapture \
  simulation::tests::every_closed_domain_that_reaches_a_record_is_on_the_alphabet
```

13 domains, 306 members enumerated exhaustively, every member's bytes asserted free of the quotation
mark `0x22`, the reverse solidus `0x5c` and every code point below `0x20`. Result: PASS.

**The emitted values use 53 of the 69 characters rule 3.3 admits, not all of them**, and the gap is
worth stating because it is easy to read the rule as a description of what the engine emits. The union
over all 306 members is `.`, `0`–`9`, `_`, the sixteen capitals `ABDFGHKMNQSTVWXZ` and twenty-five
lowercase letters — `q` appears in no member of any domain. Of rule 3.3's seven punctuation
characters, `.` and `_` are emitted and **`-`, `+`, `:`, `;` and `>` are emitted by no value at all**:
they are on the alphabet because the *text* stream's separators use them — a coordinate's colon, a
transition's arrow — not because any record field carries them. `alphabet.txt`'s closing note records
the same thing. A strict subset makes rule 3.3's escaping argument stronger, not weaker, but the rule
is the bound and the enumeration is the measurement, and they are not the same statement.

| Domain | Enumerated | `SPEC-MOK-006` rule 3.2 states | Agrees |
|---|---|---|---|
| `event` | 12 | the twelve event types | yes |
| `territory` | 2 | `A`, `B` | yes |
| `class` | 3 | `low`, `medium`, `high` | yes |
| `reason.skip` | 2 | `depleted`, `capacity` | yes |
| `direction` | **4** | **the eight fixed direction words** | **no — see below** |
| `reason.termination` | 2 | `tick_limit`, `extinction` | yes |
| `policy` | 3 | `baseline`, `reference`, `individual` | yes |
| `status` | 2 | `accepted`, `rejected` | yes |
| `source` | 3 | `baseline`, `reference`, `individual` | yes |
| `name` | 12 | the twelve fixed names | yes |
| `subject.agent` | 12 | `M[0-9]{2}` | yes |
| `subject.food` | 244 | `F[0-9]{4}` — observed count at the widest declared density, not a closed set | n/a |
| `density` | 5 | `[0-9]+\.[0-9]{2}` | n/a |

### Defect: rule 3.2 states eight direction words and the domain has four

This clause is the one that forced the comparison, so the disagreement is reported here rather than
absorbed.

`SPEC-MOK-006` rule 3.2's row for `result.proposal.direction` reads "the eight fixed direction words,
in their existing snake_case spellings". The domain has **four**: `north`, `east`, `south`, `west`.

**Why the row is wrong, precisely.** `SPEC-MOK-001` has two direction vocabularies and rule 3.2 names
the wrong one. Its rule 3 *relative* direction, used for perception, has eight words including
`north_east` (`SPEC-MOK-001:335`). Its `ProposedAction::Move` has four cardinal ones
(`SPEC-MOK-001:437`, and rule 8 at line 382: "a valid move changes one coordinate by one cell in a
**cardinal** direction"). The field rule 3.2 is describing is the *proposal's* direction, so four is
correct and eight describes the other vocabulary. The engine carries the same two types — `Direction`
with `ORDERED: [Self; 4]` at `simulation.rs:260`, and a private `RelativeDirection` with eight
variants at `simulation.rs:285` — and no record field carries a relative direction.

**Measured, not inferred:** the four diagonal words occur **0 times** in all four retained record
streams, including the traced one that carries `action_trace` records, and 0 times in all three
retained text streams.

**Why oracle 5 did not catch it.** Only three rows of rule 3.2 state a count in prose instead of
enumerating members: `event` (twelve, correct), the two name rows (twelve, correct), and this one. The
test transcribes each expected size as a literal, and for this domain the literal transcribed was the
engine's rather than the specification's — `assert_eq!(Direction::ORDERED.len(), 4)` at
`simulation.rs:5665`. So for twelve of thirteen domains the assertion compares the engine against the
specification, and for this one it compares the engine against itself. The negative control shows the
mechanism works where the number is the specification's: perturbing the event vocabulary to thirteen
failed with `left: 13, right: 12`.

**What is not affected.** Rule 3.3's escaping-freedom argument stands either way: both vocabularies
use only lowercase letters and the underscore, so the character union is unchanged and no escaping is
needed under either reading. The defect is in one stated size and in one example, not in the argument
the closed alphabet supports.

### Defect: the action-trace example emits a value no run can produce

Same conflation, one section later. `SPEC-MOK-006:541`'s *Example: an action trace record* carries
`{"action":"move","direction":"north_east"}`, and line 545 gives its text reconstruction with
`proposal:move:north_east`. No `action_trace` record can carry that, because `ProposedAction::Move`
admits four cardinal directions. Rule 6.5's own example at line 267 uses `north` and is correct.

**Both defects are in `SPEC-MOK-006`, which is approved.** Neither is corrected here. A specification
the owner approved is amended by the owner, and `ADR-MOK-005` names three documents to amend, of which
`SPEC-MOK-006` is not one — it is the new specification, not an amended one. Item 16 records them as
open.

---

## 8. Oracle 6 — metrics and run records reconciled against an event replay

```
python docs/engineering/simulation/evidence/WO-MOK-012/analysis/replay.py <capture-dir> <output-file>
```

| | |
|---|---|
| Streams replayed | 90 |
| Tick boundaries reconciled | 55,768 |
| Findings | **0** |
| Result | PASS |

The replay rebuilds the world from the event records using `SPEC-MOK-001`'s rules and compares it
against the metrics record at every tick and the run record at the end. The two figures reach the
stream by different code paths — events from transitions as applied, metrics from authoritative state
read at the tick boundary — so a disagreement means one of them is wrong, and this oracle does not
presume which.

Reconciled **per tick**: living, deaths, population per territory, standing per class per territory,
capacity against the density, permanent depletion, and the sum and extremum of each of the four
attributes. **Per regeneration opportunity**: rule 15's outcome predicted from the replay's own
standing count. **Per run**: every cumulative counter against its event count, the final territory
figures, and the twelve-Mokiterion roster with each `died_at` against the tick of its `agent_died`
event.

**The counter comparison specifically.** Each of the seven `u64` counters equals the number of its
corresponding event records, in all 90 streams: `crossings` against `territory_crossed`; the three
`consumed` classes against `food_consumed` by class; `regenerated` against `food_regenerated`; the two
`regeneration_skipped` reasons against `food_regeneration_skipped` by reason. This is what
`REQ-MOK-044` asks for, and it holds because each counter is incremented in the same statement
sequence as its event is emitted — static check 8 measures that: four private counter fields, exactly
one write each across two emitting functions, and every read in `write_run_record` alone.

**What the replay cannot witness independently.** Territory `capacity` and permanent `depleted` have
no event counterpart at all — capacity follows from the density in the header and depletion from
engine state, so the replay checks them against the density and against its own standing count rather
than deriving them from events. `VER-MOK-012`'s *Residual uncertainty* names these two as the fields
with the weakest independent witness in this contract, and that is correct. Item 16 carries it
forward; `manual-assessment.md` assessment 4 measures it.

**Independence of the replay is a claim about how the script was written, and no mechanical check
establishes it.** It was written against `SPEC-MOK-001`'s rules rather than by reading the engine's
metrics code, and it is 491 lines that arrive at the same figures by a different route. That is the
honest statement; assessment 8 is where it is judged.

---

## 9. The negative controls — the three oracles shown to fail

`WO-MOK-012`'s *Obligations worth restating* names this the second of three: "Oracles 4, 5 and 6 must
each be demonstrated to fail. A check that has never failed has not been shown to work, and these
three are the checks the whole contract's strength rests on."

| Scenario | Perturbation | Oracle | Observed |
|---|---|---|---|
| 4 | One entropy draw added to `Simulation::emit`, on the record-writing path | 4 | `a_record_sink_moves_no_entropy_draw_at_any_tick_boundary` FAILED |
| 6 | The regeneration-skip path altered so a counter and its event disagree | 6 | replay reported findings; `# result: FAIL` |
| 5 | A thirteenth event type added to the vocabulary | 5 | `every_closed_domain_that_reaches_a_record_is_on_the_alphabet` FAILED, `left: 13, right: 12` |

Scenario 5 additionally failed a check nobody wrote for the purpose: `cargo build --workspace` stopped
with `error[E0004]: non-exhaustive patterns: EventType::ThirteenthProbe not covered` at
`mokiterions-tui/src/authority.rs:20`. The observer matches exhaustively over `EventType` with no
wildcard arm, so the event vocabulary cannot grow silently in either package — the engine's own size
assertion fails and the only consumer in the workspace stops building.

**All three perturbations were reverted, and the revert is the state everything else was measured
against.** Nothing perturbed is in the shipped source: `git diff` at the recorded commit shows no
perturbation surviving, the gates in `gates.txt` were run against the reverted tree, and each
perturbation's unperturbed counterpart result is named in `negative-controls.txt` beside it.

---

## 10. Additivity — every retained pre-change capture, re-run and byte-compared

| | |
|---|---|
| Retained captures examined | 199 |
| Distinct configurations re-run | 92 |
| Identical with nothing removed from either side | 93 |
| Identical after projecting the re-run back to the capture's own shape | 106 |
| **Failing** | **0** |
| Result | PASS |

### `REQ-MOK-045`'s matrix row, taken literally, is unsatisfiable

`REQ-MOK-045` asks that "every run retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011`
reproduces byte for byte at the configuration recorded with it". **Two of those work orders retained
pre-change captures of an older text stream.** `WO-MOK-010` added `fear` and `waste_tolerance` to the
stream and `WO-MOK-011` added `name`, each under an approved amendment. A capture taken before those
additions cannot be reproduced by a build that makes them, and no correct engine could reproduce it.
This is a defect in the requirement's wording, disclosed rather than worked around.

So the retained runs are sorted into the three shapes the text stream has had, and each is compared in
the only way that carries information:

- a capture of the **current** shape is compared with nothing removed from either side — this is the
  comparison that says `WO-MOK-012` did not disturb the text stream, and it is what `REQ-MOK-045` is
  really about;
- a capture of an **older** shape is compared after the fields added since it are deleted, **and the
  deletion is separately required to be a no-op on the capture itself** — that second half is what
  stops the projection from hiding a difference instead of removing an addition, and it is
  `WO-MOK-010`'s own safeguard reused unmodified.

A capture that fails either comparison is a reproduction failure. A capture that passes the second but
not the first is not one, and the report says so per file rather than in a footnote. Generations are
detected from each capture's own bytes — the `agent_initialized` line names the attributes that
existed when it was written — not from the directory the file sits in.

`WO-MOK-002` retained no run stream: its evidence is a `cargo test` log and nine documents, and its
20-tick excerpt is a hand-trimmed digest with fields dropped and columns aligned by hand, so it is not
a capture and cannot be byte-compared to anything. What could be checked was: the line
`manual-observation.md` quotes is still quoted there, and the same line appears in a re-run at seed 42,
density 0.75%, reference, traced. Both yes.

**One commit, two build profiles.** Release against debug, seed 42, density 1.50%, individual, 1,000
ticks: exit codes 0 and 0, 1,399,597 bytes and 1,399,597 bytes, identical.

---

## 11. The failure captures, the overwrite capture, and each destination afterwards

Fourteen cases against the shipped binary at the process boundary. **The faults are external.** Three
of the five failures are platform I/O errors, and a capture of them is worth nothing if the product had
to be modified to produce them — that would be a rehearsal of a failure, not an observation of one. So
a second process opens the destination and takes an exclusive byte-range lock over a span of it; the
engine's writes into that span fail with `ERROR_LOCK_VIOLATION`. No engine code is compiled
differently, no test double is substituted.

| Case | Exit | The destination afterwards |
|---|---|---|
| Sink not creatable — parent directory does not exist | 1 | absent; **no tick ran** |
| Sink not creatable — path names a directory the platform will not open for writing | 1 | the directory, untouched; no tick ran |
| Write failure mid-run | 1 | removed — this process created it |
| Flush failure | 1 | removed — this process created it |
| Run-record write failure | 1 | removed — this process created it |
| Partial stream removed — the process created the destination | 1 | removed |
| Partial stream **not** removed — the process did not create the destination | 1 | left where it was, and the reason printed |
| A text-stream failure with no sink, for comparison | 1 | n/a — no sink |
| Reserved-spelling rejection — `-`, the empty path, no value at all, the next option taken as a value, the option given twice (5 cases) | 2 | absent; configuration error, then the usage text |
| **Overwrite — a prior run replaced** | **0** | the new run's stream, whole; none of the prior file survives |

Distinct exit codes observed: `[0, 1, 2]`. `SPEC-MOK-001` already defines all three and rule 13.6
forbids a fourth. None was added.

**A sink failure names the sink; a text-stream failure does not.** The comparison case fails its text
stream with no sink at all and its diagnostic carries no "record sink", so the two are told apart by
reading the line rather than by knowing which run produced it.

**One sink failure can produce two diagnostic lines, at two layers, and this is disclosed because it
looks like two errors and is not.** The engine reports the write it issued and cannot name the file,
because `ARCH-MOK-001` keeps the filesystem out of the library; the host then reports its own closing
flush, with the path. They are two observations of one platform error. Four diagnostic forms were
observed in total, and which layer each comes from is recorded per case in `failure-captures.txt`.

**A file this process did not create is never deleted.** That is the deliberate asymmetry: leaving a
partial stream behind is a better outcome than deleting somebody else's file, and the run says so
rather than being silent about it.

---

## 12. The census reconciliation, the interface item for item, and the rule 6 re-check

### Census

| | |
|---|---|
| Tests at the base | 212 |
| Tests now | 246 |
| Additions | 34 |
| Removals | 0 |
| Renames | 0 |
| Result | 246 passed, 0 failed, 0 ignored, 0 filtered out; no `#[ignore]` in either package |

Reconciled **by qualified name, not by count**: a case that moved between tiers or between targets
would appear as one addition and one removal even with the total unchanged. None does. Both logs come
from one `cargo test` invocation at the workspace root, which is the gate `SPEC-MOK-004` rule 11
states — neither is assembled from per-package runs, and the sources were touched before each so
nothing reported a cached result.

Three targets moved: the engine's library `+14`, `tests/cli.rs` `+3`, and `tests/records.rs` from
nothing to 17. The other 19 of 22 targets are unchanged to the test. The observer is untouched at 127.

**One pre-existing test changed:** `tests/cli.rs`'s `USAGE`-versus-parser test gained the sink option,
which `WO-MOK-012`'s *What may change* admits by name. No other pre-existing test's body or assertions
changed, which is stop-and-escalate condition 4.

### Interface, item for item

49 public items, 43 public fields, 92 `pub` lines, by `WO-MOK-011/analysis/interface.py` reused
unmodified. The pre-change enumeration is identical except for line 4, `execute`'s signature. **A
parameter is not an item**, so `SPEC-MOK-002` rule 5's enumeration grows by no item, and the rows for
`cli::Command`, `simulation::Config` and `simulation::Simulation::run` are unamended. Rule 5's two
mechanical greps each return exactly one line. The observer's 125-line enumeration is byte-identical
to `WO-MOK-011/interface.txt` lines 105–229 and is not reproduced, because copying it would create a
second place for it to drift.

### Rule 6 re-check

`SPEC-MOK-002` rule 6 is **not** amended, and the omission is recorded at the rule itself rather than
left to be noticed. Static check 4 measures why none is needed: 2 mutating methods, which is what rule
5 names; no public function returning a borrow of engine state, the one reference return being
`&'static str`; no interior-mutable type anywhere in the target; and all ten prohibited names still
private. `SplitMix64` stays private. The entropy value the projection reads is an owned `u64` behind
`#[cfg(test)]` — static check 9: `#[cfg(test)] fn entropy_state(&self) -> u64`, private, 11
internal-tier uses, no mention in the public tier or the observer.

### Amended beyond the approved list: `SPEC-MOK-004` rule 11

`ADR-MOK-005` names three documents to amend and `SPEC-MOK-004` is not one of them. Its rule 11 was
amended anyway, and the row records the amendment as **OUTSTANDING** rather than claiming approval.

The figures rule 11 carried were observer 127, engine 85, workspace 212, measured on the merge at
`2157f77`. They become **observer 127 unchanged, engine 119, workspace 246**. Rule 11's own text
delegates the correction — "a work order that adds a test corrects these figures here, and one that
loses a test has a defect" — and every figure is a measured outcome rather than a decision:
`cargo test`'s per-target counts in `gates.txt`, the name-by-name reconciliation in
`analysis/census-reconciliation.txt`, and the enumeration in `interface.txt`. What the rule cannot
delegate is the judgement that `SPEC-MOK-004` belongs in this chain at all, so the row states the
amendment as unapproved. `WO-MOK-011` faced the same situation on the same rule and recorded it the
same way; this follows that precedent rather than inventing a reading.

### Defect: the `execute` call sites named in `SPEC-MOK-002` and `WO-MOK-012` do not all exist

`SPEC-MOK-002`'s 2026-08-20 amendment row and its *Compatibility and migration* addition list four
`execute` call sites — `mokiterions-core/src/main.rs`, `mokiterions-core/tests/process.rs`,
`mokiterions-tui/src/verification.rs` and `mokiterions-tui/tests/verification.rs` — and record
`mokiterions-tui` as "passing `None`". `WO-MOK-012`'s stop-and-escalate condition 4 anticipates the
same sites.

**Measured: `execute` is called from exactly two files.** `mokiterions-core/src/main.rs`, 2 sites, and
`mokiterions-core/tests/process.rs`, 6 sites — 8 sites in 2 files. Both
`mokiterions-tui/.../verification.rs` files exist, and neither calls `execute`; the observer depends on
the engine but does not go through the process entry point.

**The conclusion the row draws is still true, and true more strongly than the row says.** The
observer's behavior is unchanged not because it passes `None` but because it never touches the seam at
all. The two named files were not edited and did not need to be. `SPEC-MOK-002` is approved and this
work order does not correct it; item 16 records it as open.

---

## 13. The four gates

Run at the candidate tree, in `gates.txt`, with the full output retained.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | clean, exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | clean, exit 0 |
| `cargo test` at the workspace root | **246 passed, 0 failed, 0 ignored, 0 filtered out**, over 22 targets |
| `cargo tree -p Mokiterions` | `Mokiterions v0.1.0` — one crate, no dependency line |

The dependency gate is the one the record stream had to be written against: the engine cannot link a
JSON library, which is why `SPEC-MOK-006` fixes the serialization by rule and why oracle 3 validates
the output with a parser this repository does not own.

---

## 14. The three amendment records, each row quoted

Every provision each row claims was looked for **twice** in disjoint text — once in the row, once in
the document body with every amendment row stripped out — because a record whose own prose satisfied a
body check would otherwise pass both with one sentence. `amendment-approvals.md` reports all 28, each
`yes` in the record and `N/N phrases` in the body, and pins the checks with 13 named controls that
each hold.

### `ARCH-MOK-001`, row at line 49 — twelve provisions

> | 2026-08-20 | The optional structured record stream, under `CAP-MOK-009`. Twelve provisions. **Components** item 1 gains the entry point's sink duties — resolving the path, creating and truncating the destination, supplying the buffered writer, flushing and closing it, and removing a file it created on failure. Item 2 gains the engine's ownership of the run's cumulative measurement counters and of record production from authoritative events and state. The observation-surface paragraph gains a second host-supplied surface: the record sink is a **fifth responsibility of the engine package and not a fourth component**, and the **library target performs no filesystem operation**. **Dependency direction**'s fourth bullet distinguishes an output destination from persistence of state — nothing is read back and no state survives the process in a form the engine consumes. **Data and control flow** gains the second, optional branch `ordered event -> record projection -> host-supplied sink`, with the text branch unchanged and unconditional, and records that the projection mutates no state and draws no entropy. **Prohibited patterns** gains three: no filesystem operation in the library target, no entropy draw from a record-writing path, and no operator-supplied, environment-derived or free-text field in the stream while `SPEC-MOK-006` rule 3.3 is the totality argument for its escaping. **Determinism** extends to the record stream and states that configuring a sink moves neither the text bytes nor the draw sequence. **Debuggability** extends from action tracing to structured recording. **Conformance checks** gains four: no filesystem operation in the library target checked against its source; identical text bytes with and without a sink across every declared seed, each policy and tracing off and on; an identical per-tick entropy draw sequence; and every string-valued field a member of `SPEC-MOK-006` rule 3.2's enumeration, checked exhaustively. **Related architecture and ADRs** gains `ADR-MOK-005`. `addresses` gains `REQ-MOK-042` and `REQ-MOK-045`; `conforms_to` gains `SPEC-MOK-006`. `decision_assessment.rationale` records the three decisions `ADR-MOK-005` makes, the triggers they fire, and that the assessment stays `adr_required`. **No engine boundary, trust boundary, dependency direction, required pattern or determinism property is relaxed, and the engine package's dependency table stays empty.** | Approved 2026-08-20 by the repository owner acting as technical owner, together with `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042` through `REQ-MOK-046`, `SPEC-MOK-006`, `VER-MOK-012` and `WO-MOK-012`, and by way of `ADR-MOK-005`, which the same owner accepted on the same date and whose *Required amendments* section states all twelve in full. The implementation agent wrote the amended text under `WO-MOK-012`; it did not decide the substance. **The 2026-08-18 row above remains OUTSTANDING and is untouched.** It belongs to `WO-MOK-005`, it awaits the same owner's separate act, and nothing in this amendment depends on it: `ADR-MOK-005` records that the prohibition this chain adds to is the one already on the page, and `VER-MOK-012` oracle 7 measures the state as it stands rather than as it would stand once that row is resolved. |

### `SPEC-MOK-001`, row at line 63 — eleven provisions

> | 2026-08-20 | The optional structured record stream, under `CAP-MOK-009`. Eleven provisions, none of which changes a simulated behavior. *Scope* stops excluding structured output and names `SPEC-MOK-006` as its contract, while keeping persistence excluded. *Actors* adds the filesystem as a destination the binary target writes and the engine never reads. *Inputs* takes `--events-path <path>` in the synopsis and one bullet: absent by default, at most once, the empty string and the single character `-` rejected as invalid configuration, and a well-formed path the platform refuses classified as a runtime failure instead. *Help output* gains the option's entry between `--trace-actions` and `--help`, stating an absence rather than a value as its default and no constraint. *Outputs* adds the stream, and records that the text stream is unaffected by the option's presence; the exit-code list is unchanged and a paragraph states that no code is added. *Error and recovery behavior* adds that failing to create, write, flush or close the sink exits `1` with no summary claimed, that a sink that cannot be created stops the run before any tick, and that a file the process created is removed on failure. *Security and privacy properties* records the sink path as the one input interpreted as a path, interpreted only by the binary target and only as a path, and adds that no record carries a path, a clock, a host, a user, an environment value or a credential. *Performance and capacity* records the stream as write-only, linear in the run and flat in memory. *Observability* adds byte-identical records for identical trace and sink configuration, and byte-identical standard output when a sink is configured. *Compatibility and migration* names the stream's own schema version and records that no existing behavior, default or exit code changes. *Explicitly unspecified decisions* records that the stream's framing, fields, alphabet, version and failure behavior are governed rather than delegated. **No rule, no decision source, no constant, no floor, no attribute, no ordering, no default and no exit code is touched, and every run's text output and entropy sequence are unchanged.** | Approved 2026-08-20 by the repository owner acting as technical owner, together with `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042` through `REQ-MOK-046`, `SPEC-MOK-006`, `ADR-MOK-005` and `VER-MOK-012`, and by way of `ADR-MOK-005`, whose *Required amendments* section states all eleven in full. The option's name, its default, its two rejected spellings, the classification of an unopenable path as a runtime failure and the decision that the library target performs no filesystem operation are the owner's decisions of the same date, recorded in `WO-MOK-012`. The implementation agent wrote the amended text under `WO-MOK-012` and did not decide the substance. `VREC-MOK-001`, which binds the 2026-08-11 content, is not edited. |

### `SPEC-MOK-002`, row at line 24 — five provisions

> | 2026-08-20 | Five provisions amended so that `SPEC-MOK-006`'s record stream can be conformed to, under `REQ-MOK-042` through `REQ-MOK-046`. **Rule 4**: `execute` gains exactly one parameter, `records: Option<&mut dyn Write>`, and nothing else; the exit codes are unchanged and none is added, a record-sink write, flush or close failure being an output failure and therefore `1`. **Rule 5**: the `execute` row reworded from "two writers" to "the caller's writers", the enumeration otherwise untouched — a parameter is not an item, so the interface grows by no item, and the rows for `cli::Command`, `simulation::Config` and `simulation::Simulation::run` are **not** amended. **Rule 5's mechanical checks**: restated as two greps for `execute`'s signature, and the mutating-method check recorded as still returning exactly `run` and `advance_tick`, with the crate-private carrier `run_recording` named so that its non-match is disclosed rather than relied on silently. **Rule 6**: **not** amended, and the omission recorded at the rule — `SplitMix64` stays private, the ten prohibited names stay ten, and the entropy value the projection reads is an owned `u64` behind `#[cfg(test)]`. **Scope and *Compatibility and migration***: `SPEC-MOK-006` named as the authority on the stream and this specification as the authority on the seam, with the four `execute` call sites listed and `mokiterions-tui` recorded as passing `None`. Nothing about mutation, dependency direction, determinism or observable text behavior is relaxed, no target or package changes, and the engine package's dependency table stays empty. | Approved 2026-08-20 by the repository owner acting as technical owner, by way of `ADR-MOK-005`, whose *Required amendments* section states this amendment in full and which the same owner accepted on the same date. The implementation agent wrote the text under `WO-MOK-012`; it did not decide the substance. **The first 2026-08-18 row remains OUTSTANDING and is untouched**: it belongs to `WO-MOK-005` and awaits the same owner's separate act, on which nothing here depends. `VREC-MOK-003` and `VREC-MOK-010`, which bind earlier content of this specification to their commits, are not edited; `VER-MOK-012` covers this amendment. |

This row's *Scope and Compatibility* provision is the one whose four call sites do not all exist; see
item 12.

### Twenty-eight provisions, where `ADR-MOK-005` lists twenty-seven

12 + 11 + 5 = 28, and `ADR-MOK-005`'s *Required amendments* section lists 27 bullets of substance
across the three documents. The 28th is the rule 4 amendment the ADR's rule 5 bullet requires without
naming rule 4 — see item 16's list of `ADR-MOK-005` defects.

### `ARCH-MOK-001`'s outstanding 2026-08-18 row, as it stands

**Still OUTSTANDING, untouched, unpaid.** Quoted from its own subject cell:

> Narrowed the prohibition on public items from "mutable **or owned** authoritative state" to a
> mutable borrow of, or a reference into, that state, and narrowed the matching con…

It belongs to `WO-MOK-005` and awaits the technical owner, which the repository owner also is. Nothing
in this amendment depends on it: `ADR-MOK-005` records that the prohibition this chain adds to is the
one already on the page, and oracle 7 measures the state as it stands rather than as it would stand
once the row is resolved.

**Two further rows were already outstanding before this work began and are also untouched**, named
here rather than counted, because a count would let a reader believe they had been looked at:

- `SPEC-MOK-002`, 2026-08-18 — "Four provisions amended so that the terminal observer of
  `SPEC-MOK-003` can be conformed to." It belongs to `WO-MOK-005`.
- `SPEC-MOK-004`, 2026-08-19 — "Recorded figures and two subject lines corrected, because
  `SPEC-MOK-003` rule 5 as amended the same day replaced the observer's four-row layout-tie…" Its
  status cell names no work order.

Oracle 7 measured the earlier layer both ways: every amendment row that existed at the base commit is
present now, unchanged and in order, and the rows whose own status cell declares them outstanding count
1, 0, 1 and 1 across the four documents at the base and the same at the candidate. No row was edited,
reordered, renumbered, summarised or folded into a later one, and no row that read OUTSTANDING reads
anything else now.

---

## 15. The eight manual assessments — all eight outstanding

**None has been decided. Nothing in `manual-assessment.md` is a judgement.** Each of the eight is
prepared with its verbatim requirement, its accountable role, the material it needs measured, and a
blank `**Decision**` line.

| # | Subject | Accountable role | State |
|---|---|---|---|
| 1 | The record stream answers `INT-MOK-009`'s question — that a run is measurable without a human reading lines | product owner | **OUTSTANDING** |
| 2 | The metric set is the right one, and the absence of an outcome label is right | product owner | **OUTSTANDING** |
| 3 | The closed value alphabet is a sound basis for hand-written serialization | technical owner | **OUTSTANDING** |
| 4 | The metrics record's redundancy against the event stream is worth its cost | technical owner | **OUTSTANDING** |
| 5 | The binary target's ownership of the sink lifetime is the right boundary | technical owner | **OUTSTANDING** |
| 6 | Overwriting an existing destination, and removing only a file this process created | technical owner | **OUTSTANDING** |
| 7 | Every figure in the evidence has stated provenance | assurance owner | **OUTSTANDING** |
| 8 | The replay consumer is independent enough for oracle 6 to mean something | assurance owner | **OUTSTANDING** |

**Who owes them.** The repository owner holds all three roles, and that does not merge the eight into
one act: each names a different question and a different body of measured material, and an approval of
one is not an approval of another. Nothing here is approved by implication.

Three of the eight carry a disclosure in their prepared material that a reader should see before
deciding:

- **1** — build identity is only *partly* present. Rule 5.4 carries the package version `0.1.0`;
  nothing in any record carries the VCS revision or the build profile. Two different builds of the
  same version produce records that claim the same provenance.
- **2** — the live stake is measured, not hypothetical: 15 of 30 declared combinations reach the
  10,000-tick limit and 15 go extinct first, and **the default configuration — seed 0, density 0.75%,
  reference — goes extinct at tick 5,423**. Every baseline-policy combination goes extinct inside 200
  ticks, so the baseline policy cannot supply a long run at any declared seed or density.
- **4** — the assessment's own wording miscites its rule and miscounts its facts; see item 16.

---

## 16. Residual uncertainty

### Open stop-and-escalate conditions

| Condition | State |
|---|---|
| 1–3, 5–7 | Not triggered |
| 4 — "any existing test's assertions would have to change, beyond the mechanical `execute` call sites and the extended `USAGE`-versus-parser test" | **Not triggered, but the condition names call sites that do not exist.** The `USAGE` test was extended exactly as admitted; no other pre-existing test's body or assertions changed. See item 12 |

### Defects found in approved artifacts, none of them corrected here

Each was found by measuring rather than by reading, and each is in a document the owner approved, so
none is corrected by this work order. Each is stated beside what is true instead and where the
measurement is, so that the owner is deciding against a figure rather than against a report.

| # | Artifact | The defect | What is true instead |
|---|---|---|---|
| 1 | `SPEC-MOK-006` rule 3.2 | The `result.proposal.direction` row states "the eight fixed direction words" | The domain has **four**: `north`, `east`, `south`, `west`. The row names `SPEC-MOK-001`'s eight *relative* directions, which reach no record field. Item 7 |
| 2 | `SPEC-MOK-006`:541, 545 | The action-trace example emits `"direction":"north_east"` and reconstructs `proposal:move:north_east` | No `action_trace` record can carry it. Measured: 0 occurrences in all four retained record streams and all three retained text streams. Item 7 |
| 3 | `SPEC-MOK-002` 2026-08-20 row and *Compatibility and migration* | Lists four `execute` call sites and records `mokiterions-tui` as passing `None` | `execute` is called from **two** files, 8 sites: `mokiterions-core/src/main.rs` (2) and `mokiterions-core/tests/process.rs` (6). The observer never calls it, so its behavior is unchanged more strongly than the row claims. Item 12 |
| 4 | `WO-MOK-012` lines 12, 238, 376 | "six cumulative counters" | `SPEC-MOK-006`'s *State model* lists **seven** `u64` counters — 1 crossing + 3 consumption + 1 regeneration + 2 skip reasons — carried in **four** struct fields (`simulation.rs:1441–1447`), plus the per-Mokiterion `Option<u64>` death tick, which is per-agent state and not a run counter. Neither four nor seven is six |
| 5 | `VER-MOK-012` manual assessment 4 | Cites "rule 7.8's three facts with no event counterpart" | Rule 7.8 states the opposite kind of thing — that no field exists for a phenomenon the engine does not compute. The facts are in rules **7.5 and 7.6**, and there are **two**, not three: capacity and permanent depletion, which is what `VER-MOK-012`'s own *Residual uncertainty* section says. The four extrema are metrics-only as fields but recoverable by replay. Item 8 |
| 6 | `ADR-MOK-005` | Counts `SPEC-MOK-001`'s amendment list as nine — five seams plus four additions — and the list carries eleven provisions | The two the arithmetic omits are *Security and privacy properties* and *Performance and capacity*, and both are in the ADR's own list, in `SPEC-MOK-001`'s row and in its body. The defect is in the sentence that counts them, not in the work |
| 7 | `ADR-MOK-005` | Attributes `execute`'s signature to `SPEC-MOK-002` rule 5 | The signature literal is rule **4**'s; rule 5 carries the enumeration row. This is why one ADR bullet became two amendments and why the row claims five provisions where the ADR lists four — and why 28 provisions were verified against 27 bullets |
| 8 | `REQ-MOK-045` matrix row | "every run retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` reproduces byte for byte" | Unsatisfiable as written: two of those work orders retained captures of older text-stream shapes that no correct engine could reproduce. Item 10 states the comparison actually made |
| 9 | `VER-MOK-012` *Evidence retention*, bullet 4 | Asks for one full sink stream per declared seed at the default density under each policy with tracing off and on — thirty whole record streams | Roughly **120 MB**. `WO-MOK-010` retained 7.1 MB of evidence and `WO-MOK-011` 8.4 MB. The retention as written cannot be satisfied; four streams are retained and the deviation is stated in `retained-sink-streams.txt` and the packet `README.md` |

**None of the nine is an approved provision the implementation failed to make.** Every provision
`ADR-MOK-005` requires is present and measured, which is item 14. Defects 1 to 7 are wrong figures,
wrong counts, wrong citations or an unreachable example in artifact text the implementation agent
wrote and the owner approved; 8 and 9 are obligations that cannot be met as literally worded. Each
item above says what was done instead. Correcting an approved artifact is the owner's act, which is
why they are reported rather than fixed.

**One of the nine weakens an oracle, and it is defect 1.** Oracle 5's closure result stands — the 306
enumerated members are escaping-free and that is the property rule 3.3 rests on — but its *size*
assertion for the `direction` domain compares the engine against itself, so for that one domain of
thirteen the oracle is not the cross-check it is for the other twelve. Nothing else in the nine moves
a measured result: 2 is an unreachable example, 3 and 5 to 7 are citations and arithmetic in prose, and
4 is a count that appears in no assertion — `static-checks.txt` item 8 states "four private counter
fields" and `ROADMAP.md:64` states "seven cumulative counters and a per-Mokiterion death tick", which
are the field reading and the logical reading of the same state, and both are right. Only
`WO-MOK-012`'s "six" matches neither. Defects 8 and 9 do change what was measured, and items 10 and
*Retention deviations* below are where the substitute measurement is stated.

### Amendment made beyond the approved list

`SPEC-MOK-004` rule 11's census figures — observer 127 unchanged, engine 85 → 119, workspace 212 → 246
— amended under the rule's own delegation clause and recorded **OUTSTANDING** in its amendment row
rather than claiming an approval nobody gave. Item 12.

### Retention deviations

Two, both stated rather than left to be inferred from what is absent.

1. **Four whole record streams of the thirty declared** (defect 9 above). The other 86 cells carry
   SHA-256, byte count and line count in `post-sink-manifest.txt` and are reproducible by `capture.sh`
   at the recorded commit.
2. **No post-change *text* stream is retained whole.** The retention list asks for the sink capture's
   standard output and only its digests are kept. Oracle 1 establishes the post-change text stream is
   byte-identical to the pre-change one in all 90 cells, with a sink and without, so the three whole
   streams in `baseline/full/` are the post-change streams too. A reader who does not accept oracle 1's
   result should not accept that substitution either, which is why it is named.

The cost is real and accepted: a reviewer cannot inspect an arbitrary cell's text without re-running
the capture. The manifests are what detect a reproduction that failed.

### Everything under `VER-MOK-012`'s own *Residual uncertainty*, carried forward

- **The metrics record's per-tick figures are reconciled, not independently derived.** For capacity and
  permanent depletion there is no event stream to replay against; they are checked against the engine's
  own state and against the density's resolution. **They are the fields with the weakest independent
  witness in this contract.** Measured in item 8 and in `manual-assessment.md` assessment 4.
- **The pre-change entropy comparison rests on consequence, not measurement**, and does not cover a
  trailing draw whose value nothing consumes. Item 6 states the argument and its residual in full.
- **The replay consumer's independence is a claim about how the script was written**, and no mechanical
  check can establish it. What *is* mechanically checked is the weaker, adjacent property: that oracle
  2's reconstructor carries no per-event-type branch — none of the 12 event names occurs in
  `render_value`, `render_result` or `event_line` in `analysis/reconstruct.py` — and that both
  reconstructors refuse an unrecognised shape rather than guessing at it, the Python one raising at
  `analysis/reconstruct.py:64` and the Rust one panicking at `mokiterions-core/tests/records.rs:656`.
  A branch per event type would make oracle 2 a proof that the text format was transcribed twice.
  Assessment 8 is where the rest is judged.
- **Build identity is only partly recorded in the stream**: the package version, and not the VCS
  revision or the build profile. Assessment 1.
- The remaining bullets of that section stand as written and are not restated here; that section is
  the authority on them and copying it would create a second place for it to drift.

### Two things this packet's own evidence corrected in itself

- `entropy-per-tick.txt`'s header said boundary 0 is "before initialization", which reads as the seed
  state. `entropy.txt` Part 4 measures otherwise — boundary 0 is after `Simulation::new`, construction
  draws, and `k(0) == k(1)` in 30 of 30 series. The header now says what is measured.
- `docs/ROADMAP.md`'s Phase 4a entry, added by this work order, named `INT-MOK-006` and `CAP-MOK-006`
  and "seven requirements". The packet is `INT-MOK-009`, `CAP-MOK-009` and `REQ-MOK-042` through
  `REQ-MOK-046` — five. `INT-MOK-006` and `CAP-MOK-006` belong to the packet approved on 2026-08-19.
  Corrected in place, since the ROADMAP edit is this work order's own.

### What closing this work order still needs

**`VREC-MOK-012` is a separate, commit-bound verification record. This work order does not write it and
cannot self-approve it.** A work order is closed by a verification record that binds a commit, and that
record is written after the commit it names — which is why `WO-MOK-012` is `in_progress` and not
`complete`, and why oracle 7 reports that status as correct rather than as a gap.

Before `VREC-MOK-012` can be written:

1. the eight manual assessments decided and dated by their accountable roles (item 15);
2. the nine defects above dispositioned — amended, accepted or rejected — by the owner of each
   artifact;
3. the `SPEC-MOK-004` rule 11 amendment confirmed or reverted;
4. the two retention deviations accepted or the retention list amended;
5. `ARCH-MOK-001`'s, `SPEC-MOK-002`'s and `SPEC-MOK-004`'s outstanding amendment rows resolved or
   explicitly carried forward again — none of them is this work order's to pay, and all three are
   unchanged.

**What this file establishes:** that seven oracles were run over the declared matrix and pass, that the
text stream and the entropy sequence are unmoved, that the gates are green, and that the amendments
this change depends on are approved. **What it does not establish:** that this change is verified.
