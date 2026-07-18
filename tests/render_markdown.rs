//    Copyright [2026] [Purnendu Kumar]
//    Apache-2.0 License

use markplus_core::parse_document;
use markplus_render::RenderEngine;

fn render(md: &str) -> String {
    let asset = parse_document(md).expect("parse failed");
    let mut out = String::new();
    for node in &asset.ast {
        markplus_render::blocks::render_markdown_node(node, &mut out);
    }
    out
}

#[test]
fn markdown_basic() {
    let out = render("# Heading\n\nParagraph with **bold** and _italic_.\n\n- list item\n");
    assert!(out.contains("# Heading"));
    assert!(out.contains("- list item"));
}

#[test]
fn markdown_fenced() {
    let out = render("```rust\nfn main() {}\n```\n");
    assert!(out.contains("```rust"));
    assert!(out.contains("fn main() {}"));
}

#[test]
fn markdown_blockquote() {
    let out = render("> Quote\n");
    assert!(out.contains("> Quote"));
}

#[test]
fn markdown_math() {
    let out = render("$$\n1+1=2\n$$\nInline $x=y$.\n");
    assert!(out.contains("$$"));
    assert!(out.contains("1+1=2"));
    assert!(out.contains("$x=y$"));
}

#[test]
fn markdown_table() {
    let out = render("| A | B |\n| - | - |\n| 1 | 2 |\n");
    assert!(out.contains("| A |"));
}

#[test]
fn markdown_hr() {
    let out = render("---\n");
    assert!(out.contains("---"));
}

#[test]
fn markdown_directive_and_footnote() {
    let out = render("::note\nhello\n::\n\n[^1]: note text\n");
    assert!(out.contains("[^1]:"));
}
