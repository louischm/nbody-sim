mod physics;
mod utils;
mod integrators;
mod simulation;
mod diagnostics;
mod config;
mod output;

use clap::Parser;
use physics::body::Body;
use nalgebra::Vector3;

use integrators::leapfrog::LeapfrogIntegrator;
use integrators::euler::EulerIntegrator;
use simulation::engine::SimulationEngine;
use config::simulation::load_config;
use output::csv_writer::CsvWriter;
use output::plot::plot_orbits;

use crate::config::simulation::SimulationConfig;
use crate::integrators::Integrator;


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "config/simulation.json")]
    config: String,

    #[arg(short, long, default_value = "leapfrog")]
    integrator: String,
}

fn main() {
    let args = Args::parse();
    let config = load_config("config/simulation.json");

    println!("Running simulation with {} steps", config.steps);


    let bodies = config.bodies.into_iter().map(|b| {
        Body::new(
            b.name,
            b.mass,
            Vector3::new(b.position[0], b.position[1], b.position[2]),
            Vector3::new(b.velocity[0], b.velocity[1], b.velocity[2]),
        )
    }).collect();

    let integrator_type = args.integrator.as_str();

    match integrator_type {
        "euler" => {
            let integrator = EulerIntegrator;
            run_simulation(bodies, integrator, config.dt, config.steps);
        }
        "leapfrog" => {
            let integrator = LeapfrogIntegrator;
            run_simulation(bodies, integrator, config.dt, config.steps);
        }
        _ => panic!("Unknown integrator"),
    }
}

fn run_simulation<I: Integrator>(
    bodies: Vec<Body>,
    integrator: I,
    dt: f64,
    steps: usize
) {
    let writer = CsvWriter::new("output.csv");

    let mut engine = SimulationEngine::new(
        bodies,
        integrator,
        dt,
        Some(writer),
    );

    engine.run(steps);
    plot_orbits("output.csv", "orbit.png").unwrap();
}