use crate::{
    error::Error,
    token::{Num, Token},
    Lexer,
};

impl<'a> Lexer<'a> {
    #[inline]
    pub(crate) fn parse_int(&mut self, end: usize) -> Result<Token, Error> {
        debug_assert!(end <= self.input.len());
        let int = &self.input[self.start..end];
        let int = u32::from_str_radix(int, 10)?;
        self.start = end;
        Ok(Token::NumLit(Num {
            int,
            frac: (0, 0),
            exp: 1,
            base: 10,
        }))
    }

    #[inline]
    pub(crate) fn parse_frac(&mut self, end: usize) -> Result<Token, Error> {
        debug_assert!(end <= self.input.len());
        let dot = self.input[self.start..end]
            .chars()
            .position(|c| c == '.')
            .unwrap_or(end - self.start)
            + self.start;

        let int = &self.input[self.start..dot];
        let int = u32::from_str_radix(int, 10)?;

        let frac = if dot == end {
            ("0", 0)
        } else {
            (&self.input[dot + 1..end], end - dot - 1)
        };
        let frac = (u32::from_str_radix(frac.0, 10)?, frac.1);
        self.start = end;
        Ok(Token::NumLit(Num {
            int,
            frac,
            exp: 1,
            base: 10,
        }))
    }
}
