use axum::{
    Router,
    routing::{get, post},
};
use crate::server::handlers::{
    get_simulation_state,
    simulation_step,
    start_simulation,
    stop_simulation,
};
use crate::server::state::AppState;
use crate::integrators::Integrator;

pub fn api_routes<I: Integrator + 'static + Send + Sync>() -> Router<AppState<I>> {
    Router::new()
        .route("/state", get(get_simulation_state::<I>))
        .route("/step", get(simulation_step::<I>))
        .route("/start", post(start_simulation::<I>))
        .route("/stop", post(stop_simulation::<I>))
}
