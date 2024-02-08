use markdown::mdast::Node;
use serde_json;

#[test]
fn serde_json_serialization() -> Result<(), String> {
    let markdown = "> Alpha bravo charlie";
    let mdast_json_str = r#"{"type":"root","children":[{"type":"blockquote","children":[{"type":"paragraph","children":[{"type":"text","value":"Alpha bravo charlie","position":{"start":{"line":1,"column":3,"offset":2},"end":{"line":1,"column":22,"offset":21}}}],"position":{"start":{"line":1,"column":3,"offset":2},"end":{"line":1,"column":22,"offset":21}}}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":22,"offset":21}}}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":22,"offset":21}}}"#;
    let mdast = markdown::to_mdast(&markdown, &markdown::ParseOptions::default()).unwrap();
    let mdast_serialized = serde_json::to_string(&mdast).unwrap();
    assert_eq!(mdast_json_str, mdast_serialized);

    Ok(())
}

#[test]
fn serde_json_deserialization() -> Result<(), String> {
    let markdown = "Alpha bravo *charlie*";
    // let mdast_json_str = r#"{"type":"root","children":[{"type":"blockquote","children":[{"type":"paragraph","children":[{"type":"text","value":"Alpha bravo charlie","position":{"start":{"line":1,"column":3,"offset":2},"end":{"line":1,"column":22,"offset":21}}}],"position":{"start":{"line":1,"column":3,"offset":2},"end":{"line":1,"column":22,"offset":21}}}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":22,"offset":21}}}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":22,"offset":21}}}"#;
    let mdast = markdown::to_mdast(&markdown, &markdown::ParseOptions::default()).unwrap();
    let mdast_serialized = serde_json::to_string(&mdast).unwrap();
    let mdast_deserialized = serde_json::from_str::<Node>(&mdast_serialized).unwrap();

    assert_eq!(mdast, mdast_deserialized);

    Ok(())
}
