//    Copyright [2026] [Purnendu Kumar]

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

//! # markplus_render
//!
//! Renderer for the MarkPlus ecosystem. Consumes a [`markplus_core::json::SiteAsset`]
//! (schema + meta + AST) and produces:
//!
//! - **HTML** — via Tera template (`*.html.tera`)
//! - **Typst source** — via Tera template (`*.typ.tera`); compile with `typst compile`
//! - **PDF bytes** — Typst source compiled in-process:
//!   - Native: `typst-as-lib` + system font discovery
//!   - Wasm (`--features wasm`): embedded `WasmWorld` + Liberation fonts (~30 MB bundle)
//!
//! ## Quick start
//!
//! ```ignore
//! use markplus_core::parse_document;
//! use markplus_render::{RenderEngine, RenderError};
//!
//! let asset = parse_document("# Hello\n\nWorld.")?;
//! let engine = RenderEngine::builder().build()?;
//! let html = engine.render_html(&asset, "default/article.html.tera")?;
//! let typ_src = engine.render_typst_string(&asset, "default/article.typ.tera")?;
//! let pdf_bytes = engine.compile_pdf(&typ_src)?;
//! ```

pub mod context;
pub mod engine;
pub mod error;
pub mod filters;
pub mod render;

#[cfg(target_arch = "wasm32")]
pub mod wasm_world;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use context::ast_to_template_context;
pub use engine::{RenderEngine, RenderEngineBuilder};
pub use error::RenderError;
