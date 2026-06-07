//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Minimal [`typst::World`] implementation for wasm targets.
//!
//! `WasmWorld` embeds all 12 Liberation fonts (Sans, Serif, Mono ×
//! Regular/Bold/Italic/BoldItalic) at compile time via `include_bytes!`.
//! No filesystem access is required, making it suitable for browser and
//! Tauri webview environments.

use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt};

/// A self-contained Typst world that compiles `.typ` source from a string,
/// using only embedded fonts — no filesystem required.
pub struct WasmWorld {
    main_id:     FileId,
    main_source: Source,
    font_book:   LazyHash<FontBook>,
    fonts:       Vec<Font>,
    library:     LazyHash<Library>,
}

impl WasmWorld {
    /// Create a new world from raw Typst source content.
    pub fn new(typst_content: String) -> Self {
        let fonts = Self::load_embedded_fonts();
        let font_book = LazyHash::new(FontBook::from_fonts(&fonts));
        let main_id = FileId::new(None, VirtualPath::new("/main.typ"));
        let main_source = Source::new(main_id, typst_content);

        Self {
            main_id,
            main_source,
            font_book,
            fonts,
            library: LazyHash::new(Library::default()),
        }
    }

    /// Load all embedded Liberation font variants.
    fn load_embedded_fonts() -> Vec<Font> {
        // Each entry: (bytes, face_index)
        // Liberation fonts have one face per file (index 0).
        let font_data: &[&[u8]] = &[
            include_bytes!("fonts/LiberationSans-Regular.ttf"),
            include_bytes!("fonts/LiberationSans-Bold.ttf"),
            include_bytes!("fonts/LiberationSans-Italic.ttf"),
            include_bytes!("fonts/LiberationSans-BoldItalic.ttf"),
            include_bytes!("fonts/LiberationSerif-Regular.ttf"),
            include_bytes!("fonts/LiberationSerif-Bold.ttf"),
            include_bytes!("fonts/LiberationSerif-Italic.ttf"),
            include_bytes!("fonts/LiberationSerif-BoldItalic.ttf"),
            include_bytes!("fonts/LiberationMono-Regular.ttf"),
            include_bytes!("fonts/LiberationMono-Bold.ttf"),
            include_bytes!("fonts/LiberationMono-Italic.ttf"),
            include_bytes!("fonts/LiberationMono-BoldItalic.ttf"),
        ];

        font_data
            .iter()
            .filter_map(|data| Font::new(Bytes::new(*data), 0))
            .collect()
    }
}

impl typst::World for WasmWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            Ok(self.main_source.clone())
        } else {
            Err(FileError::NotFound(
                id.vpath().as_rootless_path().to_path_buf(),
            ))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(
            id.vpath().as_rootless_path().to_path_buf(),
        ))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let now = js_sys::Date::new_0();
        let year  = now.get_full_year() as i32;
        let month = now.get_month() as u8 + 1;
        let day   = now.get_date() as u8;
        Datetime::from_ymd(year, month, day)
    }
}
