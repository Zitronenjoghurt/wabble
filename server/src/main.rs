use crate::state::ServerState;
use axum::routing::get;
use axum::Router;
use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use std::io::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tower_http::services::ServeDir;

mod database;
mod state;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init().expect("Failed to initialize logger");

    info!("Starting server...");
    let state = ServerState::initialize().await.unwrap();
    info!("Initialized state");

    let app = Router::new()
        .route("/ws", get(websocket::ws_handler))
        .fallback_service(ServeDir::new("./static"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 48967));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
