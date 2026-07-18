use markplus_render::plugin::{MarkplusPlugin, PluginType};
use serde_json::Value;
use std::fmt::Write;

pub struct MermaidPlugin;

impl MarkplusPlugin for MermaidPlugin {
    fn name(&self) -> &'static str {
        "mermaid"
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::Fenced
    }

    fn render_html(&self, node: &Value, out: &mut String) {
        let raw = node.get("raw").and_then(|v| v.as_str()).unwrap_or("");

        // If pre-processed SVG is available
        if let Some(svg_html) = node.get("svg_html").and_then(|v| v.as_str()) {
            out.push_str(
                "<div class=\"mermaid-diagram\" style=\"text-align: center; margin: 1.5em 0;\">\n",
            );
            out.push_str(svg_html);
            out.push_str("\n</div>\n");
        } else {
            // Wasm/Client-side fallback: emit generic HTML hook for mermaid.js
            // html-escape could be used if we brought it in, but for now we just wrap.
            out.push_str("<pre class=\"mermaid\">");
            out.push_str(raw);
            out.push_str("</pre>\n");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(&self, node: &Value, out: &mut String) {
        let raw = node.get("raw").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(svg_typst) = node.get("svg_typst").and_then(|v| v.as_str()) {
            write!(out, "#align(center)[#image.decode(\"{}\")]\n\n", svg_typst).unwrap();
        } else {
            write!(out, "```mermaid\n{}\n```\n\n", raw).unwrap();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(&self, node: &Value, out: &mut String) {
        let raw = node.get("raw").and_then(|v| v.as_str()).unwrap_or("");
        write!(out, "```mermaid\n{}\n```\n\n", raw).unwrap();
    }
}
