//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use markplus_core::json::SiteAsset;
use serde_json::Value;

/// Trait for modifying a `SiteAsset` after it has been parsed from Markdown
/// but before it is rendered to HTML or Typst.
pub trait PostProcessor {
    /// Mutates the given `SiteAsset` in-place.
    fn process(&self, asset: &mut SiteAsset) -> Result<(), anyhow::Error>;
}

/// A pipeline of post-processors to apply sequentially.
#[derive(Default)]
pub struct PostProcessPipeline {
    processors: Vec<Box<dyn PostProcessor>>,
}

impl PostProcessPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_processor<P: PostProcessor + 'static>(mut self, processor: P) -> Self {
        self.processors.push(Box::new(processor));
        self
    }

    pub fn run(&self, asset: &mut SiteAsset) -> Result<(), anyhow::Error> {
        for processor in &self.processors {
            processor.process(asset)?;
        }
        Ok(())
    }
}

/// A simple post-processor for images that rewrites paths based on a configuration.
pub struct ImagePathRewriter {
    pub prefix: String,
}

impl PostProcessor for ImagePathRewriter {
    fn process(&self, asset: &mut SiteAsset) -> Result<(), anyhow::Error> {
        for node in &mut asset.ast {
            rewrite_image_paths(node, &self.prefix);
        }
        Ok(())
    }
}

fn rewrite_image_paths(node: &mut Value, prefix: &str) {
    if let Value::Object(map) = node {
        // If this is an image node, update its src
        if map.get("t").and_then(|v| v.as_str()) == Some("image")
            && let Some(Value::String(src)) = map.get_mut("src")
                && !src.starts_with("http://")
                    && !src.starts_with("https://")
                    && !src.starts_with("data:")
                {
                    *src = format!("{}{}", prefix, src);
                }

        // Recursively process children
        if let Some(Value::Array(children)) = map.get_mut("children") {
            for child in children {
                rewrite_image_paths(child, prefix);
            }
        }

        // Check items (for lists)
        if let Some(Value::Array(items)) = map.get_mut("items") {
            for item in items {
                rewrite_image_paths(item, prefix);
            }
        }

        // Check rows and headers (for tables)
        if let Some(Value::Array(rows)) = map.get_mut("rows") {
            for row in rows {
                if let Value::Array(cells) = row {
                    for cell in cells {
                        rewrite_image_paths(cell, prefix);
                    }
                }
            }
        }
        if let Some(Value::Array(headers)) = map.get_mut("headers") {
            for header in headers {
                rewrite_image_paths(header, prefix);
            }
        }
    }
}
