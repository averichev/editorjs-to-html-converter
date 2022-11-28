use serde::{Deserialize};
use crate::models::block::Block;

#[derive(Deserialize)]
pub struct Content {
    pub blocks: Vec<Block>,
}