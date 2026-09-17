//! Rust syntax highlighting for `recz-cli`'s generated-code output, built on
//! the lightweight `synoptic` crate's bundled Rust ruleset
//! (`synoptic::from_extension("rs", ..)`) rather than a full
//! syntax-highlighting engine like `syntect`/`bat`.
//!
//! Note that synoptic's bundled ruleset lumps keywords and primitive types
//! (and even a few std types like `String`/`Vec`/`Option`) into a single
//! "keyword" category, so those aren't distinguished from one another here.

use owo_colors::{OwoColorize, Stream};
use std::fmt::Write as _;
use synoptic::TokOpt;

/// Colors the given Rust source for terminal output. Falls back to plain
/// text when the output stream doesn't support color.
pub fn highlight(code: &str) -> String {
    let mut h = synoptic::from_extension("rs", 4).expect("synoptic bundles a Rust highlighter");
    let lines: Vec<String> = code.split('\n').map(str::to_owned).collect();
    h.run(&lines);

    let mut out = String::with_capacity(code.len() + code.len() / 4);
    for (y, line) in lines.iter().enumerate() {
        if y > 0 {
            out.push('\n');
        }
        for token in h.line(y, line) {
            match token {
                TokOpt::Some(text, kind) => push(&mut out, &text, &kind),
                TokOpt::None(text) => out.push_str(&text),
            }
        }
    }
    out
}

// Ayu Dark palette (https://github.com/ayu-theme/ayu-colors), applied via
// true color so the terminal output tracks the theme's actual hues rather
// than an approximation from the 16 ANSI colors.
mod ayu {
    pub const COMMENT: (u8, u8, u8) = (0x5C, 0x67, 0x73);
    pub const STRING: (u8, u8, u8) = (0xAA, 0xD9, 0x4C);
    pub const CONSTANT: (u8, u8, u8) = (0xD2, 0xA6, 0xFF);
    pub const KEYWORD: (u8, u8, u8) = (0xFF, 0x8F, 0x40);
    pub const SPECIAL: (u8, u8, u8) = (0xE6, 0xB6, 0x73);
    pub const ENTITY: (u8, u8, u8) = (0x59, 0xC2, 0xFF);
    pub const FUNC: (u8, u8, u8) = (0xFF, 0xB4, 0x54);
    pub const OPERATOR: (u8, u8, u8) = (0xF2, 0x96, 0x68);
}

fn push(out: &mut String, text: &str, kind: &str) {
    match kind {
        "comment" => color(out, text, |s| {
            let (r, g, b) = ayu::COMMENT;
            s.truecolor(r, g, b).italic().to_string()
        }),
        "string" | "character" => color(out, text, |s| {
            let (r, g, b) = ayu::STRING;
            s.truecolor(r, g, b).to_string()
        }),
        "digit" | "boolean" => color(out, text, |s| {
            let (r, g, b) = ayu::CONSTANT;
            s.truecolor(r, g, b).to_string()
        }),
        "keyword" => color(out, text, |s| {
            let (r, g, b) = ayu::KEYWORD;
            s.truecolor(r, g, b).to_string()
        }),
        "attribute" => color(out, text, |s| {
            let (r, g, b) = ayu::SPECIAL;
            s.truecolor(r, g, b).to_string()
        }),
        "namespace" | "struct" => color(out, text, |s| {
            let (r, g, b) = ayu::ENTITY;
            s.truecolor(r, g, b).to_string()
        }),
        "macro" | "function" => color(out, text, |s| {
            let (r, g, b) = ayu::FUNC;
            s.truecolor(r, g, b).to_string()
        }),
        "operator" | "reference" => color(out, text, |s| {
            let (r, g, b) = ayu::OPERATOR;
            s.truecolor(r, g, b).to_string()
        }),
        _ => out.push_str(text),
    }
}

fn color<D: std::fmt::Display>(out: &mut String, text: &str, apply: impl Fn(&&str) -> D) {
    write!(out, "{}", text.if_supports_color(Stream::Stdout, apply)).unwrap();
}
