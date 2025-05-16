#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    Percent,
    Ampersand,
    Caret,
    Pipe,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    At,
    Colon,
    Comma,
    Dollar,
    Dot,
    Hash,
    Newline,
    QuestionMark,
    Semicolon,
    Underscore,
    If,
    Then,
    Elif,
    Else,
    True,
    False,
    Null,
    Identifier,
    Float,
    Integer,
    String,
    Unknown,
}

impl TokenKind {
    pub const fn is_primary(self) -> bool {
        matches!(
            self,
            Self::True
                | Self::False
                | Self::LeftParenthesis
                | Self::Null
                | Self::Identifier
                | Self::Integer
                | Self::String
        )
    }

    pub const fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Ampersand
                | Self::Caret
                | Self::Pipe
                | Self::Plus
                | Self::Minus
                | Self::Star
                | Self::StarStar
                | Self::Slash
                | Self::Percent
                | Self::BangEqual
                | Self::Equal
                | Self::EqualEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
                | Self::At
                | Self::Colon
                | Self::Hash
        )
    }

    pub const fn can_start_expression(self) -> bool {
        matches!(
            self,
            Self::Identifier
                | Self::Bang
                | Self::Minus
                | Self::LeftParenthesis
                | Self::True
                | Self::False
                | Self::Null
                | Self::Float
                | Self::Integer
                | Self::String
                | Self::If
        )
    }
}
