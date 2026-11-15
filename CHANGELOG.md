# Changelog

Notable changes to oxiparticle, by release. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

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
