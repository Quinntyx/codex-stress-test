# Repository Guidelines

This project follows a simple feature-sliced design.

* `src/features.rs` re-exports each feature. Each feature has a folder in `src/features` and a `components` subfolder for UI code.
* The `model` module contains non-UI state structures. Cross-feature imports are discouraged. Shared types belong in `model`.
* Keep files concise (~100 lines) when possible for clarity.
* Run `cargo build`, `cargo test`, and `cargo clippy -- -D warnings` before committing.
* Uploading binaries to the repository is **forbidden**. Text assets like `.svg` are fine.
* See the project roadmap in `README.md` for future work.
