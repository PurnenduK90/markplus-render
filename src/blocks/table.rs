//    Copyright [2026] [Purnendu Kumar]
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0

use crate::blocks::{RenderBlock, inline};
use serde_json::Value;
use std::fmt::Write;

pub struct TableRenderer;

impl RenderBlock for TableRenderer {
    fn render_html(node: &Value, out: &mut String) {
        out.push_str("<table>\n");

        let headers = node
            .get("headers")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        if !headers.is_empty() {
            out.push_str("<thead><tr>\n");
            for cell in headers {
                out.push_str("<th>");
                let children = cell
                    .get("children")
                    .and_then(|v| v.as_array())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                inline::render_html_inline(children, out);
                out.push_str("</th>\n");
            }
            out.push_str("</tr></thead>\n");
        }

        out.push_str("<tbody>\n");
        let rows = node
            .get("rows")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        for row in rows {
            out.push_str("<tr>\n");
            let cells = row.as_array().map(|v| v.as_slice()).unwrap_or(&[]);
            for cell in cells {
                out.push_str("<td>");
                let children = cell
                    .get("children")
                    .and_then(|v| v.as_array())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                inline::render_html_inline(children, out);
                out.push_str("</td>\n");
            }
            out.push_str("</tr>\n");
        }
        out.push_str("</tbody>\n");
        out.push_str("</table>\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_typst(node: &Value, out: &mut String) {
        // Basic typst table
        let headers = node
            .get("headers")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let cols = std::cmp::max(1, headers.len());

        write!(out, "#table(\n  columns: {},\n", cols).unwrap();

        for cell in headers {
            out.push_str("  [");
            let children = cell
                .get("children")
                .and_then(|v| v.as_array())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            inline::render_typst_inline(children, out);
            out.push_str("],\n");
        }

        let rows = node
            .get("rows")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        for row in rows {
            let cells = row.as_array().map(|v| v.as_slice()).unwrap_or(&[]);
            for cell in cells {
                out.push_str("  [");
                let children = cell
                    .get("children")
                    .and_then(|v| v.as_array())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                inline::render_typst_inline(children, out);
                out.push_str("],\n");
            }
        }
        out.push_str(")\n\n");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_markdown(node: &Value, out: &mut String) {
        let headers = node
            .get("headers")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        // Render headers
        if !headers.is_empty() {
            out.push_str("| ");
            for cell in headers {
                let children = cell
                    .get("children")
                    .and_then(|v| v.as_array())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                let mut inner = String::new();
                inline::render_markdown_inline(children, &mut inner);
                out.push_str(&inner.replace('\n', " "));
                out.push_str(" | ");
            }
            out.push('\n');

            // Render divider
            out.push('|');
            for _ in 0..headers.len() {
                out.push_str("---|");
            }
            out.push('\n');
        }

        // Render rows
        let rows = node
            .get("rows")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        for row in rows {
            out.push_str("| ");
            let cells = row.as_array().map(|v| v.as_slice()).unwrap_or(&[]);
            for cell in cells {
                let children = cell
                    .get("children")
                    .and_then(|v| v.as_array())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                let mut inner = String::new();
                inline::render_markdown_inline(children, &mut inner);
                out.push_str(&inner.replace('\n', " "));
                out.push_str(" | ");
            }
            out.push('\n');
        }
        out.push('\n');
    }
}
