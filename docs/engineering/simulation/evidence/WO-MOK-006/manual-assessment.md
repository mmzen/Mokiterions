# Manual assessment: the eight judgements `VER-MOK-006` asks a reader to make

`VER-MOK-006`'s *Manual assessments* section lists eight things no script decides. Each is answered
below with what was read and what was concluded. Where the answer is adverse or qualified, it says so —
the section explicitly makes several of these "an adverse observation" if they go the wrong way, and an
assessment that finds nothing adverse in eight attempts is worth less than one that finds something.

---

## 1. `mokiterions-tui/src/lib.rs` declares the modules and does nothing else

> "A `lib.rs` that has acquired an item is an adverse observation."

**Read in full.** 33 lines. 21 of them are the module doc comment. The remaining code is:

    pub mod authority;
    pub mod export;
    pub mod layout;
    pub mod options;
    pub mod render;
    pub mod spatial;
    pub mod state;

    /// `VER-MOK-005`'s cross-cutting cases that reach a `#[cfg(test)]` hook, which no test outside the
    /// crate can link. `SPEC-MOK-004` rule 10 keeps them here; the rest are in `tests/verification.rs`.
    #[cfg(test)]
    mod verification;

**Nothing adverse.** Seven `pub mod`, one `#[cfg(test)] mod`, no `use`, no `pub use` re-export, no
type, no function, no constant, no trait, no macro. It defines no item and contains no test, which is
what the requirement-to-evidence matrix requires of it.

Two things about it are worth stating because a reader could reasonably have expected otherwise. There
is **no `pub use` facade** — a `lib.rs` that re-exported `Observer` and `Filter` at the crate root
would be more convenient to write tests against, and it would also be new public surface, which
`SPEC-MOK-004` rule 7 forbids. The tests reach `mokiterions_tui::state::Observer` by its module path
instead, exactly as the binary and the cross-cutting suite already did as `crate::state::Observer`.
And the `#[cfg(test)] mod verification;` declaration is the one structural asymmetry with the engine,
whose `SPEC-MOK-002` rule 3 leaves `lib.rs` with no test. `ADR-MOK-004` states the asymmetry, states
why — the engine has no cross-module internal test and the observer has eight — and lists it as a
negative consequence rather than hiding it. The declaration carries a doc comment saying the same thing
at the point a reader meets it.

## 2. `main.rs` declares no module, is not a shim, and made nothing public

**Read in full**, and compared against its predecessor.

- **Declares no module.** Confirmed: no `mod` at any line. The seven `mod` declarations and the
  `#[cfg(test)] mod verification;` moved to `lib.rs`; `main.rs` now reaches the layer through
  `use mokiterions_tui::{layout, options, render};` and two named imports.
- **Retained content is what it was.** From `const FRAME_INTERVAL` to the last line, 396 lines, SHA-256
  `2842d01b965d983f7371b88627cd7e94c346493d10f954918f78e61a92cb3a8f` in both trees. That region holds
  `FRAME_INTERVAL`, `INPUT_INTERVAL`, `enum Launch`, `prepare`, `main`, `observe`, `tick_interval`,
  `due`, `idle_for`, `report`, and the eight-test `#[cfg(test)] mod tests`. Start-up, the launch
  decision, the loop, scheduling, the idle calculation and the diagnostic report — all present, none
  altered.
- **It is not a shim.** 425 lines, of which `observe` alone is 74. `ADR-MOK-004`'s Option 4 is the
  rejected alternative and its rejection is on the record: making it thin would require a public entry
  point, and the four start-up tests would still reach `prepare` and `Launch` privately, so it would
  add public surface and move no test. The engine's `main.rs` is 19 lines and the observer's is 425;
  the two binaries are shaped differently on purpose.
- **Nothing was made public to leave it as it is.** `grep` for `pub` at any indentation in `main.rs`
  returns nothing. Every item in it — `Launch`, `prepare`, `observe`, `tick_interval`, `due`,
  `idle_for`, `report` — is private, as it was.

