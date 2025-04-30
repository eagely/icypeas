use crate::lexer::enums::TokenKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Comparison,
    Term,
    Factor,
    Exponentiation,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Primary,
}

impl From<TokenKind> for Precedence {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Equal => Self::Assignment,
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
            TokenKind::At | TokenKind::Colon | TokenKind::Hash => Self::Primary,
            _ => Self::None,
        }
    }
}
