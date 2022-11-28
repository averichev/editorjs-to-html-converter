use serde::{Deserialize};
use crate::models::data::{Data};
use crate::models::r#type::Type;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub r#type: Type,
    pub data: Data
}