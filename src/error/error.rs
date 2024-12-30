#[derive(Debug)]
pub enum Error {
    ExpectedExpression,
    InvalidIdentifier,
    MissingClosingParenthesis,
    NotANumber,
    UnexpectedEndOfFile,
    UnterminatedString,
}
