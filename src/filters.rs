//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

//! Custom MiniJinja filters for MarkPlus templates.
//!
//! Registered automatically by [`crate::engine::RenderEngineBuilder`].

use minijinja::{Environment, Error as JinjaError, State};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Filter: slugify
// ---------------------------------------------------------------------------

fn filter_slugify(_state: &State, value: String) -> Result<String, JinjaError> {
    let slug = value
        .chars()
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
        .join("-");
    Ok(slug)
}

// ---------------------------------------------------------------------------
// Filter: date_fmt
// ---------------------------------------------------------------------------

fn filter_date_fmt(
    _state: &State,
    value: String,
    kwargs: minijinja::value::Value,
) -> Result<String, JinjaError> {
    let mut fmt = "%B %d, %Y".to_string();

    // minijinja kwargs are passed as an object
    if let Some(args) = kwargs.downcast_object_ref::<HashMap<String, String>>() {
        if let Some(f) = args.get("format") {
            fmt = f.clone();
        }
    } else if let Some(f) = kwargs.as_str() {
        // sometimes passed as a positional arg
        fmt = f.to_string();
    }

    // Attempt ISO 8601 parse (YYYY-MM-DD)
    if value.len() >= 10 {
        let parts: Vec<&str> = value[..10].split('-').collect();
        if parts.len() == 3
            && let (Ok(y), Ok(m), Ok(d)) = (
                parts[0].parse::<i32>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                let formatted = fmt
                    .replace("%Y", &format!("{:04}", y))
                    .replace("%m", &format!("{:02}", m))
                    .replace("%d", &format!("{:02}", d))
                    .replace("%B", month_name(m));
                return Ok(formatted);
            }
    }
    // Pass-through if unparseable
    Ok(value)
}

fn month_name(m: u8) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

// ---------------------------------------------------------------------------
// Filter: safe_html
// ---------------------------------------------------------------------------

fn filter_safe_html(_state: &State, value: String) -> Result<String, JinjaError> {
    let escaped = value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");
    Ok(escaped)
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

pub fn register_filters(env: &mut Environment) {
    env.add_filter("slugify", filter_slugify);
    env.add_filter("date_fmt", filter_date_fmt);
    env.add_filter("safe_html", filter_safe_html);
}
