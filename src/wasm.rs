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
//!   "default/article.html.jinja": htmlTemplateStr,
//!   "default/article.typ.jinja":  typstTemplateStr,
//! };
//! const renderer = new MarkplusRenderWasm(templates);
//!
//! const html = renderer.render_html(assetJson, "default/article.html.jinja");
//! const typ  = renderer.render_typst(assetJson, "default/article.typ.jinja");
//! const pdf  = renderer.compile_pdf_from_asset(assetJson, "default/article.typ.jinja");
//! ```

use serde_json::Value;
use wasm_bindgen::prelude::*;

use markplus_core::json::SiteAsset;

/// Main wasm entry point for `markplus_render`.
#[wasm_bindgen]
pub struct MarkplusRenderWasm;

#[wasm_bindgen]
impl MarkplusRenderWasm {
    /// Construct the WASM renderer.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<MarkplusRenderWasm, JsValue> {
        #[cfg(feature = "wasm")]
        console_error_panic_hook::set_once();
        Ok(MarkplusRenderWasm)
    }

    /// Render HTML from a [`SiteAsset`] JSON object.
    ///
    /// This converts the JSON AST `body` directly into raw HTML block strings.
    pub fn render_html(&self, ast_json: JsValue) -> Result<String, JsValue> {
        let ast: Vec<Value> = serde_wasm_bindgen::from_value(ast_json)?;
        let mut out = String::new();
        for node in &ast {
            crate::blocks::render_html_node(node, &mut out);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// Removed asset_from_js as we only pass the AST array now
