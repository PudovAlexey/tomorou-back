
use crate::dto::ping as ping_dto;
use crate::router::ping as ping_router;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        ping_router::post_ping_handler,
        ping_router::get_all_ip_handler,
    ),
    components(
        schemas(ping_dto::PostPingResponse),
        schemas(ping_dto::ApiStateResponce),
    )
)]
pub struct ApiDoc;