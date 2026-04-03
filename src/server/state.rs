use std::sync::{Arc, Mutex};
use crate::physics::body::Body;
use crate::simulation::engine::SimulationEngine;
use crate::integrators::Integrator;

pub struct AppState<I: Integrator> {
    pub engine: Arc<Mutex<SimulationEngine<I>>>,
    pub running: Arc<Mutex<bool>>,
}

impl<I: Integrator> AppState<I> {
    pub fn new(engine: SimulationEngine<I>) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            running: Arc::new(Mutex::new(false)),
        }
    }
}

impl<I: Integrator> Clone for AppState<I> {
    fn clone(&self) -> Self {
        Self {
            engine: Arc::clone(&self.engine),
            running: Arc::clone(&self.running),
        }
    }
}
