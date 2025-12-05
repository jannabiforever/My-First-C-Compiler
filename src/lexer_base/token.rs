use std::borrow::Cow;

/// Static tokens (keywords and symbols with fixed string representation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticToken {
    // Keywords
    Int,
    Void,
    Return,

    // Symbols
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

impl StaticToken {
    pub const fn as_str(&self) -> &'static str {
        match self {
            StaticToken::Int => "int",
            StaticToken::Void => "void",
            StaticToken::Return => "return",
            StaticToken::Semicolon => ";",
            StaticToken::LParen => "(",
            StaticToken::RParen => ")",
            StaticToken::LBrace => "{",
            StaticToken::RBrace => "}",
        }
    }
}

pub const ALL_KEYWORDS: &[StaticToken] =
    &[StaticToken::Int, StaticToken::Void, StaticToken::Return];

/// Span represents the location information of a token in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// byte offset in source (inclusive)
    pub start: usize,
    /// byte offset in source (exclusive)
    pub end: usize,
    /// 1-based line number
    pub line: usize,
    /// 1-based column number (at start of token)
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

/// Token represents a lexical token with its type and location
#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenType<'a>,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

/// For testing convenience, compare only the kind, ignoring span
impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType<'a> {
    // Identifiers and literals
    Identifier(Cow<'a, str>), // [a-zA-Z_]\w*
    Constant(i32),            // [0-9]+

    // Static tokens (keywords and symbols)
    Static(StaticToken),
}

impl<'a> TokenType<'a> {
    pub fn identifier(input: &'a str) -> Self {
        TokenType::Identifier(Cow::Borrowed(input))
    }

    pub fn ascii_length(&self) -> usize {
        match self {
            TokenType::Identifier(s) => s.len(),
            TokenType::Constant(n) => n.to_string().len(),
            TokenType::Static(st) => st.as_str().len(),
        }
    }
}

impl<'a> std::fmt::Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Identifier(name) => write!(f, "identifier '{}'", name),
            TokenType::Constant(value) => write!(f, "constant {}", value),
            TokenType::Static(st) => write!(f, "'{}'", st.as_str()),
        }
    }
}
