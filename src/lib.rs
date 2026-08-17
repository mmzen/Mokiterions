//! The Mokiterions simulation engine.
//!
//! This crate owns every simulation rule fixed by `SPEC-MOK-001` and the read-only
//! observation surface fixed by `SPEC-MOK-002`. It has no external dependencies, and
//! `ARCH-MOK-001` as amended admits no exception to that.
//!
//! Two hosts drive the same surface: the `mokiterions` binary, which streams the
//! `REQ-MOK-010` text record to standard output, and the `mokiterions-tui` observer.
//! Neither is privileged; the engine owns all mutable state and validates every
//! proposed action regardless of its source.

pub mod cli;
pub mod simulation;
