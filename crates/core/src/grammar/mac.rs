/// macro for tokenizing symbols & keywords
#[macro_export]
macro_rules! t {
    // Keywords
    ("int") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Int)
    };
    ("void") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Void)
    };
    ("return") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Return)
    };
    ("if") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::If)
    };
    ("else") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Else)
    };
    ("while") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::While)
    };
    ("do") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Do)
    };
    ("for") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::For)
    };
    ("break") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Break)
    };
    ("continue") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Continue)
    };

    // Grouping & Delimiters
    (";") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Semicolon)
    };
    ("(") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LParen)
    };
    (")") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::RParen)
    };
    ("{") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LBrace)
    };
    ("}") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::RBrace)
    };
    ("[") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LBracket)
    };
    ("]") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::RBracket)
    };
    (",") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Comma)
    };
    (":") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Colon)
    };
    ("?") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Question)
    };

    // Arithmetic operators
    ("+") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Plus)
    };
    ("-") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Minus)
    };
    ("*") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Star)
    };
    ("/") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Slash)
    };
    ("%") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Percent)
    };

    // Bitwise operators
    ("&") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Ampersand)
    };
    ("|") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Pipe)
    };
    ("^") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Caret)
    };
    ("~") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Tilde)
    };
    ("<<") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LeftShift)
    };
    (">>") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::RightShift)
    };

    // Logical operators
    ("!") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Not)
    };
    ("&&") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LogicalAnd)
    };
    ("||") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LogicalOr)
    };

    // Comparison operators
    ("<") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LessThan)
    };
    (">") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::GreaterThan)
    };
    ("<=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LessThanOrEqual)
    };
    (">=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::GreaterThanOrEqual)
    };
    ("==") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::EqualEqual)
    };
    ("!=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::NotEqual)
    };

    // Assignment operators
    ("=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Equal)
    };
    ("+=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::PlusEqual)
    };
    ("-=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::MinusEqual)
    };
    ("*=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::StarEqual)
    };
    ("/=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::SlashEqual)
    };
    ("%=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::PercentEqual)
    };
    ("&=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::AmpersandEqual)
    };
    ("|=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::PipeEqual)
    };
    ("^=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::CaretEqual)
    };
    ("<<=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::LeftShiftEqual)
    };
    (">>=") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::RightShiftEqual)
    };

    // Increment/Decrement
    ("++") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::PlusPlus)
    };
    ("--") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::MinusMinus)
    };

    // Member access
    (".") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Dot)
    };
    ("->") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Arrow)
    };

    // Other
    ("...") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Ellipsis)
    };
    ("#") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::Hash)
    };
    ("##") => {
        $crate::grammar::TokenType::Static($crate::grammar::StaticToken::DoubleHash)
    };
}
