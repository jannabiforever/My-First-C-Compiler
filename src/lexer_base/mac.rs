/// macro for tokenizing symbols & keywords
#[macro_export]
macro_rules! t {
    (";") => {
        $crate::lexer_base::token::TokenType::Static(
            $crate::lexer_base::token::StaticToken::Semicolon,
        )
    };
    ("{") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::LBrace)
    };
    ("}") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::RBrace)
    };
    ("(") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::LParen)
    };
    (")") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::RParen)
    };
    ("int") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::Int)
    };
    ("void") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::Void)
    };
    ("return") => {
        $crate::lexer_base::token::TokenType::Static($crate::lexer_base::token::StaticToken::Return)
    };
}
