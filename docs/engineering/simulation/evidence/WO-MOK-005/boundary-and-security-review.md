# Boundary and security review — WO-MOK-005

`SPEC-MOK-003`'s data-and-interface contract and its security-and-privacy properties are claims
about the shape of a surface, which a test can only sample. This file enumerates the surface instead
and states, for each claim, whether it holds. Where it does not hold as written, the deviation is
stated before the argument for it, not after.

Path citations re-based on 2026-08-19, because `WO-MOK-006` moved the engine package into
`mokiterions-core/` after this file was written. The engine package is byte-identical to
`origin/master` at 05dc6ac, so every finding below stands as measured; what changed is where the
files that were measured now live, and a citation that no longer resolves cannot be re-checked by
a reviewer. Two greps were re-run rather than re-based, because their results are line numbers and
those did move.

## The engine's complete public surface

`mokiterions-core/src/lib.rs:26-27` declares two public modules and nothing else, so the surface is
everything `pub` in `mokiterions-core/src/cli.rs` and `mokiterions-core/src/simulation.rs`.
Enumerated with `awk` over both files:

```
mokiterions::cli
  const USAGE: &str
  enum Command { Help, Run(Config) }
  fn parse<I, S>(args: I) -> Result<Command, String>

mokiterions::simulation
  const CELLS_PER_TERRITORY: usize
  struct Config { seed, tick_limit, policy, density, trace_actions }   all fields pub
  struct Density                          const DEFAULT: Self
                                          fn parse(&str) -> Result<Self, String>
                                          fn resources_per_territory(self) -> usize
  enum Policy                             fn parse(&str) -> Option<Self>
  enum Territory · struct Coordinate · enum Direction · enum FoodClass · enum Action
  enum TerminationReason · enum RegenerationSkipReason
  struct RunSummary                       fn reason(&self) -> TerminationReason
                                          fn ticks(&self) -> u64
                                          fn survivors(&self) -> usize
                                          fn deaths(&self) -> usize
  enum EventType                          const ALL: [Self; 12]
                                          fn as_str(self) -> &'static str
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

The crate is `mokiterions` in snake case; see rule 3 below for why the package is `Mokiterions` and
the library target is not.

### Rule 2 — "`advance_tick` is the only operation that changes simulation state, and the surface exposes no other `&mut self` method that does"

**This did not hold as written, and `SPEC-MOK-003` rule 2 has been amended to state the surface as
it is.** The surface exposes two `&mut self` methods that change simulation state: `advance_tick`
and `run`. `grep -rn 'pub fn .*&mut self' mokiterions-core/src/` returns exactly those two and no
others: `simulation.rs:1205` and `simulation.rs:1261`.

`run` is the `REQ-MOK-010` whole-run entry point, and this work did not expose it. At `origin/master`
— 05dc6ac — `mokiterions-core/src/lib.rs:26-27` already declared `pub mod cli` and `pub mod
simulation`, and `mokiterions-core/src/simulation.rs:1205` already read
`pub fn run<W: Write>(&mut self, …)`. This file previously cited the same declaration at
`src/simulation.rs:821` against the earlier baseline 903c9943; the finding is the same one, re-read
at the current baseline, and `git show 903c9943:src/simulation.rs` still shows line 821 for anyone
checking the superseded citation. Both were established by
`WO-MOK-003`, which created the library target, and both are bound by a `verified` `VREC-MOK-003`.
So `run`'s public reachability is inherited, not introduced, and the rule as originally written was
already inaccurate about the surface before this work began. What this work added above it is the
observation surface; what it did not do is add a second way to change state.

Narrowing `run` again would mean either relocating the engine's sources — which `WO-MOK-005` puts
out of scope and `SPEC-MOK-003` rule 3 of the component layout forbids, because the text stream must
not move — or duplicating the run loop, which would give two implementations of the rules. Neither
is available inside this work order's envelope, and `WO-MOK-005` states that a mismatch between the
specification and the tree "requires an amended specification and re-approval, never a quietly
adjusted constraint or a relaxed assertion". The amendment is therefore the correct disposition and
is recorded in `SPEC-MOK-003`'s amendment record, marked **outstanding**: writing the amended text
is implementation, and approving it is the technical owner's act.

What is true, and is the property the rule exists to protect, is that the *observer* calls only
`advance_tick`. Every `mokiterions` import in `mokiterions-tui` is listed under rule 3 below; none
reaches `run`, and `verification::one_advance_is_one_tick_and_a_finished_run_refuses` and
`verification::observed_and_unobserved_runs_are_identical_on_every_declared_seed` are the measured
consequence. Both checks the rule *can* meet are met; the one it cannot is that the count is two.

Two further `&self` methods are on the surface but were absent from the rule's method listing:
`termination_reason` and `initialization_events`. Neither mutates, so the mutation count is
untouched, but the listing was not the whole surface. `new` returns `Result<Self, String>` and
`advance_tick` returns `Result<TickOutcome, String>` where the listing showed the bare types. The
listing has been corrected to the real signatures in the same amendment.

### Rule 1 — "Every snapshot type contains owned values only: no reference into engine state, no shared handle, no interior mutability, and no method that mutates"

**Holds.** Each of `WorldSnapshot`, `TerritorySnapshot`, `AgentSnapshot`, `ResourceSnapshot`,
`DecisionSnapshot`, `TickOutcome` and `Event` is a plain struct or enum of `u8`, `u64`, `usize`,
`bool`, `String`, `Vec<T>`, `Option<T>`, `[T; 2]` and the engine's own `Copy` enums. No field is a
reference, so no snapshot type has a lifetime parameter. `grep -rnE 'RefCell|Cell<|Rc<|Arc<|Mutex|RwLock|AtomicU|static mut'`
over `mokiterions-core/src/` and `mokiterions-tui/src/` returns nothing, so there is no interior
mutability and no shared handle anywhere in either package. The only `impl` blocks on these types are `event_type`
on `Event` and `EventDetail`, both `&self`, and derived `Debug`, `Clone`, `PartialEq`, `Eq`
(`Copy` where the type has no `String`).

`Simulation` has seven private fields and no public one, so nothing outside the module can reach
`agents`, `foods` or `entropy` at all. That is what makes "the observer receives no mutable handle
to world, agent, resource or event-log state" a type-level fact rather than a discipline.

The corroborating measurement is `verification::drawing_is_pure`, which clones a snapshot, draws at
every declared viewport in both zooms, and requires the clone unchanged.

### Rule 3 — "Dependency direction is one-way"

**Holds.** `mokiterions-tui/Cargo.toml` depends on `Mokiterions` by path; the engine manifest's
`[dependencies]` table is empty and carries a comment saying `SPEC-MOK-002` rule 1 and
`ARCH-MOK-001` both require it to stay so. `dependency-review.txt` records
`cargo tree -p Mokiterions` on every edge kind resolving to the package alone, and the observer
absent from every engine resolution.

The engine's *code* contains no reference to the observer. Re-run on 2026-08-19, to be exact about
what `grep -rn 'mokiterions-tui\|mokiterions_tui' mokiterions-core/src mokiterions-core/tests
mokiterions-core/Cargo.toml Cargo.toml` does return, since "no reference" would be wrong:
`mokiterions-core/src/lib.rs:7` and `:20`, both doc comments — one naming the dependency the engine
may not share, one naming the two hosts; `mokiterions-core/Cargo.toml:26`, the comment on the empty
`[dependencies]` table; and `Cargo.toml:16`, the workspace `members` list at the virtual root.
Nothing under `mokiterions-core/tests/` matches. The `members` entry is a workspace-root declaration
that `SPEC-MOK-003`'s component layout mandates, not a dependency edge —
`cargo tree -p Mokiterions` is the check that distinguishes them, and it resolves to the engine
alone. There is no `use`, no `extern crate` and no dependency entry.

In the other direction, `grep -rn 'mokiterions::|use mokiterions|mokiterions =' mokiterions-tui/src/`
returns exactly thirteen sites, re-measured the same day: `authority.rs:7`, `main.rs:24,26,27`,
`options.rs:4,8,9`, `render.rs:13,937`, `spatial.rs:8`, `state.rs:10,11`, `verification.rs:22`. The
set has changed since this file was written, because `WO-MOK-006` split the observer into a library
and a binary and moved 77 tests out of `src/`; the count is thirteen either way by coincidence, not
because the same thirteen sites are there.

Four of the thirteen do not reach the engine at all. `options.rs:4` is a doc comment, and
`main.rs:24`, `:26` and `:27` are `use mokiterions_tui::…` — the binary importing the observer's own
library, which matches the grep's `use mokiterions` alternative and is not an engine edge.

Three of the remaining nine are compiled out of the shipped binary: `state.rs:10`, gated by the
`#[cfg(test)]` at `state.rs:9`; `render.rs:937`, inside the `#[cfg(test)] mod tests` that opens at
`render.rs:933`; and `verification.rs:22`, whose whole module is `#[cfg(test)]` at
`mokiterions-tui/src/lib.rs:32-33`. **Six `use` statements reach the engine in shipped code** —
`authority.rs:7`, `options.rs:8`, `options.rs:9`, `render.rs:13`, `spatial.rs:8` and `state.rs:11` —
one fewer than the seven this file recorded before the split, and every one of them a read.

