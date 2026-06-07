//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Core rendering methods on [`RenderEngine`].
//!
//! - [`RenderEngine::render_html`]          — AST → HTML string via Tera
//! - [`RenderEngine::render_typst_string`]  — AST → Typst source string via Tera
//! - [`RenderEngine::compile_pdf`]          — Typst source → PDF bytes
//!   - Native: `typst-as-lib` + system fonts
//!   - Wasm:   `WasmWorld` + embedded Liberation fonts
//! - [`RenderEngine::render_to_file`]       — convenience wrapper (native only)

use markplus_core::json::SiteAsset;

use crate::context::ast_to_template_context;
use crate::engine::RenderEngine;
use crate::error::RenderError;

impl RenderEngine {
    /// Render HTML from a [`SiteAsset`] using the named Tera template.
    ///
    /// `template_name` must match a template loaded at engine construction,
    /// e.g. `"default/article.html.tera"`.
    pub fn render_html(
        &self,
        asset: &SiteAsset,
        template_name: &str,
    ) -> Result<String, RenderError> {
        let ctx_val = ast_to_template_context(asset);
        let ctx = tera::Context::from_value(ctx_val)?;
        self.tera.render(template_name, &ctx).map_err(RenderError::from)
    }

    /// Render a Typst source string from a [`SiteAsset`] using the named template.
    ///
    /// The returned string is valid `.typ` source that can be:
    /// - Written to disk and compiled with `typst compile out.typ out.pdf`
    /// - Passed directly to [`Self::compile_pdf`] for in-process compilation
    pub fn render_typst_string(
        &self,
        asset: &SiteAsset,
        template_name: &str,
    ) -> Result<String, RenderError> {
        let ctx_val = ast_to_template_context(asset);
        let ctx = tera::Context::from_value(ctx_val)?;
        self.tera.render(template_name, &ctx).map_err(RenderError::from)
    }

    // -----------------------------------------------------------------------
    // PDF compilation — native path
    // -----------------------------------------------------------------------

    /// Compile a Typst source string into PDF bytes.
    ///
    /// **Native**: uses `typst-as-lib` with system / bundled font discovery.
    /// **Wasm**: uses [`crate::wasm_world::WasmWorld`] with embedded Liberation fonts.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn compile_pdf(&self, typst_src: &str) -> Result<Vec<u8>, RenderError> {
        use typst::diag::Warned;
        use typst::layout::PagedDocument;
        use typst_as_lib::TypstEngine;
        use typst_as_lib::typst_kit_options::TypstKitFontOptions;

        let engine = TypstEngine::builder()
            .with_static_source_file_resolver([("main.typ", typst_src.to_string())])
            .with_package_file_resolver()
            .search_fonts_with(TypstKitFontOptions::default())
            .build();

        let warned: Warned<Result<PagedDocument, _>> = engine.compile("main.typ");

        for warning in &warned.warnings {
            eprintln!("typst warning: {}", warning.message);
        }

        let document: PagedDocument = warned
            .output
            .map_err(|e| RenderError::TypstCompile(format!("{:?}", e)))?;

        if document.pages.is_empty() {
            return Err(RenderError::TypstCompile("document has no pages".into()));
        }

        let options = typst_pdf::PdfOptions::default();
        typst_pdf::pdf(&document, &options)
            .map_err(|e| RenderError::TypstCompile(format!("{:?}", e)))
    }

    // -----------------------------------------------------------------------
    // PDF compilation — wasm path
    // -----------------------------------------------------------------------

    /// Compile a Typst source string into PDF bytes (wasm target).
    ///
    /// Uses [`crate::wasm_world::WasmWorld`] which embeds Liberation fonts so
    /// no filesystem access is required.
    #[cfg(target_arch = "wasm32")]
    pub fn compile_pdf(&self, typst_src: &str) -> Result<Vec<u8>, RenderError> {
        use crate::wasm_world::WasmWorld;
        use typst::layout::PagedDocument;

        let world = WasmWorld::new(typst_src.to_string());
        let warned = typst::compile(&world);

        let document: PagedDocument = warned.output.map_err(|errors| {
            let msg = errors
                .into_iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join("\n");
            RenderError::TypstCompile(msg)
        })?;

        if document.pages.is_empty() {
            return Err(RenderError::TypstCompile("document has no pages".into()));
        }

        let options = typst_pdf::PdfOptions::default();
        typst_pdf::pdf(&document, &options)
            .map_err(|e| RenderError::TypstCompile(format!("{:?}", e)))
    }

    // -----------------------------------------------------------------------
    // render_to_file — native convenience
    // -----------------------------------------------------------------------

    /// Render a document to a file on disk.
    ///
    /// The output format is determined by the file extension of `dest`:
    /// - `.html` — renders HTML template and writes the string
    /// - `.typ`  — renders Typst template and writes the source string
    /// - `.pdf`  — renders Typst template then compiles to PDF bytes
    ///
    /// Only available on native targets.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_to_file(
        &self,
        asset: &SiteAsset,
        template_name: &str,
        dest: &std::path::Path,
    ) -> Result<(), RenderError> {
        use std::fs;

        let ext = dest.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext {
            "html" => {
                let html = self.render_html(asset, template_name)?;
                fs::write(dest, html).map_err(RenderError::from)?;
            }
            "typ" => {
                let typ_src = self.render_typst_string(asset, template_name)?;
                fs::write(dest, typ_src).map_err(RenderError::from)?;
            }
            "pdf" => {
                let typ_src = self.render_typst_string(asset, template_name)?;
                let pdf_bytes = self.compile_pdf(&typ_src)?;
                fs::write(dest, pdf_bytes).map_err(RenderError::from)?;
            }
            other => {
                return Err(RenderError::Io(format!(
                    "unsupported output extension: {:?} (use .html, .typ, or .pdf)",
                    other
                )));
            }
        }
        Ok(())
    }
}
