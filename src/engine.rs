//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use anyhow::Context as _;
use minijinja::Environment;
use std::collections::HashMap;
use std::path::Path;

use crate::error::RenderError;

pub struct RenderEngine {
    pub(crate) env: Environment<'static>,
}

#[derive(Default)]
pub struct RenderEngineBuilder {
    templates_dir: Option<String>,
}

impl RenderEngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_templates(mut self, path: &Path) -> Self {
        self.templates_dir = Some(path.to_string_lossy().into_owned());
        self
    }

    pub fn build(self) -> Result<RenderEngine, RenderError> {
        let dir = self.templates_dir.as_deref().unwrap_or("templates");

        let mut env = Environment::new();
        let glob_path = format!("{}/**/*", dir);
        for entry in glob::glob(&glob_path).unwrap().filter_map(Result::ok) {
            if entry.is_file() {
                let name = entry
                    .strip_prefix(dir)
                    .unwrap()
                    .to_string_lossy()
                    .replace("\\", "/");
                let content = std::fs::read_to_string(&entry)
                    .with_context(|| format!("Failed to read template file: {:?}", entry))
                    .map_err(|e| RenderError::TemplateRender(e.to_string()))?;
                env.add_template_owned(name, content)
                    .map_err(|e| RenderError::TemplateRender(e.to_string()))?;
            }
        }

        crate::filters::register_filters(&mut env);

        Ok(RenderEngine { env })
    }

    pub fn build_with_templates(
        self,
        templates: HashMap<String, String>,
    ) -> Result<RenderEngine, RenderError> {
        let mut env = Environment::new();
        for (name, content) in templates {
            env.add_template_owned(name, content)
                .map_err(|e| RenderError::TemplateRender(e.to_string()))?;
        }
        crate::filters::register_filters(&mut env);
        Ok(RenderEngine { env })
    }
}

impl RenderEngine {
    pub fn builder() -> RenderEngineBuilder {
        RenderEngineBuilder::new()
    }
}
