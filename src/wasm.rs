//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! `wasm-bindgen` exports for `markplus_render`.
//!
//! Enabled only when compiled with `--features wasm`.
//!
//! ## JavaScript usage
//!
//! ```js
//! import init, { MarkplusRenderWasm } from "./markplus_render.js";
//!
//! await init();
//!
//! const templates = {
//!   "default/article.html.tera": htmlTemplateStr,
//!   "default/article.typ.tera":  typstTemplateStr,
//! };
//! const renderer = new MarkplusRenderWasm(templates);
//!
//! const html = renderer.render_html(assetJson, "default/article.html.tera");
//! const typ  = renderer.render_typst(assetJson, "default/article.typ.tera");
//! const pdf  = renderer.compile_pdf_from_asset(assetJson, "default/article.typ.tera");
//! ```

use wasm_bindgen::prelude::*;
use serde_json::Value;

use crate::engine::RenderEngine;
use markplus_core::json::SiteAsset;

/// Main wasm entry point for `markplus_render`.
#[wasm_bindgen]
pub struct MarkplusRenderWasm {
    engine: RenderEngine,
}

#[wasm_bindgen]
impl MarkplusRenderWasm {
    /// Construct from a JS object mapping template names to template source strings.
    ///
    /// ```js
    /// const r = new MarkplusRenderWasm({ "default/article.html.tera": "<html>..." });
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(templates_js: JsValue) -> Result<MarkplusRenderWasm, JsValue> {
        console_error_panic_hook::set_once();
        let templates: std::collections::HashMap<String, String> =
            serde_wasm_bindgen::from_value(templates_js)?;
        let engine = RenderEngine::builder()
            .build_with_templates(templates)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(MarkplusRenderWasm { engine })
    }

