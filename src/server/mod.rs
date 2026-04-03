pub mod routes;
pub mod state;
pub mod handlers;

use axum::{
    Router,
    routing::get,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use crate::integrators::Integrator;

pub async fn run_server<I: Integrator + 'static + Send + Sync>(
    addr: SocketAddr,
    state: state::AppState<I>
) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "N-Body Simulation API" }))
        .nest("/api", routes::api_routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .with_state(state);

    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
