use actix_web::web::Json;
use homebrew::abv::Abv;
use serde::{Deserialize, Serialize};
use tracing;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GravityReadings {
    original_gravity: Option<f32>,
    final_gravity: Option<f32>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbvResponse {
    abv: f32,
}

#[tracing::instrument(fields(request_id=%uuid::Uuid::new_v4()))]
pub async fn calculate_abv(gravity_readings: Json<GravityReadings>) -> Json<AbvResponse> {
    let og = gravity_readings.original_gravity.unwrap_or(1.0);
    let fg = gravity_readings.final_gravity.unwrap_or(1.0);
    let abv = Abv::new().original_gravity(og).final_gravity(fg);
    Json(AbvResponse { abv: abv.abv() })
}
