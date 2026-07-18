//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use markplus_core::json::SiteAsset;
use serde_json::json;

use crate::engine::RenderEngine;
use crate::error::RenderError;

impl RenderEngine {
    pub fn render_html(
        &self,
        asset: &SiteAsset,
        template_name: &str,
    ) -> Result<String, RenderError> {
        let mut html_body = String::new();
        for node in &asset.ast {
            crate::blocks::render_html_node(node, &mut html_body);
        }

        let ctx = json!({
            "meta": asset.meta,
            "toc": crate::context::build_toc(&asset.ast),
            "body": html_body,
        });

        let tmpl = self
            .env
            .get_template(template_name)
            .map_err(|e| RenderError::TemplateRender(e.to_string()))?;

        tmpl.render(ctx)
            .map_err(|e| RenderError::TemplateRender(e.to_string()))
    }

    pub fn render_typst_string(
        &self,
        asset: &SiteAsset,
        template_name: &str,
    ) -> Result<String, RenderError> {
        let mut typst_body = String::new();
        for node in &asset.ast {
            crate::blocks::render_typst_node(node, &mut typst_body);
        }

        let ctx = json!({
            "meta": asset.meta,
            "toc": crate::context::build_toc(&asset.ast),
            "body": typst_body,
        });

        let tmpl = self
            .env
            .get_template(template_name)
            .map_err(|e| RenderError::TemplateRender(e.to_string()))?;

        tmpl.render(ctx)
            .map_err(|e| RenderError::TemplateRender(e.to_string()))
    }

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
