use std::fmt::{Debug};
use serde::{Deserialize};
use crate::blocks::{Header, List, Paragraph};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Data {
    Header(Header),
    Paragraph(Paragraph),
    List(List),
    Unknown(serde_json::Value),
}