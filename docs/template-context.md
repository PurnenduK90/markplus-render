# markplus_render — Template Context Reference

Every MiniJinja template receives a single context object. This document describes
every field available, its type, and when it is present.

## Top-level shape

```json
{
  "meta":  { ... } | null,
  "slug":  "my-document-title",
  "toc":   [ { "level": 1, "text": "Intro", "slug": "intro" }, ... ],
  "body":  [ <block node>, ... ]
}
```

| Field  | Type | Always present | Description |
|--------|------|---------------|-------------|
| `meta` | object \| null | ✓ | Parsed YAML frontmatter (or `null` if none) |
| `slug` | string | ✓ | URL-safe slug derived from `meta.title` or first heading |
| `toc`  | array | ✓ | Heading entries for table-of-contents (empty if no headings) |
| `body` | array | ✓ | Full AST node array from `markplus_core` |

---

## `meta` — frontmatter fields

`meta` is whatever YAML keys appear in the document frontmatter. Common
conventions used by the built-in templates:

| Key | Type | Example |
|-----|------|---------|
| `title` | string | `"My Article"` |
| `author` | string | `"Ada Lovelace"` |
| `date` | string (ISO 8601) | `"2026-06-07"` |
| `tags` | string array | `["rust", "wasm"]` |
| `category` | string | `"hardware"` |

Any key in the frontmatter is accessible in templates as `meta.<key>`.

```yaml
---
title: My Article
date: 2026-06-07
tags: [rust, wasm]
custom_field: hello
---
```

```jinja2
{{ meta.title }}          {# "My Article" #}
{{ meta.date | date_fmt }} {# "June 07, 2026" #}
{{ meta.custom_field }}    {# "hello" #}
```

---

## `slug` — derived URL slug

A lowercase, hyphen-separated identifier:

- Derived from `meta.title` if present
- Falls back to the text of the first heading in `body`
- Defaults to `"document"` if neither is available

```jinja2
<article id="{{ slug }}">
```

---

## `toc` — table of contents entries

An array extracted from all `heading` nodes in `body` (including those nested
in blockquotes or list items).

```json
[
  { "level": 1, "text": "Introduction",     "slug": "introduction" },
  { "level": 2, "text": "Getting Started",  "slug": "getting-started" },
  { "level": 2, "text": "Configuration",    "slug": "configuration" }
]
```

| Field | Type | Description |
|-------|------|-------------|
| `level` | integer (1–6) | Heading level |
| `text` | string | Plain text of the heading (inline markup stripped) |
| `slug` | string | URL-safe slug of the heading text |

```jinja2
{% if toc | length > 1 %}
<nav>
  {% for entry in toc %}
  <a href="#{{ entry.slug }}" class="level-{{ entry.level }}">{{ entry.text }}</a>
  {% endfor %}
</nav>
{% endif %}
```

---

## `body` — AST node array

The `body` array contains the same JSON nodes produced by `markplus_core`.
Each node has a `"t"` field (type discriminant). The full AST reference is in
[`markplus-core/docs/ast-reference.md`](../../markplus-core/docs/ast-reference.md).

Below is a practical template-focused summary.

---

### Block nodes

#### `heading`

```json
{ "t": "heading", "level": 2, "children": [ <inline nodes> ] }
```

| Field | Type | Notes |
|-------|------|-------|
| `level` | integer 1–6 | Heading depth |
| `children` | inline node array | Content (text, math, code, etc.) |

```jinja2
{% if node.t == "heading" %}
<h{{ node.level }}>
  {% for child in node.children %}{{ child.text | default(value="") }}{% endfor %}
</h{{ node.level }}>
{% endif %}
```

---

#### `paragraph`

```json
{ "t": "paragraph", "children": [ <inline nodes> ] }
```

```jinja2
{% if node.t == "paragraph" %}
<p>{% for child in node.children %}...{% endfor %}</p>
{% endif %}
```

---

#### `fenced`

Code block or named plugin block (mermaid, simby, etc.).

```json
{
  "t": "fenced",
  "name": "rust",
  "raw": "fn main() {}",
  "attrs": { "execute": "true", "linenos": true }
}
```

