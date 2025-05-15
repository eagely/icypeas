use crate::model::TokenKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Definition,
    Conditional,
    Comparison,
    Term,
    Factor,
    Exponentiation,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Prefix,
    Application,
}

impl From<TokenKind> for Precedence {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Equal => Self::Definition,
            TokenKind::If => Self::Conditional,
            TokenKind::BangEqual
            | TokenKind::EqualEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => Self::Comparison,
            TokenKind::Plus | TokenKind::Minus => Self::Term,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Self::Factor,
            TokenKind::StarStar => Self::Exponentiation,
            TokenKind::Pipe => Self::BitwiseOr,
            TokenKind::Caret => Self::BitwiseXor,
            TokenKind::Ampersand => Self::BitwiseAnd,
            _ => Self::None,
        }
    }
}
