use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Header {
    pub text: String,
    pub level: u8,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Paragraph {
    pub text: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct List {
    pub style: String,
    pub items: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Unknown {
    pub text: String,
}