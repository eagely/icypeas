#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Identifier(String),
    Boolean(bool),
    Float(f64),
    Integer(i128),
    String(String),
    Use(String),
    Unknown(char),
    None,
}
