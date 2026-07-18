//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use serde_json::Value;

use crate::blocks::RenderBlock;

pub struct DirectiveRenderer;

impl RenderBlock for DirectiveRenderer {
    fn render_html(node: &Value, out: &mut String) {
        let name = node
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        if let Some(plugin) = crate::plugin::get_plugin(crate::plugin::PluginType::Directive, name)
        {
            plugin.render_html(node, out);
            return;
        }

        out.push_str(&format!("<div class=\"directive {}\">\n", name));

        // Render optional title/header
        out.push_str(&format!(
            "<div class=\"directive-header\">{}</div>\n",
            name.to_uppercase()
        ));
        out.push_str("<div class=\"directive-body\">\n");

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                crate::blocks::render_html_node(child, out);
            }
        }

        out.push_str("</div>\n</div>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        let name = node
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        if let Some(plugin) = crate::plugin::get_plugin(crate::plugin::PluginType::Directive, name)
        {
            plugin.render_typst(node, out);
            return;
        }

        // Render a stylized block for typst
        out.push_str("#block(fill: luma(245), width: 100%, inset: 8pt, radius: 4pt)[\n");
        out.push_str(&format!("  *{}*\n  #v(0.5em)\n", name.to_uppercase()));

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                crate::blocks::render_typst_node(child, out);
            }
        }

        out.push_str("]\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let name = node
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        if let Some(plugin) = crate::plugin::get_plugin(crate::plugin::PluginType::Directive, name)
        {
            plugin.render_markdown(node, out);
            return;
        }

        out.push_str(&format!(":::{} \n", name));

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                crate::blocks::render_markdown_node(child, out);
            }
        }

        out.push_str(":::\n\n");
    }
}
