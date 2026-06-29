use crate::lexer::{Lexer, Token};
use crate::ast::*;

pub struct Parser {
    lexer: Lexer,
    current: Token,
    pub line: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let line = lexer.line;
        Parser { lexer, current, line }
    }

    fn advance(&mut self) -> Token {
        let prev = self.current.clone();
        self.current = self.lexer.next_token();
        self.line = self.lexer.line;
        prev
    }

    fn expect(&mut self, expected: Token) -> Result<Token, String> {
        if self.current == expected {
            Ok(self.advance())
        } else {
            Err(format!(
                "line {}: expected {:?} but found {:?}",
                self.line, expected, self.current
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut contexts = vec![];

        while self.current != Token::EOF {
            let ctx = self.parse_context()?;
            contexts.push(ctx);
        }

        Ok(Program { contexts })
    }

    fn parse_context(&mut self) -> Result<Context, String> {
        self.expect(Token::Context)?;

        let name = match self.advance() {
            Token::Ident(s) => s,
            other => return Err(format!("expected context name, found {:?}", other)),
        };

        self.expect(Token::LBrace)?;

        let mut body = vec![];

        while self.current != Token::RBrace && self.current != Token::EOF {
            let stmt = self.parse_stmt()?;
            body.push(stmt);
        }

        self.expect(Token::RBrace)?;

        Ok(Context { name, body })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.current.clone() {
            Token::Ident(_) => self.parse_binding(),
            Token::Receiver(_) => self.parse_receiver(),
            Token::Constraint => self.parse_constraint(),
            Token::Observe => self.parse_observe(),
            other => Err(format!("unexpected token in context body: {:?}", other)),
        }
    }

    fn parse_binding(&mut self) -> Result<Stmt, String> {
        let name = match self.advance() {
            Token::Ident(s) => s,
            other => return Err(format!("expected identifier, found {:?}", other)),
        };

        self.expect(Token::Bind)?;

        let state = self.parse_state()?;

        Ok(Stmt::Binding { name, state })
    }

    fn parse_constraint(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Constraint)?;
    
        let target = match self.advance() {
            Token::Ident(s) => s,
            Token::Receiver(s) => format!("^{}", s),
            other => return Err(format!("expected identifier, found {:?}", other)),
        };
    
        self.expect(Token::Bind)?;
    
        let condition = self.parse_state()?;
    
        Ok(Stmt::Constraint { target, condition })
    }

    fn parse_receiver(&mut self) -> Result<Stmt, String> {
        let name = match self.advance() {
            Token::Receiver(s) => s,
            other => return Err(format!("expected receiver name, found {:?}", other)),
        };
    
        self.expect(Token::Bind)?;
    
        let state = self.parse_state()?;
    
        Ok(Stmt::Receiver { name, state })
    }

    fn parse_observe(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Observe)?;

        let target = match self.advance() {
            Token::Ident(s) => s,
            Token::Receiver(s) => format!("^{}", s),
            other => return Err(format!("expected identifier, found {:?}", other)),
        };

        self.expect(Token::Bind)?;

        let condition = self.parse_state()?;

        let transmit = if self.current == Token::Transmit {
            self.advance();
            let scope = match self.current.clone() {
                            Token::Ident(s) if s == "*" => {
                                self.advance();
                                TransmitScope::Propagate
                            }
                            Token::Ident(s) if s == "^" => {
                                self.advance();
                                TransmitScope::Escalate
                            }
                            Token::Ident(s) if s == "emit" => {
                                self.advance();
                                TransmitScope::Emit
                            }
                            _ => TransmitScope::Local,
                        };
            let message = match self.advance() {
                Token::Str(s) => s,
                other => return Err(format!("expected transmission message, found {:?}", other)),
            };
            Some(Box::new(Transmission { scope, message }))
        } else {
            None
        };

        Ok(Stmt::Observe {
            target,
            condition,
            transmit,
        })
    }

    fn parse_state(&mut self) -> Result<State, String> {
        match self.advance() {
            Token::Unknown => Ok(State::Unknown),
            Token::Resolved => {
                match self.current.clone() {
                    Token::Str(s) => {
                        self.advance();
                        Ok(State::Str(s))
                    }
                    Token::Number(n) => {
                        self.advance();
                        Ok(State::Value(n))
                    }
                    _ => Ok(State::Resolved),
                }
            }
            Token::Decaying => {
                match self.current.clone() {
                    Token::Number(n) => {
                        self.advance();
                        Ok(State::DecayingValue(n))
                    }
                    _ => Ok(State::Decaying),
                }
            }
            Token::Corrupted => Ok(State::Corrupted),
            Token::Number(n) => Ok(State::Value(n)),
            Token::Str(s) => Ok(State::Str(s)),
            other => Err(format!(
                "line {}: expected state, found {:?}",
                self.line, other
            )),
        }
    }
}