    /// Render HTML from a [`SiteAsset`] JSON object.
    ///
    /// `asset_json` — the full JSON object produced by `markplus_core::parse_document`
    /// (or the JS equivalent from `parse_document_to_json`).
    pub fn render_html(&self, asset_json: JsValue, template_name: &str) -> Result<String, JsValue> {
        let asset = asset_from_js(asset_json)?;
        self.engine
            .render_html(&asset, template_name)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Render a Typst source string from a [`SiteAsset`] JSON object.
    pub fn render_typst(&self, asset_json: JsValue, template_name: &str) -> Result<String, JsValue> {
        let asset = asset_from_js(asset_json)?;
        self.engine
            .render_typst_string(&asset, template_name)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Compile a Typst source string directly into PDF bytes.
    pub fn compile_pdf(&self, typst_src: &str) -> Result<Vec<u8>, JsValue> {
        self.engine
            .compile_pdf(typst_src)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Render Typst source from a [`SiteAsset`] and immediately compile to PDF bytes.
    ///
    /// Combines `render_typst` + `compile_pdf` in one call.
    pub fn compile_pdf_from_asset(
        &self,
        asset_json: JsValue,
        template_name: &str,
    ) -> Result<Vec<u8>, JsValue> {
        let asset = asset_from_js(asset_json)?;
        let typ_src = self.engine
            .render_typst_string(&asset, template_name)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.engine
            .compile_pdf(&typ_src)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Lightweight HTML rendering without Tera — converts AST nodes to HTML inline.
    ///
    /// Useful when no template is loaded or for fast previews. Does not require
    /// a template to be registered.
    pub fn render_html_simple(&self, asset_json: JsValue) -> Result<String, JsValue> {
        let asset = asset_from_js(asset_json)?;
        Ok(render_ast_simple(&asset))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn asset_from_js(js: JsValue) -> Result<SiteAsset, JsValue> {
    let val: Value = serde_wasm_bindgen::from_value(js)?;
    serde_json::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Simple non-Tera AST → HTML renderer for lightweight wasm previews.
fn render_ast_simple(asset: &SiteAsset) -> String {
    let mut out = String::new();
    for node in &asset.ast {
        render_node_simple(node, &mut out);
    }
    out
}

fn render_node_simple(node: &Value, out: &mut String) {
    match node["t"].as_str().unwrap_or("") {
        "heading" => {
            let level = node["level"].as_u64().unwrap_or(1).min(6);
            let text = collect_inline_text(node.get("children").and_then(Value::as_array).unwrap_or(&[]));
            out.push_str(&format!("<h{level}>{text}</h{level}>"));
        }
        "paragraph" => {
            let text = collect_inline_text(node.get("children").and_then(Value::as_array).unwrap_or(&[]));
            out.push_str(&format!("<p>{text}</p>"));
        }
        "fenced" => {
            let name = node["name"].as_str().unwrap_or("");
            let raw = escape_html(node["raw"].as_str().unwrap_or(""));
            out.push_str(&format!("<pre><code class=\"language-{name}\">{raw}</code></pre>"));
        }
        "math_block" => {
            let src = escape_html(node["src"].as_str().unwrap_or(""));
            out.push_str(&format!("<p class=\"math-block\">{src}</p>"));
        }
        "blockquote" => {
            out.push_str("<blockquote>");
            if let Some(ch) = node.get("children").and_then(Value::as_array) {
                for child in ch { render_node_simple(child, out); }
            }
            out.push_str("</blockquote>");
        }
        "list" => {
            let ordered = node["ordered"].as_bool().unwrap_or(false);
            let tag = if ordered { "ol" } else { "ul" };
            out.push_str(&format!("<{tag}>"));
            if let Some(items) = node.get("items").and_then(Value::as_array) {
                for item in items {
                    out.push_str("<li>");
                    if let Some(ch) = item.get("children").and_then(Value::as_array) {
                        for c in ch { render_node_simple(c, out); }
                    }
                    out.push_str("</li>");
                }
            }
            out.push_str(&format!("</{tag}>"));
        }
        "table" => {
            out.push_str("<table>");
            if let Some(head) = node.get("headers").and_then(Value::as_array) {
                out.push_str("<thead><tr>");
                for cell in head {
                    let text = collect_inline_text(cell.get("children").and_then(Value::as_array).unwrap_or(&[]));
                    out.push_str(&format!("<th>{text}</th>"));
                }
                out.push_str("</tr></thead>");
            }
            if let Some(rows) = node.get("rows").and_then(Value::as_array) {
                out.push_str("<tbody>");
                for row in rows {
                    out.push_str("<tr>");
                    if let Some(cells) = row.as_array() {
                        for cell in cells {
                            let text = collect_inline_text(cell.get("children").and_then(Value::as_array).unwrap_or(&[]));
                            out.push_str(&format!("<td>{text}</td>"));
                        }
                    }
                    out.push_str("</tr>");
                }
                out.push_str("</tbody>");
            }
            out.push_str("</table>");
        }
        "hr" => out.push_str("<hr>"),
        _ => {}
    }
}

fn collect_inline_text(children: &[Value]) -> String {
    let mut out = String::new();
    for child in children {
        match child["t"].as_str().unwrap_or("") {
            "text"        => out.push_str(&escape_html(child["text"].as_str().unwrap_or(""))),
            "strong"      => {
                let inner = collect_inline_text(child.get("children").and_then(Value::as_array).unwrap_or(&[]));
                out.push_str(&format!("<strong>{inner}</strong>"));
            }
            "em"          => {
                let inner = collect_inline_text(child.get("children").and_then(Value::as_array).unwrap_or(&[]));
                out.push_str(&format!("<em>{inner}</em>"));
            }
            "code"        => out.push_str(&format!("<code>{}</code>", escape_html(child["text"].as_str().unwrap_or("")))),
            "link"        => {
                let href = child["href"].as_str().unwrap_or("#");
                let inner = collect_inline_text(child.get("children").and_then(Value::as_array).unwrap_or(&[]));
                out.push_str(&format!("<a href=\"{href}\">{inner}</a>"));
            }
            "image"       => {
                let src = child["src"].as_str().unwrap_or("");
                let alt = child["alt"].as_str().unwrap_or("");
                out.push_str(&format!("<img src=\"{src}\" alt=\"{alt}\">"));
            }
            "math_inline" => out.push_str(&format!("<span class=\"math-inline\">{}</span>", escape_html(child["src"].as_str().unwrap_or("")))),
            "hard_break"  => out.push_str("<br>"),
            "soft_break"  => out.push(' '),
            "del"         => {
                let inner = collect_inline_text(child.get("children").and_then(Value::as_array).unwrap_or(&[]));
                out.push_str(&format!("<del>{inner}</del>"));
            }
            _ => {
                if let Some(t) = child["text"].as_str() { out.push_str(&escape_html(t)); }
            }
        }
    }
    out
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
