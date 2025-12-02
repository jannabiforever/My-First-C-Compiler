use crate::lexer_base::token::{KeywordType, SymbolType, Token};
use crate::parser_base::grammar::*;

fn expect(tokens: &[Token], expected: Token) -> Result<&[Token], String> {
    if tokens.is_empty() {
        return Err(format!("Expected {:?}, but found end of input", expected));
    }

    if &tokens[0] == &expected {
        Ok(&tokens[1..])
    } else {
        Err(format!(
            "Expected {:?}, but found {:?}",
            expected, tokens[0]
        ))
    }
}
fn parse_expression(tokens: &[Token]) -> Result<(Expression, &[Token]), String> {
    if tokens.is_empty() {
        return Err("Expected expression, but found end of input".to_string());
    }

    match &tokens[0] {
        Token::Constant(value) => {
            let expr = Expression::Constant(*value);
            Ok((expr, &tokens[1..]))
        }
        _ => Err(format!("Expected expression, but found {:?}", tokens[0])),
    }
}
fn parse_statement(tokens: &[Token]) -> Result<(Statement, &[Token]), String> {
    let tail = expect(tokens, Token::Keyword(KeywordType::Return))?;
    let (expr, tail) = parse_expression(tail)?;
    let tail = expect(tail, Token::Symbol(SymbolType::Semicolon))?;
    Ok((Statement::Return(expr), tail))
}
fn parse_func_def(tokens: &[Token]) -> Result<(FuncDef, &[Token]), String> {
    if tokens.is_empty() {
        return Err("Expected function definition, but found end of input".to_string());
    }
    let tail = expect(tokens, Token::Keyword(KeywordType::Int))?;
    let func_name = if let Token::Identifier(name) = &tail[0] {
        name.clone()
    } else {
        return Err(format!(
            "Expected function name identifier, but found {:?}",
            tail[0]
        ));
    };
    let tail = &tail[1..];
    let tail = expect(tail, Token::Symbol(SymbolType::LParen))?;
    let tail = match expect(tail, Token::Symbol(SymbolType::RParen)) {
        Ok(t) => t,
        Err(e) => {
            if let Ok(t) = expect(tail, Token::Keyword(KeywordType::Void)) {
                expect(t, Token::Symbol(SymbolType::RParen))?
            } else {
                return Err(e);
            }
        }
    };
    let tail = expect(tail, Token::Symbol(SymbolType::LBrace))?;
    let (stmt, tail) = parse_statement(tail)?;
    let tail = expect(tail, Token::Symbol(SymbolType::RBrace))?;
    let func_def = FuncDef::Fn(func_name, stmt);
    Ok((func_def, tail))
}

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    let (func_def, tail) = parse_func_def(tokens)?;
    if !tail.is_empty() {
        return Err(format!("Unexpected tokens after function definition: {:?}", tail));
    }
    Ok(Program::Program(func_def))
}
