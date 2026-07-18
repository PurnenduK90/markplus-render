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

pub struct MathBlockRenderer;

fn safe_html(text: &str) -> String {
    html_escape::encode_text(text).into_owned()
}

impl RenderBlock for MathBlockRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let src = node.get("src").and_then(|v| v.as_str()).unwrap_or("");
        writeln!(
            out,
            "<p class=\"math-block\" data-math=\"{src}\">$${src}$$</p>",
            src = safe_html(src)
        )
        .unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        let src = node.get("src").and_then(|v| v.as_str()).unwrap_or("");
        write!(out, "$ {} $\n\n", src).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let src = node.get("src").and_then(|v| v.as_str()).unwrap_or("");
        write!(out, "$$\n{}\n$$\n\n", src).unwrap();
    }
}
