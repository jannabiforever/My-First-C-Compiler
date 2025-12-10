use crate::{
    grammar::BreakStmt,
    parser_base::{Parser, error::ParseResult},
    t,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_break_statement(&mut self) -> ParseResult<BreakStmt<'a>> {
        self.expect_token(t!("break"))?;
        self.expect_token(t!(";"))?;
        Ok(BreakStmt::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer_base::Lexer;

    fn parse_break(input: &str) -> ParseResult<BreakStmt<'_>> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_break_statement()
    }

    #[test]
    fn test_parse_break() {
        let result = parse_break("break;");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_break_error_missing_semicolon() {
        let result = parse_break("break");
        assert!(result.is_err());
    }
}
