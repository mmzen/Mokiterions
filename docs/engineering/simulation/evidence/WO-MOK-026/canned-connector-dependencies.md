# Item 11: the canned connector's dependency declaration, and `VER-MOK-018` case `S2`

Measured at candidate `6e9ca13` on 2026-08-29.

## The declaration

The canned connector is a `[[bin]]` of the engine's package, `mokiterions-core/Cargo.toml`:

```toml
[[bin]]
name = "canned-connector"
path = "tests/support/canned_connector.rs"
```

**It declares no dependency and adds none to the package's resolved graph.** That package's
`[dependencies]` table is empty, and `SPEC-MOK-002` rule 13's declared set is what
`scripts/check_declared_dependencies.py` compares it against in both directions.

The check's own words at this candidate:

```
targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu, aarch64-apple-darwin
cargo metadata --locked reports 182 packages, which is the whole lockfile across every target and both members and is not any package's set
[workspace.dependencies] holds no entry
Mokiterions — declared by SPEC-MOK-002
  Mokiterions: 0 declared, 0 in the manifest
  Mokiterions on aarch64-apple-darwin: 0 external crates, no figure declared
  Mokiterions on x86_64-pc-windows-msvc: 0 external crates, no figure declared
  Mokiterions on x86_64-unknown-linux-gnu: 0 external crates, no figure declared
  Mokiterions on aarch64-apple-darwin: 0 external crates (1 node including workspace members)
  Mokiterions on x86_64-pc-windows-msvc: 0 external crates (1 node including workspace members)
  Mokiterions on x86_64-unknown-linux-gnu: 0 external crates (1 node including workspace members)
  Mokiterions: builds and tests offline from the committed lockfile
…
Every declared set matches its resolved graph. 8.4a-8.4d pass.
```

`cargo tree -p Mokiterions --locked` reads, in full:

```
Mokiterions v0.1.0 (C:\Users\mathi\Mokiterions-understand-20260829-1340\mokiterions-core)
```

One node. **The connector added a target, not an edge**, and `SPEC-MOK-002` rule 13's set is untouched —
which is `ADR-MOK-007` decision 3's whole value: no dependency artifact moves for the live path.

The count is taken **at every edge kind including dev-only**, and it is taken at three targets rather than
at the one this measurement ran on, so a platform-conditional dependency could not hide in it.

## The connector's own imports

`mokiterions-core/tests/support/canned_connector.rs`, its complete import list:

```rust
use std::env;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;
```

Three items of the standard library. **No network type is reachable from it**, no `use mokiterions::` and no
`crate::` path anywhere in the file. Its three occurrences of the string `mokiterions`, case-insensitively,
are all in prose: two in its module documentation and one in the name of the environment variable it reads.

That absence is `S2`'s substance and also item 12's: the fixture cannot have been written against the
engine's internals, because it does not name them.

## What `S2` establishes

That **this** connector — the one this repository owns — declares nothing, resolves nothing, and reaches no
network. The spawn, the framing, both of rule 13's gates, the usage accounting, the cost arithmetic, the
spend ceiling and the retry path are all exercised through it, at no cost and with no provider, and
`gate-matrix.md`, `retry-evidence.md` and `ceiling-evidence.md` are those exercises.

## What `S2` does not establish, stated rather than implied

**Nothing here establishes anything about an operator's connector.** `SPEC-MOK-007` rule 10.6 is explicit
that this specification does not constrain, and cannot constrain, a program the operator supplies — its
language, its dependency surface, what it links, what it opens, or where it sends bytes. A connector is a
separate program on the operator's machine, spawned by path.

`VER-MOK-018` records this as a **limit rather than a gap**, and the difference matters: a gap is something
this work order could have closed and did not. This one cannot be closed by any check inside this
repository, at any cost, because the artifact being reasoned about is not in it. The only thing the
repository can do is document the obligations, which `docs/CONNECTOR_PROTOCOL.md` does as a checklist a
connector's author may work through — and a checklist is a request, not a guarantee.

The live run of this work order used a connector written outside this repository and never committed. Its
dependency surface is **not** measured here and is not measurable here. What is recorded about it is an
attestation — `credential-attestation.md` — and an attestation is a different kind of evidence from a
measurement, which is why the two are separate files.
