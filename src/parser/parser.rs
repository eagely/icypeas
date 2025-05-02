use crate::err;
use crate::error::{ErrorKind, Result};
use crate::model::Expression;
use crate::model::ExpressionKind;
use crate::model::Token;
use crate::model::TokenKind;
use crate::parser::precedence::Precedence;

macro_rules! try_consume_any {
    ($self:expr, $($token_type:expr),+) => {{
        false $(|| {
            if $self.current_is($token_type) {
                $self.advance();
                true
            } else {
                false
            }
        })+
    }};
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub const fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
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

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        self.current().is_some_and(|t| t.kind == kind)
    }

    fn next_is(&self, n: usize, kind: TokenKind) -> bool {
        self.next(n).is_some_and(|t| t.kind == kind)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn is_end_of_expression(&self) -> bool {
        self.is_eof()
            || self.current_is(TokenKind::Newline)
            || self.current_is(TokenKind::Semicolon)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        let expr = self.parse_declaration()?;

        Ok(expr)
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
                self.advance();
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
            _ => err!(
                ErrorKind::ExpectedExpression,
                location,
                "This should be an identifier.",
            ),
        }
    }

    fn parse_lambda(&mut self) -> Result<Expression> {
        if !self.current_is(TokenKind::Identifier)
            || self.tokens[self.index..]
                .iter()
                .find(|t| t.kind != TokenKind::Identifier)
                .is_none_or(|t| t.kind != TokenKind::Dollar)
        {
            return self.parse_assignment();
        }

        let parameter = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;

        self.advance();

        if self.current_is(TokenKind::Dollar) {
            self.advance();
            let body = self.parse_expression()?;
            let location = body.location.clone();
            Ok(Expression::new(
                ExpressionKind::Lambda {
                    parameter,
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
                .find(|t| !t.kind.is_primary())
                .is_none_or(|t| t.kind != TokenKind::Equal)
        {
            return self.parse_if();
        }

        let name = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;

        if !self.next_is(1, TokenKind::Identifier) {
            return err!(
                ErrorKind::ExpectedExpression,
                name.location,
                "Missing parameter for function assignment.",
            );
        }

        self.advance();

        let parameter = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;

        self.advance();
        self.advance();

        let body = Box::new(self.parse_expression()?);
        let location = body.location.clone();
        Ok(Expression::new(
            ExpressionKind::Assignment {
                name,
                parameter,
                body,
            },
            location,
        ))
    }

    fn parse_if(&mut self) -> Result<Expression> {
        if !self.current_is(TokenKind::If) {
            return self.parse_binary();
        }

        self.advance();
        let condition = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_expression()?);
        let location = condition.location.clone();

        let mut branches = vec![(condition, body)];
        while self.current_is(TokenKind::Elif) {
            self.advance();
            let condition = self.parse_expression()?;
            let body = self.parse_expression()?;

            branches.push((Box::new(condition), Box::new(body)));

            if !self.current_is(TokenKind::Elif) {
                break;
            }
        }

        let otherwise = if self.current_is(TokenKind::Else) {
            self.advance();
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        Ok(Expression::new(
            ExpressionKind::If {
                branches,
                otherwise,
            },
            location,
        ))
    }

    fn parse_binary(&mut self) -> Result<Expression> {
        self.parse_precedence(Precedence::None)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expression> {
        let mut left = self.parse_unary()?;
        while !self.is_eof() {
            let Some(current_token) = self.current() else {
                break;
            };

            let current_precedence = Precedence::from(current_token.kind);
            if current_precedence <= precedence {
                break;
            }

            if try_consume_any!(*self, TokenKind::Semicolon, TokenKind::Newline) {
                break;
            }

            if !self.current().is_some_and(|token| token.kind.is_operator()) {
                break;
            }

            let operator = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
            self.advance();
            let location = operator.location.clone();
            let right = self.parse_precedence(current_precedence)?;

            left = Expression::new(
                ExpressionKind::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                location,
            );
        }
        Ok(left)
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
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;
        while !self.is_end_of_expression()
            && self.current().is_some_and(|token| token.kind.is_primary())
        {
            let arg = self.parse_primary()?;
            let location = arg.location.clone();
            expr = Expression::new(
                ExpressionKind::Call {
                    function: Box::new(expr),
                    argument: Box::new(arg),
                },
                location,
            );
        }
        Ok(expr)
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
                    return err!(
                        ErrorKind::MissingClosingParenthesis,
                        token.location,
                        "Consider inserting a ')' after this expression.",
                    );
                }
                Ok(expression)
            }
            _ => err!(
                ErrorKind::ExpectedExpression,
                token.location,
                "This is not valid syntax.",
            ),
        }
    }
}
