pub mod euler;

use crate::physics::body::Body;

pub trait Integrator {
    fn step(&self, bodies: &mut Vec<Body>, dt: f64);
}