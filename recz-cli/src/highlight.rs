//! A minimal, dependency-free (beyond `owo-colors`) Rust syntax highlighter.
//!
//! It's a hand-rolled lexer over already-formatted source text (as produced
//! by `prettyplease`), not a general-purpose Rust parser: it only needs to
//! classify tokens well enough for terminal coloring, and preserves every
//! byte of whitespace/formatting verbatim since it never rebuilds the layout.

use owo_colors::{OwoColorize, Stream};
use std::fmt::Write as _;

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "Self", "self", "static", "struct", "super", "trait", "type", "union", "unsafe",
    "use", "where", "while", "yield",
];

/// Keyword-shaped literals, e.g. booleans. Colored like other literals
/// (numbers, strings) rather than like control-flow/declaration keywords.
const LITERAL_KEYWORDS: &[&str] = &["true", "false"];

/// Primitive types, which are lowercase and so wouldn't otherwise be
/// distinguished from plain identifiers or user-defined types.
const BUILTIN_TYPES: &[&str] = &[
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "str", "u8", "u16",
    "u32", "u64", "u128", "usize",
];

/// Colors the given Rust source for terminal output. Falls back to plain
/// text when the output stream doesn't support color.
pub fn highlight(code: &str) -> String {
    let bytes = code.as_bytes();
    let mut out = String::with_capacity(code.len() + code.len() / 4);
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                let start = i;
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                push(&mut out, &code[start..i], |s| s.bright_black().to_string());
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                let start = i;
                i += 2;
                while i < bytes.len() && !(bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/')) {
                    i += 1;
                }
                i = (i + 2).min(bytes.len());
                push(&mut out, &code[start..i], |s| s.bright_black().to_string());
            }
            b'"' => {
                let start = i;
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    i += if bytes[i] == b'\\' { 2 } else { 1 };
                }
                i = (i + 1).min(bytes.len());
                push(&mut out, &code[start..i], |s| s.green().to_string());
            }
            b'\'' => {
                let start = i;
                i += 1;
                if bytes.get(i) == Some(&b'\\') {
                    // escape sequence: consume until the closing quote.
                    i += 1;
                    while i < bytes.len() && bytes[i] != b'\'' {
                        i += 1;
                    }
                    i = (i + 1).min(bytes.len());
                    push(&mut out, &code[start..i], |s| s.green().to_string());
                } else if bytes.get(i + 1) == Some(&b'\'') {
                    // a single-character literal, e.g. 'x'.
                    i += 2;
                    push(&mut out, &code[start..i], |s| s.green().to_string());
                } else {
                    // a lifetime, e.g. 'a, 'static, '_.
                    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_')
                    {
                        i += 1;
                    }
                    push(&mut out, &code[start..i], |s| s.magenta().to_string());
                }
            }
            b'0'..=b'9' => {
                let start = i;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
                {
                    i += 1;
                }
                push(&mut out, &code[start..i], |s| s.green().to_string());
            }
            b'_' | b'a'..=b'z' | b'A'..=b'Z' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let word = &code[start..i];
                if KEYWORDS.contains(&word) {
                    push(&mut out, word, |s| s.blue().to_string());
                } else if LITERAL_KEYWORDS.contains(&word) {
                    push(&mut out, word, |s| s.green().to_string());
                } else if BUILTIN_TYPES.contains(&word) {
                    push(&mut out, word, |s| s.bright_cyan().to_string());
                } else if bytes.get(i) == Some(&b'!') {
                    push(&mut out, word, |s| s.cyan().to_string());
                } else if word.chars().next().is_some_and(char::is_uppercase) {
                    push(&mut out, word, |s| s.cyan().to_string());
                } else {
                    push(&mut out, word, |s| s.yellow().to_string());
                }
            }
            _ => {
                out.push(c as char);
                i += 1;
            }
        }
    }

    out
}

fn push<D: std::fmt::Display>(out: &mut String, text: &str, apply: impl Fn(&&str) -> D) {
    write!(out, "{}", text.if_supports_color(Stream::Stdout, apply)).unwrap();
}
