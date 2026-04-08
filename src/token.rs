use std::fmt::{self, Display, Formatter, Write};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Delim(char),
    Part(&'a str),
    Eof,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Delim(c) => f.write_char(*c),
            Token::Part(s) => f.write_str(s),
            Token::Eof => f.write_str("EOF"),
        }
    }
}

#[cfg(test)]
mod test {
    use p_test::p_test;

    use crate::token::Token;

    #[p_test(
        display_token_delim, (Token::Delim(':'), ":"),
        display_token_part,  (Token::Part("http"), "http"),
        display_token_eof,   (Token::Eof, "EOF")
    )]
    fn test_token_display(token: Token, s: &str) {
        assert_eq!(token.to_string(), s);
    }
}
