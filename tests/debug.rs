use markplus_core::parse_document;

#[test]
fn debug_list() {
    let md = "- item one\n- item two\n";
    let asset = parse_document(md).unwrap();
    println!("{}", serde_json::to_string_pretty(&asset.ast).unwrap());
}
