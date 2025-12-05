/// macro for tokenizing symbols & keywords
#[macro_export]
macro_rules! t {
    ("+") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Plus)
    };
    ("-") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Minus)
    };
    ("*") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Star)
    };
    ("/") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Slash)
    };
    ("!") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Not)
    };
    (";") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Semicolon)
    };
    ("{") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::LBrace)
    };
    ("}") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::RBrace)
    };
    ("(") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::LParen)
    };
    (")") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::RParen)
    };
    ("int") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Int)
    };
    ("void") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Void)
    };
    ("return") => {
        $crate::lexer_base::TokenType::Static($crate::lexer_base::StaticToken::Return)
    };
}
