//! Two-body decay kinematics from the PDG masses.
//!
//! In the rest frame of a particle of mass `M` decaying to masses `m₁` and
//! `m₂`, each daughter has momentum
//!
//! ```text
//! p* = √[(M² − (m₁ + m₂)²)(M² − (m₁ − m₂)²)] / 2M
//! ```
//!
//! and the energy released is `Q = M − m₁ − m₂`. These numbers set what a
//! detector sees: a Z gives two 45 GeV leptons, a K⁰_S two slow pions, and
//! a decay with `Q < 0` cannot happen at all.
//!
//! ```sh
//! cargo run --example two_body_decays
//! ```

use oxiparticle::Particle;

/// The PDG mass of `name` in MeV; neutrinos, whose masses are not in the
/// table, count as massless.
fn mass(name: &str) -> f64 {
    let p = Particle::from_name(name).unwrap_or_else(|| panic!("{name} is not in the table"));
    match p.mass() {
        Some(m) => m,
        None if p.pdgid().is_neutrino() => 0.0,
        None => panic!("{name} has no mass in the table"),
    }
}

/// The daughters' momentum in the parent's rest frame, or `None` below
/// threshold.
fn p_star(parent: f64, m1: f64, m2: f64) -> Option<f64> {
    let a = parent * parent - (m1 + m2).powi(2);
    let b = parent * parent - (m1 - m2).powi(2);
    (a >= 0.0).then(|| (a * b).sqrt() / (2.0 * parent))
}

fn main() {
    let decays = [
        ("Z0", "mu-", "mu+"),
        ("W+", "e+", "nu(e)"),
        ("H0", "b", "b~"),
        ("J/psi(1S)", "mu-", "mu+"),
        ("Upsilon(1S)", "mu-", "mu+"),
        ("B+", "J/psi(1S)", "K+"),
        ("B0", "K+", "pi-"),
        ("D0", "K-", "pi+"),
        ("K(S)0", "pi+", "pi-"),
        ("Lambda", "p", "pi-"),
        ("phi(1020)", "K+", "K-"),
        ("rho(770)0", "pi+", "pi-"),
        // Below threshold: the φ cannot decay to a proton pair.
        ("phi(1020)", "p", "p~"),
    ];

    println!(
        "{:<26} {:>12} {:>12} {:>12}",
        "decay", "M [MeV]", "p* [MeV]", "Q [MeV]"
    );
    for (parent, d1, d2) in decays {
        let (m, m1, m2) = (mass(parent), mass(d1), mass(d2));
        let q = m - m1 - m2;
        let label = format!("{parent} → {d1} {d2}");
        match p_star(m, m1, m2) {
            Some(p) => println!("{label:<26} {m:>12.2} {p:>12.2} {q:>12.2}"),
            None => println!("{label:<26} {m:>12.2} {:>12} {q:>12.2}", "forbidden"),
        }
    }

    // A lepton from Z decay at rest: the Jacobian peak of its transverse
    // momentum sits at p* = M_Z / 2, the classic W/Z mass handle.
    let z = mass("Z0");
    let mu = mass("mu-");
    println!(
        "\nZ → μμ: each muon carries {:.2} GeV, so the lepton pT spectrum peaks near {:.1} GeV.",
        p_star(z, mu, mu).unwrap() / 1e3,
        z / 2e3
    );
}
