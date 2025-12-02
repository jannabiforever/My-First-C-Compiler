#[derive(Debug, Clone, PartialEq)]
pub enum KeywordType {
    Int,
    Void,
    Return,
}
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),   // [a-zA-Z_]\w*
    Constant(i32),        // [0-9]+
    Keyword(KeywordType), // int void return
    Symbol(SymbolType),   // ; , ( ) { }
}
