use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Header,
    Paragraph,
    List,
    #[serde(other)]
    Unknown
}