mod parser;
mod scanner;
mod token;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use parser::Parser;
use scanner::Scanner;

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

impl FromStr for Uri {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
            f.write_fmt(format_args!(":{p}"))?
        }
        f.write_str(&self.path)?;
        match &self.query {
            Some(q) => f.write_fmt(format_args!("?{q}"))?,
            None => (),
        }
        match &self.fragment {
            Some(q) => f.write_fmt(format_args!("#{q}")),
            None => Ok(()),
        }
    }
}
