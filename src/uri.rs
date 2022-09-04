pub struct Uri {
    scheme: String,
    hostname: String,
    port: u16,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

#[derive(Debug)]
pub enum Token<'a> {
    Delim(char),
    Part(&'a str),
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            source: input,
            tokens: Vec::new(),
        }
    }

    pub fn tokens(&mut self) -> &Vec<Token>{
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
                        self.add_token(Token::Part(&self.source[start..current-1]));
                    }
                    self.add_token(Token::Delim(ch));
                    start = current ;
                }
            } else {
                if start < current - 1 {
                    self.add_token(Token::Part(&self.source[start..current-1]));
                }
                break;
            }
        }
    }

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn is_delimiter(&mut self, ch: &char) -> bool {
        match ch {
            ':'|'/'|'?'|'#' => true,
            _ => false,
        }
    }
}
