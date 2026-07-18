//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use serde_json::Value;

use crate::blocks::RenderBlock;

pub struct HrRenderer;

impl RenderBlock for HrRenderer {
    fn render_html(_node: &Value, out: &mut String) {
        out.push_str("<hr>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(_node: &Value, out: &mut String) {
        out.push_str("#line(length: 100%, stroke: 0.5pt)\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(_node: &Value, out: &mut String) {
        out.push_str("---\n\n");
    }
}
