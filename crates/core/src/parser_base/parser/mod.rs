mod block_stmt;
mod break_stmt;
mod continue_stmt;
mod decl_stmt;
mod expr;
mod for_stmt;
mod if_stmt;
mod null_stmt;
mod return_stmt;
mod util;
mod while_stmt;

use std::{borrow::Cow, iter::Peekable};

use crate::{
    error::IntoCompilerError,
    grammar::*,
    lexer_base::Lexer,
    parser_base::{CompilerParseError, ParseError},
    t,
};

#[derive(Clone)]
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    eof_span: Span,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let eof_span = lexer.eof_span();
        Self {
            lexer: lexer.peekable(),
            eof_span,
        }
    }

    /// Parse a complete program (one or more function definitions)
    pub fn parse(mut self) -> Result<Program<'a>, CompilerParseError> {
        let mut functions = Vec::new();

        // Parse all functions until EOF
        while self.peek_token()?.is_some() {
            functions.push(self.parse_function()?);
        }

        if functions.is_empty() {
            return Err(
                ParseError::unexpected_eof("function or variable declaration")
                    .with_span(self.eof_span),
            );
        }

        Ok(Program { functions })
    }

    /// Parse a function definition: int name(void) { ... }
    fn parse_function(&mut self) -> Result<FuncDef<'a>, CompilerParseError> {
        let (return_type, name) = self.parse_typed_identifier()?;
        let params = self.parse_function_parameters()?;
        let body = self.parse_block_statement()?;

        Ok(FuncDef {
            return_type,
            name,
            params,
            body,
        })
    }

    fn parse_function_parameters(
        &mut self,
    ) -> Result<Vec<(Type, Cow<'a, str>)>, CompilerParseError> {
        self.expect_token(t!("("))?;
        self.eat(t!("void"))?;
        let mut params = Vec::new();
        while !self.eat(t!(")"))? {
            params.push(self.parse_typed_identifier()?);
            self.eat(t!(","))?;
        }
        Ok(params)
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Statement<'a>, CompilerParseError> {
        match self.peek_token()? {
            Some(Token {
                kind: t!("break"), ..
            }) => self.parse_break_statement().map(Into::into),
            Some(Token { kind, .. }) if Type::from_token_type(&kind).is_some() => {
                self.parse_declaration_statement().map(Into::into)
            }
            Some(Token {
                kind: t!("continue"),
                ..
            }) => self.parse_continue_statement().map(Into::into),
            Some(Token { kind: t!("do"), .. }) => self.parse_do_while_statement().map(Into::into),
            Some(Token {
                kind: t!("for"), ..
            }) => self.parse_for_statement().map(Into::into),
            Some(Token { kind: t!("if"), .. }) => self.parse_if_statement().map(Into::into),
            Some(Token {
                kind: t!("return"), ..
            }) => self.parse_return_statement().map(Into::into),
            Some(Token {
                kind: t!("while"), ..
            }) => self.parse_while_statement().map(Into::into),
            Some(Token { kind: t!(";"), .. }) => self.parse_null_statement().map(Into::into),
            Some(Token { kind: t!("{"), .. }) => self.parse_block_statement().map(Into::into),
            Some(token) => {
                let expr = self.parse_expression().map_err(|_| {
                    ParseError::unexpected_token("statement", &token.kind).with_span(token.span)
                })?;
                self.expect_token(t!(";"))?;
                Ok(ExprStmt { expr }.into())
            }
            None => Err(ParseError::unexpected_eof("statement").with_span(self.eof_span)),
        }
    }

    /// Parses an identifier token
    pub(super) fn parse_identifier(&mut self) -> Result<Cow<'a, str>, CompilerParseError> {
        self.expect_with(
            |tt| match tt {
                TokenType::Identifier(name) => Some(name.clone()),
                _ => None,
            },
            "identifier",
        )
    }

    /// Parses a type token and an identifier token
    pub(super) fn parse_typed_identifier(
        &mut self,
    ) -> Result<(Type, Cow<'a, str>), CompilerParseError> {
        let ty = self.expect_with(Type::from_token_type, "type")?;
        let name = self.parse_identifier()?;
        Ok((ty, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer_base::Lexer;

    fn parse_program(input: &str) -> Result<Program<'_>, CompilerParseError> {
        let lexer = Lexer::new(input);
        Parser::new(lexer).parse()
    }

    // === Integration Tests: Smoke Tests ===

    #[test]
    fn test_parse_simple_function() {
        let input = "int main(void) { return 0; }";
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].return_type, Type::Int);
        assert_eq!(program.functions[0].name, "main");
        assert_eq!(program.functions[0].body.statements.len(), 1);
    }

    #[test]
    fn test_parse_multiple_functions() {
        let input = r#"
            int foo(void) { return 1; }
            int bar(void) { return 2; }
            void baz() { return 0; }
        "#;
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions.len(), 3);
        assert_eq!(program.functions[0].name, "foo");
        assert_eq!(program.functions[0].return_type, Type::Int);
        assert_eq!(program.functions[1].name, "bar");
        assert_eq!(program.functions[1].return_type, Type::Int);
        assert_eq!(program.functions[2].name, "baz");
        assert_eq!(program.functions[2].return_type, Type::Void);
    }

    // === Integration Tests: Feature Integration ===

    #[test]
    fn test_parse_nested_control_flow() {
        let input = r#"
            int main(void) {
                if (x > 0) {
                    while (y < 10) {
                        for (i; i < 5; i) {
                            x;
                        }
                    }
                }
                return 0;
            }
        "#;
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions[0].body.statements.len(), 2);
    }

    #[test]
    fn test_parse_complex_expression_in_statement() {
        let input = "int main(void) { return (a + b) * (c - d) / e; }";
        let result = parse_program(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_multiple_statements_in_block() {
        let input = r#"
            int main(void) {
                x;
                y;
                z;
                return 0;
            }
        "#;
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions[0].body.statements.len(), 4);
    }

    #[test]
    fn test_parse_all_statement_types() {
        let input = r#"
            int main(void) {
                if (1) { x; }
                while (1) { x; }
                do { x; } while (1);
                for (;;) { x; }
                break;
                continue;
                return 0;
            }
        "#;
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions[0].body.statements.len(), 7);
    }

    #[test]
    fn test_parse_deeply_nested_blocks() {
        let input = r#"
            int main(void) {
                {
                    {
                        {
                            return 0;
                        }
                    }
                }
            }
        "#;
        let result = parse_program(input);
        assert!(result.is_ok());
    }

    // === Integration Tests: Critical Error Cases ===

    #[test]
    fn test_error_empty_input() {
        let input = "";
        let result = parse_program(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err.error, ParseError::UnexpectedEof { .. }));
    }

    #[test]
    fn test_error_incomplete_function() {
        let input = "int main(void)";
        let result = parse_program(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_no_function_definition() {
        let input = "return 0;";
        let result = parse_program(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_missing_function_body() {
        let input = "int main(void);";
        let result = parse_program(input);
        assert!(result.is_err());
    }

    // === Edge Cases ===

    #[test]
    fn test_parse_minimal_valid_program() {
        let input = "int f() { x; }";
        let result = parse_program(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_excessive_whitespace() {
        let input = "int   main  (  void  )  {  return   0  ;  }";
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions[0].return_type, Type::Int);
    }

    #[test]
    fn test_parse_empty_block() {
        let input = "int main(void) { }";
        let result = parse_program(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.functions[0].body.statements.len(), 0);
    }
}
