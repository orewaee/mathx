#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Number(i32),
    Float(i32, i32),
    Plus,
    Minus,
    Mul,
    Div,
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}
