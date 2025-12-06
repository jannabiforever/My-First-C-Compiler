use std::borrow::Cow;

use crate::parser_base::{Expression, grammar::Type};

/// A function definition
#[derive(Debug)]
pub struct FuncDef<'a> {
    pub return_type: Type,
    pub name: Cow<'a, str>,
    pub params: Vec<Cow<'a, str>>,
    pub body: Block<'a>,
}

/// A statement within a function
#[derive(Debug)]
pub enum Statement<'a> {
    Block(Block<'a>),
    If(IfStmt<'a>),
    Return(ReturnStmt<'a>),
}

impl<'a> Statement<'a> {
    pub fn block_stmt(statements: Vec<Statement<'a>>) -> Self {
        Self::Block(Block { statements })
    }

    pub fn if_stmt(
        cond: Expression<'a>,
        then_block: Statement<'a>,
        else_block: Option<Statement<'a>>,
    ) -> Self {
        Self::If(IfStmt {
            cond,
            then_block: Box::new(then_block),
            else_block: else_block.map(Box::new),
        })
    }

    pub fn return_stmt(expr: Expression<'a>) -> Self {
        Self::Return(ReturnStmt { expr })
    }
}

/// A block of statements, enclosed in curly braces {}
#[derive(Debug)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct IfStmt<'a> {
    pub cond: Expression<'a>,
    pub then_block: Box<Statement<'a>>,
    pub else_block: Option<Box<Statement<'a>>>,
}

#[derive(Debug)]
pub struct ReturnStmt<'a> {
    pub expr: Expression<'a>,
}
