use std::borrow::Cow;
use std::iter::Peekable;

use crate::{
    error::{CompilerError, IntoCompilerError},
    lexer_base::{CompilerLexError, LexError, Lexer, Span, Token, TokenType},
    parser_base::{CompilerParseError, Expression, FuncDef, ParseError, Program, Statement},
    t,
};

#[derive(Clone)]
pub struct Parser<'a, I>
where
    I: Iterator<Item = Result<Token<'a>, CompilerLexError>>,
{
    lexer: Peekable<I>,
    eof_span: Span,
}

impl<'a> Parser<'a, Lexer<'a>> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let eof_span = lexer.eof_span();
        Self {
            lexer: lexer.peekable(),
            eof_span: eof_span,
        }
    }
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Result<Token<'a>, CompilerLexError>>,
{
    pub fn parse(mut self) -> Result<Program<'a>, CompilerParseError> {
        let fd = self.parse_function_def()?;
        let remaining_tokens = self
            .lexer
            .collect::<Result<Vec<Token<'a>>, CompilerError<LexError>>>()
            .map_err(|e| e.convert_error::<ParseError>())?;
        if let [first_token, ..] = remaining_tokens.as_slice() {
            Err(ParseError::UnexpectedTokensAfterFunction {
                tokens: first_token.kind.to_string(),
            }
            .with_span(first_token.span))
        } else {
            Ok(Program::Program(fd))
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, CompilerParseError> {
        match self
            .lexer
            .next()
            .transpose()
            .map_err(|e| e.convert_error::<ParseError>())?
        {
            Some(Token {
                kind: TokenType::Constant(value),
                ..
            }) => Ok(Expression::Constant(value)),
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "expression starter token".to_string(),
                found: Some(format!("{:?}", token.kind)),
            }
            .with_span(token.span)),
            None => Err(ParseError::ExpectedExpression.with_span(self.eof_span)),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, CompilerParseError> {
        self.expect(t!("return"))?;
        let inner = self.parse_expression()?;
        self.expect(t!(";"))?;
        Ok(Statement::Return(inner))
    }

    pub fn parse_function_def(&mut self) -> Result<FuncDef<'a>, CompilerParseError> {
        self.expect(t!("int"))?;
        let func_name = self.expect_identifier()?;
        self.expect(t!("("))?;
        self.expect_optional(t!("void"))?;
        self.expect(t!(")"))?;
        self.expect(t!("{"))?;
        let body = self.parse_statement()?;
        self.expect(t!("}"))?;

        Ok(FuncDef::Fn(func_name, body))
    }

    fn expect(&mut self, expected: TokenType<'static>) -> Result<(), CompilerParseError> {
        match self
            .lexer
            .next()
            .transpose()
            .map_err(|e| e.convert_error())?
        {
            Some(token) if token.kind == expected => Ok(()),
            Some(unexpected) => Err(
                ParseError::unexpected_token(expected, Some(unexpected.kind))
                    .with_span(unexpected.span),
            ),
            None => Err(ParseError::unexpected_end_of_input(expected).with_span(self.eof_span)),
        }
    }

    fn expect_identifier(&mut self) -> Result<Cow<'a, str>, CompilerParseError> {
        match self
            .lexer
            .next()
            .transpose()
            .map_err(|e| e.convert_error::<ParseError>())?
        {
            Some(token) if matches!(token.kind, TokenType::Identifier(_)) => {
                if let TokenType::Identifier(name) = token.kind {
                    Ok(name)
                } else {
                    unreachable!()
                }
            }
            Some(token) => {
                Err(ParseError::expected_identifier(Some(token.kind)).with_span(token.span))
            }
            None => Err(ParseError::expected_identifier(None).with_span(self.eof_span)),
        }
    }

    fn expect_optional(
        &mut self,
        expected: TokenType<'static>,
    ) -> Result<bool, CompilerParseError> {
        match self.lexer.peek() {
            Some(Ok(token)) if token.kind == expected => {
                self.lexer.next(); // Consume it
                Ok(true)
            }
            Some(Ok(_)) => {
                // Token doesn't match, don't consume
                Ok(false)
            }
            Some(Err(_)) => {
                // Consume and return the error
                match self.lexer.next() {
                    Some(Err(err)) => Err(err.convert_error::<ParseError>()),
                    _ => unreachable!("next and peek should return same result"),
                }
            }
            None => Ok(false), // End of input
        }
    }
}
