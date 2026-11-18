//! How far do long-lived particles fly before decaying?
//!
//! A particle of momentum `p` and mass `m` travels on average `βγcτ`, with
//! `βγ = p/m`. Whether it decays at the vertex, at a displaced vertex a
//! tracker can resolve, inside the calorimeter or not at all is what decides
//! how it is reconstructed: B mesons give millimetre displacements (b-tagging),
//! K⁰_S and Λ centimetres (V0 finding), charged pions and muons leave the
//! detector.
//!
//! ```sh
//! cargo run --example flight_distance
//! ```

use oxiparticle::Particle;

/// A human-readable length from millimetres.
fn length(mm: f64) -> String {
    if !mm.is_finite() {
        "stable".into()
    } else if mm >= 1e6 {
        format!("{:.1} km", mm / 1e6)
    } else if mm >= 1e3 {
        format!("{:.1} m", mm / 1e3)
    } else if mm >= 10.0 {
        format!("{:.1} cm", mm / 10.0)
    } else if mm >= 1.0 {
        format!("{mm:.2} mm")
    } else if mm >= 1e-3 {
        format!("{:.0} µm", mm * 1e3)
    } else if mm >= 1e-6 {
        format!("{:.1} nm", mm * 1e6)
    } else {
        // Femtometres: a resonance decays within a few proton radii.
        let fm = mm * 1e12;
        if fm >= 100.0 {
            format!("{fm:.0} fm")
        } else {
            format!("{fm:.3} fm")
        }
    }
}

fn main() {
    let names = [
        "B0",
        "B+",
        "B(s)0",
        "Lambda(b)0",
        "D0",
        "D+",
        "tau-",
        "K(S)0",
        "Lambda",
        "Sigma+",
        "K+",
        "pi+",
        "mu-",
        "pi0",
        "J/psi(1S)",
        "Z0",
    ];
    let momenta_gev = [5.0, 20.0, 100.0];

    print!("{:<12} {:>12} {:>10}", "particle", "mass [MeV]", "cτ");
    for p in momenta_gev {
        print!(" {:>13}", format!("L @ {p} GeV"));
    }
    println!();

    for name in names {
        let Some(particle) = Particle::from_name(name) else {
            println!("{name:<12} (not in the table)");
            continue;
        };
        let (Some(mass), Some(ctau)) = (particle.mass(), particle.ctau()) else {
            continue;
        };
        print!("{name:<12} {mass:>12.3} {:>10}", length(ctau));
        for p in momenta_gev {
            // βγ = p/m (momentum in MeV, like the mass).
            let beta_gamma = p * 1e3 / mass;
            print!(" {:>13}", length(beta_gamma * ctau));
        }
        println!();
    }

    // The b-tagging argument in one line: a B meson from a top decay carries
    // tens of GeV, so it flies millimetres, far beyond a pixel detector's
    // ~20 µm impact-parameter resolution.
    let b = Particle::from_name("B0").expect("B0 is in the table");
    let flight = 50e3 / b.mass().unwrap() * b.ctau().unwrap();
    println!(
        "\nA 50 GeV B⁰ flies {} on average: {:.0}× a 20 µm vertex resolution.",
        length(flight),
        flight / 0.020
    );
}
