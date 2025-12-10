use crate::{
    grammar::*,
    parser_base::{Parser, error::ParseResult},
    t,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_null_statement(&mut self) -> ParseResult<NullStmt<'a>> {
        self.expect_token(t!(";"))?;
        Ok(NullStmt::default())
    }
}
