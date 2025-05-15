use super::{Located, Token, located::LocatedExt};

#[derive(Clone, Debug)]
pub enum Expression {
    Unary {
        operator: Located<Token>,
        expression: Box<Located<Expression>>,
    },
    Binary {
        left: Box<Located<Expression>>,
        operator: Located<Token>,
        right: Box<Located<Expression>>,
    },
    Call {
        function: Box<Located<Expression>>,
        argument: Box<Located<Expression>>,
    },
    Identifier {
        token: Located<Token>,
    },
    If {
        branches: Vec<(Box<Located<Expression>>, Box<Located<Expression>>)>,
        otherwise: Box<Located<Expression>>,
    },
    Lambda {
        parameter: Located<Token>,
        body: Box<Located<Expression>>,
    },
    Literal {
        token: Located<Token>,
    },
}

impl LocatedExt<Self> for Expression {
    fn at(self, location: std::rc::Rc<super::Location>) -> Located<Self> {
        Located {
            node: self,
            location,
        }
    }
}
