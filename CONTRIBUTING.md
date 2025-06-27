# Contributing to Codeite

* Follow the design guidelines in `AGENTS.md`.
* Organize UI code by features inside `src/features/<feature>/components`.
* Add new modules in `src/features.rs` so features can be imported directly.
* Ensure `cargo clippy -- -D warnings` passes before submitting changes.
