//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use crate::blocks::inline::{collect_text, slugify_text};
use crate::blocks::{RenderBlock, inline};
use serde_json::Value;
use std::fmt::Write;

pub struct HeadingRenderer;

impl RenderBlock for HeadingRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let level = node.get("level").and_then(|v| v.as_u64()).unwrap_or(1);
        let children = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        // Compute slug for ID
        let text = collect_text(children);
        let slug = slugify_text(&text);

        write!(out, "<h{} id=\"{}\">", level, slug).unwrap();
        inline::render_html_inline(children, out);
        writeln!(out, "</h{}>", level).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        let level = node.get("level").and_then(|v| v.as_u64()).unwrap_or(1);
        let prefix = "=".repeat(level as usize);
        out.push_str(&prefix);
        out.push(' ');
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            crate::blocks::inline::render_typst_inline(children, out);
        }
        out.push_str("\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let level = node.get("level").and_then(|v| v.as_u64()).unwrap_or(1);
        let prefix = "#".repeat(level as usize);
        out.push_str(&prefix);
        out.push(' ');
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            crate::blocks::inline::render_markdown_inline(children, out);
        }
        out.push_str("\n\n");
    }
}
