//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use serde_json::Value;

use crate::blocks::{RenderBlock, inline};

pub struct ParagraphRenderer;

impl RenderBlock for ParagraphRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let children = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        out.push_str("<p>");
        inline::render_html_inline(children, out);
        out.push_str("</p>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            crate::blocks::inline::render_typst_inline(children, out);
        }
        out.push_str("\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            crate::blocks::inline::render_markdown_inline(children, out);
        }
        out.push_str("\n\n");
    }
}
