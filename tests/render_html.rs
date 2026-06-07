//    Copyright [2026] [Purnendu Kumar]
//    Apache-2.0 License

//! Integration tests for HTML rendering.

use std::collections::HashMap;
use markplus_core::parse_document;
use markplus_render::RenderEngine;

const TEMPLATE: &str = include_str!("../templates/default/article.html.tera");

fn engine() -> RenderEngine {
    RenderEngine::builder()
        .build_with_templates(HashMap::from([
            ("default/article.html.tera".into(), TEMPLATE.into()),
        ]))
        .expect("engine build failed")
}

fn render(md: &str) -> String {
    let asset = parse_document(md).expect("parse failed");
    engine().render_html(&asset, "default/article.html.tera").expect("render failed")
}

// ---------------------------------------------------------------------------
// Heading tests
// ---------------------------------------------------------------------------

#[test]
fn html_h1_heading() {
    let out = render("# My Title\n");
    assert!(out.contains("<h1"), "expected h1 tag: {}", out);
    assert!(out.contains("My Title"), "expected heading text: {}", out);
}

#[test]
fn html_h2_heading() {
    let out = render("## Sub-section\n");
    assert!(out.contains("<h2"), "{}", out);
    assert!(out.contains("Sub-section"), "{}", out);
}

// ---------------------------------------------------------------------------
// Paragraph tests
// ---------------------------------------------------------------------------

#[test]
fn html_paragraph() {
    let out = render("Hello world.\n");
    assert!(out.contains("<p>"), "{}", out);
    assert!(out.contains("Hello world"), "{}", out);
}

#[test]
fn html_strong_and_em() {
    let out = render("**bold** and _italic_\n");
    assert!(out.contains("<strong>"), "{}", out);
    assert!(out.contains("<em>"), "{}", out);
}

// ---------------------------------------------------------------------------
// Fenced code block
// ---------------------------------------------------------------------------

#[test]
fn html_fenced_block() {
    let out = render("```rust\nfn main() {}\n```\n");
    assert!(out.contains("language-rust"), "{}", out);
    assert!(out.contains("fn main()"), "{}", out);
}

#[test]
fn html_fenced_block_escapes_html_chars() {
    let out = render("```html\n<div>hello</div>\n```\n");
    assert!(out.contains("&lt;div&gt;"), "{}", out);
}

// ---------------------------------------------------------------------------
// Table
// ---------------------------------------------------------------------------

#[test]
fn html_table_has_headers() {
    let out = render("| A | B |\n| - | - |\n| 1 | 2 |\n");
    assert!(out.contains("<table>"), "{}", out);
    assert!(out.contains("<th>"), "{}", out);
    assert!(out.contains("<td>"), "{}", out);
}

// ---------------------------------------------------------------------------
// Math
// ---------------------------------------------------------------------------

#[test]
fn html_math_block() {
    let out = render("$$\nE = mc^2\n$$\n");
    assert!(out.contains("math-block"), "{}", out);
    assert!(out.contains("mc^2"), "{}", out);
}

#[test]
fn html_math_inline() {
    let out = render("Inline $x^2$.\n");
    assert!(out.contains("math-inline"), "{}", out);
}

// ---------------------------------------------------------------------------
// Blockquote
// ---------------------------------------------------------------------------

#[test]
fn html_blockquote() {
    let out = render("> A quote.\n");
    assert!(out.contains("<blockquote"), "{}", out);
    assert!(out.contains("A quote"), "{}", out);
}

#[test]
fn html_gfm_note_alert() {
    let out = render("> [!NOTE]\n> This is a note.\n");
    assert!(out.contains("class=\"note\"") || out.contains("<blockquote"), "{}", out);
}

// ---------------------------------------------------------------------------
// List
// ---------------------------------------------------------------------------

#[test]
fn html_unordered_list() {
    let out = render("- alpha\n- beta\n- gamma\n");
    assert!(out.contains("<ul>"), "{}", out);
    assert!(out.contains("<li>"), "{}", out);
}

#[test]
fn html_ordered_list() {
    let out = render("1. first\n2. second\n");
    assert!(out.contains("<ol"), "{}", out);
}

// ---------------------------------------------------------------------------
// Meta / frontmatter
// ---------------------------------------------------------------------------

#[test]
fn html_meta_title_in_head() {
    let md = "---\ntitle: Test Doc\n---\n# Hello\n";
    let out = render(md);
    assert!(out.contains("Test Doc"), "{}", out);
}

#[test]
fn html_meta_tags_rendered() {
    let md = "---\ntitle: T\ntags:\n  - rust\n  - wasm\n---\n# T\n";
    let out = render(md);
    assert!(out.contains("rust"), "{}", out);
    assert!(out.contains("wasm"), "{}", out);
}

// ---------------------------------------------------------------------------
// TOC
// ---------------------------------------------------------------------------

#[test]
fn html_toc_appears_for_multiple_headings() {
    let out = render("# A\n\n## B\n\n## C\n");
    assert!(out.contains("class=\"toc\""), "{}", out);
}

// ---------------------------------------------------------------------------
// Horizontal rule
// ---------------------------------------------------------------------------

#[test]
fn html_hr() {
    let out = render("---\n\ntext\n\n---\n");
    assert!(out.contains("<hr>"), "{}", out);
}
