use std::fs;
use editorjs_to_html_converter::{convert_to_html};
use html_parser::Dom;

#[test]
fn it_works() {
    let content = fs::read_to_string("tests/test_content/test.json").unwrap();
    let result = convert_to_html(&content);
    assert!(Dom::parse(result.as_str()).is_ok());
}