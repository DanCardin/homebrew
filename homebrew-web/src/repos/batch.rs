use serde::Deserialize;

pub mod fermentable;
pub mod note;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchId {
    pub batch_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTarget {
    pub batch_id: i32,
    pub target: String,
}
