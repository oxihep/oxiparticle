# Changelog

Notable changes to oxiparticle, by release. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- **Rust 1.100 and edition 2024.** The minimum supported Rust version
  (`rust-version`) moves from 1.95 to 1.100, the current stable release that
  CI already builds with, and the crates move to the 2024 edition (migrated
  with `cargo fix --edition`; formatting follows the 2024 style).

## [0.1.0]

First release under the `oxiparticle` name. The code is `oxiroot-particle`
0.3.0 from the [oxiroot](https://github.com/oxihep/oxiroot) workspace, moved to
its own repository with its history.

### Changed

- Renamed from `oxiroot-particle` to `oxiparticle`: replace `oxiroot_particle::`
  with `oxiparticle::` (oxiroot users keep `oxiroot::particle`, which re-exports
  this crate).
- `scripts/gen_particle.py` now writes next to itself instead of to a fixed
  local path.
