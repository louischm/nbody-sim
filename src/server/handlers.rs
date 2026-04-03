use axum::{
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::server::state::AppState;
use crate::integrators::Integrator;

#[derive(Serialize, Deserialize)]
pub struct BodyData {
    pub name: String,
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub color: String,
}

#[derive(Serialize)]
pub struct SimulationState {
    pub bodies: Vec<BodyData>,
    pub time: f64,
    pub running: bool,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

pub async fn get_simulation_state<I: Integrator + 'static>(
    State(state): State<AppState<I>>,
) -> impl IntoResponse {
    let engine = state.engine.lock().unwrap();
    let running = *state.running.lock().unwrap();

    let bodies: Vec<BodyData> = engine
        .bodies
        .iter()
        .map(|b| BodyData {
            name: b.name.clone(),
            mass: b.mass,
            position: [b.position.x, b.position.y, b.position.z],
            velocity: [b.velocity.x, b.velocity.y, b.velocity.z],
            color: b.color.clone(),
        })
        .collect();

    let response = ApiResponse {
        success: true,
        data: Some(SimulationState {
            bodies,
            time: engine.time,
            running,
        }),
        message: None,
    };

    Json(response)
}

pub async fn simulation_step<I: Integrator + 'static>(
    State(state): State<AppState<I>>,
) -> impl IntoResponse {
    let mut engine = state.engine.lock().unwrap();
    engine.step();

    let bodies: Vec<BodyData> = engine
        .bodies
        .iter()
        .map(|b| BodyData {
            name: b.name.clone(),
            mass: b.mass,
            position: [b.position.x, b.position.y, b.position.z],
            velocity: [b.velocity.x, b.velocity.y, b.velocity.z],
            color: b.color.clone(),
        })
        .collect();

    let response = ApiResponse {
        success: true,
        data: Some(SimulationState {
            bodies,
            time: engine.time,
            running: true,
        }),
        message: Some("Step executed".to_string()),
    };

    Json(response)
}

pub async fn start_simulation<I: Integrator + 'static>(
    State(state): State<AppState<I>>,
) -> impl IntoResponse {
    let mut running = state.running.lock().unwrap();
    *running = true;

    let response = ApiResponse {
        success: true,
        data: Some("Simulation started"),
        message: None,
    };

    Json(response)
}

pub async fn stop_simulation<I: Integrator + 'static>(
    State(state): State<AppState<I>>,
) -> impl IntoResponse {
    let mut running = state.running.lock().unwrap();
    *running = false;

    let response = ApiResponse {
        success: true,
        data: Some("Simulation stopped"),
        message: None,
    };

    Json(response)
}
