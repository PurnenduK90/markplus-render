//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Error type for `markplus_render`.

use std::fmt;

/// All errors that can occur during rendering or PDF compilation.
#[derive(Debug)]
pub enum RenderError {
    /// Tera template rendering failed.
    TeraRender(String),
    /// Typst compilation failed (native or wasm).
    TypstCompile(String),
    /// File I/O error (native only).
    Io(String),
    /// The context produced from the asset was invalid.
    InvalidContext(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TeraRender(msg)      => write!(f, "template render error: {msg}"),
            Self::TypstCompile(msg)    => write!(f, "typst compile error: {msg}"),
            Self::Io(msg)              => write!(f, "io error: {msg}"),
            Self::InvalidContext(msg)  => write!(f, "invalid context: {msg}"),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<tera::Error> for RenderError {
    fn from(e: tera::Error) -> Self {
        Self::TeraRender(e.to_string())
    }
}

impl From<serde_json::Error> for RenderError {
    fn from(e: serde_json::Error) -> Self {
        Self::InvalidContext(e.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for RenderError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
