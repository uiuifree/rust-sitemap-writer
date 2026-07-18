# AGENTS.md

Guidance for AI coding agents working on this repository.

## Project

`sitemap-writer` is a Rust library crate for generating XML sitemaps
(sitemaps.org protocol, sitemap index, 50,000-URL splitting, gzip, and the
Google image/video/news/hreflang extensions). Source lives in `src/`, runnable
examples in `examples/`.

## Commands

- Test (both feature configurations must pass): `cargo test && cargo test --all-features`
- Lint: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings`
- Docs: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`

CI (`.github/workflows/ci.yml`) runs all of the above; everything must be
clean before committing.

## Conventions

- Output XML is single-line and deterministic. Tests compare full XML strings
  byte-for-byte; never change existing expected strings unless the output
  format is intentionally changing (that is a breaking change).
- Every public item needs rustdoc (`#![warn(missing_docs)]` is enforced via
  clippy `-D warnings`) with a runnable doctest where practical. `Result`
  functions document failure cases under an `# Errors` heading.
- Tests live in `src/lib.rs` `mod tests`, write files only to
  `std::env::temp_dir()`, and clean up after themselves.
- The library performs no validation of user-provided values (URLs, dates,
  priority ranges) by design; only XML escaping. Do not add validation.
- gzip support is behind the optional `gzip` feature (`flate2`); the crate
  must keep building without it. Gate gzip items and tests with
  `#[cfg(feature = "gzip")]`.
- Do not add new required dependencies.
- Struct construction is documented as builder-first; struct literals need
  `..Default::default()`.