**Nothing adverse.** The 396-line digest is the strongest form this assessment could take: it is not a
reviewer's impression that the content is unchanged.

## 3. Each of the eight public-tier files has a subject a reader would recognise

> "A file that is a grab bag is an adverse observation even when every test in it is correctly placed."

| File | Tests | Lines | Subject |
|---|---|---|---|
| `tests/authority.rs` | 4 | 72 | the event-to-authority mapping |
| `tests/export.rs` | 7 | 144 | the export format and its filter semantics |
| `tests/layout.rs` | 7 | 146 | layout selection and the fidelity tiers |
| `tests/options.rs` | 7 | 114 | start-up input parsing and rejection |
| `tests/render.rs` | 8 | 219 | what a pane draws into the buffer |
| `tests/spatial.rs` | 7 | 134 | world-to-canvas mapping |
| `tests/state.rs` | 21 | 419 | observer state and what operator input does to it |
| `tests/verification.rs` | 16 | 772 | the properties that span the whole observer |

Seven of the eight are named for the module they exercise and contain that module's tests and no
others — `SPEC-MOK-004` rule 9's arrangement, and the arrangement a reader looking for a layout test
would guess. Each carries a header naming rule 9, stating that every test in it came from that
module's `#[cfg(test)] mod tests` block, and stating that the assertions are verbatim.

**Two qualified observations, neither adverse on the definition given.**

`tests/state.rs` carries 21 tests over 419 lines, more than twice any other per-module file. That is
not a grab bag: `state` is the observer's largest module by public surface — 54 `pub` occurrences
against `render`'s 25 — and its subject is a single one, the observer's presentation state and the
effect of each key on it. The tests are what a reader would look for under that heading. The file is
large because the module is.

`tests/verification.rs` is the one file whose subject is *by construction* not a single module: 772
lines for 16 tests, each spanning the whole observer. Judged against the standard as written, its
subject is recognisable — it is `VER-MOK-005`'s cross-cutting suite, it says so in its first line, and
"the properties that span the whole observer" is a subject a reader recognises. A reader who expected
one file per module will nonetheless meet one file that is not, and `SPEC-MOK-004` rule 9 naming it
explicitly is what keeps that from being a surprise.

## 4. No relocated test reads as weaker than its predecessor

**Read the comparison, not an impression.** `verbatim-comparison.txt` compares each of the 77 relocated
bodies against its predecessor-commit form: **76 are byte-identical** and the 77th differs in exactly
one path, twice — `crate::state::EVENT_CAPACITY` became `mokiterions_tui::state::EVENT_CAPACITY`, which
is the same constant reached by the same module path with the crate named instead of assumed.

The instruction says to confirm that where an access path changed the value asserted is the same value.
For 76 tests there is nothing to confirm, because nothing changed. For the 77th, the two occurrences
are of one constant, and the assertion around them is byte-identical. **Nothing adverse.**

A weakening this assessment could have caught: a relocated test that dropped an assertion it could no
longer reach, or that loosened `assert_eq!` to a range. A byte-identical body cannot have done either.
The 32 internal-tier tests were compared the same way and are also verbatim.

## 5. Each of the 32 inline tests stayed for the access it requires, not because moving it looked hard

> "The 12 in `render.rs` are the ones to press on, since that module has the largest private surface
> and the largest inline remainder."

**Pressed on all 12.** Each names a private item of `render` or one of the four hooks, and
`test-placement.md` records which:

