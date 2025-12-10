use crate::{
    grammar::*,
    parser_base::{CompilerParseError, Parser},
    t,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_null_statement(&mut self) -> Result<NullStmt<'a>, CompilerParseError> {
        self.expect_token(t!(";"))?;
        Ok(NullStmt::default())
    }
}
