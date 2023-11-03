#[macro_use]
mod utils;
mod config;
mod endpoints;
mod models;
use axum::{http::StatusCode, routing::get, Router};
use reqwest::Url;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient};
use std::net::SocketAddr;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    println!("ccip_server: starting v{}", env!("CARGO_PKG_VERSION"));
    let conf = config::load();

    let shared_state = Arc::new(models::AppState {
        conf: conf.clone(),
        provider: JsonRpcClient::new(HttpTransport::new(
            Url::parse(&conf.starknet.rpc_url).unwrap(),
        )),
    });

    let cors = CorsLayer::new().allow_headers(Any).allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/resolve", get(endpoints::resolve::handler))
        .with_state(shared_state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], conf.server.port));
    println!("server: listening on http://0.0.0.0:{}", conf.server.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> (StatusCode, String) {
    (
        StatusCode::ACCEPTED,
        format!("quest_server v{}", env!("CARGO_PKG_VERSION")),
    )
}
