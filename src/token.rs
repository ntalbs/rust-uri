use std::fmt::{Display, Formatter, self, Write};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Delim(char),
    Part(&'a str),
    Eof,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Delim(c) => f.write_char(*c),
            Token::Part(s) => f.write_str(s),
            Token::Eof => f.write_str("EOF"),
        }
    }
}
