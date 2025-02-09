use super::enums::Expression;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::{Location, Token, TokenKind};
use crate::parser::enums::ExpressionKind;
use crate::parser::precedence::Precedence;

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
            if self.is_eof() {
                break;
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

        let name = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = name.location.clone();

        match name.kind {
            TokenKind::Identifier => {
                self.advance();
                try_consume_any!(*self, TokenKind::Colon);
                let mut types = Vec::new();
                while self.current_is(TokenKind::Underscore)
                    || self.current_is(TokenKind::Identifier)
                {
                    let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
                    self.advance();
                    types.push(token);
                }
                Ok(Expression::new(
                    ExpressionKind::Declaration { name, types },
                    location,
                ))
            }
            _ => Err(Error::with_help(
                ErrorKind::ExpectedExpression,
                name.location,
                "This should be an identifier.",
            )),
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
            parameters.push(self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?);
            self.advance();
        }

        if self.current_is(TokenKind::Dollar) {
            try_consume_any!(*self, TokenKind::Dollar);
            let body = self.parse_expression()?;
            let location = body.location.clone();
            Ok(Expression::new(
                ExpressionKind::Lambda {
                    parameters,
                    body: Box::new(body),
                },
                location,
            ))
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
                .map_or(true, |t| t.kind != TokenKind::Equal)
        {
            return self.parse_if();
        }

        let identifier = self.current().ok_or(ErrorKind::ExpectedExpression)?;

        self.advance();
        let mut parameters = Vec::new();

        while let Some(t) = self.current() {
            if !t.kind.is_primary() {
                break;
            }
            self.advance();
            parameters.push(t);
        }

        try_consume_any!(*self, TokenKind::Equal);

        let expr = self.parse_expression()?;
        let location = expr.location.clone();
        Ok(Expression::new(
            ExpressionKind::Assignment {
                identifier,
                parameters,
                expression: Box::new(expr),
            },
            location,
        ))
    }

    fn parse_if(&mut self) -> Result<Expression> {
        if !self.current_is(TokenKind::If) {
            return self.parse_binary();
        }

        try_consume_any!(*self, TokenKind::If);
        let condition = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_expression()?);
        let location = condition.location.clone();

        let mut branches = vec![(condition, body)];
        loop {
            try_consume_any!(*self, TokenKind::Elif);
            let condition = self.parse_expression()?;
            let body = self.parse_expression()?;

            branches.push((Box::new(condition), Box::new(body)));

            if !self.current_is(TokenKind::Elif) {
                break;
            }
        }

        let mut otherwise = None;
        if self.current_is(TokenKind::Else) {
            try_consume_any!(*self, TokenKind::Else);
            otherwise = Some(Box::new(self.parse_expression()?));
        }

        Ok(Expression::new(
            ExpressionKind::If {
                branches,
                otherwise,
            },
            location,
        ))
    }

    fn parse_binary(&mut self) -> Result<Expression> {
        self.parse_binary_with_precedence(Precedence::None)
    }

    fn parse_binary_with_precedence(&mut self, precedence: Precedence) -> Result<Expression> {
        let mut lhs = self.parse_unary()?;
        while !self.is_eof() {
            let Some(current_token) = self.current() else {
                break;
            };

            let current_precedence = Precedence::from(current_token.kind);
            if current_precedence <= precedence {
                break;
            }

            if !try_consume_any!(
                *self,
                TokenKind::Ampersand,
                TokenKind::Caret,
                TokenKind::Pipe,
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Percent,
                TokenKind::Equal,
                TokenKind::EqualEqual,
                TokenKind::Less,
                TokenKind::LessEqual,
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                TokenKind::At,
                TokenKind::Colon,
                TokenKind::Hash
            ) {
                break;
            }
            let operator = self.previous().ok_or(ErrorKind::UnexpectedEndOfFile)?;
            let location = operator.location.clone();
            let rhs = self.parse_binary_with_precedence(current_precedence)?;

            lhs = Expression::new(
                ExpressionKind::Binary {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                },
                location,
            );
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = token.location.clone();

        match token.kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();
                Ok(Expression::new(
                    ExpressionKind::Unary {
                        operator: token,
                        expression: Box::new(self.parse_primary()?),
                    },
                    location,
                ))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = token.location.clone();

        match token.kind {
            TokenKind::True
            | TokenKind::False
            | TokenKind::Null
            | TokenKind::Float
            | TokenKind::Integer
            | TokenKind::String
            | TokenKind::Underscore => {
                self.advance();
                Ok(Expression::new(ExpressionKind::Literal { token }, location))
            }
            TokenKind::Identifier => {
                self.advance();
                Ok(Expression::new(
                    ExpressionKind::Identifier { token },
                    location,
                ))
            }
            TokenKind::LeftParenthesis => {
                self.advance();
                let expression = self.parse_expression()?;
                if !try_consume_any!(*self, TokenKind::RightParenthesis) {
                    return Err(Error::with_help(
                        ErrorKind::MissingClosingParenthesis,
                        token.location,
                        "Consider inserting a ')' after this expression.",
                    ));
                }
                Ok(expression)
            }
            _ => Err(Error::with_help(
                ErrorKind::ExpectedExpression,
                token.location,
                "This is not valid syntax.",
            )),
        }
    }
}
