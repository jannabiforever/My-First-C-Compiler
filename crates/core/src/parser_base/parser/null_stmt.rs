use crate::{
    grammar::*,
    parser_base::{CompilerParseError, Parser},
    t,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_null_statement(&mut self) -> Result<NullStmt<'a>, CompilerParseError> {
        self.expect(t!(";"))?;
        Ok(NullStmt::default())
    }
}
