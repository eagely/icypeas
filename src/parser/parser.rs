use super::enums::Expression;
use crate::error::{Error, Result};
use crate::lexer::enums::{Token, TokenKind};

macro_rules! try_consume_any {
    ($self:expr, $($token_type:expr),+) => {{
        let mut found = false;
        $(
            if $self.matches($token_type) {
                $self.advance();
                found = true;
            }
        )+
        found
    }};
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();

        while !self.is_eof() {
            if try_consume_any!(*self, TokenKind::Semicolon, TokenKind::Newline) {
                continue;
            }
            expressions.push(self.expression()?);
        }
        Ok(expressions)
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }

    fn next(&self) -> Option<Token> {
        self.tokens.get(self.index + 1).cloned()
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn matches(&self, kind: TokenKind) -> bool {
        self.current().map_or(false, |t| t.kind == kind)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn expression(&mut self) -> Result<Expression> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(Error::UnexpectedEndOfFile)?;

        match token.kind {
            TokenKind::True
            | TokenKind::False
            | TokenKind::Null
            | TokenKind::Number
            | TokenKind::String => {
                self.advance();
                Ok(Expression::Literal { token })
            }
            TokenKind::LeftParenthesis => {
                self.advance();
                let expression = self.expression()?;
                dbg!(&self.current());
                if !try_consume_any!(*self, TokenKind::RightParenthesis) {
                    return Err(Error::MissingClosingParenthesis);
                }
                Ok(expression)
            }
            _ => Err(Error::ExpectedExpression),
        }
    }
}