The crate is imported as `mokiterions`, in snake case, while the package and binary are
`Mokiterions`: the package's `[lib] name` is what an importing crate names, and `SPEC-MOK-002`
rule 2 fixes it in snake case because the declared lint gate implies `non_snake_case`. The path
dependency in the observer manifest is keyed by package name, and since `WO-MOK-006` its value
points into the moved package: `Mokiterions = { path = "../mokiterions-core" }`.

### Rule 4 — "Snapshot ordering is stable and specified"

**Holds.** `snapshot` filters to living agents and sorts by identifier, filters to standing
resources in the engine's insertion order, and copies the retained decision list, which
`advance_tick` builds in the same ascending identifier order the rules process agents in.
`verification::every_presented_value_is_the_snapshots` and
`state::tests::initialization_events_are_retained_in_authoritative_order` measure the consequence.

### Rule 5 — "The engine's own `SPEC-MOK-001` behavior is unchanged"

**Holds, and is the strongest-evidenced claim in this file.** `additivity-proof.txt` now shows the
whole engine *package* byte-identical to `origin/master` at 05dc6ac, not merely its test corpus:
`git diff --stat origin/master -- mokiterions-core` and `git status --porcelain -- mokiterions-core`
are both empty, so every tracked file — sources, manifest, the five files under
`mokiterions-core/tests/` and the inline `#[cfg(test)] mod tests` alike — is unchanged, and the 60
engine tests pass on those bytes. That subsumes what this paragraph previously claimed by three
separate diffs against 903c9943, and it is a stronger claim, because a package that does not differ
at all cannot have changed behavior. `export-fidelity.txt` shows the observer's exports
byte-identical to the engine binary process's own stdout on five seeds at two tick counts.

