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
//! - **HTML** — via MiniJinja template (`*.html.jinja`)
//! - **Typst source** — via MiniJinja template (`*.typ.jinja`); compile with `typst compile`
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
//! let html = engine.render_html(&asset, "default/article.html.jinja")?;
//! let typ_src = engine.render_typst_string(&asset, "default/article.typ.jinja")?;
//! let pdf_bytes = engine.compile_pdf(&typ_src)?;
//! ```

pub mod blocks;

#[cfg(not(target_arch = "wasm32"))]
pub mod context;
#[cfg(not(target_arch = "wasm32"))]
pub mod engine;
#[cfg(not(target_arch = "wasm32"))]
pub mod error;
#[cfg(not(target_arch = "wasm32"))]
pub mod filters;
pub mod plugin;
pub mod postprocess;
#[cfg(not(target_arch = "wasm32"))]
pub mod render;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use engine::{RenderEngine, RenderEngineBuilder};
#[cfg(not(target_arch = "wasm32"))]
pub use error::RenderError;
pub use postprocess::{ImagePathRewriter, PostProcessPipeline, PostProcessor};
