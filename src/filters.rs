//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Custom Tera filters for MarkPlus templates.
//!
//! Registered automatically by [`crate::engine::RenderEngineBuilder`].
//!
//! | Filter      | Input  | Args              | Output                        |
//! |-------------|--------|-------------------|-------------------------------|
//! | `slugify`   | string | —                 | lowercase hyphen-slug         |
//! | `date_fmt`  | string | `format` (opt.)   | formatted date string         |
//! | `safe_html` | string | —                 | HTML-entity-escaped string    |

use std::collections::HashMap;
use tera::{Tera, Value, Error as TeraError};

// ---------------------------------------------------------------------------
// Filter: slugify
// ---------------------------------------------------------------------------

fn filter_slugify(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = value.as_str().ok_or_else(|| TeraError::msg("slugify: expected string"))?;
    let slug = s
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    Ok(Value::String(slug))
}

// ---------------------------------------------------------------------------
// Filter: date_fmt
// ---------------------------------------------------------------------------

fn filter_date_fmt(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = value.as_str().ok_or_else(|| TeraError::msg("date_fmt: expected string"))?;
    let fmt = args
        .get("format")
        .and_then(Value::as_str)
        .unwrap_or("%B %d, %Y");

    // Attempt ISO 8601 parse (YYYY-MM-DD)
    if s.len() >= 10 {
        let parts: Vec<&str> = s[..10].split('-').collect();
        if parts.len() == 3 {
            if let (Ok(y), Ok(m), Ok(d)) = (
                parts[0].parse::<i32>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                let formatted = fmt
                    .replace("%Y", &format!("{:04}", y))
                    .replace("%m", &format!("{:02}", m))
                    .replace("%d", &format!("{:02}", d))
                    .replace("%B", month_name(m));
                return Ok(Value::String(formatted));
            }
        }
    }
    // Pass-through if unparseable
    Ok(Value::String(s.to_owned()))
}

fn month_name(m: u8) -> &'static str {
    match m {
        1 => "January", 2 => "February", 3 => "March",    4 => "April",
        5 => "May",     6 => "June",      7 => "July",     8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "",
    }
}

// ---------------------------------------------------------------------------
// Filter: safe_html
// ---------------------------------------------------------------------------

fn filter_safe_html(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = value.as_str().ok_or_else(|| TeraError::msg("safe_html: expected string"))?;
    let escaped = s
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");
    Ok(Value::String(escaped))
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

/// Register all custom filters on a Tera instance.
/// Called automatically by [`crate::engine::RenderEngineBuilder`].
pub fn register_filters(tera: &mut Tera) {
    tera.register_filter("slugify",   filter_slugify);
    tera.register_filter("date_fmt",  filter_date_fmt);
    tera.register_filter("safe_html", filter_safe_html);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn run(filter: fn(&Value, &HashMap<String, Value>) -> tera::Result<Value>, val: Value, args: HashMap<String, Value>) -> String {
        filter(&val, &args).unwrap().as_str().unwrap().to_owned()
    }

    #[test]
    fn slugify_basic() {
        assert_eq!(run(filter_slugify, json!("Hello World"), HashMap::new()), "hello-world");
    }

    #[test]
    fn slugify_special_chars() {
        assert_eq!(run(filter_slugify, json!("Rust & Typst (2026)"), HashMap::new()), "rust-typst-2026");
    }

    #[test]
    fn date_fmt_default() {
        let result = run(filter_date_fmt, json!("2026-06-07"), HashMap::new());
        assert_eq!(result, "June 07, 2026");
    }

    #[test]
    fn date_fmt_custom() {
        let mut args = HashMap::new();
        args.insert("format".into(), json!("%Y-%m-%d"));
        let result = run(filter_date_fmt, json!("2026-06-07"), args);
        assert_eq!(result, "2026-06-07");
    }

    #[test]
    fn date_fmt_passthrough_on_invalid() {
        let result = run(filter_date_fmt, json!("not-a-date"), HashMap::new());
        assert_eq!(result, "not-a-date");
    }

    #[test]
    fn safe_html_escapes() {
        let result = run(filter_safe_html, json!("<script>alert(\"xss\")</script>"), HashMap::new());
        assert_eq!(result, "&lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;");
    }
}
