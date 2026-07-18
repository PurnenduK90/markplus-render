//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Converts a [`markplus_core::json::SiteAsset`] into a template-friendly
//! [`serde_json::Value`] that MiniJinja templates can consume directly.
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
// ---------------------------------------------------------------------------
// TOC extraction
// ---------------------------------------------------------------------------

pub fn build_toc(ast: &[Value]) -> Vec<Value> {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_handles_special_chars() {
        assert_eq!(slugify_text("Hello, World! (2026)"), "hello-world-2026");
    }
}
