use std::collections::HashMap;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PostPingResponse {
    pub count: usize,
    pub ip: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiStateResponce {
   pub ips: HashMap<String, usize>,
}