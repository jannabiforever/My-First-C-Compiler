use crate::{
    grammar::*,
    parser_base::{CompilerParseError, Parser},
    t,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_declaration_statement(
        &mut self,
    ) -> Result<DeclarationStmt<'a>, CompilerParseError> {
        let var_type = self.expect_type()?;
        let name = self.expect_identifier()?;
        let initializer = self
            .eat(t!("="))?
            .then(|| self.parse_expression())
            .transpose()?;
        self.expect(t!(";"))?;

        Ok(DeclarationStmt {
            var_type,
            name,
            initializer,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grammar::Expression, lexer_base::Lexer};

    fn parse_declaration(input: &str) -> Result<DeclarationStmt<'_>, CompilerParseError> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_declaration_statement()
    }

    #[test]
    fn test_parse_declaration_without_initializer() {
        let result = parse_declaration("int x;");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        assert_eq!(stmt.var_type, Type::Int);
        assert_eq!(stmt.name.as_ref(), "x");
        assert!(stmt.initializer.is_none());
    }

    #[test]
    fn test_parse_declaration_with_constant_initializer() {
        let result = parse_declaration("int x = 42;");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        assert_eq!(stmt.var_type, Type::Int);
        assert_eq!(stmt.name.as_ref(), "x");
        assert!(matches!(stmt.initializer, Some(Expression::Constant(42))));
    }

    #[test]
    fn test_parse_declaration_error_missing_semicolon() {
        let result = parse_declaration("int x");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_declaration_error_missing_identifier() {
        let result = parse_declaration("int;");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_declaration_error_missing_type() {
        let result = parse_declaration("x = 5;");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_declaration_error_missing_expression_after_equal() {
        let result = parse_declaration("int x = ;");
        assert!(result.is_err());
    }
}
