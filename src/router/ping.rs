use serde_json::Value;
use utoipa;

use crate::state::api_state;
use crate::dto::ping::{
    PostPingResponse,
    ApiStateResponce
};
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Json,
    },
    routing::{get, post},
    extract::{
        ConnectInfo,
        State
    },
    Router,
};


pub fn router(shared_state: api_state::ApiState) -> Router {
    Router::new()
    .route("/ping", post(post_ping_handler))
    .route("/ping", get(get_all_ip_handler))
    .with_state(shared_state)
}

#[utoipa::path(
    post,
    path = "/ping",
    responses(
        (status = 200, description = "Save IP", body = PostPingResponse),
    ),
)]
pub async fn post_ping_handler(
    State(state): State<api_state::ApiState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let new_count = state.insert_ip(addr);

    Ok((StatusCode::OK, Json(PostPingResponse {
        count: new_count,
        ip: addr.to_string()
    })))
}


#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Get Ping state", body = String),
    ),
)]
pub async fn get_all_ip_handler(
    State(state): State<api_state::ApiState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let serialize = api_state::ApiStateSerialized::new(&state);

    let schema = ApiStateResponce {
        ips: serialize.ips
    };

    Ok((StatusCode::OK, Json(schema)))
}