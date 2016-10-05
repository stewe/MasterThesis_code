static RUST_KEYWORDS: &'static [&'static str] = &[
    "as",
    "break",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "static",
    "self",
    "Self",
    "struct",
    "super",
    "true",
    "trait",
    "type",
    "unsafe",
    "use",
    "while",
    "continue",
    "box",
    "const",
    "where",
    "virtual",
    "proc",
    "alignof",
    "become",
    "offsetof",
    "priv",
    "pure",
    "sizeof",
    "typeof",
    "unsized",
    "yield",
    "do",
    "abstract",
    "final",
    "override",
];

pub fn is_rust_keyword(ident: &str) -> bool {
    RUST_KEYWORDS.contains(&ident)
}
