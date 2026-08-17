# WO-MOK-003 evidence: trust-boundary review

`VER-MOK-003` requires a recorded confirmation that no public item yields mutable or owned
authoritative state, in any build configuration including test builds. `SPEC-MOK-002` rule 6 is the
security-relevant rule of this change, and it exists because `REQ-MOK-004` and `ADR-MOK-001` make the
engine the sole authority over world state and prohibit exposing that state to a decision source.

This review is a claim about reachability, so it is argued from signatures and from the compiler
rather than from intent.

## What is authoritative state

`ARCH-MOK-001` and `SPEC-MOK-001` make these authoritative: the world grid, the agent collection, the
resource collection, the tick counter, the entropy state, and the event log. In `src/simulation.rs`
they are the six fields of `Simulation` — `config`, `tick`, `agents`, `foods`, `entropy`,
`next_food_id` — with the event log being the writer `run` is handed rather than a stored collection.

All six fields are private. `git diff master -- src/simulation.rs` shows no `pub` token added to any
field of any type.

## Signature-level review

Every public function and constant, with what it can yield:

| Public item | Signature | What a caller receives |
| --- | --- | --- |
| `execute` | `fn(args: I, &mut W, &mut E) -> u8` | a `u8`. The two writers are the caller's own; nothing engine-owned is written into them beyond the bytes the program already prints |
| `cli::USAGE` | `&'static str` | a shared reference to a string literal in the binary image; not state |
| `cli::parse` | `fn(args: I) -> Result<Command, String>` | an owned `Command` or an owned `String` |
| `simulation::CELLS_PER_TERRITORY` | `usize` constant | a compile-time integer |
| `Density::DEFAULT` | `Self` constant | an owned `Copy` value |
| `Density::parse` | `fn(&str) -> Result<Self, String>` | an owned value or an owned message |
| `Density::resources_per_territory` | `fn(self) -> usize` | an integer, computed from a `Copy` value the caller already holds |
| `Policy::parse` | `fn(&str) -> Option<Self>` | an owned `Copy` value |
| `RunSummary::reason` | `fn(&self) -> TerminationReason` | an owned `Copy` value |
| `RunSummary::ticks` | `fn(&self) -> u64` | an integer |
| `RunSummary::survivors` | `fn(&self) -> usize` | an integer |
| `RunSummary::deaths` | `fn(&self) -> usize` | an integer |
| `Simulation::new` | `fn(Config) -> Result<Self, String>` | an owned `Simulation` whose fields are all private |
| `Simulation::run` | `fn(&mut self, &mut W) -> io::Result<RunSummary>` | an owned `RunSummary` by value |

No public signature returns `&T`, `&mut T`, `impl Trait`, `Box<dyn Trait>`, an iterator, a slice, or a
collection. A grep of all public signatures for those return forms matches nothing. That is checked
mechanically rather than by reading, because a single future `-> &Vec<Mokiterion>` would be the whole
violation.

Two signatures deserve individual argument.

**`Simulation::run` takes `&mut self`.** A mutable borrow is required to advance the engine, and this
is unchanged from before the refactor. What matters for rule 6 is that the borrow is consumed inside
the call and never handed outward: the return type is `io::Result<RunSummary>`, an owned value, and
`RunSummary` holds one enum and seven integer-valued fields — two of them fixed-size arrays — all
copied out at the end of the run. No lifetime in the
return type is tied to `&mut self`, so the compiler itself guarantees no reference into the engine
escapes. A caller can drive a run and read its outcome; it cannot reach into the run.

**`Simulation::new` returns an owned `Simulation`.** The caller owns the handle, which is exactly what
the binary target has always done. The handle is opaque: no public field, and no method beyond `new`
and `run`. Owning the box is not the same as owning its contents, and rule 6 prohibits the latter.

## The trust boundary itself is untouched

The engine-to-decision-source boundary is `Observation -> ProposedAction`: the engine builds an
immutable observation, a decision source returns a typed proposal, and the engine validates and
applies it. Nothing on the public interface reaches that boundary. `Observation`, `PerceivedFood`,
`PerceivedMokiterion`, `Action`, `ActionResult`, `DecisionSource`, `DecisionEntropy` and `SplitMix64`
are all private, and rustdoc generated no page for any of them.

A caller outside the crate cannot supply a decision source, observe a proposal, or intercept
validation. `Policy` selects between the two built-in sources by value; it is not a hook.

The internal test `the_reference_source_cannot_mutate_authoritative_state` — which is what actually
verifies the boundary — stayed in `src/simulation.rs`, where it still has the access it needs to
assert against the boundary directly. Relocating it would have required exposing the very state it
proves is unexposed.

## No conditional-visibility seam

Rule 6's second paragraph prohibits any mechanism that makes prohibited state reachable in some build
configuration. Checked:

- **Features.** `Cargo.toml` declares no `[features]` table. There is no feature to enable.
- **`cfg` attributes.** The only `#[cfg]` attribute anywhere in `src/` is the single
  `#[cfg(test)]` on `src/simulation.rs:1351`, which gates the internal test module. It gates tests,
  not visibility: the module contains no `pub` item, so nothing inside it is reachable from another
  crate even when compiled.
- **Dev-dependencies and self-dependency.** `[dev-dependencies]` is absent and `[dependencies]` is
  empty. The package does not depend on itself, so there is no path by which an integration test
  gets a differently-configured build of the library.
- **`pub(crate)` widening.** There is no `pub(crate)`, `pub(super)`, or `pub(in ...)` anywhere in
  `src/`, so no restricted item was widened to `pub` and none remains as a half-step.
- **`Cargo.lock`.** Unchanged; `git diff master -- Cargo.lock` is empty.

The consequence is that the public interface is the same in a test build as in a release build. The
integration tests under `tests/` link the ordinary library target with no special configuration,
which is also why they are proof of the interface's sufficiency: if the 15 relocated tests compile,
they compile against exactly what an external consumer would get.

## One observation, recorded rather than waived

`RunSummary` derives `Debug`, and `Debug` on a struct renders its private fields, including the
per-territory population and per-class resource counts that were deliberately left without
accessors. So a caller can read those numbers as formatted text even though no accessor returns them.

Assessment: this is not a rule 6 violation and not a change introduced here.

- `RunSummary` was already public with this derive before the refactor; `git diff` shows no `derive`
  attribute added or edited anywhere in the change.
- The values are the same ones the program prints on its summary line to standard output on every
  run, so no information becomes available that an operator did not already have.
- `Debug` yields a `String`. It is a copy of a value, not a borrow of state and not a mutation path,
  which is the distinction rule 6 and the *Data and interface contracts* section actually draw.

It is recorded because "the accessors were deliberately withheld" and "the numbers are unreadable"
are different claims, and only the first is true.

## Conclusion

No public item yields mutable authoritative state, a reference into authoritative state, or an owned
copy of the world grid, the agent collection, the resource collection, the entropy state, the tick
counter, or the event log. Every value crossing the interface outward is a copy of a fact the program
already reports. This holds in test builds because no mechanism exists to make it differ. Rule 6 is
satisfied, and with it `REQ-MOK-004` and `ADR-MOK-001`'s prohibition.
