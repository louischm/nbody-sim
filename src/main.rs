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


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1000)]
    steps: usize
}

fn main() {

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

    let leapfrog = LeapfrogIntegrator;
    let writer = CsvWriter::new("output.csv");

    let mut engine_leapfrog = SimulationEngine::new(
        bodies,
        leapfrog,
        config.dt, 
        Some(writer),
    );
    
    engine_leapfrog.run(config.steps);

    plot_orbits("output.csv", "orbit.png").unwrap();

}