| Field | Type | Notes |
|-------|------|-------|
| `name` | string | Language / plugin name (`""` for plain code blocks) |
| `raw` | string | Raw block content (trimmed trailing newline) |
| `attrs` | object | Optional key=value attributes from info string |

```jinja2
{% if node.t == "fenced" %}
<pre><code class="language-{{ node.name }}">{{ node.raw | safe_html }}</code></pre>
{% endif %}
```

---

#### `math_block`

Display math (`$$ … $$`).

```json
{ "t": "math_block", "src": "\\int_0^\\infty e^{-x}\\,dx = 1" }
```

```jinja2
{% if node.t == "math_block" %}
<p class="math-block">$${{ node.src }}$$</p>
{% endif %}
```

---

#### `blockquote`

Regular and GFM alert blockquotes.

```json
{
  "t": "blockquote",
  "kind": "warning",
  "children": [ <block nodes> ]
}
```

| Field | Type | Notes |
|-------|------|-------|
| `kind` | string \| absent | `"note"`, `"tip"`, `"warning"`, `"caution"`, `"important"` — only present for GFM `[!…]` alerts |
| `children` | block node array | Nested paragraphs, fenced blocks, etc. |

```jinja2
{% if node.t == "blockquote" %}
<blockquote class="{{ node.kind | default(value="") }}">
  {% for child in node.children %}...{% endfor %}
</blockquote>
{% endif %}
```

---

#### `list`

Ordered or unordered list.

```json
{
  "t": "list",
  "ordered": false,
  "items": [ <list_item nodes> ]
}
```

For ordered lists an additional field is present:

```json
{ "t": "list", "ordered": true, "start": 1, "items": [...] }
```

| Field | Type | Notes |
|-------|------|-------|
| `ordered` | bool | `true` for numbered lists |
| `start` | integer | Start number (ordered lists only, default 1) |
| `items` | list_item array | Each item is `{ "t": "list_item", "children": [...] }` |

**Important — tight vs loose lists:**

For tight lists (no blank lines between items), list_item `children` contain
inline nodes directly (e.g. `text`, `strong`). For loose lists (blank lines
between items), children contain `paragraph` nodes. Templates must handle both:

```jinja2
{% if node.t == "list" %}
{% if node.ordered %}<ol>{% else %}<ul>{% endif %}
  {% for item in node.items %}
  <li>
    {% for child in item.children %}
      {% if child.t == "paragraph" %}
        {% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}
      {% elif child.t == "text" %}
        {{ child.text | default(value="") | safe_html }}
      {% endif %}
    {% endfor %}
  </li>
  {% endfor %}
{% if node.ordered %}</ol>{% else %}</ul>{% endif %}
{% endif %}
```

---

#### `table`

GFM table.

```json
{
  "t": "table",
  "align": ["left", "center", "right"],
  "headers": [
    { "t": "_cell", "children": [ { "t": "text", "text": "Name" } ] }
  ],
  "rows": [
    [
      { "t": "_cell", "children": [ { "t": "text", "text": "Alice" } ] }
    ]
  ]
}
```

| Field | Type | Notes |
|-------|------|-------|
| `align` | string array | `"left"`, `"right"`, `"center"`, `"none"` — one per column |
| `headers` | cell array | Header row cells, each `{ "t": "_cell", "children": [...] }` |
| `rows` | array of cell arrays | Data rows; each row is an array of cell objects |

```jinja2
{% if node.t == "table" %}
<table>
  <thead><tr>
    {% for cell in node.headers %}
    <th>{% for c in cell.children %}{{ c.text | default(value="") }}{% endfor %}</th>
    {% endfor %}
  </tr></thead>
  <tbody>
    {% for row in node.rows %}
    <tr>
      {% for cell in row %}
      <td>{% for c in cell.children %}{{ c.text | default(value="") }}{% endfor %}</td>
      {% endfor %}
    </tr>
    {% endfor %}
  </tbody>
</table>
{% endif %}
```

---

#### `hr`

Horizontal rule (`---` / `***`).

```json
{ "t": "hr" }
```

---

#### `footnote_def`

Footnote definition block (`[^label]: body`).

