use crate::token::Token;

pub(crate) struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Scanner {
            source: input,
            tokens: Vec::new(),
        }
    }

    pub(crate) fn tokens(&mut self) -> &[Token<'_>] {
        self.scan_tokens();
        &self.tokens
    }

    fn scan_tokens(&mut self) {
        let mut start = 0;
        let mut current = 0;
        let mut chars = self.source.chars();

        loop {
            current += 1;
            if let Some(ch) = chars.next() {
                if self.is_delimiter(&ch) {
                    if start < current - 1 {
                        self.add_token(Token::Part(&self.source[start..current - 1]));
                    }
                    self.add_token(Token::Delim(ch));
                    start = current;
                }
            } else {
                if start < current - 1 {
                    self.add_token(Token::Part(&self.source[start..current - 1]));
                }
                break;
            }
        }
        self.add_token(Token::Eof);
    }

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn is_delimiter(&mut self, ch: &char) -> bool {
        matches!(ch, ':' | '/' | '?' | '#')
    }
}

#[cfg(test)]
mod test {
    use crate::{scanner::Scanner, token::Token};
    use p_test::p_test;

    #[p_test(
        (
            "http://localhost:3000/index.html", 
            &vec![
                Token::Part("http"),
                Token::Delim(':'),
                Token::Delim('/'),
                Token::Delim('/'),
                Token::Part("localhost"),
                Token::Delim(':'),
                Token::Part("3000"),
                Token::Delim('/'),
                Token::Part("index.html"),
                Token::Eof
                ]
        )
    )]
    fn test_scanner(url: &str, expected: &[Token]) {
        let mut scanner = Scanner::new(url);
        let tokens = scanner.tokens();
        assert_eq!(tokens, expected);
    }
}
