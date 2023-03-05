use crate::{Token, Uri};

pub(crate) struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Result<Uri, String> {
        Ok(Uri {
            scheme: self.scheme()?,
            hostname: self.hostname()?,
            port: self.port(),
            path: self.path(),
            query: self.query(),
            fragment: self.fragment(),
        })
    }

    fn scheme(&mut self) -> Result<String, String> {
        let token = self.advance();
        let scheme = match token {
            Token::Part(s) => s.to_string(),
            _ => return Err("Scheme not found".to_string()),
        };

        self.consume(Token::Delim(':'))?;
        self.consume(Token::Delim('/'))?;
        self.consume(Token::Delim('/'))?;
        Ok(scheme)
    }

    fn hostname(&mut self) -> Result<String, String> {
        match self.advance() {
            Token::Part(hostname) => Ok(hostname.to_string()),
            Token::Delim(c) => Err(format!("Expected hostname, but was {}", *c)),
            Token::Eof => Err("Expected hostname but was Eof".to_string()),
        }
    }

    fn port(&mut self) -> Option<u16> {
        match self.consume(Token::Delim(':')) {
            Ok(()) => {
                if let Token::Part(p) = self.advance() {
                    let port: u16 = p.parse().unwrap();
                    Some(port)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn path(&mut self) -> String {
        let mut path = String::from('/');

        match self.peek() {
            Token::Delim('/') => self.advance(),
            _ => return path,
        };

        loop {
            if let Token::Part(p) = self.advance() {
                path.push_str(p);
            }
            match self.peek() {
                Token::Delim('/') => {
                    path.push('/');
                    self.advance();
                }
                _ => return path,
            }
        }
    }

    fn query(&mut self) -> Option<String> {
        match self.peek() {
            Token::Delim('?') => self.advance(),
            _ => return None,
        };

        if let Token::Part(q) = self.advance() {
            Some(q.to_string())
        } else {
            None
        }
    }

    fn fragment(&mut self) -> Option<String> {
        match self.peek() {
            Token::Delim('#') => self.advance(),
            _ => return None,
        };

        if let Token::Part(f) = self.advance() {
            Some(f.to_string())
        } else {
            None
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token) -> Result<(), String> {
        if self.is_at_end() {
            return Err("is at end".to_owned());
        }
        let current = self.peek();
        if *current == token {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected: {token}, but: {current}"))
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        *self.peek() == Token::Eof
    }
}
