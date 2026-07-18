use markplus_render::plugin::{MarkplusPlugin, PluginType};
use serde_json::Value;

pub struct TabsPlugin;

impl MarkplusPlugin for TabsPlugin {
    fn name(&self) -> &'static str {
        "tabs"
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::Directive
    }

    fn render_html(&self, node: &Value, out: &mut String) {
        out.push_str("<div class=\"directive tabs\">\n");
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                // Usually tabs will have a specific "tab" child directive, but we just recurse.
                markplus_render::blocks::render_html_node(child, out);
            }
        }
        out.push_str("</div>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(&self, node: &Value, out: &mut String) {
        out.push_str("#block(fill: luma(250), width: 100%, inset: 8pt)[\n");
        out.push_str("  *Tabs*\n");
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                markplus_render::blocks::render_typst_node(child, out);
            }
        }
        out.push_str("]\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(&self, node: &Value, out: &mut String) {
        out.push_str(":::tabs \n");
        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                markplus_render::blocks::render_markdown_node(child, out);
            }
        }
        out.push_str(":::\n\n");
    }
}
