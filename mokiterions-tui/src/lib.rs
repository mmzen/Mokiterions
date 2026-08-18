//! The Mokiterions terminal observer's presentation layer.
//!
//! This is the observer package's library target. It carries layout selection, world-to-canvas
//! mapping, pane rendering, key dispatch, event retention and export — component 5 of
//! `ARCH-MOK-002`. The binary target beside it acquires the terminal, decides whether to launch,
//! schedules and loops; nothing here touches a terminal, and nothing here mutates simulation
//! state.
//!
//! The target exists so that the observer's contract can be exercised from outside the crate.
//! Before it, `mokiterions-tui` built one target — a binary — and a Rust integration test links a
//! library target, so every observer test was inside the implementation where it could reach any
//! private item. `REQ-MOK-028` is the requirement and `SPEC-MOK-004` rules 4 to 6 are the
//! contract.
//!
//! **The public interface is closed by provenance.** It is exactly the items that were already
//! public before this target existed, because every module was already `pub mod` inside the binary
//! and the cross-cutting suite already reached them that way. No item was widened to create this
//! target, and none may be widened to reach it from a test: `SPEC-MOK-004` rule 7 prohibits it and
//! rule 6 counts the interface so that a widening is visible. A test that will not compile outside
//! the crate is telling you its tier.

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
