use crate::err;
use crate::error::{ErrorKind, Result};
use crate::model::Token;
use crate::model::TokenKind;
use crate::model::{Expression, Location};
use crate::model::{ExpressionKind, Statement, StatementKind};
use crate::parser::precedence::Precedence;
use std::rc::Rc;

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
    pub const fn new() -> Self {
        Self {
            tokens: vec![],
            index: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Statement>> {
        self.tokens = tokens;
        let mut statements = vec![];

        while !self.is_eof() {
            if try_consume_any!(*self, TokenKind::Newline) {
                continue;
            }

            if self.is_eof() {
                break;
            }

            let statement = self.parse_statement()?;

            statements.push(statement);
        }
        Ok(statements)
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

    fn parse_statement(&mut self) -> Result<Statement> {
        self.parse_declaration()
    }

    fn parse_declaration(&mut self) -> Result<Statement> {
        if !self.next_is(1, TokenKind::Colon) {
            return self.parse_definition();
        }

        let name = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = name.location.clone();

        match name.kind {
            TokenKind::Identifier => {
                self.advance();
                self.advance();
                let mut types = vec![];
                while self.current_is(TokenKind::Underscore)
                    || self.current_is(TokenKind::Identifier)
                {
                    let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
                    self.advance();
                    types.push(token);
                }
                Ok(Statement::new(
                    StatementKind::Declaration { name, types },
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

    fn parse_definition(&mut self) -> Result<Statement> {
        if !self.current_is(TokenKind::Identifier)
            || self.tokens[self.index..]
                .iter()
                .find(|t| !t.kind.is_primary())
                .is_none_or(|t| t.kind != TokenKind::Equal)
        {
            let expression = self.parse_expression(Precedence::None)?;
            let location = expression.location.clone();
            return Ok(Statement::new(
                StatementKind::Expression { expression },
                location,
            ));
        }

        let name = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;

        if !self.next_is(1, TokenKind::Identifier) {
            return err!(
                ErrorKind::ExpectedExpression,
                name.location,
                "Missing parameter for function definition.",
            );
        }

        self.advance();

        let mut parameters = vec![];

        while let Some(t) = self.current() {
            if !t.kind.is_primary() {
                break;
            }
            parameters.push(t);
            self.advance();
        }

        self.advance();

        let body = self.parse_expression(Precedence::None)?;
        let location = body.location.clone();
        Self::curry_definition(name, parameters, body, location)
    }

    fn curry_definition(
        name: Token,
        parameters: Vec<Token>,
        body: Expression,
        location: Rc<Location>,
    ) -> Result<Statement> {
        let mut curried_lambda = body;

        let mut parameters = parameters.into_iter();
        let Some(first) = parameters.next() else {
            return err!(
                ErrorKind::MissingParameter,
                location,
                "Expected at least one parameter in a function definition."
            );
        };

        for parameter in parameters.rev() {
            curried_lambda = Expression::new(
                ExpressionKind::Lambda {
                    parameter: parameter.clone(),
                    body: Box::new(curried_lambda),
                },
                location.clone(),
            );
        }

        Ok(Statement::new(
            StatementKind::Definition {
                name,
                parameter: first,
                body: curried_lambda,
            },
            location,
        ))
    }

    fn parse_lambda(&mut self) -> Result<Expression> {
        let mut parameters = vec![];
        let mut location = self
            .current()
            .ok_or(ErrorKind::UnexpectedEndOfFile)?
            .location;

        while self.current_is(TokenKind::Identifier) {
            let parameter = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
            location = parameter.location.clone();
            parameters.push(parameter);
            self.advance();
        }

        if !self.current_is(TokenKind::Dollar) {
            return err!(
                ErrorKind::ExpectedExpression,
                location,
                "Expected $ after lambda parameters."
            );
        }

        self.advance();

        let body = self.parse_expression(Precedence::None)?;

        let mut curried_lambda = body;
        for parameter in parameters.into_iter().rev() {
            curried_lambda = Expression::new(
                ExpressionKind::Lambda {
                    parameter,
                    body: Box::new(curried_lambda),
                },
                location.clone(),
            );
        }

        Ok(curried_lambda)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
        let mut left = self.parse_prefix()?;

        while !self.is_eof() {
            let Some(current_token) = self.current() else {
                break;
            };

            let current_precedence = Precedence::from(current_token.kind);

            if current_precedence <= precedence {
                break;
            }

            left = self.parse_infix(left, current_precedence)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = token.location.clone();

        match token.kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::new(
                    ExpressionKind::Unary {
                        operator: token,
                        expression: Box::new(right),
                    },
                    location,
                ))
            }

            TokenKind::Identifier => {
                if self.current_is(TokenKind::Identifier)
                    && self.tokens[self.index..]
                        .iter()
                        .find(|t| t.kind != TokenKind::Identifier)
                        .is_some_and(|t| t.kind == TokenKind::Dollar)
                {
                    self.parse_lambda()
                } else {
                    self.advance();
                    let expression =
                        Expression::new(ExpressionKind::Identifier { token }, location);
                    self.parse_call(expression)
                }
            }

            TokenKind::LeftParenthesis => {
                self.advance();
                let expression = self.parse_expression(Precedence::None)?;
                if !self.current_is(TokenKind::RightParenthesis) {
                    return err!(
                        ErrorKind::MissingClosingParenthesis,
                        location,
                        "Expected a closing parenthesis.",
                    );
                }
                self.advance();
                Ok(expression)
            }
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
            TokenKind::If => {
                self.advance();
                self.parse_if()
            }
            _ => err!(
                ErrorKind::ExpectedExpression,
                location,
                "Unexpected token in expression."
            ),
        }
    }

    fn parse_infix(&mut self, left: Expression, precedence: Precedence) -> Result<Expression> {
        let operator = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        self.advance();

        let right = self.parse_expression(precedence)?;
        let location = operator.location.clone();

        Ok(Expression::new(
            ExpressionKind::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
            location,
        ))
    }

    fn parse_call(&mut self, mut expression: Expression) -> Result<Expression> {
        while !self.is_end_of_expression() {
            if let Some(current_token) = self.current() {
                match current_token.kind {
                    TokenKind::Identifier
                    | TokenKind::LeftParenthesis
                    | TokenKind::True
                    | TokenKind::False
                    | TokenKind::Null
                    | TokenKind::Float
                    | TokenKind::Integer
                    | TokenKind::String
                    | TokenKind::Underscore
                    | TokenKind::If => {
                        let argument = self.parse_expression(Precedence::Application)?;
                        let location = argument.location.clone();

                        expression = Expression::new(
                            ExpressionKind::Call {
                                function: Box::new(expression),
                                argument: Box::new(argument),
                            },
                            location,
                        );
                    }

                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn parse_if(&mut self) -> Result<Expression> {
        let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = token.location;
        let condition = self.parse_expression(Precedence::None)?;

        if !self.current_is(TokenKind::Then) {
            return err!(
                ErrorKind::IncompleteIf,
                location,
                "Missing then keyword after if condition."
            );
        }
        self.advance();
        let body = self.parse_expression(Precedence::None)?;

        let mut branches = vec![(Box::new(condition), Box::new(body))];
        while self.current_is(TokenKind::Elif) {
            self.advance();
            let condition = self.parse_expression(Precedence::None)?;

            if !self.current_is(TokenKind::Then) {
                return err!(
                    ErrorKind::IncompleteIf,
                    location,
                    "Missing then keyword after if condition."
                );
            }

            self.advance();

            let body = self.parse_expression(Precedence::None)?;
            branches.push((Box::new(condition), Box::new(body)));
        }
        if !self.current_is(TokenKind::Else) {
            return err!(
                ErrorKind::IncompleteIf,
                location,
                "Missing else branch in if expression."
            );
        }
        self.advance();

        Ok(Expression::new(
            ExpressionKind::If {
                branches,
                otherwise: Box::new(self.parse_expression(Precedence::None)?),
            },
            location,
        ))
    }
}
