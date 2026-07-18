//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use crate::blocks::RenderBlock;
use serde_json::Value;
use std::fmt::Write;

pub struct ListRenderer;

impl RenderBlock for ListRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let ordered = node
            .get("ordered")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let items = node
            .get("items")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        if ordered {
            let start = node.get("start").and_then(|v| v.as_u64()).unwrap_or(1);
            writeln!(out, "<ol start=\"{}\">", start).unwrap();
        } else {
            out.push_str("<ul>\n");
        }

        for item in items {
            out.push_str("<li>");
            let children = item
                .get("children")
                .and_then(|v| v.as_array())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            for child in children {
                let t = child.get("t").and_then(|v| v.as_str()).unwrap_or("");
                if matches!(t, "heading" | "paragraph" | "fenced" | "math_block" | "blockquote" | "list" | "table" | "hr" | "footnote_def" | "directive") {
                    crate::blocks::render_html_node(child, out);
                } else {
                    crate::blocks::inline::render_html_inline(std::slice::from_ref(child), out);
                }
            }
            out.push_str("</li>\n");
        }

        if ordered {
            out.push_str("</ol>\n");
        } else {
            out.push_str("</ul>\n");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        let ordered = node
            .get("ordered")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let items = node
            .get("items")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        // Typst uses + for enum, - for list
        let prefix = if ordered { "+ " } else { "- " };

        for item in items {
            out.push_str(prefix);
            let children = item
                .get("children")
                .and_then(|v| v.as_array())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            for child in children {
                let t = child.get("t").and_then(|v| v.as_str()).unwrap_or("");
                if matches!(t, "heading" | "paragraph" | "fenced" | "math_block" | "blockquote" | "list" | "table" | "hr" | "footnote_def" | "directive") {
                    crate::blocks::render_typst_node(child, out);
                } else {
                    crate::blocks::inline::render_typst_inline(std::slice::from_ref(child), out);
                }
            }
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
        out.push('\n');
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let ordered = node
            .get("ordered")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let items = node
            .get("items")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        for (i, item) in items.iter().enumerate() {
            let prefix = if ordered {
                format!("{}. ", i + 1)
            } else {
                "- ".to_string()
            };
            out.push_str(&prefix);

            let mut inner = String::new();
            if let Some(children) = item.get("children").and_then(|v| v.as_array()) {
                for child in children {
                    let t = child.get("t").and_then(|v| v.as_str()).unwrap_or("");
                    if matches!(t, "heading" | "paragraph" | "fenced" | "math_block" | "blockquote" | "list" | "table" | "hr" | "footnote_def" | "directive") {
                        crate::blocks::render_markdown_node(child, &mut inner);
                    } else {
                        crate::blocks::inline::render_markdown_inline(std::slice::from_ref(child), &mut inner);
                    }
                }
            }

            let mut first = true;
            for line in inner.trim_end().lines() {
                if !first {
                    out.push_str(&" ".repeat(prefix.len()));
                }
                out.push_str(line);
                out.push('\n');
                first = false;
            }
            if first {
                out.push('\n');
            }
        }
        out.push('\n');
    }
}
