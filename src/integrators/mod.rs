pub mod euler;
pub mod leapfrog;

use crate::physics::body::Body;

pub trait Integrator {
    fn step(&self, bodies: &mut Vec<Body>, dt: f64);
}