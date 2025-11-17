use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::State,
    routing::{Router, get},
};
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("HTTP server started at {:?}:{}", path, port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} Listening on {}", path, addr);
    let state = HttpServeState { path };

    let router = Router::new().route("/", get(index_handler).with_state(Arc::new(state)));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await?;
    Ok(())
}

async fn index_handler(State(state): State<Arc<HttpServeState>>) -> String {
    format!("the path is {:?}", state.path)
}
