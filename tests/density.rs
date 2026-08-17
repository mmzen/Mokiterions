//! Public tier, `SPEC-MOK-002` rule 8: resolved resources per territory.
//!
//! Relocated from `src/simulation.rs` under `WO-MOK-003`. Both tests need only
//! `Density::parse`, `Density::DEFAULT`, `Density`'s `Display`, and the two rule 5
//! additions `Density::resources_per_territory` and `CELLS_PER_TERRITORY`. Assertions are
//! verbatim, as rule 12 requires.
//!
//! Rule 8 also names "the relationship between density, initial endowment, and capacity"
//! as this file's subject. That case stays in `src/simulation.rs` because it reads
//! per-territory resource counts before a run and drives regeneration directly, neither of
//! which rule 5 exposes; rule 7 places it in the internal tier and rule 12 forbids
//! weakening it to move it.

use mokiterions::simulation::{CELLS_PER_TERRITORY, Density};

#[test]
fn density_resolves_to_the_specified_resource_count() {
    // The mapping is fixed by `SPEC-MOK-001`: hundredths of a percent times the cells
    // in one territory, divided by 10,000, truncating toward zero. These three counts
    // are the ones the specification states.
    assert_eq!(CELLS_PER_TERRITORY, 8_192);
    for (density, expected) in [("0.15", 12), ("0.75", 61), ("1.50", 122)] {
        assert_eq!(
            Density::parse(density).unwrap().resources_per_territory(),
            expected,
            "density {density}% must resolve to {expected} resources per territory"
        );
    }
    assert_eq!(Density::DEFAULT.resources_per_territory(), 61);

    // Written forms that denote the same density resolve identically, and the display
    // form round-trips.
    assert_eq!(Density::parse("1.5"), Density::parse("1.50"));
    assert_eq!(Density::parse(".75"), Density::parse("0.75"));
    assert_eq!(Density::DEFAULT.to_string(), "0.75");
}

#[test]
fn a_density_resolving_to_no_resources_is_rejected() {
    // Truncation makes `0.01%` resolve to zero. That is invalid configuration rather
    // than an empty world, because an emptied territory can never regenerate.
    let error = Density::parse("0.01").unwrap_err();
    assert!(error.contains("zero resources"), "{error}");
    assert!(
        error.contains("0.02"),
        "the usable floor must be reported: {error}"
    );
    assert_eq!(Density::parse("0.02").unwrap().resources_per_territory(), 1);

    assert!(Density::parse("0").is_err());
    assert!(Density::parse("0.751").is_err());
    assert!(Density::parse("101").is_err());
    assert!(Density::parse("").is_err());
    assert!(Density::parse("-1").is_err());
}
