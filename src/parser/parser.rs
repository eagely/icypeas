use crate::err;
use crate::error::{ErrorKind, Result};
use crate::model::{Expression, Located, LocatedExt, Location, Statement, Token, TokenKind};
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
    tokens: Vec<Located<Token>>,
    index: usize,
}

impl Parser {
    pub const fn new() -> Self {
        Self {
            tokens: vec![],
            index: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Located<Token>>) -> Result<Vec<Located<Statement>>> {
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

            if !self.is_eof() && !try_consume_any!(self, TokenKind::Newline, TokenKind::Semicolon) {
                let location = self
                    .current()
                    .ok_or(ErrorKind::UnexpectedEndOfFile)?
                    .location;
                return err!(
                    ErrorKind::UnexpectedToken,
                    location,
                    "Expected a newline or semicolon."
                );
            }

            statements.push(statement);
        }
        Ok(statements)
    }

    fn current(&self) -> Option<Located<Token>> {
        self.tokens.get(self.index).cloned()
    }

    fn next(&self, n: usize) -> Option<Located<Token>> {
        self.tokens.get(self.index + n).cloned()
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        self.current().is_some_and(|t| t.node.kind == kind)
    }

    fn next_is(&self, n: usize, kind: TokenKind) -> bool {
        self.next(n).is_some_and(|t| t.node.kind == kind)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn is_end_of_expression(&self) -> bool {
        self.is_eof()
            || self.current_is(TokenKind::Newline)
            || self.current_is(TokenKind::Semicolon)
    }

    fn parse_statement(&mut self) -> Result<Located<Statement>> {
        self.parse_declaration()
    }

    fn parse_declaration(&mut self) -> Result<Located<Statement>> {
        if !self.next_is(1, TokenKind::Colon) {
            return self.parse_definition();
        }

        let name = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = name.location.clone();

        match name.node.kind {
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
                Ok(Statement::Declaration { name, types }.at(location))
            }
            _ => err!(
                ErrorKind::ExpectedExpression,
                location,
                "This should be an identifier.",
            ),
        }
    }

    fn parse_definition(&mut self) -> Result<Located<Statement>> {
        if !self.current_is(TokenKind::Identifier)
            || self.tokens[self.index..]
                .iter()
                .find(|t| !t.node.kind.is_primary())
                .is_none_or(|t| t.node.kind != TokenKind::Equal)
        {
            let expression = self.parse_expression(Precedence::None)?;
            let location = expression.location.clone();
            return Ok(Statement::Expression { expression }.at(location));
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
            if !t.node.kind.is_primary() {
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
        name: Located<Token>,
        parameters: Vec<Located<Token>>,
        body: Located<Expression>,
        location: Rc<Location>,
    ) -> Result<Located<Statement>> {
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
            curried_lambda = Expression::Lambda {
                parameter,
                body: Box::new(curried_lambda),
            }
            .at(location.clone());
        }

        Ok(Statement::Definition {
            name,
            parameter: first,
            body: curried_lambda,
        }
        .at(location))
    }

    fn parse_lambda(&mut self) -> Result<Located<Expression>> {
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
            curried_lambda = Expression::Lambda {
                parameter,
                body: Box::new(curried_lambda),
            }
            .at(location.clone());
        }

        Ok(curried_lambda)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Located<Expression>> {
        let mut left = self.parse_prefix()?;

        while !self.is_eof() && !self.is_end_of_expression() {
            if self.current_is(TokenKind::If) && Precedence::Conditional > precedence {
                self.advance();
                left = self.parse_if()?;
                continue;
            }

            if let Some(token) = self.current() {
                let current_precedence = Precedence::from(token.node.kind);
                if current_precedence > precedence {
                    left = self.parse_infix(left, current_precedence)?;
                    continue;
                }
            }

            if self
                .current()
                .is_some_and(|t| t.node.kind.can_start_expression())
                && !self.current().is_some_and(|t| t.node.kind.is_operator())
                && Precedence::Application > precedence
            {
                let location = left.location.clone();
                left = Expression::Call {
                    function: Box::new(left),
                    argument: Box::new(self.parse_prefix()?),
                }
                .at(location);
                continue;
            }

            break;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Located<Expression>> {
        let token = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        let location = token.location.clone();

        match token.node.kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::Unary {
                    operator: token,
                    expression: Box::new(right),
                }
                .at(location))
            }

            TokenKind::Identifier => {
                if self.current_is(TokenKind::Identifier)
                    && self.tokens[self.index..]
                        .iter()
                        .find(|t| t.node.kind != TokenKind::Identifier)
                        .is_some_and(|t| t.node.kind == TokenKind::Dollar)
                {
                    self.parse_lambda()
                } else {
                    self.advance();
                    Ok(Expression::Identifier { token }.at(location))
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
                Ok(Expression::Literal { token }.at(location))
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

    fn parse_infix(
        &mut self,
        left: Located<Expression>,
        precedence: Precedence,
    ) -> Result<Located<Expression>> {
        let operator = self.current().ok_or(ErrorKind::UnexpectedEndOfFile)?;
        self.advance();

        let right = self.parse_expression(precedence)?;
        let location = operator.location.clone();

        Ok(Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
        .at(location))
    }

    fn parse_if(&mut self) -> Result<Located<Expression>> {
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

        Ok(Expression::If {
            branches,
            otherwise: Box::new(self.parse_expression(Precedence::None)?),
        }
        .at(location))
    }
}
