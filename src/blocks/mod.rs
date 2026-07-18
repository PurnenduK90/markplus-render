//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use serde_json::Value;

pub mod blockquote;
pub mod directive;
pub mod fenced;
pub mod footnote;
pub mod heading;
pub mod hr;
pub mod inline;
pub mod list;
pub mod math;
pub mod paragraph;
pub mod table;

pub trait RenderBlock {
    fn render_html(node: &Value, out: &mut String);

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String);

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String);
}

/// Dispatches an AST node to the appropriate block renderer.
pub fn render_html_node(node: &Value, out: &mut String) {
    let t = node.get("t").and_then(|v| v.as_str()).unwrap_or("");
    match t {
        "heading" => heading::HeadingRenderer::render_html(node, out),
        "paragraph" => paragraph::ParagraphRenderer::render_html(node, out),
        "fenced" => fenced::FencedRenderer::render_html(node, out),
        "math_block" => math::MathBlockRenderer::render_html(node, out),
        "blockquote" => blockquote::BlockquoteRenderer::render_html(node, out),
        "list" => list::ListRenderer::render_html(node, out),
        "table" => table::TableRenderer::render_html(node, out),
        "hr" => hr::HrRenderer::render_html(node, out),
        "footnote_def" => footnote::FootnoteDefRenderer::render_html(node, out),
        "directive" => directive::DirectiveRenderer::render_html(node, out),
        _ => {
            // Ignore unknown block nodes or handle generic
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_typst_node(node: &Value, out: &mut String) {
    let t = node.get("t").and_then(|v| v.as_str()).unwrap_or("");
    match t {
        "heading" => heading::HeadingRenderer::render_typst(node, out),
        "paragraph" => paragraph::ParagraphRenderer::render_typst(node, out),
        "fenced" => fenced::FencedRenderer::render_typst(node, out),
        "math_block" => math::MathBlockRenderer::render_typst(node, out),
        "blockquote" => blockquote::BlockquoteRenderer::render_typst(node, out),
        "list" => list::ListRenderer::render_typst(node, out),
        "table" => table::TableRenderer::render_typst(node, out),
        "hr" => hr::HrRenderer::render_typst(node, out),
        "footnote_def" => footnote::FootnoteDefRenderer::render_typst(node, out),
        "directive" => directive::DirectiveRenderer::render_typst(node, out),
        _ => {
            // Ignore unknown block nodes or handle generic
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_markdown_node(node: &Value, out: &mut String) {
    let t = node.get("t").and_then(|v| v.as_str()).unwrap_or("");
    match t {
        "heading" => heading::HeadingRenderer::render_markdown(node, out),
        "paragraph" => paragraph::ParagraphRenderer::render_markdown(node, out),
        "fenced" => fenced::FencedRenderer::render_markdown(node, out),
        "math_block" => math::MathBlockRenderer::render_markdown(node, out),
        "blockquote" => blockquote::BlockquoteRenderer::render_markdown(node, out),
        "list" => list::ListRenderer::render_markdown(node, out),
        "table" => table::TableRenderer::render_markdown(node, out),
        "hr" => hr::HrRenderer::render_markdown(node, out),
        "footnote_def" => footnote::FootnoteDefRenderer::render_markdown(node, out),
        "directive" => directive::DirectiveRenderer::render_markdown(node, out),
        _ => {
            // Ignore unknown block nodes or handle generic
        }
    }
}
