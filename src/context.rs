//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Converts a [`markplus_core::json::SiteAsset`] into a template-friendly
//! [`serde_json::Value`] that Tera templates can consume directly.
//!
//! ## Context shape
//!
//! ```json
//! {
//!   "meta":  { "title": "...", "tags": [...], ... },
//!   "slug":  "my-document-title",
//!   "toc":   [{ "level": 1, "text": "Intro", "slug": "intro" }],
//!   "body":  [ <node>, ... ]
//! }
//! ```
//!
//! Each `body` node preserves the original AST `"t"` field plus any
//! renderer-friendly additions (e.g. `"html"` for inline HTML snippets).

use markplus_core::json::SiteAsset;
use serde_json::{Value, json};

/// Convert a [`SiteAsset`] into a Tera-ready context value.
///
/// The returned value has four top-level keys:
/// - `meta`  — frontmatter object (or `null`)
/// - `slug`  — URL-safe slug derived from `meta.title` or first heading
/// - `toc`   — array of `{ level, text, slug }` entries from headings
/// - `body`  — the full AST node array (unchanged from the asset)
fn process_mermaid_nodes(nodes: &mut [Value]) {
    for node in nodes.iter_mut() {
        if node["t"] == "fenced" && node["name"] == "mermaid" {
            if let Some(raw) = node.get("raw").and_then(Value::as_str) {
                match mermaid_rs_renderer::render(raw) {
                    Ok(svg) => {
                        let obj = node.as_object_mut().unwrap();
                        obj.insert("svg_html".to_string(), json!(svg));
                        let typst_svg = svg.replace('"', "\\\"").replace('\n', "");
                        obj.insert("svg_typst".to_string(), json!(typst_svg));
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to render mermaid diagram: {}", e);
                    }
                }
            }
        }
        
        // Recurse into children
        if let Some(children) = node.get_mut("children").and_then(Value::as_array_mut) {
            process_mermaid_nodes(children);
        }
        
        // Recurse into items (e.g. lists)
        if let Some(items) = node.get_mut("items").and_then(Value::as_array_mut) {
            for item in items.iter_mut() {
                if let Some(ch) = item.get_mut("children").and_then(Value::as_array_mut) {
                    process_mermaid_nodes(ch);
                }
            }
        }
        
        // Recurse into tabs
        if let Some(tabs) = node.get_mut("tabs").and_then(Value::as_array_mut) {
            for tab in tabs.iter_mut() {
                if let Some(ast) = tab.get_mut("ast").and_then(Value::as_array_mut) {
                    process_mermaid_nodes(ast);
                }
            }
        }
    }
}

pub fn ast_to_template_context(asset: &SiteAsset) -> Value {
    let toc = build_toc(&asset.ast);
    let slug = derive_slug(asset);
    
    // Process mermaid diagrams recursively
    let mut body = asset.ast.clone();
    process_mermaid_nodes(&mut body);

    json!({
        "meta": asset.meta,
        "slug": slug,
        "toc":  toc,
        "body": body,
    })
}

// ---------------------------------------------------------------------------
// TOC extraction
// ---------------------------------------------------------------------------

fn build_toc(ast: &[Value]) -> Vec<Value> {
    let mut toc = Vec::new();
    collect_headings(ast, &mut toc);
    toc
}

fn collect_headings(nodes: &[Value], out: &mut Vec<Value>) {
    for node in nodes {
        if node["t"] == "heading" {
            let text = collect_text(node.get("children").and_then(Value::as_array).map_or(&[], |v| v));
            let slug = slugify_text(&text);
            let level = node["level"].as_u64().unwrap_or(1);
            out.push(json!({ "level": level, "text": text, "slug": slug }));
        }
        // Recurse into blockquotes / list items that may contain headings
        if let Some(children) = node.get("children").and_then(Value::as_array) {
            collect_headings(children, out);
        }
        if let Some(items) = node.get("items").and_then(Value::as_array) {
            for item in items {
                if let Some(ch) = item.get("children").and_then(Value::as_array) {
                    collect_headings(ch, out);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Slug derivation
// ---------------------------------------------------------------------------

fn derive_slug(asset: &SiteAsset) -> String {
    // Prefer meta.title
    if let Some(title) = asset.meta.as_ref().and_then(|m| m.get("title")).and_then(Value::as_str) {
        return slugify_text(title);
    }
    // Fall back to first heading text
    for node in &asset.ast {
        if node["t"] == "heading" {
            let text = collect_text(node.get("children").and_then(Value::as_array).map_or(&[], |v| v));
            if !text.is_empty() {
                return slugify_text(&text);
            }
        }
    }
    "document".into()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Collect plain text from an inline children array (recursive).
pub fn collect_text(children: &[Value]) -> String {
    let mut out = String::new();
    for child in children {
        if let Some(t) = child.get("text").and_then(Value::as_str) {
            out.push_str(t);
        } else if let Some(src) = child.get("src").and_then(Value::as_str) {
            out.push_str(src);
        } else if let Some(ch) = child.get("children").and_then(Value::as_array) {
            out.push_str(&collect_text(ch));
        }
    }
    out
}

/// Convert a string to a URL-safe lowercase slug.
pub fn slugify_text(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use markplus_core::parse_document;

    const DOC: &str = "---\ntitle: Hello World\ntags:\n  - rust\n---\n# Hello World\n\nSome text.\n\n## Sub-section\n\nMore text.\n";

    #[test]
    fn context_has_meta_slug_toc_body() {
        let asset = parse_document(DOC).unwrap();
        let ctx = ast_to_template_context(&asset);
        assert_eq!(ctx["meta"]["title"], "Hello World");
        assert_eq!(ctx["slug"], "hello-world");
        assert_eq!(ctx["toc"].as_array().unwrap().len(), 2);
        assert!(!ctx["body"].as_array().unwrap().is_empty());
    }

    #[test]
    fn toc_entries_have_level_text_slug() {
        let asset = parse_document(DOC).unwrap();
        let ctx = ast_to_template_context(&asset);
        let first = &ctx["toc"][0];
        assert_eq!(first["level"], 1);
        assert_eq!(first["text"], "Hello World");
        assert_eq!(first["slug"], "hello-world");
    }

    #[test]
    fn slug_falls_back_to_first_heading() {
        let asset = parse_document("# My Heading\n\nBody.\n").unwrap();
        let ctx = ast_to_template_context(&asset);
        assert_eq!(ctx["slug"], "my-heading");
    }

    #[test]
    fn slug_defaults_to_document_when_empty() {
        let asset = parse_document("Just a paragraph.\n").unwrap();
        let ctx = ast_to_template_context(&asset);
        assert_eq!(ctx["slug"], "document");
    }

    #[test]
    fn slugify_handles_special_chars() {
        assert_eq!(slugify_text("Hello, World! (2026)"), "hello-world-2026");
    }
}
