use crate::{token::Token, Uri};

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
            port: self.port()?,
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
            Token::Eof => Err("Expected hostname, but was Eof".to_string()),
        }
    }

    fn port(&mut self) -> Result<Option<u16>, String> {
        match self.peek() {
            Token::Delim(':') => self.advance(),
            _ => return Ok(None),
        };

        let token = self.advance();
        if let Token::Part(p) = token {
            if let Ok(port) = p.parse::<u16>() {
                Ok(Some(port))
            } else {
                Err("Invalid port number".to_string())
            }
        } else {
            Err(format!("Expected port, but was {}", token))
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

        let mut query = String::new();
        loop {
            match self.peek() {
                Token::Part(str) => query.push_str(str),
                Token::Delim('#') => break,
                Token::Delim(c) => query.push(*c),
                _ => break,
            }
            self.advance();
        }
        Some(query)
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

    fn advance(&mut self) -> &Token<'_> {
        if !self.is_at_end() {
            self.current += 1;
            &self.tokens[self.current - 1]
        } else {
            &self.tokens[self.current]
        }
    }

    fn consume(&mut self, token: Token) -> Result<(), String> {
        let current = self.peek();
        if *current == token {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected: {token}, but: {current}"))
        }
    }

    fn peek(&self) -> &Token<'_> {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        *self.peek() == Token::Eof
    }
}

#[cfg(test)]
mod test {
    use crate::Uri;
    use p_test::p_test;
    use std::str::FromStr;

    #[p_test(
        simple,
        (
            "https://example.com",
            Ok(Uri {
                scheme: "https".to_string(),
                hostname: "example.com".to_string(),
                port: None,
                path: "/".to_string(),
                query: None,
                fragment: None,
            })
        ),
        full,
        (
            "https://example.com:443/path/to?q1=10&q2=20#fragment",
            Ok(Uri {
                scheme: "https".to_string(),
                hostname: "example.com".to_string(),
                port: Some(443),
                path: "/path/to".to_string(),
                query: Some("q1=10&q2=20".to_string()),
                fragment: Some("fragment".to_string()),
            })
        ),
        no_port,
        (
            "https://example.com/path/to?q1=10&q2=20#fragment",
            Ok(Uri {
                scheme: "https".to_string(),
                hostname: "example.com".to_string(),
                port: None,
                path: "/path/to".to_string(),
                query: Some("q1=10&q2=20".to_string()),
                fragment: Some("fragment".to_string()),
            })
        ),
        no_path,
        (
            "https://example.com:443?q1=10&q2=20#fragment",
            Ok(Uri {
                scheme: "https".to_string(),
                hostname: "example.com".to_string(),
                port: Some(443),
                path: "/".to_string(),
                query: Some("q1=10&q2=20".to_string()),
                fragment: Some("fragment".to_string()),
            })
        ),
        no_query,
        (
            "https://example.com:443/path/to#fragment",
            Ok(Uri {
                scheme: "https".to_string(),
                hostname: "example.com".to_string(),
                port: Some(443),
                path: "/path/to".to_string(),
                query: None,
                fragment: Some("fragment".to_string()),
            })
        ),
        no_fragment,
        (
            "https://example.com:443/path/to?q1=10&q2=20",
            Ok(Uri {
                scheme: "https".to_string(),
                hostname: "example.com".to_string(),
                port: Some(443),
                path: "/path/to".to_string(),
                query: Some("q1=10&q2=20".to_string()),
                fragment: None,
            })
        ),
        no_scheme,
        (
            "///example.com:443/path/to?q1=10&q2=20",
            Err("Scheme not found".to_string()),
        ),
        invalid_delimeters_after_scheme,
        (
            "https:///example.com:443/path/to?q1=10&q2=20",
            Err("Expected hostname, but was /".to_string())
        ),
        no_hostname_but_eof,
        (
            "https://",
            Err("Expected hostname, but was Eof".to_string())
        ),
        invalid_missing_token1,
        (
            "https",
            Err("Expected: :, but: EOF".to_string()),
        ),
        invalid_missing_token2,
        (
            "https:",
            Err("Expected: /, but: EOF".to_string()),
        ),
        invalid_missing_token3,
        (
            "https:/",
            Err("Expected: /, but: EOF".to_string()),
        ),
        invalid_port,
        (
            "http://localhost::3000",
            Err("Expected port, but was :".to_string())
        ),
        invalid_port_number,
        (
            "http://localhost:abc",
            Err("Invalid port number".to_string())
        ),
        query_str_with_delim1,
        (
            "http://localhost???a=10",
            Ok(Uri {
                scheme: "http".to_string(),
                hostname: "localhost".to_string(),
                port: None,
                path: "/".to_string(),
                query: Some("??a=10".to_string()),
                fragment: None,
            })

        )
    )]
    fn test_uri(uri: &str, expected: Result<Uri, String>) {
        assert_eq!(Uri::from_str(uri), expected);
    }
}
