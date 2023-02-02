use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct LabelInfo {
    color: i32,
    rate: i32,
}