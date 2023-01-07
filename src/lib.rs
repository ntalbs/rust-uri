use std::fmt::{self, Display, Formatter, Write};

/// ```notrust
/// abc://username:password@example.com:123/path/data?key1=value1&key2=value2#frag1
/// |-|   |---------------| |---------| |-||--------| |---------------------| |---|
///  |          userinfo        host    port   path              |              |
///  |    |-----------------------------------------|            |              |
/// scheme                  authority                          query         fragment
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Uri {
    pub scheme: String,
    pub hostname: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Uri {
    pub fn from_str(input: &str) -> Result<Uri, String> {
        let mut scanner = Scanner::new(input);
        let tokens = scanner.tokens();
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(uri) => Ok(uri),
            Err(error) => Err(error),
        }
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.scheme)?;
        f.write_str("://")?;
        f.write_str(&self.hostname)?;

        if let Some(p) = self.port {
            f.write_fmt(format_args!(":{}", p))?
        }
        f.write_str(&self.path)?;
        match &self.query {
            Some(q) => f.write_fmt(format_args!("?{}", q))?,
            None => (),
        }
        match &self.fragment {
            Some(q) => f.write_fmt(format_args!("#{}", q)),
            None => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
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

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Scanner {
            source: input,
            tokens: Vec::new(),
        }
    }

    fn tokens(&mut self) -> &[Token] {
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

struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Result<Uri, String> {
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
            Err(format!("Expected: {}, but: {}", token, current))
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        *self.peek() == Token::Eof
    }
}

#[test]
fn simple() {
    assert_eq!(
        Uri::from_str("https://example.com").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: None,
            path: "/".to_string(),
            query: None,
            fragment: None,
        }
    );
}

#[test]
fn full() {
    assert_eq!(
        Uri::from_str("https://example.com:443/path/to?q1=10&q2=20#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_port() {
    assert_eq!(
        Uri::from_str("https://example.com/path/to?q1=10&q2=20#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: None,
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_path() {
    assert_eq!(
        Uri::from_str("https://example.com:443?q1=10&q2=20#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_query() {
    assert_eq!(
        Uri::from_str("https://example.com:443/path/to#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: None,
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_fragment() {
    assert_eq!(
        Uri::from_str("https://example.com:443/path/to?q1=10&q2=20").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: None,
        }
    );
}

#[test]
fn no_scheme() {
    match Uri::from_str("///example.com:443/path/to?q1=10&q2=20") {
        Ok(_) => panic!("expect error, but was ok"),
        Err(e) => assert_eq!("Scheme not found", e),
    }
}

#[test]
fn invalid_delimeters_after_scheme() {
    match Uri::from_str("https:///example.com:443/path/to?q1=10&q2=20") {
        Ok(_) => panic!("expect error, but was ok"),
        Err(e) => assert_eq!("Expected hostname, but was /", e),
    }
}
