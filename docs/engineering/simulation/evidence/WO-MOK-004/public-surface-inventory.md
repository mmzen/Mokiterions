# WO-MOK-004 evidence: public surface inventory, regenerated

`VER-MOK-004` requires the public-surface inventory to be regenerated and diffed against the inventory
retained under `WO-MOK-003`, "confirming that the interface did not grow to serve the new tests". That is
the specific hazard: eight tests were added, and the cheapest way to make a test see something is to make
it public.

## Method

The same oracle as `WO-MOK-003/public-surface-inventory.md`: `cargo doc --no-deps --lib` after
`rm -rf target/doc`, then the generated pages listed. Rustdoc reports reachability rather than syntax, so
an item declared `pub` inside a private module would not appear and a re-export would.

## Result

15 HTML pages, of which `index.html` and `all.html` are not items. **13 items**, identical to the 13
recorded under `WO-MOK-003`:

```
all.html                                        <- not an item
index.html                                      <- not an item
cli/index.html                                  simulation/index.html
cli/constant.USAGE.html                         simulation/constant.CELLS_PER_TERRITORY.html
cli/enum.Command.html                           simulation/struct.Config.html
cli/fn.parse.html                               simulation/struct.Density.html
fn.execute.html                                 simulation/enum.Policy.html
                                                simulation/struct.RunSummary.html
                                                simulation/struct.Simulation.html
                                                simulation/enum.TerminationReason.html
```

Diff against `WO-MOK-003`'s inventory: **no addition, no removal, no change of form.** The public
interface is item for item `SPEC-MOK-002` rule 5 as verified under `VREC-MOK-003`.

## `cli::USAGE` kept its form

The page is `cli/constant.USAGE.html`. Had `USAGE` been converted to a function — which stable Rust would
have needed for a formatted string, and which was the obvious way to generate the defaults instead of
stating them — the page would be `cli/fn.USAGE.html` and item #4 of rule 5's closed enumeration would
have changed. It did not. In source:

```rust
pub const USAGE: &str = concat!(
```

`concat!` is a `std` macro evaluated at compile time, so the item is still a `&'static str` constant of
the same form, and `REQ-MOK-018`'s interface constraint holds.

## Declared `pub` items per file, unchanged

| File | `pub` declarations |
| --- | ---: |
| `src/lib.rs` | 3 |
| `src/cli.rs` | 3 |
| `src/simulation.rs` | 22 |

## Nothing was made public for the tests

The eight added tests use `cli::USAGE`, `cli::parse`, `cli::Command`, `simulation::Config`, `execute`,
and — in one helper — the text of `src/cli.rs` read through `include_str!`. Every one of those items was
already public before this work order. `include_str!` reads a file at compile time and grants no access
to anything private; it is why the coverage-totality property could be written over the parser's own
`match` arms without exposing them.

No feature flag, no `#[cfg(test)]` visibility widening, no test-support seam, and no
`#[allow(dead_code)]` was introduced. `SPEC-MOK-002` rule 6's prohibited list stays absent from the
interface, item for item, and rustdoc generated no page for any of it.
