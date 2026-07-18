use markplus_render::plugin::{MarkplusPlugin, PluginType};
use serde_json::Value;

pub struct ColumnsPlugin;

impl MarkplusPlugin for ColumnsPlugin {
    fn name(&self) -> &'static str {
        "columns"
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::Directive
    }

    fn render_html(&self, node: &Value, out: &mut String) {
        out.push_str("<div class=\"directive columns\" style=\"display: flex; gap: 1rem;\">\n");
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                out.push_str("<div class=\"col\" style=\"flex: 1;\">\n");
                markplus_render::blocks::render_html_node(child, out);
                out.push_str("</div>\n");
            }
        }
        out.push_str("</div>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(&self, node: &Value, out: &mut String) {
        let count = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|v| v.len())
            .unwrap_or(1);
        out.push_str(&format!("#columns({})[\n", std::cmp::max(1, count)));

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                markplus_render::blocks::render_typst_node(child, out);
                out.push_str("#colbreak()\n");
            }
        }
        out.push_str("]\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(&self, node: &Value, out: &mut String) {
        out.push_str(":::columns \n");
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                out.push_str(":::col \n");
                markplus_render::blocks::render_markdown_node(child, out);
                out.push_str(":::\n");
            }
        }
        out.push_str(":::\n\n");
    }
}
