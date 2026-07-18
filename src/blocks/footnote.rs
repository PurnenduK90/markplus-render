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

pub struct FootnoteDefRenderer;

impl RenderBlock for FootnoteDefRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let label = node.get("label").and_then(|v| v.as_str()).unwrap_or("");
        let children = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        writeln!(out, "<div class=\"footnote-def\" id=\"fn-{}\">", label).unwrap();
        writeln!(out, "  <sup>{}</sup>", label).unwrap();

        for child in children {
            crate::blocks::render_html_node(child, out);
        }

        out.push_str("</div>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        let label = node.get("label").and_then(|v| v.as_str()).unwrap_or("");
        let children = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        write!(out, "#footnote[<fn-{}> ", label).unwrap();
        for child in children {
            crate::blocks::render_typst_node(child, out);
        }
        out.push_str("]\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let label = node.get("label").and_then(|v| v.as_str()).unwrap_or("");
        write!(out, "[^{}]: ", label).unwrap();

        let mut inner = String::new();
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                crate::blocks::render_markdown_node(child, &mut inner);
            }
        }

        let mut first = true;
        for line in inner.trim_end().lines() {
            if !first {
                out.push_str("    ");
            }
            out.push_str(line);
            out.push('\n');
            first = false;
        }
        if first {
            out.push('\n');
        }
        out.push('\n');
    }
}
