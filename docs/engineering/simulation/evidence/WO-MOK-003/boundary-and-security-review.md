# Boundary and security review — WO-MOK-003

`SPEC-MOK-002`'s data-and-interface contract and its security-and-privacy properties are claims
about the shape of a surface, which a test can only sample. This file enumerates the surface instead
and states, for each claim, whether it holds. Where it does not hold as written, the deviation is
stated before the argument for it, not after.

## The engine's complete public surface

`src/lib.rs` declares two public modules and nothing else, so the surface is everything `pub` in
`src/cli.rs` and `src/simulation.rs`. Enumerated with `awk` over both files:

```
mokiterions_core::cli
  const USAGE: &str
  enum Command { Help, Run(Config) }
  fn parse<I, S>(args: I) -> Result<Command, String>

mokiterions_core::simulation
  struct Config { seed, tick_limit, policy, density, trace_actions }   all fields pub
  struct Density                          fn parse(&str) -> Result<Self, String>
  enum Policy                             fn parse(&str) -> Option<Self>
  enum Territory · struct Coordinate · enum Direction · enum FoodClass · enum Action
  enum TerminationReason · struct RunSummary · enum RegenerationSkipReason
  enum EventType                          fn as_str(self) -> &'static str
  enum EventDetail                        fn event_type(&self) -> EventType
  struct Event { tick, subject, detail }  fn event_type(&self) -> EventType
  enum DecisionOutcome · struct DecisionSnapshot · struct TerritorySnapshot
  struct AgentSnapshot · struct ResourceSnapshot · struct WorldSnapshot · struct TickOutcome
  struct Simulation                       (no public field)
    fn new(Config)              -> Result<Self, String>
    fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary>
    fn advance_tick(&mut self)  -> Result<TickOutcome, String>
    fn is_finished(&self)       -> bool
    fn termination_reason(&self)-> Option<TerminationReason>
    fn configuration(&self)     -> Config
    fn snapshot(&self)          -> WorldSnapshot
    fn initialization_events(&self) -> Vec<Event>
```

### Rule 2 — "`advance_tick` is the only operation that changes simulation state, and the surface exposes no other `&mut self` method that does"

**This does not hold as written.** The surface exposes two `&mut self` methods that change
simulation state: `advance_tick` and `run`. `grep -n 'pub fn .*&mut self'` over the whole engine
returns exactly those two and no others.

`run` is the `REQ-MOK-010` whole-run entry point. It predates this work; what this work changed is
its reachability, because at `48d16bd4` the engine had no library target, so `simulation` was a
private module of the binary and nothing in it was externally callable. Adding `pub mod simulation`
to expose the observation surface exposed `run` with it. Narrowing it again would mean either
relocating the engine's sources — which `WO-MOK-003` puts out of scope and `SPEC-MOK-002` rule 3 of
the component layout forbids, because the text stream must not move — or duplicating the run loop,
which would give two implementations of the rules.

What is true, and is the property the rule exists to protect, is that the *observer* calls only
`advance_tick`. Every `mokiterions_core` import in `mokiterions-tui` is listed below; none reaches
`run`, and `verification::one_advance_is_one_tick_and_a_finished_run_refuses` and
`verification::observed_and_unobserved_runs_are_identical_on_every_declared_seed` are the measured
consequence. The deviation is that the rule as written describes the surface, and the surface has
one more mutating operation than it says.

Two further `&self` methods are on the surface but absent from the rule's listing at
`SPEC-MOK-002` line 443: `termination_reason` and `initialization_events`. Neither mutates, so
rule 2 is untouched, but the listing is not the whole surface and should not be read as one.
`new` returns `Result<Self, String>` and `advance_tick` returns `Result<TickOutcome, String>` where
the listing shows the bare types; that shape difference is recorded in the completion summary.

### Rule 1 — "Every snapshot type contains owned values only: no reference into engine state, no shared handle, no interior mutability, and no method that mutates"

