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

pub struct BlockquoteRenderer;

impl RenderBlock for BlockquoteRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let kind = node.get("kind").and_then(|v| v.as_str()).unwrap_or("");
        let children = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        if !kind.is_empty() {
            writeln!(
                out,
                "<blockquote class=\"{}\">",
                html_escape::encode_text(kind)
            )
            .unwrap();
        } else {
            out.push_str("<blockquote>\n");
        }

        for child in children {
            crate::blocks::render_html_node(child, out);
        }

        out.push_str("</blockquote>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        out.push_str("#quote(block: true)[\n");
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                crate::blocks::render_typst_node(child, out);
            }
        }
        out.push_str("]\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let mut inner = String::new();
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                crate::blocks::render_markdown_node(child, &mut inner);
            }
        }
        for line in inner.lines() {
            out.push_str("> ");
            out.push_str(line);
            out.push('\n');
        }
        out.push('\n');
    }
}
