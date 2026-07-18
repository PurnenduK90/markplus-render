use markplus_render::plugin::{MarkplusPlugin, PluginType};
use serde_json::Value;

pub struct TooltipPlugin;

impl MarkplusPlugin for TooltipPlugin {
    fn name(&self) -> &'static str {
        "tooltip"
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::Widget
    }

    fn render_html(&self, node: &Value, out: &mut String) {
        let text = node.get("text").and_then(|v| v.as_str()).unwrap_or("");
        // A widget could store its tooltip string in `data` or `tooltip` field.
        // We'll check both.
        let tip = node
            .get("data")
            .or_else(|| node.get("tooltip"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        out.push_str("<span class=\"widget tooltip\" title=\"");
        out.push_str(tip);
        out.push_str("\">");
        out.push_str(text);
        out.push_str("</span>");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(&self, node: &Value, out: &mut String) {
        let text = node.get("text").and_then(|v| v.as_str()).unwrap_or("");
        // In Typst, tooltips aren't natively supported in PDFs (PDF tooltips are very limited),
        // but we can wrap it in a stylized box or footnote.
        out.push_str(&format!(
            "#box(fill: luma(240), inset: 2pt, radius: 2pt)[{}]",
            text
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(&self, node: &Value, out: &mut String) {
        let text = node.get("text").and_then(|v| v.as_str()).unwrap_or("");
        let tip = node
            .get("data")
            .or_else(|| node.get("tooltip"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Emitting standard markdown hack for tooltip using link title
        out.push_str(&format!("[{}](# \"{}\")", text, tip));
    }
}
