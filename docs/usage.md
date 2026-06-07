# markplus_render — Usage Guide

Renders a [`SiteAsset`](../markplus-core/docs/usage.md) (schema + meta + AST) produced by
`markplus_core` into **HTML**, **Typst source**, and **PDF**.

## Contents

1. [Architecture overview](#1-architecture-overview)
2. [Quick start](#2-quick-start)
3. [Building the engine](#3-building-the-engine)
4. [Rendering HTML](#4-rendering-html)
5. [Rendering Typst source](#5-rendering-typst-source)
6. [Compiling PDF](#6-compiling-pdf)
7. [render_to_file convenience API](#7-render_to_file-convenience-api)
8. [Template system](#8-template-system)
9. [Custom filters reference](#9-custom-filters-reference)
10. [WebAssembly usage](#10-webassembly-usage)
11. [Feature flags](#11-feature-flags)
12. [Error handling](#12-error-handling)
13. [End-to-end pipeline](#13-end-to-end-pipeline)

---

## 1. Architecture overview

```
Markdown (.md)
     │
     ▼
markplus_core::parse_document()
     │
     ▼
SiteAsset { schema, meta, ast }
     │
     ├──── render_html()  ─────────────────► HTML string (.html)
     │         │
     │    Tera template (*.html.tera)
     │
     ├──── render_typst_string()  ─────────► Typst source (.typ)
     │         │                                    │
     │    Tera template (*.typ.tera)          typst compile
     │                                              │
     │                                             PDF
     │
     └──── compile_pdf()  ─────────────────► PDF bytes (Vec<u8>)
               │
         native: typst-as-lib + system fonts
         wasm:   WasmWorld + embedded Liberation fonts
```

**Key design rule:** HTML and Typst rendering use identical Tera context — only
the template file differs. `compile_pdf` accepts any valid Typst source string,
so it can also compile hand-written `.typ` files.

---

## 2. Quick start

```toml
# Cargo.toml
[dependencies]
markplus_core  = { path = "../markplus-core" }
markplus_render = { path = "../markplus-render" }
```

```rust
use markplus_core::parse_document;
use markplus_render::RenderEngine;

fn main() -> anyhow::Result<()> {
    let md = std::fs::read_to_string("my_note.md")?;
    let asset = parse_document(&md)?;

    let engine = RenderEngine::builder().build()?;   // loads templates/ dir

    // HTML
    let html = engine.render_html(&asset, "default/article.html.tera")?;
    std::fs::write("my_note.html", html)?;

    // Typst → PDF
    let typ_src = engine.render_typst_string(&asset, "default/article.typ.tera")?;
    let pdf = engine.compile_pdf(&typ_src)?;
    std::fs::write("my_note.pdf", pdf)?;

    Ok(())
}
```

Or with the single-call convenience method:

```rust
use std::path::Path;

engine.render_to_file(&asset, "default/article.html.tera", Path::new("out.html"))?;
engine.render_to_file(&asset, "default/article.typ.tera",  Path::new("out.typ"))?;
engine.render_to_file(&asset, "default/article.typ.tera",  Path::new("out.pdf"))?;
// .pdf extension automatically compiles Typst → PDF
```

---

## 3. Building the engine

### From a templates directory (native, recommended)

```rust
use markplus_render::RenderEngine;
use std::path::Path;

// Default: loads templates/**/* from the "templates" directory
let engine = RenderEngine::builder().build()?;

// Override the templates directory
let engine = RenderEngine::builder()
    .with_templates(Path::new("/my/custom/templates"))
    .build()?;
```

Template names are relative to the templates root, e.g.
`"default/article.html.tera"`.

### From in-memory strings (wasm / testing)

```rust
use std::collections::HashMap;
use markplus_render::RenderEngine;

let html_tpl = include_str!("templates/default/article.html.tera");
let typ_tpl  = include_str!("templates/default/article.typ.tera");

let engine = RenderEngine::builder()
    .build_with_templates(HashMap::from([
        ("default/article.html.tera".into(), html_tpl.into()),
        ("default/article.typ.tera".into(),  typ_tpl.into()),
    ]))?;
```

---

## 4. Rendering HTML

```rust
let html: String = engine.render_html(&asset, "default/article.html.tera")?;
```

The returned string is a complete `<!DOCTYPE html>` document. It includes:

- `<title>` from `meta.title`
- `<time>` formatted date from `meta.date`
- Tag pills from `meta.tags`
- Automatic TOC when there are ≥ 2 headings
- All AST node types: headings, paragraphs, fenced code, tables, lists,
  blockquotes (with GFM alert classes), math, footnotes, images, links, widgets

To use a custom template, register it at engine build time and pass its name:

```rust
let engine = RenderEngine::builder()
    .build_with_templates(HashMap::from([
        ("blog/post.html.tera".into(), MY_TEMPLATE.into()),
    ]))?;

let html = engine.render_html(&asset, "blog/post.html.tera")?;
```

---

## 5. Rendering Typst source

```rust
let typ_src: String = engine.render_typst_string(&asset, "default/article.typ.tera")?;
```

The returned string is valid Typst markup that can be:

- Written to a `.typ` file and compiled with `typst compile`
- Passed to [`compile_pdf`](#6-compiling-pdf) for in-process compilation

The default template sets up an A4 page with Liberation fonts, numbered
headings, and renders all block types supported by the HTML template.

### Writing the `.typ` file for manual compilation

```rust
std::fs::write("out.typ", &typ_src)?;
// Then externally: typst compile out.typ out.pdf
```

---

## 6. Compiling PDF

`compile_pdf` accepts any valid Typst source string — typically the output of
`render_typst_string`, but also hand-written `.typ` content.

```rust
let pdf_bytes: Vec<u8> = engine.compile_pdf(&typ_src)?;
std::fs::write("out.pdf", &pdf_bytes)?;
```

### Native compilation (default)

Uses `typst-as-lib` with system font discovery via `typst-kit`. No external
process is spawned — compilation happens in-process.

```
Typst source string
       │
  typst-as-lib
  + typst-kit (system fonts)
  + typst-pdf
       │
    PDF bytes
```

### Wasm compilation (`--features wasm`, ~30 MB bundle)

Uses `WasmWorld` — a minimal `typst::World` implementation with 12 Liberation
font variants embedded at compile time. No filesystem access required.

```
Typst source string
       │
  WasmWorld (embedded fonts)
  + typst::compile()
  + typst-pdf
       │
    PDF bytes
```

**Embedded fonts:** Liberation Sans (Regular/Bold/Italic/BoldItalic),
Liberation Serif (Regular/Bold/Italic/BoldItalic),
Liberation Mono (Regular/Bold/Italic/BoldItalic).

---

## 7. `render_to_file` convenience API

Available on native targets only. The output format is selected by file extension.

```rust
use std::path::Path;
use markplus_render::RenderEngine;

let engine = RenderEngine::builder().build()?;

// Write HTML
engine.render_to_file(&asset, "default/article.html.tera", Path::new("doc.html"))?;

// Write Typst source
engine.render_to_file(&asset, "default/article.typ.tera",  Path::new("doc.typ"))?;

// Render Typst then compile → PDF in one call
engine.render_to_file(&asset, "default/article.typ.tera",  Path::new("doc.pdf"))?;
```

| Extension | Action |
|-----------|--------|
| `.html`   | `render_html()` → write string |
| `.typ`    | `render_typst_string()` → write string |
| `.pdf`    | `render_typst_string()` → `compile_pdf()` → write bytes |

---

## 8. Template system

Templates use [Tera](https://keats.github.io/tera/) syntax (Jinja2-like).

### Naming convention

```
templates/
└── <theme>/
    ├── <doctype>.html.tera   — HTML output
    └── <doctype>.typ.tera    — Typst source output
```

Example: `templates/default/article.html.tera`

### Context shape

Every template receives the same context object — see
[`docs/template-context.md`](./template-context.md) for the full reference.

```jinja2
{# Access frontmatter #}
{% if meta and meta.title %}{{ meta.title }}{% endif %}

{# Render body nodes #}
{% for node in body %}
  {% if node.t == "heading" %}
    <h{{ node.level }}>
      {% for child in node.children %}{{ child.text | default(value="") }}{% endfor %}
    </h{{ node.level }}>
  {% elif node.t == "paragraph" %}
    <p>...</p>
  {% endif %}
{% endfor %}
```

### Built-in templates

| Template name | Description |
|---|---|
| `default/article.html.tera` | HTML article with TOC, meta header, all node types |
| `default/article.typ.tera`  | Typst article → PDF with numbered headings, A4 page |

### Creating a custom template

1. Create `templates/blog/post.html.tera`
2. Use the same `meta`, `toc`, `body` variables (see [context reference](./template-context.md))
3. Load it at engine build time or place it in the templates directory
4. Call `engine.render_html(&asset, "blog/post.html.tera")`

---

## 9. Custom filters reference

These Tera filters are pre-registered on every engine instance.

### `slugify`

Convert a string to a URL-safe lowercase slug.

```jinja2
{{ meta.title | slugify }}
{# "Hello, World! (2026)" → "hello-world-2026" #}
```

### `date_fmt`

Format an ISO 8601 date string. Optional `format` argument accepts `%Y`, `%m`,
`%d`, `%B` tokens.

```jinja2
{{ meta.date | date_fmt }}
{# "2026-06-07" → "June 07, 2026" #}

{{ meta.date | date_fmt(format="%Y-%m-%d") }}
{# "2026-06-07" → "2026-06-07" #}

{{ meta.date | date_fmt(format="%B %Y") }}
{# "2026-06-07" → "June 2026" #}
```

If the value is not a recognisable ISO date it is returned unchanged.

### `safe_html`

Escape HTML special characters (`&`, `<`, `>`, `"`, `'`).

```jinja2
{{ node.raw | safe_html }}
{# "<script>" → "&lt;script&gt;" #}
```

> **Note:** Tera's built-in `{{ value }}` auto-escapes in HTML mode when
> `autoescape` is enabled. Use `safe_html` explicitly for attribute values or
> raw blocks where you need guaranteed escaping.

---

## 10. WebAssembly usage

### Building the wasm package

```bash
cd markplus-render
wasm-pack build --target web --release --features wasm
# Output: pkg/
```

### JavaScript / TypeScript API

```js
import init, { MarkplusRenderWasm } from "./pkg/markplus_render.js";
await init();

// 1. Load templates (fetch or embed as strings)
const htmlTpl = await fetch("/templates/default/article.html.tera").then(r => r.text());
const typTpl  = await fetch("/templates/default/article.typ.tera").then(r => r.text());

// 2. Create renderer with template map
const renderer = new MarkplusRenderWasm({
    "default/article.html.tera": htmlTpl,
    "default/article.typ.tera":  typTpl,
});

// 3. Render HTML preview (fast, no PDF)
const asset = JSON.parse(markplusCore.parse_document_to_json(rawMarkdown));
const html = renderer.render_html(asset, "default/article.html.tera");
document.getElementById("preview").innerHTML = html;

// 4. Render + compile PDF (requires wasm feature, ~30 MB bundle)
const pdfBytes = renderer.compile_pdf_from_asset(asset, "default/article.typ.tera");
const blob = new Blob([pdfBytes], { type: "application/pdf" });
window.open(URL.createObjectURL(blob));
```

### Wasm API surface

| Method | Description |
|---|---|
| `new(templates: Record<string, string>)` | Construct renderer from template map |
| `render_html(asset, templateName)` | Render HTML string |
| `render_typst(asset, templateName)` | Render Typst source string |
| `compile_pdf(typstSrc)` | Compile Typst source → PDF bytes |
| `compile_pdf_from_asset(asset, templateName)` | Render Typst + compile PDF in one call |
| `render_html_simple(asset)` | Lightweight HTML without Tera (no template needed) |

### `render_html_simple` — templateless preview

When no template is loaded (e.g. for instant previews), `render_html_simple`
converts the AST to HTML using pure Rust code — no Tera engine required.

```js
const html = renderer.render_html_simple(asset);
```

Supported node types: headings, paragraphs (with inline formatting), fenced
code blocks, math (data-attr), blockquotes, ordered/unordered lists, tables,
`<hr>`, links, images, hard/soft breaks.

---

## 11. Feature flags

| Feature | Default | Notes |
|---|---|---|
| _(none)_ | ✓ | Native: HTML + Typst + PDF. No wasm exports. |
| `wasm` | off | Adds `wasm-bindgen` exports + in-browser PDF. Requires `--target wasm32-unknown-unknown`. |

### Cargo.toml examples

```toml
# Native only (default)
markplus_render = { path = "../markplus-render" }

# Wasm build with PDF support
markplus_render = { path = "../markplus-render", features = ["wasm"] }
```

### wasm-pack build commands

```bash
# HTML-only wasm (small bundle, no PDF)
wasm-pack build --target web --release
# Note: without --features wasm, wasm-bindgen exports are not emitted.
# Use --features wasm for the JS-callable API.

# Full wasm with PDF (~30 MB)
wasm-pack build --target web --release --features wasm
```

---

## 12. Error handling

All render methods return `Result<_, RenderError>`.

```rust
use markplus_render::RenderError;

match engine.render_html(&asset, "default/article.html.tera") {
    Ok(html) => { /* use html */ }
    Err(RenderError::TeraRender(msg))     => eprintln!("template error: {msg}"),
    Err(RenderError::TypstCompile(msg))   => eprintln!("typst compile error: {msg}"),
    Err(RenderError::Io(msg))             => eprintln!("io error: {msg}"),
    Err(RenderError::InvalidContext(msg)) => eprintln!("context error: {msg}"),
}
```

| Variant | Cause |
|---|---|
| `TeraRender` | Template not found, invalid template syntax, missing required variable |
| `TypstCompile` | Typst source has syntax errors or produces no pages |
| `Io` | File write failed (native only, from `render_to_file`) |
| `InvalidContext` | JSON serialization of the AST context failed (should not occur with valid `SiteAsset`) |

---

## 13. End-to-end pipeline

### Native deploy pass

```
.md file
  │
  ├─ markplus_core::parse_document()
  │        │
  │    SiteAsset
  │        │
  │        ├── render_html()         → note.html
  │        ├── render_typst_string() → note.typ
  │        └── compile_pdf()         → note.pdf
  │
  └─ strip_frontmatter()             → note.body.md  (for AI tools)
```

### Wasm live preview

```
User types Markdown
       │
markplus_core (wasm) → parse_document_to_json()
       │
   SiteAsset (JSON)
       │
markplus_render (wasm) → render_html_simple()  [instant, no template]
       │                  or render_html()      [with template]
       ▼
  Preview pane (innerHTML)
       │
   [on export] compile_pdf_from_asset() → PDF download
```

### CLI integration

```bash
# Step 1: parse
mpc note.md > note.json

# Step 2: render HTML (using a separate CLI tool or script that calls markplus_render)
render-cli --input note.json --template default/article.html.tera --output note.html

# Step 3: render PDF
render-cli --input note.json --template default/article.typ.tera --output note.pdf
# (render-cli calls compile_pdf() internally)
```

See [`examples/render_doc.rs`](../examples/render_doc.rs) for a working
end-to-end example that reads a `.md` file and writes `.html`, `.typ`, and
`.pdf` outputs.
