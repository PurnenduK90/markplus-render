//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use serde_json::Value;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginType {
    Fenced,
    Directive,
    Widget,
}

pub trait MarkplusPlugin: Send + Sync {
    /// The specific name this plugin triggers on (e.g. "mermaid" for fenced, "tabs" for directive)
    fn name(&self) -> &'static str;

    /// What kind of AST node this plugin intercepts
    fn plugin_type(&self) -> PluginType;

    /// Render to HTML
    fn render_html(&self, node: &Value, out: &mut String);

    /// Render to Typst (Native only)
    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(&self, node: &Value, out: &mut String);

    /// Render to Markdown (Native only)
    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(&self, node: &Value, out: &mut String);
}

use std::sync::Arc;

// A global registry of plugins.
// Keyed by (PluginType, Name)
type RegistryKey = (PluginType, String);
type RegistryMap = HashMap<RegistryKey, Arc<dyn MarkplusPlugin>>;

static REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();

fn get_registry() -> &'static RwLock<RegistryMap> {
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a new plugin globally
pub fn register_plugin(plugin: Arc<dyn MarkplusPlugin>) {
    let key = (plugin.plugin_type(), plugin.name().to_string());
    let mut map = get_registry().write().unwrap();
    map.insert(key, plugin);
}

/// Get a plugin if it's registered
pub fn get_plugin(ptype: PluginType, name: &str) -> Option<Arc<dyn MarkplusPlugin>> {
    let map = get_registry().read().unwrap();
    let key = (ptype, name.to_string());
    map.get(&key).cloned()
}
