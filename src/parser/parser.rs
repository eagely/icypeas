use super::enums::Expression;
use crate::error::{Error, Result};
use crate::lexer::enums::{Token, TokenKind};

macro_rules! try_consume_any {
    ($self:expr, $($token_type:expr),+) => {{
        let mut found = false;
        $(
            if $self.current_is($token_type) {
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
            expressions.push(self.parse_expression()?);
        }
        Ok(expressions)
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }

    fn next(&self, n: usize) -> Option<Token> {
        self.tokens.get(self.index + n).cloned()
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.index - 1).cloned()
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        self.current().map_or(false, |t| t.kind == kind)
    }

    fn next_is(&self, n: usize, kind: TokenKind) -> bool {
        self.next(n).map_or(false, |t| t.kind == kind)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_declaration()
    }

    fn parse_declaration(&mut self) -> Result<Expression> {
        if !self.next_is(1, TokenKind::Colon) {
            return self.parse_lambda();
        }

        let name = self.current().ok_or(Error::UnexpectedEndOfFile)?;

        match name.kind {
            TokenKind::Identifier => {
                self.advance();
                try_consume_any!(*self, TokenKind::Colon);
                let mut types = Vec::new();
                while self.current_is(TokenKind::Underscore)
                    || self.current_is(TokenKind::Identifier)
                {
                    let token = self.current().ok_or(Error::UnexpectedEndOfFile)?;
                    self.advance();
                    types.push(token);
                }
                Ok(Expression::Declaration { name, types })
            }
            _ => Err(Error::ExpectedExpression),
        }
    }

    fn parse_lambda(&mut self) -> Result<Expression> {
        if self.tokens[self.index..]
            .iter()
            .skip_while(|t| t.kind == TokenKind::Identifier)
            .next()
            .map_or(true, |t| t.kind != TokenKind::Dollar)
        {
            return self.parse_assignment();
        }

        let mut parameters = Vec::new();

        while self.current_is(TokenKind::Identifier) {
            parameters.push(self.current().ok_or(Error::UnexpectedEndOfFile)?);
            self.advance();
        }

        if self.current_is(TokenKind::Dollar) {
            try_consume_any!(*self, TokenKind::Dollar);
            Ok(Expression::Lambda {
                parameters,
                body: Box::new(self.parse_expression()?),
            })
        } else {
            self.parse_if()
        }
    }

    fn parse_assignment(&mut self) -> Result<Expression> {
        if !self.current_is(TokenKind::Identifier)
            || self.tokens[self.index..]
                .iter()
                .skip_while(|t| t.kind.is_primary())
                .next()
                .map_or(true, |t| t.kind != TokenKind::Equals)
        {
            return self.parse_if();
        }

        let identifier = self.current().ok_or(Error::ExpectedExpression)?;

        self.advance();
        let mut parameters = Vec::new();

        while let Some(t) = self.current() {
            if !t.kind.is_primary() {
                break;
            }
            self.advance();
            parameters.push(t);
        }

        try_consume_any!(*self, TokenKind::Equals);

        Ok(Expression::Assignment {
            identifier,
            parameters,
            expression: Box::new(self.parse_expression()?),
        })
    }

    fn parse_if(&mut self) -> Result<Expression> {
        if !self.current_is(TokenKind::If) {
            return self.parse_binary();
        }

        let mut branches = Vec::new();
        try_consume_any!(*self, TokenKind::If);
        loop {
            try_consume_any!(*self, TokenKind::Elif);
            branches.push((
                Box::new(self.parse_expression()?),
                Box::new(self.parse_expression()?),
            ));
            if !self.current_is(TokenKind::Elif) {
                break;
            }
        }

        let mut otherwise = None;
        if self.current_is(TokenKind::Else) {
            try_consume_any!(*self, TokenKind::Else);
            otherwise = Some(Box::new(self.parse_expression()?));
        }

        Ok(Expression::If {
            branches,
            otherwise,
        })
    }

    fn parse_binary(&mut self) -> Result<Expression> {
        let mut expression = self.parse_unary()?;
        if self.is_eof() {
            return Ok(expression);
        }
        while try_consume_any!(
            *self,
            TokenKind::Ampersand,
            TokenKind::Caret,
            TokenKind::Pipe,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::Equals,
            TokenKind::Less,
            TokenKind::Greater,
            TokenKind::At,
            TokenKind::Colon,
            TokenKind::Hash
        ) {
            let operator = self.previous().ok_or(Error::UnexpectedEndOfFile)?;
            let rhs = Box::new(self.parse_unary()?);
            expression = Expression::Binary {
                lhs: Box::new(expression),
                operator,
                rhs,
            }
        }
        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(Error::UnexpectedEndOfFile)?;

        match token.kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();
                Ok(Expression::Unary {
                    operator: token,
                    expression: Box::new(self.parse_primary()?),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(Error::UnexpectedEndOfFile)?;

        match token.kind {
            TokenKind::True
            | TokenKind::False
            | TokenKind::Null
            | TokenKind::Number
            | TokenKind::String
            | TokenKind::Underscore => {
                self.advance();
                Ok(Expression::Literal { token })
            }
            TokenKind::Identifier => {
                self.advance();
                Ok(Expression::Identifier { token })
            }
            TokenKind::LeftParenthesis => {
                self.advance();
                let expression = self.parse_expression()?;
                if !try_consume_any!(*self, TokenKind::RightParenthesis) {
                    return Err(Error::MissingClosingParenthesis);
                }
                Ok(expression)
            }
            _ => Err(Error::ExpectedExpression),
        }
    }
}
