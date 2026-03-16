use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub mass: f64,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub acceleration: Vector3<f64>,
}

impl Body {
    pub fn new(name: String, mass: f64, position: Vector3<f64>, velocity: Vector3<f64>) -> Self {
        Self {
            name,
            mass, 
            position,
            velocity,
            acceleration: Vector3::zeros(),
        }
    }

    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.norm_squared()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_body_creation() {
        let body = Body::new(
            "Earth".to_string(),
            5.972e24,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 30000.0, 0.0),
        );

        assert_eq!(body.mass, 5.972e24);
        assert_eq!(body.position.x, 0.0);
    }

    #[test]
    fn test_kinetic_energy() {
        let body = Body::new(
            "Test".to_string(),
            2.0,
            Vector3::zeros(),
            Vector3::new(3.0, 0.0, 0.0),
        );

        let ke = body.kinetic_energy();

        assert_eq!(ke, 9.0);
    }
}