## Security and privacy properties

| Claim | Finding |
|---|---|
| No network access | Holds. `grep -rn 'std::net\|TcpStream\|UdpSocket\|reqwest\|hyper::'` over both packages returns nothing. `dependency-review.txt` shows no networking crate in either resolution. |
| No credential, no model provider | Holds. Nothing in either package reads a credential store, a key file or a provider endpoint; there is no such code and no such dependency. |
| No asynchronous runtime | Holds. No `async`, no `.await`, no `tokio`, no `futures` in either resolution — `dependency-review.txt` records the crate list. The observer's timing is `std::thread::sleep` against two fixed intervals. |
| No database | Holds. No storage crate in either resolution; the only persistence is the export file. |
| The filesystem is written once per requested export and never read | Holds in shipped code. Every filesystem call in `mokiterions-core/src/` is: none. Every filesystem call in the observer's shipped code is `fs::File::create` and, on a write failure only, `fs::remove_file` of the path this call just created (`export.rs:40,46`). There is no `read_to_string`, no `OpenOptions`, no `include_str!` and no directory walk. The `fs::read_to_string`, `create_dir_all` and `remove_dir_all` calls that `grep` finds are all in test code and are compiled out of the shipped binary: one in `src/`, `remove_dir_all` at `main.rs:348` inside the `#[cfg(test)] mod tests` opening at `main.rs:280`, and the rest in `mokiterions-tui/tests/` (`export.rs:126,131,138`, `verification.rs:694`), which `WO-MOK-006` moved out of `src/`. |
| An operator-supplied export path is data; never code, never read | Holds. `options.rs:79-89` checks only that the value is present, non-empty and appears at most once, then stores the `&str` as an owned `String`. It is not canonicalized, not joined to anything, not compared against a root, not passed to a shell, and never opened at start-up. It reaches exactly one call, `fs::File::create(path)` (`export.rs:40`), when the operator presses the export key. `options::tests::an_export_path_is_taken_verbatim_as_data` asserts verbatim retention for four strings — `-x`, `a b/c.log`, `../../events.log` and `sub/dir/events.log` — so a value that looks like a flag, holds a space, or traverses upward is still a string. The accompanying disclosure is that a writable path outside the working directory *is* written if the operator supplies one, which is the specified behavior and is the operator's own instruction. |
| No credential, secret, environment variable, absolute path or wall-clock value in a frame or an export | Holds. `grep -rn 'env::\|option_env!'` over shipped code returns three sites: `env::args()` in each binary — `mokiterions-core/src/main.rs:13` and `mokiterions-tui/src/main.rs:114` — which is operator input rather than environment state, and `option_env!("MOKITERIONS_COMMIT")` at `render.rs:34`, which is a compile-time substitution with no run-time read. The grep finds three further sites, all `env::temp_dir()` in test code that is compiled out: `mokiterions-tui/src/main.rs:347`, `tests/export.rs:125` and `tests/verification.rs:693`. No `SystemTime`, no `Instant` value and no `Local::now` reaches a rendered string. `Instant` appears only in `mokiterions-tui/src/main.rs`, where it schedules sleeps and computes the idle wait; it is never formatted into a frame or an export. `SystemTime` and `UNIX_EPOCH` appear nowhere in either package, so no wall-clock reading exists to leak. The measured part is narrower than the claim: `verification::no_frame_carries_an_environment_value` searches every frame at every renderable viewport across 40 interacting ticks, and `export::tests::nothing_environment_specific_reaches_the_file` searches the export bytes, for a **fixed forbidden list** — `C:\`, `/home/`, `/Users/`, `AppData`, `PATH=`, `TEMP`, `token`, `secret`, `api_key`, `ANTHROPIC` — not for the live environment's own values. The export test additionally requires every line to be either a `tick=` record or the single trailer, which is a whitelist and therefore the stronger of the two. `render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific` pins the footer's fields to the four specified ones. |
| The observer offers the operator no control that mutates the world | Holds. `state.rs` handles every bound key; the only branch that reaches the engine is the one calling `advance_tick`, and it passes no operator data — `advance_tick(&mut self)` takes no argument, so there is no channel by which a key press could become a rule input. Speed, pause, zoom, pan, follow, select, filter, overlay and export all change observer state only. `verification::a_filter_changes_what_is_presented_and_nothing_else` and `verification::holding_consumes_nothing_however_long_it_is_held` measure it. |
| No `unsafe` | Holds. `grep -rn unsafe` over both packages returns nothing. The observer inherits `unsafe` from its dependencies, which is what `ADR-MOK-003` decides on; `dependency-review.txt` carries the crate list that decision rests on. |
| Dependency surface is confined to the observer | Holds; see rule 3. The figure `SPEC-MOK-003` states as "57 crates" at its lines 57, 528 and 589 is the `--edges normal` count. With build edges it is 59 and without proc-macros 37, so the specified number is one of three defensible readings rather than the only one — recorded in the completion summary and measured in `dependency-review.txt`, which now names the two build-edge-only crates. |

## Two properties this review cannot establish

1. **That no future call to `run` can be made through the surface.** Rule 2's deviation above is
   a shape finding, not a behavior finding. Nothing prevents a third host from calling
   `mokiterions::simulation::Simulation::run`, and nothing did before this work either — that
   reachability arrived with `WO-MOK-003`. What is established is that the observer does not call
   it. Closing it would need an artifact change, which is the owner's decision, not this work's.
2. **That the terminal is restored on a path other than the three specified ones.**
   `terminal-restoration.txt` measures the normal exit, the refusal exit and the panic exit, and
   states plainly that a signal-terminated process was not measured because a signal is not one of
   the three paths `SPEC-MOK-003` names. A process killed outside the panic hook's reach leaves the
   terminal in raw mode, and no code in either package can change that.
