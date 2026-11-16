# oxiparticle

[![CI](https://github.com/oxihep/oxiparticle/actions/workflows/ci.yml/badge.svg)](https://github.com/oxihep/oxiparticle/actions/workflows/ci.yml)
[![License: MIT AND BSD-3-Clause](https://img.shields.io/badge/license-MIT%20AND%20BSD--3--Clause-blue.svg)](LICENSE)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange.svg)](https://www.rust-lang.org)

Pure-Rust PDG particle data — a Rust take on
[scikit-hep `particle`](https://github.com/scikit-hep/particle), with no
dependencies:

- **`PdgId`** decodes a [PDG Monte Carlo particle ID](https://pdg.lbl.gov/current/reviews/rpp2023-rev-monte-carlo-numbering.pdf)
  purely from its digits: lepton, meson, baryon or nucleus; charge, spin and
  quark content. It is a port of scikit-hep's `pdgid.functions`, checked against
  it.
- **`Particle`** looks particles up in a bundled ~600-entry PDG table for mass,
  width, lifetime, isospin and parity, by ID or name.

```toml
[dependencies]
oxiparticle = { git = "https://github.com/oxihep/oxiparticle", tag = "v0.1.0" }
```

```rust
use oxiparticle::{Particle, PdgId};

// Decode an ID with no lookup, just arithmetic on the digits.
assert!(PdgId::new(2212).is_baryon()); // proton
assert_eq!(PdgId::new(-11).charge(), Some(1.0)); // positron

// Physical properties from the bundled table (MeV, ns).
let muon = Particle::from_name("mu-").unwrap();
assert!((muon.mass().unwrap() - 105.6583755).abs() < 1e-4);
```

The [particle data guide](https://oxihep.github.io/oxiroot/guide/particle/)
covers every classifier and accessor (it uses the `oxiroot::particle` re-export,
which is this crate).

## Regenerating the table

`src/data.rs` and the test fixtures are generated from the scikit-hep
`particle` package:

```bash
pip install particle
python scripts/gen_particle.py
```

## The oxihep family

Rust crates for high-energy physics, split the way Python's
[Scikit-HEP](https://scikit-hep.org) is. Each works on its own; together they
cover reading data, histogramming, fitting and plotting.

| Crate | What it does | Python counterpart |
| --- | --- | --- |
| [oxiroot](https://github.com/oxihep/oxiroot) | Read and write ROOT files: `TTree`, RNTuple, ROOT histograms and graphs | uproot |
| [oxihist](https://github.com/oxihep/oxihist) | Histograms: typed axes, weighted and profile storages | boost-histogram, hist |
| [oxiplot](https://github.com/oxihep/oxiplot) | Plotting to SVG/PNG/PDF with an mplhep look | matplotlib, mplhep |
| [oxifit](https://github.com/oxihep/oxifit) | Curve fitting with Minuit2, and a `TFormula` engine | iminuit |
| [oxistats](https://github.com/oxihep/oxistats) | Statistics, HEP lineshapes and intervals | scipy.stats, hepstats |
| **oxiparticle** (this crate) | PDG particle data and ID decoding | particle |

The crates are not on crates.io yet: depend on them from GitHub, pinned to a
release tag, as shown above.

## History

This crate was `oxiroot-particle` inside the
[oxiroot](https://github.com/oxihep/oxiroot) workspace; its history was carried
over when it moved here.

## License

MIT for the Rust code (see [LICENSE](LICENSE)). The bundled PDG table and the
`PdgId` decoder derive from scikit-hep `particle`, which is BSD-3-Clause (see
[LICENSE-3rdparty](LICENSE-3rdparty)).
