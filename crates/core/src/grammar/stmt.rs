use std::borrow::Cow;

use compiler_macros::statement_enum;

use crate::grammar::{Expression, Type};

/// A function definition
#[derive(Debug)]
pub struct FuncDef<'a> {
    pub return_type: Type,
    pub name: Cow<'a, str>,
    pub params: Vec<(Type, Cow<'a, str>)>,
    pub body: BlockStmt<'a>,
}

statement_enum! {
    #[derive(Debug, Clone)]
    pub struct BlockStmt<'a> {
        pub statements: Vec<Statement<'a>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct BreakStmt<'a> {
        _m: std::marker::PhantomData< &'a()> ,
    }

    #[derive(Debug, Clone, Default)]
    pub struct ContinueStmt<'a> {
        _m: std::marker::PhantomData< &'a()> ,
    }

    #[derive(Debug, Clone)]
    pub struct DeclarationStmt<'a> {
        pub var_type: Type,
        pub name: Cow<'a, str>,
        pub initializer: Option<Expression<'a>>,
    }

    #[derive(Debug, Clone)]
    pub struct DoWhileStmt<'a> {
        pub cond: Expression<'a>,
        pub body: Box<Statement<'a>>,
    }

    #[derive(Debug, Clone)]
    pub struct ExprStmt<'a> {
        pub expr: Expression<'a>,
    }

    #[derive(Debug, Clone)]
    pub struct ForStmt<'a> {
        pub init: Option<Expression<'a>>,
        pub cond: Option<Expression<'a>>,
        pub post: Option<Expression<'a>>,
        pub body: Box<Statement<'a>>,
    }

    #[derive(Debug, Clone)]
    pub struct IfStmt<'a> {
        pub cond: Expression<'a>,
        pub then_block: Box<Statement<'a>>,
        pub else_block: Option<Box<Statement<'a>>>,
    }

    /// Represents a statement with one semicolon.
    #[derive(Debug, Clone, Default)]
    pub struct NullStmt<'a> {
        _m: std::marker::PhantomData< &'a()>,
    }

    #[derive(Debug, Clone)]
    pub struct ReturnStmt<'a> {
        pub expr: Expression<'a>,
    }

    #[derive(Debug, Clone)]
    pub struct WhileStmt<'a> {
        pub cond: Expression<'a>,
        pub body: Box<Statement<'a>>,
    }
}
