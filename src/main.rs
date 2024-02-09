use markdown::{md_to_hast, ParseOptions};
use serde_json::to_string_pretty;

fn main() {
    let markdown = r#"[link](https://example.com)"#.trim();

    let mdast = markdown::to_mdast(&markdown, &markdown::ParseOptions::default()).unwrap();
    let mdast_serialized = to_string_pretty(&mdast).unwrap();
    let hast = md_to_hast(&markdown, ParseOptions::gfm());

    println!("{}", mdast_serialized);
    println!("{:?}", hast);
    println!("{}", to_string_pretty(&hast).unwrap());
}
