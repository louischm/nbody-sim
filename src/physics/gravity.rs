use nalgebra::Vector3;

use crate::physics::body::Body;
use crate::utils::constants::G;

const SOFTENING: f64 = 1e-9;

pub fn acceleration_between(a: &Body, b: &Body) -> Vector3<f64> {
    let r = b.position - a.position;
    let distance_sq = r.norm_squared() + SOFTENING;
    let inv_r3 = 1.0 / distance_sq.powf(1.5);
    G * b.mass * r * inv_r3
}

pub fn compute_accelerations(bodies: &mut Vec<Body>) {
    let n = bodies.len();

    for i in 0..n {
        bodies[i].acceleration = Vector3::zeros();

        for j in 0..n {
            if i == j {
                continue;
            }

            let acc = acceleration_between(&bodies[i], &bodies[j]);

            bodies[i].acceleration += acc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_gravity_direction() {
        let a = Body::new(
            "A".to_string(),
            1.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::zeros(),
            "yellow"
        );

        let b = Body::new(
            "B".to_string(),
            10.0,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::zeros(),
            "blue"
        );

        let acc = acceleration_between(&a, &b);

        assert!(acc.x > 0.0);
    }

    #[test]
    fn test_force_symmetry() {
        let a = Body::new(
            "A".to_string(),
            5.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::zeros(),
            "yellow"
        );

        let b = Body::new(
            "B".to_string(),
            10.0,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::zeros(),
            "blue"
        );

        let acc_a = acceleration_between(&a, &b);
        let acc_b  = acceleration_between(&b, &a);

        assert!(acc_a.x > 0.0);
        assert!(acc_b.x  < 0.0);
    }
}