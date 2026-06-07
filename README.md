# markplus_render

HTML and PDF renderer for the [MarkPlus](https://github.com/PurnenduK90/markplus-core) ecosystem.

Consumes a `SiteAsset` (schema + frontmatter + AST) produced by `markplus_core` and emits:

| Target | How |
|---|---|
| **HTML** | AST → Tera `*.html.tera` template → `.html` string |
| **Typst source** | AST → Tera `*.typ.tera` template → `.typ` string |
| **PDF (native)** | Typst source → `typst-as-lib` → PDF bytes |
| **PDF (wasm)** | Typst source → embedded `WasmWorld` → PDF bytes (`--features wasm`, ~30 MB) |

## Quick start

```rust
use markplus_core::parse_document;
use markplus_render::RenderEngine;

let asset = parse_document("# Hello\n\n**World.**")?;
let engine = RenderEngine::builder().build()?; // loads templates/ dir

let html      = engine.render_html(&asset, "default/article.html.tera")?;
let typst_src = engine.render_typst_string(&asset, "default/article.typ.tera")?;
let pdf_bytes = engine.compile_pdf(&typst_src)?;
```

## Feature flags

| Feature | Default | Description |
|---|---|---|
| _(none)_ | ✓ | Native HTML + Typst + PDF |
| `wasm` | off | Adds `wasm-bindgen` exports + in-browser PDF via embedded fonts |

## Template directory

Default templates live in `templates/default/`:

```
templates/
└── default/
    ├── article.html.tera   ← HTML article
    └── article.typ.tera    ← Typst article (→ PDF)
```

Override at runtime:

```rust
let engine = RenderEngine::builder()
    .with_templates(Path::new("/my/custom/templates"))
    .build()?;
```

## PDF pipeline

```
Markdown → markplus_core → SiteAsset
                               │
                    render_typst_string()
                               │
                           .typ source
                               │
                 ┌─────────────┴─────────────┐
                 │ native                    │ wasm
          typst-as-lib                  WasmWorld
          + system fonts           + Liberation fonts
                 │                           │
                 └──────────── PDF bytes ────┘
```

## Example

```sh
cargo run --example render_doc -- my_note.md
# Writes my_note.html, my_note.typ, my_note.pdf
```

## License

Apache-2.0