**Holds.** Each of `WorldSnapshot`, `TerritorySnapshot`, `AgentSnapshot`, `ResourceSnapshot`,
`DecisionSnapshot`, `TickOutcome` and `Event` is a plain struct or enum of `u8`, `u64`, `usize`,
`bool`, `String`, `Vec<T>`, `Option<T>`, `[T; 2]` and the engine's own `Copy` enums. No field is a
reference, so no snapshot type has a lifetime parameter. `grep -rn 'RefCell|Cell<|Rc<|Arc<|Mutex|RwLock|AtomicU|static mut'`
over `src/` and `mokiterions-tui/src/` returns nothing, so there is no interior mutability and no
shared handle anywhere in either package. The only `impl` blocks on these types are `event_type`
on `Event` and `EventDetail`, both `&self`, and derived `Debug`, `Clone`, `PartialEq`, `Eq`
(`Copy` where the type has no `String`).

`Simulation` has seven private fields and no public one, so nothing outside the module can reach
`agents`, `foods` or `entropy` at all. That is what makes "the observer receives no mutable handle
to world, agent, resource or event-log state" a type-level fact rather than a discipline.

The corroborating measurement is `verification::drawing_is_pure`, which clones a snapshot, draws at
every declared viewport in both zooms, and requires the clone unchanged.

### Rule 3 — "Dependency direction is one-way"

**Holds.** `mokiterions-tui/Cargo.toml` depends on `mokiterions-core` by path; the engine manifest's
`[dependencies]` table is empty and carries a comment saying `ARCH-MOK-001` requires it to stay so.
`dependency-review.txt` records `cargo tree -p mokiterions-core` on every edge kind resolving to the
package alone, and the observer absent from every engine resolution.

The engine's *code* contains no reference to the observer. To be exact about what `grep -rn
'mokiterions-tui\|mokiterions_tui' src/ Cargo.toml` does return, since "no reference" would be
wrong: `src/lib.rs:8`, a doc comment naming the two hosts; `Cargo.toml:2`, the workspace
`members` list; and `Cargo.toml:18`, the comment on the empty `[dependencies]` table. The
`members` entry is a workspace-root declaration that `SPEC-MOK-002`'s component layout mandates,
not a dependency edge — `cargo tree -p mokiterions-core` is the check that distinguishes them, and
it resolves to the engine alone. There is no `use`, no `extern crate` and no dependency entry.

In the other direction, `mokiterions_core` appears in the observer at exactly twelve sites —
`authority.rs:7,70`, `export.rs:55`, `options.rs:8,9,151`, `render.rs:13,936`, `spatial.rs:8`,
`state.rs:10,11`, `verification.rs:20` — of which five are compiled out of the shipped binary: the
four inside `mod tests` blocks, plus `verification.rs`, whose whole module is `#[cfg(test)]`
(`main.rs:21-22`).

### Rule 4 — "Snapshot ordering is stable and specified"

**Holds.** `snapshot` filters to living agents and sorts by identifier, filters to standing
resources in the engine's insertion order, and copies the retained decision list, which
`advance_tick` builds in the same ascending identifier order the rules process agents in.
`verification::every_presented_value_is_the_snapshots` and
`state::tests::initialization_events_are_retained_in_authoritative_order` measure the consequence.

### Rule 5 — "The engine's own `SPEC-MOK-001` behavior is unchanged"

**Holds, and is the strongest-evidenced claim in this file.** `additivity-proof.txt` shows all three
engine test modules byte-identical to `48d16bd4` and passing, and `export-fidelity.txt` shows the
observer's exports byte-identical to the engine binary process's own stdout on five seeds at two
tick counts.

## Security and privacy properties

