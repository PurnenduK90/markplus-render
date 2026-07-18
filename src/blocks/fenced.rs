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

pub struct FencedRenderer;

fn safe_html(text: &str) -> String {
    html_escape::encode_text(text).into_owned()
}

impl RenderBlock for FencedRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(plugin) = crate::plugin::get_plugin(crate::plugin::PluginType::Fenced, name) {
            plugin.render_html(node, out);
            return;
        }

        let raw = node.get("raw").and_then(|v| v.as_str()).unwrap_or("");
        writeln!(
            out,
            "<pre><code class=\"language-{}\">{}</code></pre>",
            safe_html(name),
            safe_html(raw)
        )
        .unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(plugin) = crate::plugin::get_plugin(crate::plugin::PluginType::Fenced, name) {
            plugin.render_typst(node, out);
            return;
        }

        let raw = node.get("raw").and_then(|v| v.as_str()).unwrap_or("");
        write!(out, "```{}\n{}\n```\n\n", name, raw).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(plugin) = crate::plugin::get_plugin(crate::plugin::PluginType::Fenced, name) {
            plugin.render_markdown(node, out);
            return;
        }

        let raw = node.get("raw").and_then(|v| v.as_str()).unwrap_or("");
        write!(out, "```{}\n{}\n```\n\n", name, raw).unwrap();
    }
}
