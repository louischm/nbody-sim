use crate::physics::body::Body;
use crate::integrators::Integrator;

pub struct EulerIntegrator;

impl Integrator for EulerIntegrator {
    fn step(&self, bodies: &mut Vec<Body>, dt: f64) {
        for body in bodies.iter_mut() {
            body.velocity += body.acceleration * dt;
            body.position += body.velocity * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_velocity_update() {
        let mut body = Body::new(
            "Test".to_string(),
            1.0,
            Vector3::zeros(),
            Vector3::zeros(),
        );

        body.acceleration = Vector3::new(1.0, 0.0, 0.0);

        let integrator = EulerIntegrator;

        integrator.step(&mut vec![body], 1.0);
    }
}