| Claim | Finding |
|---|---|
| No network access | Holds. `grep -rn 'std::net\|TcpStream\|UdpSocket\|reqwest\|hyper::'` over both packages returns nothing. `dependency-review.txt` shows no networking crate in either resolution. |
| No credential, no model provider | Holds. Nothing in either package reads a credential store, a key file or a provider endpoint; there is no such code and no such dependency. |
| No asynchronous runtime | Holds. No `async`, no `.await`, no `tokio`, no `futures` in either resolution — `dependency-review.txt` records the crate list. The observer's timing is `std::thread::sleep` against two fixed intervals. |
| No database | Holds. No storage crate in either resolution; the only persistence is the export file. |
| The filesystem is written once per requested export and never read | Holds in shipped code. Every filesystem call in `src/` is: none. Every filesystem call in the observer's shipped code is `fs::File::create` and, on a write failure only, `fs::remove_file` of the path this call just created (`export.rs:40,46`). There is no `read_to_string`, no `OpenOptions`, no `include_str!` and no directory walk. The `fs::read_to_string`, `create_dir_all` and `remove_dir_all` calls that `grep` finds are all inside `#[cfg(test)]` modules and are compiled out of the shipped binary. |
| An operator-supplied export path is data; never code, never read | Holds. `options.rs:79-89` checks only that the value is present, non-empty and appears at most once, then stores the `&str` as an owned `String`. It is not canonicalized, not joined to anything, not compared against a root, not passed to a shell, and never opened at start-up. It reaches exactly one call, `fs::File::create(path)` (`export.rs:40`), when the operator presses the export key. `options::tests::an_export_path_is_taken_verbatim_as_data` asserts verbatim retention for four strings — `-x`, `a b/c.log`, `../../events.log` and `sub/dir/events.log` — so a value that looks like a flag, holds a space, or traverses upward is still a string. The accompanying disclosure is that a writable path outside the working directory *is* written if the operator supplies one, which is the specified behavior and is the operator's own instruction. |
| No credential, secret, environment variable, absolute path or wall-clock value in a frame or an export | Holds. `grep -rn 'env::'` over shipped code returns three sites: `env::args()` in each binary, which is operator input rather than environment state, and `option_env!("MOKITERIONS_COMMIT")` at `render.rs:34`, which is a compile-time substitution with no run-time read. No `SystemTime`, no `Instant` value and no `Local::now` reaches a rendered string. `Instant` appears only in `main.rs`, where it schedules sleeps and computes the idle wait; it is never formatted into a frame or an export. `SystemTime` and `UNIX_EPOCH` appear nowhere in either package, so no wall-clock reading exists to leak. The measured part is narrower than the claim: `verification::no_frame_carries_an_environment_value` searches every frame at every renderable viewport across 40 interacting ticks, and `export::tests::nothing_environment_specific_reaches_the_file` searches the export bytes, for a **fixed forbidden list** — `C:\`, `/home/`, `/Users/`, `AppData`, `PATH=`, `TEMP`, `token`, `secret`, `api_key`, `ANTHROPIC` — not for the live environment's own values. The export test additionally requires every line to be either a `tick=` record or the single trailer, which is a whitelist and therefore the stronger of the two. `render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific` pins the footer's fields to the four specified ones. |
| The observer offers the operator no control that mutates the world | Holds. `state.rs` handles every bound key; the only branch that reaches the engine is the one calling `advance_tick`, and it passes no operator data — `advance_tick(&mut self)` takes no argument, so there is no channel by which a key press could become a rule input. Speed, pause, zoom, pan, follow, select, filter, overlay and export all change observer state only. `verification::a_filter_changes_what_is_presented_and_nothing_else` and `verification::holding_consumes_nothing_however_long_it_is_held` measure it. |
| No `unsafe` | Holds. `grep -rn unsafe` over both packages returns nothing. The observer inherits `unsafe` from its dependencies, which is what `ADR-MOK-002` decides on; `dependency-review.txt` carries the crate list that decision rests on. |
| Dependency surface is confined to the observer | Holds; see rule 3. The figure `SPEC-MOK-002` line 507 states as "57 crates" is the `--edges normal` count. With build edges it is 59 and without proc-macros 37, so the specified number is one of three defensible readings rather than the only one — recorded in the completion summary. |

## Two properties this review cannot establish

1. **That no future call to `run` can be made through the surface.** Rule 2's deviation above is
   a shape finding, not a behavior finding. Nothing prevents a third host from calling
   `mokiterions_core::simulation::Simulation::run`; what is established is that the observer does
   not. Closing it would need an artifact change, which is the owner's decision, not this work's.
2. **That the terminal is restored on a path other than the three specified ones.**
   `terminal-restoration.txt` measures the normal exit, the refusal exit and the panic exit, and
   states plainly that a signal-terminated process was not measured because a signal is not one of
   the three paths `SPEC-MOK-002` names. A process killed outside the panic hook's reach leaves the
   terminal in raw mode, and no code in either package can change that.
