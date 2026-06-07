//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Tera-based render engine for MarkPlus.
//!
//! [`RenderEngine`] wraps a Tera instance pre-loaded with templates and
//! custom filters. Build one via [`RenderEngineBuilder`]:
//!
//! ```ignore
//! // Load templates from a directory (native)
//! let engine = RenderEngine::builder()
//!     .with_templates(Path::new("templates"))
//!     .build()?;
//!
//! // In-memory templates (wasm / testing)
//! let engine = RenderEngine::builder()
//!     .build_with_templates(HashMap::from([
//!         ("default/article.html.tera".into(), TEMPLATE_STR.into()),
//!     ]))?;
//! ```

use std::collections::HashMap;
use anyhow::Context as _;
use tera::Tera;

use crate::error::RenderError;
use crate::filters::register_filters;

/// The core render engine — a configured Tera instance with custom filters.
pub struct RenderEngine {
    pub(crate) tera: Tera,
}

/// Builder for [`RenderEngine`].
#[derive(Default)]
pub struct RenderEngineBuilder {
    templates_dir: Option<String>,
}

impl RenderEngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the templates directory (default: `"templates"`).
    pub fn with_templates(mut self, path: &std::path::Path) -> Self {
        self.templates_dir = Some(path.to_string_lossy().into_owned());
        self
    }

    /// Build by scanning a templates directory on the filesystem.
    ///
    /// Available on native targets. Uses `templates/**/*` glob pattern.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self) -> Result<RenderEngine, RenderError> {
        let dir = self.templates_dir.as_deref().unwrap_or("templates");
        let glob = format!("{}/**/*", dir);
        let mut tera = Tera::new(&glob)
            .with_context(|| format!("Failed to load templates from: {}", dir))
            .map_err(|e| RenderError::TeraRender(e.to_string()))?;
        register_filters(&mut tera);
        Ok(RenderEngine { tera })
    }

    /// Build from an in-memory map of `template_name → template_source`.
    ///
    /// Used for wasm targets and unit tests where the filesystem is
    /// unavailable or undesirable.
    pub fn build_with_templates(
        self,
        templates: HashMap<String, String>,
    ) -> Result<RenderEngine, RenderError> {
        let mut tera = Tera::default();
        for (name, content) in templates {
            tera.add_raw_template(&name, &content)
                .with_context(|| format!("Failed to add template: {}", name))
                .map_err(|e| RenderError::TeraRender(e.to_string()))?;
        }
        register_filters(&mut tera);
        Ok(RenderEngine { tera })
    }
}

impl RenderEngine {
    /// Create a new [`RenderEngineBuilder`].
    pub fn builder() -> RenderEngineBuilder {
        RenderEngineBuilder::new()
    }
}
