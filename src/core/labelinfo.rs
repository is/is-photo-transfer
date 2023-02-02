use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct LabelInfo {
    color: i32,
    rate: i32,
}
