# Contributing to Codeite

* Follow the design guidelines below.
* Organize UI code by features inside `src/features/<feature>/components`.
* Add new modules in `src/features.rs` so features can be imported directly.
* Ensure `cargo build`, `cargo test`, and `cargo clippy -- -D warnings` pass before submitting changes.
* Uploading binaries is **forbidden**. Only text-based assets (like `.svg`) may be committed.
* Keep files concise and organize UI code in `src/features/<feature>/components`. Add new modules in `src/features.rs`.
