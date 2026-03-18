# Rust N-Body Simulation

A high performance gravitational N-body simulator written in Rust.

## Goals

- Simulate orbital mechanics
- Explore numerical integration
- Demonstrate high performance Rust

## Features

- Modular simulation engine
- Multiple integrators (Euler and Leapfrog)
- Diagnostics and energy tracking
- Configurable simulations in JSON

![Orbit](orbit.png)

## Run instructions
cargo run -- --config config/simulation.json --integrator leapfrog