```json
{
  "t": "footnote_def",
  "label": "1",
  "children": [ { "t": "paragraph", "children": [...] } ]
}
```

---

### Inline nodes

Inline nodes appear inside `children` arrays of block nodes (paragraphs,
headings, list items, table cells, etc.).

#### `text`

```json
{ "t": "text", "text": "Hello, world." }
```

#### `strong`, `em`, `del`, `sup`, `sub`

```json
{ "t": "strong", "children": [ { "t": "text", "text": "bold" } ] }
```

All share the same shape: `t` + `children` array of further inline nodes.

#### `code` (inline code span)

```json
{ "t": "code", "text": "fn main() {}" }
```

Note: uses `text` field directly, not `children`.

#### `math_inline`

```json
{ "t": "math_inline", "src": "E = mc^2" }
```

#### `link`

```json
{
  "t": "link",
  "href": "https://example.com",
  "title": "Example",
  "children": [ { "t": "text", "text": "click here" } ],
  "attrs": { "download": "true" }
}
```

`attrs` is only present when the link has a `{…}` attribute block.

#### `image`

```json
{
  "t": "image",
  "src": "./diagram.png",
  "title": "",
  "children": [ { "t": "text", "text": "alt text" } ],
  "attrs": { "width": "480", "class": "diagram" }
}
```

`attrs` is only present when the image has a `{…}` attribute block.

#### `widget`

Inline widget: `:[visible text]{name key=value}`.

```json
{
  "t": "widget",
  "name": "tooltip",
  "text": "LO",
  "attrs": { "text": "Local oscillator" }
}
```

#### `footnote_ref`

```json
{ "t": "footnote_ref", "label": "1" }
```

#### `hard_break` / `soft_break`

```json
{ "t": "hard_break" }
{ "t": "soft_break" }
```

---

## Full inline rendering pattern

```jinja2
{% macro inline(nodes) %}
  {% for child in nodes %}
    {% if child.t == "text" %}{{ child.text | safe_html }}
    {% elif child.t == "strong" %}<strong>{% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}</strong>
    {% elif child.t == "em" %}<em>{% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}</em>
    {% elif child.t == "del" %}<del>{% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}</del>
    {% elif child.t == "sup" %}<sup>{% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}</sup>
    {% elif child.t == "sub" %}<sub>{% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}</sub>
    {% elif child.t == "code" %}<code>{{ child.text | safe_html }}</code>
    {% elif child.t == "math_inline" %}<span class="math">${{ child.src }}$</span>
    {% elif child.t == "link" %}<a href="{{ child.href }}">{% for c in child.children %}{{ c.text | default(value="") }}{% endfor %}</a>
    {% elif child.t == "image" %}<img src="{{ child.src }}" alt="{{ child.children | join(sep="") }}">
    {% elif child.t == "hard_break" %}<br>
    {% elif child.t == "footnote_ref" %}<sup><a href="#fn-{{ child.label }}">[{{ child.label }}]</a></sup>
    {% else %}{{ child.text | default(value="") | safe_html }}
    {% endif %}
  {% endfor %}
{% endmacro %}
```

> **Note:** Tera macros do not support true recursion, so deeply nested inline
> markup (e.g. bold inside a link inside a list item) requires explicit depth
> handling or a simplified fallback.

---

## Typst template conventions

For Typst templates, the same context is available. Common mappings:

| HTML | Typst |
|------|-------|
| `<h1>text</h1>` | `= text` |
| `<h2>text</h2>` | `== text` |
| `**bold**` | `*bold*` |
| `_italic_` | `_italic_` |
| `` `code` `` | `` `code` `` |
| `$math$` | `$math$` |
| `$$display$$` | `$ display $` |
| `<a href="url">text</a>` | `#link("url")[text]` |
| unordered list item | `- item` |
| ordered list item | `+ item` |
| `<del>text</del>` | `#strike[text]` |
| `<sup>text</sup>` | `#super[text]` |
| `<sub>text</sub>` | `#sub[text]` |
| `<hr>` | `#line(length: 100%, stroke: 0.3pt)` |
| `<table>` | `#table(columns: N, ...)` |
