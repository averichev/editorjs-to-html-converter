mod models;
mod blocks;

use crate::models::content::{Content};
use crate::models::data::{Data};

pub fn convert_to_html(output: &String) -> String {
    let deserialize_result: Content = serde_json::from_str(&output)
        .expect("deserialize error");
    let blocks = deserialize_result.blocks;
    let mut html = String::new();
    for block in blocks {
        let data = block.data;
        match data {
            Data::Header(header) => {
                let header_html = format!("<h{l}>{t}</h{l}>",
                                          l = header.level,
                                          t = header.text
                );
                html.push_str(header_html.as_str());
            }
            Data::Paragraph(paragraph) => {
                let paragraph_html = format!("<p>{}</p>", paragraph.text);
                html.push_str(paragraph_html.as_str());
            }
            Data::List(list) => {
                let style = format!(" data-style=\"{s}\"", s = list.style);
                let mut items_html = String::new();
                for item in list.items {
                    let item_html = format!("<li>{t}</li>", t = item);
                    items_html.push_str(item_html.as_str())
                }
                let list_html = format!("<ul{s}>{i}</ul>",
                                        s = style,
                                        i = items_html
                );
                html.push_str(list_html.as_str());
            }
            _ => {}
        }
    }
    html
}