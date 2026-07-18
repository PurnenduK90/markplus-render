//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use serde_json::Value;
use std::fmt::Write;

fn safe_html(text: &str) -> String {
    html_escape::encode_text(text).into_owned()
}

pub fn render_html_inline(children: &[Value], out: &mut String) {
    for child in children {
        let t = child.get("t").and_then(|v| v.as_str()).unwrap_or("");
        match t {
            "text" => {
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "{}", safe_html(text)).unwrap();
            }
            "strong" => {
                out.push_str("<strong>");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_html_inline(ch, out);
                }
                out.push_str("</strong>");
            }
            "em" => {
                out.push_str("<em>");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_html_inline(ch, out);
                }
                out.push_str("</em>");
            }
            "code" => {
                out.push_str("<code>");
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "{}", safe_html(text)).unwrap();
                out.push_str("</code>");
            }
            "link" => {
                let href = child.get("href").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "<a href=\"{}\">", safe_html(href)).unwrap();
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_html_inline(ch, out);
                }
                out.push_str("</a>");
            }
            "image" => {
                let src = child.get("src").and_then(|v| v.as_str()).unwrap_or("");
                let alt = child.get("alt").and_then(|v| v.as_str()).unwrap_or("");
                write!(
                    out,
                    "<img src=\"{}\" alt=\"{}\">",
                    safe_html(src),
                    safe_html(alt)
                )
                .unwrap();
            }
            "math_inline" => {
                let src = child.get("src").and_then(|v| v.as_str()).unwrap_or("");
                write!(
                    out,
                    "<span class=\"math-inline\" data-math=\"{src}\">${src}$</span>",
                    src = safe_html(src)
                )
                .unwrap();
            }
            "hard_break" => {
                out.push_str("<br>");
            }
            "del" => {
                out.push_str("<del>");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_html_inline(ch, out);
                }
                out.push_str("</del>");
            }
            "sup" => {
                out.push_str("<sup>");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_html_inline(ch, out);
                }
                out.push_str("</sup>");
            }
            "sub" => {
                out.push_str("<sub>");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_html_inline(ch, out);
                }
                out.push_str("</sub>");
            }
            "footnote_ref" => {
                let label = child.get("label").and_then(|v| v.as_str()).unwrap_or("");
                write!(
                    out,
                    "<sup><a href=\"#fn-{}\" id=\"fnref-{}\">[{}]</a></sup>",
                    label, label, label
                )
                .unwrap();
            }
            "widget" => {
                let name = child.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                write!(
                    out,
                    "<span class=\"widget {}\">{}</span>",
                    safe_html(name),
                    safe_html(text)
                )
                .unwrap();
            }
            _ => {
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "{}", safe_html(text)).unwrap();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_typst_inline(children: &[Value], out: &mut String) {
    for child in children {
        let t = child.get("t").and_then(|v| v.as_str()).unwrap_or("");
        match t {
            "text" => {
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(text); // TODO typst escaping
            }
            "strong" => {
                out.push('*');
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_typst_inline(ch, out);
                }
                out.push('*');
            }
            "em" => {
                out.push('_');
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_typst_inline(ch, out);
                }
                out.push('_');
            }
            "code" => {
                out.push('`');
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(text);
                out.push('`');
            }
            "link" => {
                let href = child.get("href").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str("#link(\"");
                out.push_str(href);
                out.push_str("\")[");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_typst_inline(ch, out);
                }
                out.push(']');
            }
            "image" => {
                let src = child.get("src").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "#image(\"{}\")", src).unwrap();
            }
            "math_inline" => {
                let src = child.get("src").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "${}$", src).unwrap();
            }
            "hard_break" => {
                out.push_str("\\ \n");
            }
            "del" => {
                out.push_str("#strike[");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_typst_inline(ch, out);
                }
                out.push(']');
            }
            "sup" => {
                out.push_str("#super[");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_typst_inline(ch, out);
                }
                out.push(']');
            }
            "sub" => {
                out.push_str("#sub[");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_typst_inline(ch, out);
                }
                out.push(']');
            }
            "footnote_ref" => {
                let label = child.get("label").and_then(|v| v.as_str()).unwrap_or("");
                write!(out, "#footnote(<fn-{}>)", label).unwrap();
            }
            "widget" => {
                let name = child.get("name").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(plugin) =
                    crate::plugin::get_plugin(crate::plugin::PluginType::Widget, name)
                {
                    plugin.render_typst(child, out);
                } else {
                    let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                    // Simple inline box for typst
                    out.push_str(&format!(
                        "#box(fill: luma(240), inset: 2pt, radius: 2pt)[{}]",
                        text
                    ));
                }
            }
            _ => {
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(text);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_markdown_inline(children: &[Value], out: &mut String) {
    for child in children {
        let t = child.get("t").and_then(|v| v.as_str()).unwrap_or("");
        match t {
            "text" => {
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(text);
            }
            "strong" => {
                out.push_str("**");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_markdown_inline(ch, out);
                }
                out.push_str("**");
            }
            "em" => {
                out.push('*');
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_markdown_inline(ch, out);
                }
                out.push('*');
            }
            "code" => {
                out.push('`');
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(text);
                out.push('`');
            }
            "link" => {
                let href = child.get("href").and_then(|v| v.as_str()).unwrap_or("");
                out.push('[');
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_markdown_inline(ch, out);
                }
                out.push_str(&format!("]({})", href));
            }
            "image" => {
                let src = child.get("src").and_then(|v| v.as_str()).unwrap_or("");
                let alt = child.get("alt").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(&format!("![{}]({})", alt, src));
            }
            "math_inline" => {
                let src = child.get("src").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(&format!("${}$", src));
            }
            "hard_break" => {
                out.push_str("  \n");
            }
            "del" => {
                out.push_str("~~");
                if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
                    render_markdown_inline(ch, out);
                }
                out.push_str("~~");
            }
            "footnote_ref" => {
                let label = child.get("label").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(&format!("[^{}]", label));
            }
            "widget" => {
                let name = child.get("name").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(plugin) =
                    crate::plugin::get_plugin(crate::plugin::PluginType::Widget, name)
                {
                    plugin.render_html(child, out);
                } else {
                    let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                    out.push_str(&format!("<span class=\"widget {}\">{}</span>", name, text));
                }
            }
            _ => {
                let text = child.get("text").and_then(|v| v.as_str()).unwrap_or("");
                out.push_str(text);
            }
        }
    }
}

pub fn collect_text(children: &[Value]) -> String {
    let mut out = String::new();
    for child in children {
        if let Some(text) = child.get("text").and_then(|v| v.as_str()) {
            out.push_str(text);
        } else if let Some(ch) = child.get("children").and_then(|v| v.as_array()) {
            out.push_str(&collect_text(ch));
        }
    }
    out
}

pub fn slugify_text(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
