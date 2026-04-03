use crate::physics::body::Body;

use nalgebra::Vector3;
use crate::utils::constants::G;

pub fn kinetic_energy(bodies: &Vec<Body>) -> f64 {
    bodies.iter().map(|b| b.kinetic_energy()).sum()
}

pub fn potential_energy(bodies: &Vec<Body>) -> f64 {
    let mut energy = 0.0;

    let n = bodies.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let r = bodies[j].position - bodies[i].position;
            let distance = r.norm();

            energy -= G * bodies[i].mass * bodies[j].mass / distance;
        }
    }

    energy
}

pub fn total_energy(bodies: &Vec<Body>) -> f64 {
    kinetic_energy(bodies) + potential_energy(bodies)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_energy_nonzero() {
        let bodies = vec![
            Body::new(
                "A".to_string(),
                1.0,
                Vector3::zeros(),
                Vector3::zeros(),
                "yellow"
            ),
            Body::new(
                "B".to_string(),
                1.0,
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::zeros(),
                "blue"
            ),
        ];

        let e = total_energy(&bodies);

        assert!(e.is_finite());
    }
}