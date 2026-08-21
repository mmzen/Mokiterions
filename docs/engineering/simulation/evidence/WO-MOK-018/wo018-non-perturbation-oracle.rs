//! TEMPORARY ORACLE for `WO-MOK-018`. Not part of the product.
//!
//! Retained as evidence. It was **appended to `mokiterions-tui/tests/verification.rs`**, run
//! once, and removed, on the precedent of `WO-MOK-013`'s `wo013-oracle.rs`. It is not a
//! standalone file and will not compile as one: it reuses that file's declared sets and helpers
//! — `SEEDS`, `VIEWPORTS`, `unobserved`, `observed_run`, `observed_lines`, `summary_from` and
//! `interact` — which is the whole point of appending it rather than writing a second
//! implementation of the contract's own comparison. It produced `non-perturbation.txt`.
//!
//! **It asserts nothing**, and it added no item to any interface. The test census in this pack
//! was measured on the tree without it.
// ---- TEMPORARY ORACLE for WO-MOK-018 ---------------------------------------------------------
//
// Appended to this file, run once, removed, and retained as `wo018-non-perturbation-oracle.rs` in
// `evidence/WO-MOK-018/`, on the precedent of `WO-MOK-013`'s `wo013-oracle.rs`. **It asserts
// nothing.** It reuses this file's own declared sets and helpers — `SEEDS`, `VIEWPORTS`,
// `unobserved`, `observed_run`, `observed_lines`, `summary_from` and `interact` — so the capture is
// the contract's own comparison written down rather than a second implementation of it. It lives
// here rather than in a file of its own for exactly that reason.

#[test]
fn wo018_capture_non_perturbation() {
    use std::fmt::Write as _;
    use std::fs;
    use std::path::PathBuf;

    let mut out = String::new();
    out.push_str(
        "WO-MOK-018 non-perturbation comparison\n\
         \n\
         `REQ-MOK-025` at every declared seed. The observed run is drawn at a rotating viewport\n\
         over the declared viewport set and interacted with on every round with the twelve-key\n\
         script `tests/verification.rs` drives its own observed runs with; the unobserved run is\n\
         `Simulation::run`, which is the engine binary's whole behaviour, writing into memory.\n\
         \n\
         Captured by a temporary oracle appended to that file, which asserts nothing. Every figure\n\
         below is counted from the two streams.\n\n",
    );

    for (label, ticks) in [("the contract's depth", "60"), ("WO-MOK-013's depth", "300")] {
        writeln!(out, "=== {ticks} ticks — {label}\n").unwrap();
        writeln!(
            out,
            "{:>6}  {:>10}  {:>10}  {:>9}  {:>16}  {}",
            "seed", "unobserved", "observed", "identical", "first difference", "final state"
        )
        .unwrap();
        for seed in SEEDS {
            let seed_text = seed.to_string();
            let args = ["--seed", &seed_text, "--ticks", ticks];
            let (expected, summary) = unobserved(&args);
            let observer = observed_run(&args);
            let observed = observed_lines(&observer);

            let first = observed
                .iter()
                .zip(expected.iter())
                .position(|(left, right)| left != right);
            let identical = first.is_none() && observed.len() == expected.len();
            let state = if summary_from(&observer) == summary {
                "agrees"
            } else {
                "DIFFERS"
            };
            writeln!(
                out,
                "{:>6}  {:>10}  {:>10}  {:>9}  {:>16}  {}",
                seed,
                expected.len(),
                observed.len(),
                if identical { "yes" } else { "NO" },
                first.map_or("none".to_string(), |index| index.to_string()),
                state
            )
            .unwrap();
        }
        writeln!(out).unwrap();
    }

    // The engine's own words for one seed, retained so the comparison is legible rather than only
    // tabulated.
    let args = ["--seed", "42", "--ticks", "300"];
    let (expected, summary) = unobserved(&args);
    let observer = observed_run(&args);
    writeln!(out, "=== seed 42, 300 ticks, in full\n").unwrap();
    writeln!(out, "engine summary line (unobserved):\n  {summary}\n").unwrap();
    writeln!(
        out,
        "the same line reconstructed from the observer's final state:\n  {}\n",
        summary_from(&observer)
    )
    .unwrap();
    writeln!(
        out,
        "unobserved records          {}\nobserved retained records   {}\n",
        expected.len(),
        observed_lines(&observer).len()
    )
    .unwrap();

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .join("docs")
        .join("engineering")
        .join("simulation")
        .join("evidence")
        .join("WO-MOK-018");
    fs::create_dir_all(&root).expect("the output directory");
    let path = root.join("non-perturbation.txt");
    fs::write(&path, &out).expect("writing the capture");
    println!("wrote {}", path.display());
}
