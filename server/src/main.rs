use crate::state::ServerState;
use axum::routing::get;
use axum::Router;
use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use std::io::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tower_http::services::ServeDir;

mod config;
mod crypto;
mod database;
mod services;
mod state;
mod stores;
mod types;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init().expect("Failed to initialize logger");

    info!("Starting server...");
    let state = ServerState::initialize().await.unwrap();
    info!("Initialized state");

    let mut router = Router::new().route("/ws", get(websocket::ws_handler));
    if !state.config.ws_only {
        router = router.fallback_service(ServeDir::new("./static"));
    } else {
        info!("Running in ws-only mode, skipping static file server");
    }

    let app = router.with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 48967));
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
