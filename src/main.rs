mod router;
mod state;
mod openapi;
mod dto;

use tokio;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use std::net::SocketAddr;

use state::api_state;
use router::ping;
use openapi::ApiDoc;

#[tokio::main]
async fn main() {
    let state = api_state::ApiState::new();

    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            let ips = state_clone.get_ips();
            println!("Current IP counts: {:?}", ips);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let app = Router::new()
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(ping::router(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}