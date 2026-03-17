mod physics;
mod utils;
mod integrators;
mod simulation;

use clap::Parser;
use physics::body::Body;
use nalgebra::Vector3;

use integrators::leapfrog::LeapfrogIntegrator;
use simulation::engine::SimulationEngine;


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1000)]
    steps: usize
}

fn main() {
    let args = Args::parse();

    println!("Running simulation with {} steps", args.steps);

    let bodies = vec![
        Body::new(
            "Sun".to_string(),
            1.989e30,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::zeros(),
        ),
        Body::new(
            "Earth".to_string(),
            5.972e24,
            Vector3::new(1.496e11, 0.0, 0.0),
            Vector3::new(0.0, 29780.0, 0.0),
        ),
    ];

    let integrator = LeapfrogIntegrator;
    let dt = 60.0;

    let mut engine = SimulationEngine::new(bodies, integrator, dt);
    
    engine.run(args.steps);
}