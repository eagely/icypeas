use super::{Expression, Located, Token, located::LocatedExt};

#[derive(Debug)]
pub enum Statement {
    Declaration {
        name: Located<Token>,
        types: Vec<Located<Token>>,
    },
    Definition {
        name: Located<Token>,
        parameter: Located<Token>,
        body: Located<Expression>,
    },
    Expression {
        expression: Located<Expression>,
    },
    Use {
        path: Vec<Located<Token>>,
    },
    Variable {
        name: Located<Token>,
        body: Located<Expression>,
    },
}

impl LocatedExt<Self> for Statement {
    fn at(self, location: std::rc::Rc<super::Location>) -> super::Located<Self> {
        Located {
            node: self,
            location,
        }
    }
}
