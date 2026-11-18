//! Decode the particles of a Monte Carlo truth record from their PDG IDs.
//!
//! A generator's event record (Pythia, Herwig, a `GenPart` collection) stores
//! each particle as a signed integer. `PdgId` reads what it is — lepton,
//! meson, baryon, nucleus, BSM state — and its charge, spin and quark content
//! from the digits alone, with no table lookup; `Particle` then adds the
//! measured properties from the bundled PDG table.
//!
//! ```sh
//! cargo run --example mc_truth
//! ```

use oxiparticle::{Particle, PdgId};

/// What kind of particle an ID is, from the digits alone.
fn family(id: PdgId) -> &'static str {
    if id.is_quark() {
        "quark"
    } else if id.is_lepton() {
        if id.is_neutrino() {
            "neutrino"
        } else {
            "lepton"
        }
    } else if id.is_gauge_boson_or_higgs() {
        "gauge/Higgs boson"
    } else if id.is_meson() {
        "meson"
    } else if id.is_baryon() {
        "baryon"
    } else if id.is_nucleus() {
        "nucleus"
    } else if id.is_susy() {
        "SUSY"
    } else if id.is_generator_specific() {
        "generator-specific"
    } else {
        "other"
    }
}

/// The heavy flavours a hadron contains.
fn flavours(id: PdgId) -> String {
    let mut f = String::new();
    for (has, q) in [
        (id.has_strange(), 's'),
        (id.has_charm(), 'c'),
        (id.has_bottom(), 'b'),
    ] {
        if has {
            f.push(q);
        }
    }
    if f.is_empty() { "-".into() } else { f }
}

fn main() {
    // A slice of a ttH(→bb) event record, plus a few states worth recognising:
    // a deuteron from a heavy-ion generator and a SUSY neutralino.
    let record = [
        6,
        -6,
        25,
        24,
        -24,
        5,
        -5,
        21,
        22,
        -11,
        12,
        13,
        -13,
        211,
        -321,
        111,
        130,
        310,
        3122,
        421,
        -511,
        531,
        443,
        5122,
        2212,
        1_000_010_020,
        1_000_022,
        81,
    ];

    println!(
        "{:>11}  {:<14} {:<19} {:>6} {:>5}  {:>6}  {:>12}",
        "PDG ID", "name", "family", "charge", "J", "heavy", "mass [MeV]"
    );
    for raw in record {
        let id = PdgId::new(raw);
        let particle = Particle::from_pdgid(raw);
        let name = particle.map_or("?", |p| p.name());
        let charge = id.charge().map_or("?".into(), |q| format!("{q:+.2}"));
        let spin = id.j().map_or("?".into(), |j| format!("{j}"));
        let mass = particle
            .and_then(|p| p.mass())
            .map_or("-".into(), |m| format!("{m:.3}"));
        println!(
            "{raw:>11}  {name:<14} {:<19} {charge:>6} {spin:>5}  {:>6}  {mass:>12}",
            family(id),
            flavours(id),
        );
    }

    // Counting what a selection would keep: the charged, stable-at-detector
    // particles of the record (what a tracker sees).
    let tracks: Vec<i32> = record
        .iter()
        .copied()
        .filter(|&raw| {
            let id = PdgId::new(raw);
            id.charge().is_some_and(|q| q != 0.0)
                && matches!(id.abspid(), 11 | 13 | 211 | 321 | 2212)
        })
        .collect();
    println!("\ncharged long-lived particles a tracker would see: {tracks:?}");
}
