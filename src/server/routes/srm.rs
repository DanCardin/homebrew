// use serde::{Deserialize, Serialize};
// use homebrew::srm::SRM_TO_HEX;
// #[derive(Deserialize)]
// struct SrmRequest {
//     value: u8,
// }
//
// #[derive(Serialize)]
// struct SrmResponse {
//     value: String,
// }
//
// #[post("/srm.convert", format = "json", data = "<srm>")]
// fn srm_convert(srm: Json<SrmRequest>) -> Json<SrmResponse> {
//     Json(SrmResponse {
//         value: SRM_TO_HEX.get(&srm.value).unwrap_or(&"#000000").to_string(),
//     })
// }
