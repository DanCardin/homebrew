use actix_web::web::Json;
use homebrew::srm::SRM_TO_HEX;
use serde::{Deserialize, Serialize};
use tracing;

#[derive(Debug, Deserialize)]
pub struct SrmRequest {
    value: u8,
}

#[derive(Debug, Serialize)]
pub struct SrmResponse {
    value: String,
}

#[tracing::instrument(fields(request_id=%uuid::Uuid::new_v4()))]
pub async fn to_hex(srm: Json<SrmRequest>) -> Json<SrmResponse> {
    Json(SrmResponse {
        value: SRM_TO_HEX.get(&srm.value).unwrap_or(&"#000000").to_string(),
    })
}
