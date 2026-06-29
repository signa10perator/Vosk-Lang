#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Structure
    Context,
    Constraint,
    Observe,
    Transmit,
    Bind,
    Arrow,
    Escalate,

    // States
    Unknown,
    Resolved,
    Decaying,
    Corrupted,

    // Delimiters
    LBrace,
    RBrace,

    // Values
    Ident(String),
    Receiver(String),   // ^NAME
    Str(String),
    Number(f64),

    // End
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
            Lexer {
                input: source.chars().collect(),
                pos: 0,
                line: 1,
            }
        }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current();
        self.pos += 1;
        ch
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    fn skip_whitespace(&mut self) {
            while let Some(ch) = self.current() {
                if ch == '#' {
                    while let Some(c) = self.current() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                } else if ch == '\n' {
                    self.line += 1;
                    self.advance();
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current() {
            None => Token::EOF,

            Some('{') => { self.advance(); Token::LBrace }
            Some('}') => { self.advance(); Token::RBrace }
            Some('?') => { self.advance(); Token::Unknown }
            Some('+') => { self.advance(); Token::Resolved }
            Some('%') => { self.advance(); Token::Decaying }
            Some('@') => { self.advance(); Token::Observe }

            Some('~') => {
                self.advance();
                if self.current() == Some('>') {
                    self.advance();
                    Token::Transmit
                } else {
                    Token::Context
                }
            }

            Some('!') => { self.advance(); Token::Constraint }
            Some('^') => {
                self.advance();
                if let Some(c) = self.current() {
                    if c.is_alphabetic() || c == '_' {
                        let mut name = String::new();
                        while let Some(c) = self.current() {
                            if c.is_alphanumeric() || c == '_' {
                                name.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        Token::Receiver(name)
                    } else {
                        Token::Escalate
                    }
                } else {
                    Token::Escalate
                }
            }

            Some(':') => {
                self.advance();
                if self.current() == Some(':') {
                    self.advance();
                    Token::Bind
                } else {
                    Token::Ident(":".to_string())
                }
            }

            Some('-') => {
                self.advance();
                if self.current() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Ident("-".to_string())
                }
            }

            Some('"') => {
                self.advance();
                let mut s = String::new();
                while let Some(ch) = self.current() {
                    if ch == '"' { self.advance(); break; }
                    s.push(ch);
                    self.advance();
                }
                Token::Str(s)
            }

            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let mut ident = String::new();
                while let Some(c) = self.current() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "x" => Token::Corrupted,
                    _ => Token::Ident(ident),
                }
            }

            Some(ch) if ch.is_numeric() => {
                let mut num = String::new();
                while let Some(c) = self.current() {
                    if c.is_numeric() || c == '.' {
                        num.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                Token::Number(num.parse().unwrap_or(0.0))
            }

            Some(ch) => {
                self.advance();
                Token::Ident(ch.to_string())
            }
        }
    }
}