| Test | What holds it inside |
|---|---|
| `the_territory_rule_marks_the_row_between_the_territories` | `BOUNDARY_GLYPH` |
| `the_bar_row_reproduces_the_specified_form` | `BAR_ROW_OVERHEAD`, `FULL_BAR`, `count`, `entry_lines` |
| `a_bar_row_shrinks_to_its_pane_and_never_overflows_it` | `BAR_ROW_OVERHEAD`, `FULL_BAR`, `bar_width` |
| `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | `ABSENT`, `entry_lines` |
| `a_depleted_territory_is_stated_in_words_at_every_width` | `count`, `territory_line` |
| `the_footer_survives_the_narrowest_viewport` | `count` |
| `the_help_overlay_lists_every_bound_key` | `help_lines` |
| `the_inspector_states_absence_rather_than_inventing_a_subject` | hook `select_for_test` |
| `the_log_shows_the_newest_records_and_reports_an_empty_filter` | hook `select_for_test` |
| `a_resize_changes_the_layout_and_nothing_else` | hook `select_for_test` |
| `an_overlay_covers_the_body_and_leaves_the_header_and_the_footer` | hook `set_overlay_for_test` |
| `the_authority_overlay_names_identifiers_for_every_event_type` | hook `set_overlay_for_test` |

Seven name a private constant or a private function of the module — `render` declares 39 private items
against 2 public ones, which is why this is where the inline remainder concentrates. Five name a hook,
and a hook is decisive rather than arguable: it does not exist in the build a `tests/` file links, so
the test could not compile there under any arrangement.

**The pressing question is whether any of the 12 could have been rewritten to assert the same thing
publicly.** For the five hook tests, no: each reaches presentation state that no key sequence sets,
which is why the hook exists. For the seven, a rewrite would have to assert a private constant's value
through rendered output that happens to contain it — which is a weaker test of a different thing, and
`SPEC-MOK-004` rule 7's test is "with its assertions unchanged". So the answer is that none of the 12
could move without weakening, and none stayed for effort.

The other 20 were checked to the same standard: 4 in `state.rs` (one private function `scroll_log`,
three hooks), 8 in `src/verification.rs` (all hook-using), 8 in `main.rs` (all naming `prepare` or
`Launch`, both private). **Nothing adverse.** The measured split is 77/32, which is exactly what
`SPEC-MOK-004` rules 9 and 10 predicted, so rule 9's provision for a corrected count was not needed.

## 6. Is the provenance-closed interface legible to a reader who had not seen the change?

> "An interface a reader cannot enumerate without running a script is an adverse observation, and the
> per-module counts are what the reader is given instead of a table."

**This one is qualified, and the qualification is adverse in part.** Stated plainly: a reader cannot
enumerate the observer's public interface from any approved artifact. `SPEC-MOK-002` rule 5 lets a
reader enumerate the *engine's* interface by reading a list. `SPEC-MOK-004` rule 6 gives per-module
counts — `authority` 5, `export` 3, `layout` 13, `options` 8, `render` 2, `spatial` 19, `state` 47 —
and a definition. To learn *which* 97 items, a reader must read the code or read
`public-item-census.txt` in this packet, which enumerates all 97 with 25 fields and 22 variants.

What can be said in mitigation, and it is real:

- The counts do let a reader detect a change without enumerating. A surplus or a shortfall is visible
  from seven integers, which is a cheaper check than reading a list of 97 and is a check the engine's
  enumerated form does not offer.
- The closure property is *more* mechanical than an enumeration, not less: "no item's visibility differs
  from before" is decided by a digest comparison, and an enumeration in a document can silently drift
  out of date while the digest cannot.
- `ADR-MOK-004` argues the case for provenance over enumeration on the grounds that the observer holds
  no authority, and it is right that maintaining a 97-item list for a non-trust-boundary would cost
  more than it protects and would rot on the first refactor.

**The honest verdict: legibility was traded for maintainability, deliberately, and the trade is
recorded rather than concealed.** A reader who wants the list has it in this packet. A reader who wants
it in an approved artifact does not, and that is the cost of the decision `ADR-MOK-004` took. It is
carried to the completion report as an observation, not as a defect, because rule 6 was approved in
this form.

## 7. Every enumerated path reference is required by the move

**Read all seven in `file-comparison.txt`.** Each is checked against the question "could the move have
been done without this?":

1. **Root `Cargo.toml` replaced, the old one moved.** Required. The old file was two manifests at once;
   `REQ-MOK-030` requires the root to declare no package, so the `[workspace]` table had to separate
   from the `[package]` tables. Every `[package]`, `[lib]`, `[[bin]]` and `[dependencies]` table moved
   unchanged, which the manifest diff shows: one hunk, at the top, and nothing below it.
2. **`[workspace]` removed from the engine's manifest, comment rewritten.** Required — the same
   separation, seen from the other side. The comment is prose, and it is the one edit in the seven that
   was not strictly forced; it is retained as an edit because a comment claiming the package "stays at
   the root" would have been false.
3. **`path = ".."` became `path = "../mokiterions-core"`.** Required. The engine is no longer the
   parent directory. Still keyed by the package name `Mokiterions`, no feature and no version added.
4. **Seven `mod` declarations in the observer's `main.rs` became three `use` statements.** Required,
   and required in the strong sense: declaring the modules in both targets would compile them twice
   and give the package two copies of every type.
5. **`mokiterions-tui/src/lib.rs` is new.** Required. It is the library target `REQ-MOK-028` asks for.
6. **`crate::state::EVENT_CAPACITY` became `mokiterions_tui::state::EVENT_CAPACITY`,** twice. Required.
   A `tests/` file is a separate crate, so `crate::` names itself.
7. **`.idea/Mokiterions.iml` source roots.** Required in the sense that the IDE had already written
   four new entries and left two stale ones pointing at a root `src/` and `tests/` that no longer
   exist. Corrected to the four real roots. This is editor configuration and affects no build.

**Nothing adverse.** Six of the seven are structurally forced; the seventh is a comment that would
otherwise have stated a falsehood. No edit was taken while a file was in transit — the nine engine
files and the seven observer modules are byte-identical, which is what makes that claim checkable
rather than assertable.

## 8. Only the directory names changed for the engine

**Read `mokiterions-core/Cargo.toml` against the predecessor's root manifest.** `diff` reports one
hunk, at lines 1–12: the comment block rewritten and the `[workspace]` table removed. Everything from
`[package]` to the end of the file is identical, which includes:

    [package] name = "Mokiterions"   version = "0.1.0"   edition = "2024"
    [lib]     name = "mokiterions"   path = "src/lib.rs"
    [[bin]]   name = "Mokiterions"   path = "src/main.rs"
    [dependencies]                   (empty)

The two target paths did not change, because a manifest's paths were always relative to the manifest —
the fact `SPEC-MOK-002`'s amended *Paths* clause records, and the reason this move touched so little.

Package name, library target name, binary target name, both target kinds, edition, version and the
empty dependency table: all unchanged. **Nothing adverse.** The engine's nine source files are
byte-identical at blob level, so "only the directory names changed" is exact rather than approximate.

---

## Summary of the eight

| # | Assessment | Verdict |
|---|---|---|
| 1 | `lib.rs` declares the modules and nothing else | nothing adverse |
| 2 | `main.rs` declares no module, is not a shim, made nothing public | nothing adverse |
| 3 | Each public-tier file has a recognisable subject | nothing adverse; two files noted |
| 4 | No relocated test reads as weaker | nothing adverse; 76 of 77 byte-identical |
| 5 | Each of the 32 stayed for required access | nothing adverse; all 12 in `render.rs` pressed |
| 6 | The provenance-closed interface is legible | **qualified adverse** — see above |
| 7 | Each path reference is required by the move | nothing adverse |
| 8 | Only directory names changed for the engine | nothing adverse |

One qualified adverse finding, on legibility, which is a property of `SPEC-MOK-004` rule 6 as approved
rather than of this implementation. It is carried to the completion report. Nothing in the eight is a
defect in the change.
