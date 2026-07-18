//    Copyright [2026] [Purnendu Kumar]
//    Apache-2.0 License

//! Integration tests for Typst source rendering.

use markplus_core::parse_document;
use markplus_render::RenderEngine;
use std::collections::HashMap;

const TEMPLATE: &str = include_str!("../templates/default/article.typ.jinja");

fn engine() -> RenderEngine {
    RenderEngine::builder()
        .build_with_templates(HashMap::from([(
            "default/article.typ.jinja".into(),
            TEMPLATE.into(),
        )]))
        .expect("engine build failed")
}

fn render_typ(md: &str) -> String {
    let asset = parse_document(md).expect("parse failed");
    engine()
        .render_typst_string(&asset, "default/article.typ.jinja")
        .expect("render failed")
}

// ---------------------------------------------------------------------------
// Heading
// ---------------------------------------------------------------------------

#[test]
fn typst_h1_uses_equals_sign() {
    let out = render_typ("# My Title\n");
    assert!(
        out.contains("= My Title"),
        "expected '= My Title' in:\n{}",
        out
    );
}

#[test]
fn typst_h2_uses_double_equals() {
    let out = render_typ("## Sub\n");
    assert!(out.contains("== Sub"), "{}", out);
}

#[test]
fn typst_h3_uses_triple_equals() {
    let out = render_typ("### Deep\n");
    assert!(out.contains("=== Deep"), "{}", out);
}

// ---------------------------------------------------------------------------
// Paragraph
// ---------------------------------------------------------------------------

#[test]
fn typst_paragraph_text_present() {
    let out = render_typ("Hello, Typst world.\n");
    assert!(out.contains("Hello, Typst world"), "{}", out);
}

#[test]
fn typst_strong_uses_asterisks() {
    let out = render_typ("**bold text**\n");
    assert!(out.contains("*bold text*"), "{}", out);
}

#[test]
fn typst_em_uses_underscores() {
    let out = render_typ("_italic text_\n");
    assert!(out.contains("_italic text_"), "{}", out);
}

// ---------------------------------------------------------------------------
// Fenced code block
// ---------------------------------------------------------------------------

#[test]
fn typst_fenced_block_backtick_fence() {
    let out = render_typ("```rust\nfn main() {}\n```\n");
    assert!(out.contains("```rust"), "{}", out);
    assert!(out.contains("fn main()"), "{}", out);
}

// ---------------------------------------------------------------------------
// Math
// ---------------------------------------------------------------------------

#[test]
fn typst_math_block_dollar_syntax() {
    let out = render_typ("$$\nE = mc^2\n$$\n");
    assert!(out.contains("mc^2"), "{}", out);
}

#[test]
fn typst_math_inline_dollar() {
    let out = render_typ("Inline $x^2$.\n");
    assert!(out.contains("$x^2$"), "{}", out);
}

// ---------------------------------------------------------------------------
// Table
// ---------------------------------------------------------------------------

#[test]
fn typst_table_contains_table_call() {
    let out = render_typ("| A | B |\n| - | - |\n| 1 | 2 |\n");
    assert!(out.contains("table("), "{}", out);
}

// ---------------------------------------------------------------------------
// List
// ---------------------------------------------------------------------------

#[test]
fn typst_unordered_list_uses_dash() {
    let out = render_typ("- item one\n- item two\n");
    assert!(out.contains("- item one"), "{}", out);
}

// ---------------------------------------------------------------------------
// Horizontal rule
// ---------------------------------------------------------------------------

#[test]
fn typst_hr_uses_line() {
    let out = render_typ("text\n\n---\n\nmore\n");
    assert!(out.contains("#line("), "{}", out);
}

// ---------------------------------------------------------------------------
// Meta / frontmatter
// ---------------------------------------------------------------------------

#[test]
fn typst_meta_title_in_document_set() {
    let md = "---\ntitle: My Article\n---\n# My Article\n";
    let out = render_typ(md);
    assert!(out.contains("My Article"), "{}", out);
}

#[test]
fn typst_meta_tags_present() {
    let md = "---\ntitle: T\ntags:\n  - rust\n---\n# T\n";
    let out = render_typ(md);
    assert!(out.contains("rust"), "{}", out);
}

// ---------------------------------------------------------------------------
// Document structure
// ---------------------------------------------------------------------------

#[test]
fn typst_output_starts_with_set_document() {
    let out = render_typ("# Hello\n");
    assert!(out.contains("#set document("), "{}", out);
}

#[test]
fn typst_output_sets_page() {
    let out = render_typ("# Hello\n");
    assert!(out.contains("#set page("), "{}", out);
}
