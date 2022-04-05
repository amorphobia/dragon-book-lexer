use internship::IStr;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Num {
    /// Integer part
    pub int: u32,
    /// Fractional part
    pub frac: (u32, usize),
    /// Exponent
    pub exp: i64,
    /// Base
    pub base: u8,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // symbols
    Lt,     // <
    Assign, // =
    Gt,     // >
    Not,    // !
    Le,     // <=
    Eq,     // ==
    Ge,     // >=
    Ne,     // !=

    NumLit(Num),
    // use `String` for now
    Id(IStr),

    // keywords
    TRUE,
    FLASE,

    // temporary enum variant
    Char(u8),
}

pub const SYMBOLS: phf::Map<&'static str, Token> = phf::phf_map! {
    "<" => Token::Lt,
    "=" => Token::Assign,
    ">" => Token::Gt,
    "!" => Token::Not,
    "<=" => Token::Ne,
    "==" => Token::Eq,
    ">=" => Token::Ge,
    "!=" => Token::Ne,
};
