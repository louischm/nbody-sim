mod physics;
mod utils;
mod integrators;
mod simulation;
mod diagnostics;
mod config;
mod output;
mod render;
mod server;

use std::io::empty;
use clap::Parser;
use macroquad::input::KeyCode::L;
use physics::body::Body;
use nalgebra::Vector3;

use integrators::leapfrog::LeapfrogIntegrator;
use integrators::euler::EulerIntegrator;
use simulation::engine::SimulationEngine;
use config::simulation::load_config;
use output::csv_writer::CsvWriter;
use output::plot::plot_orbits;
use macroquad::prelude::*;
use crate::config::simulation::BodyConfig;
use crate::integrators::Integrator;


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "config/simulation.json")]
    config: String,

    #[arg(short, long, default_value = "leapfrog")]
    integrator: String,

    #[arg(short, long, default_value = "gui")]
    mode: String,

    #[arg(short, long, default_value = "3000")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    if args.mode == "server" {
        // Run server mode with Tokio runtime
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(run_server_mode(args));
    } else {
        // Run GUI mode with macroquad
        macroquad::Window::new("NBody Simulation", run_gui_mode_wrapper(args));
    }
}

fn run_gui_mode_wrapper(args: Args) -> impl std::future::Future<Output = ()> {
    async move {
        run_gui_mode(args).await;
    }
}

async fn run_server_mode(args: Args) {
    use std::net::SocketAddr;
    use server::state::AppState;

    let config = load_config(&args.config);
    let bodies = parse_bodies_from_config(config.bodies);
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    
    match args.integrator.as_str() {
        "euler" => {
            let integrator = EulerIntegrator;
            let engine = SimulationEngine::new(bodies, integrator, config.dt, None);
            let state = AppState::new(engine);

            println!("Starting server on http://localhost:{}", args.port);
            server::run_server(addr, state).await.unwrap();
        }
        "leapfrog" => {
            let integrator = LeapfrogIntegrator;
            let engine = SimulationEngine::new(bodies, integrator, config.dt, None);
            let state = AppState::new(engine);

            println!("Starting server on http://localhost:{}", args.port);
            server::run_server(addr, state).await.unwrap();
        }
        _ => panic!("Unknown integrator"),
    }
}

async fn run_gui_mode(args: Args) {
    let config = load_config(&args.config);

    println!("Running simulation with {} steps", config.steps);


    let bodies = config
        .bodies
        .into_iter()
        .map(|b| {
            Body::new(
                b.name,
                b.mass,
                Vector3::new(b.position[0], b.position[1], b.position[2]),
                Vector3::new(b.velocity[0], b.velocity[1], b.velocity[2]),
                b.color.as_str(),
            )
        }).collect();

    let dt = config.dt;

    match args.integrator.as_str() {
        "euler" => {
            let integrator = EulerIntegrator;
            let mut engine = SimulationEngine::new(bodies, integrator, dt, None);
            loop {
                clear_background(BLACK);

                engine.step();
                render::draw_bodies(&engine.bodies);

                next_frame().await;
            }
        }
        "leapfrog" => {
            let integrator = LeapfrogIntegrator;
            let mut engine = SimulationEngine::new(bodies, integrator, dt, None);
            loop {
                clear_background(BLACK);

                engine.step();
                render::draw_bodies(&engine.bodies);

                next_frame().await;
            }
        }
        _ => panic!("Unknown integrator"),
    };
}

fn parse_bodies_from_config(body_configs: Vec<BodyConfig>) -> Vec<Body> {
    body_configs
        .into_iter()
        .map(|b| {
            Body::new(
                b.name,
                b.mass,
                Vector3::new(b.position[0], b.position[1], b.position[2]),
                Vector3::new(b.velocity[0], b.velocity[1], b.velocity[2]),
                b.color.as_str(),
            )
        })
        .collect()
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