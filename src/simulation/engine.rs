use crate::physics::body::Body;
use crate::integrators::Integrator;
use crate::physics::gravity::compute_accelerations;
use crate::diagnostics::energy::total_energy;

pub struct SimulationEngine<I: Integrator> {
    pub bodies: Vec<Body>,
    pub integrator: I,
    pub dt: f64,
}

impl<I: Integrator> SimulationEngine<I> {
    pub fn new(bodies: Vec<Body>, integrator: I, dt: f64) -> Self {
        Self {
            bodies,
            integrator,
            dt,
        }
    }

    pub fn step(&mut self) {
        compute_accelerations(&mut self.bodies);
        self.integrator.step(&mut self.bodies, self.dt);
    }

    pub fn run(&mut self, steps: usize) {
        let mut initial_energy = total_energy(&self.bodies);

        for step in 0..steps {
            self.step();

            if step % 10 == 0 {
                let energy = total_energy(&self.bodies);
                let relative_error = (energy - initial_energy) / initial_energy.abs();

                println!(
                    "Step {} | Energy: {:.3e} | Delta: {:.3e} | Rel: {:.3e}", 
                    step,
                    energy,
                    energy - initial_energy,
                    relative_error,
                );
            }
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;
    use crate::integrators::leapfrog::LeapfrogIntegrator;

    #[test]
    fn test_engine_runs() {
        let bodies = vec![
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

        let integrator = LeapfrogIntegrator;

        let mut engine = SimulationEngine::new(bodies, integrator, 0.1);

        engine.run(10);
    }
}