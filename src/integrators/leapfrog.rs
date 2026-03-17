use crate::physics::body::Body;
use crate::physics::gravity::compute_accelerations;
use crate::integrators::Integrator;

pub struct LeapfrogIntegrator;

impl Integrator for LeapfrogIntegrator {
    fn step(&self, bodies: &mut Vec<Body>, dt: f64) {
        // 1. First half-step velocity update
        for body in bodies.iter_mut() {
            body.velocity += body.acceleration * (0.5 * dt);
        }

        // 2. Full-step position update
        for body in bodies.iter_mut() {
            body.position += body.velocity * dt;
        }

        // 3. Recompute accelerations at new positions
        compute_accelerations(bodies);

        // 4. Second half-step velocity update
        for body in bodies.iter_mut() {
            body.velocity += body.acceleration * (0.5 * dt);
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_leapfrog_runs() {
        let mut bodies = vec![
            Body::new(
                "A".to_string(),
                1.0,
                Vector3::zeros(),
                Vector3::zeros(),
            ),
            Body::new(
                "B".to_string(),
                1.0,
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::zeros(),
            ),
        ];

        compute_accelerations(&mut bodies);

        let integrator = LeapfrogIntegrator;

        integrator.step(&mut bodies, 0.1);
    }
}