use internship::IStr;

use crate::{token::Token, Lexer};

impl<'a> Lexer<'a> {
    #[inline]
    pub(crate) fn parse_id(&mut self, end: usize) -> Token {
        debug_assert!(end <= self.input.len());
        let id = &self.input[self.start..end];
        self.start = end;
        KEYWORDS
            .get(id)
            .cloned()
            .unwrap_or(Token::Id(IStr::new(id)))
    }
}

const KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "true" => Token::TRUE,
    "false" => Token::FLASE,
};
