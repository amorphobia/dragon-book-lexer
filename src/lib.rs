mod error;
mod id;
mod num;
mod token;

use crate::error::Error;
use crate::token::{Token, SYMBOLS};

#[derive(Debug, Clone, Copy)]
pub struct Lexer<'a> {
    input: &'a str,
    start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, start: 0 }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut result = Vec::new();
        println!("input len is {:?}", self.input.len());
        while self.start < self.input.len() {
            println!("    start is {:?}", self.start);
            result.push(self.scan()?);
            self.skip_blank_and_comments();
        }
        Ok(result)
    }

    pub(crate) fn scan(&mut self) -> Result<Token, Error> {
        self.skip_blank_and_comments();
        if self.peek_char().is_none() {
            return Err(Error::LexingIncomplete);
        }

        let peek = self.peek_char().unwrap();
        if peek.is_ascii_digit() {
            let mut frac = false;
            let start = self.start;
            while let Some(d) = self.peek_char() {
                match d {
                    d if d.is_ascii_digit() => {
                        self.next_char();
                    }
                    d if d == '.' => {
                        if frac {
                            break;
                        } else {
                            frac = true;
                            self.next_char();
                        }
                    }
                    _ => break,
                }
            }
            let end = self.start;
            self.start = start;

            if frac {
                self.parse_frac(end)
            } else {
                self.parse_int(end)
            }
        } else if peek.is_ascii_alphabetic() {
            let start = self.start;
            while let Some(a) = self.peek_char() {
                match a {
                    a if a.is_ascii_alphanumeric() => {
                        self.next_char();
                    }
                    _ => break,
                }
            }
            let end = self.start;
            self.start = start;
            Ok(self.parse_id(end))
        } else if "<=>!".contains(peek) {
            let mut s = String::from(peek);
            if let Some('=') = self.peek_char() {
                s.push('=');
            }
            SYMBOLS.get(&s).cloned().ok_or(Error::LexingIncomplete)
        } else {
            let t = self.next_char().unwrap() as u8;
            Ok(Token::Char(t))
        }
    }
}

// will switch to state machine in the future
impl<'a> Lexer<'a> {
    fn peek_char(&self) -> Option<char> {
        self.input[self.start..].chars().peekable().peek().cloned()
    }

    fn peek_two_chars(&self) -> (Option<char>, Option<char>) {
        let mut peekable = self.input[self.start..].chars().peekable();
        match peekable.next() {
            Some(c) => (Some(c), peekable.next()),
            None => (None, None),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let mut peekable = self.input[self.start..].char_indices().peekable();
        let next = peekable.next();
        match peekable.peek().cloned() {
            Some((pos, _)) => self.start += pos,
            None => self.start = self.input.len(),
        }
        next.map(|(_, c)| c)
    }
}

// will switch to state machine in the future
impl<'a> Lexer<'a> {
    fn skip_blank_and_comments(&mut self) {
        while let Some(peek) = self.peek_char() {
            match peek {
                ' ' | '\t' | '\r' | '\n' => {
                    self.next_char();
                }
                '/' => match self.peek_two_chars() {
                    (Some('/'), Some('/')) => {
                        self.next_char();
                        self.next_char();
                        self.skip_to_line_end();
                    }
                    (Some('/'), Some('*')) => {
                        self.next_char();
                        self.next_char();
                        self.skip_to_comment_close();
                    }
                    _ => break,
                },
                _ => break,
            }
        }
    }

    fn skip_to_line_end(&mut self) {
        while let Some(next) = self.next_char() {
            match next {
                '\n' => break,
                _ => continue,
            }
        }
    }

    fn skip_to_comment_close(&mut self) {
        // will skip to EOF if comment not closed
        while let (Some(p1), Some(p2)) = self.peek_two_chars() {
            match (p1, p2) {
                ('*', '/') => {
                    self.next_char();
                    self.next_char();
                    break;
                }
                _ => {
                    self.next_char();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Lexer;
    #[test]
    fn ascii() {
        let mut lexer = Lexer::new("ab");
        assert_eq!(lexer.start, 0);
        let a = lexer.next_char();
        assert_eq!(Some('a'), a);
        assert_eq!(lexer.start, 1);
        let b = lexer.peek_char();
        assert_eq!(Some('b'), b);
        assert_eq!(lexer.start, 1);
    }

    #[test]
    fn utf8() {
        let mut lexer = Lexer::new("一二");
        assert_eq!(lexer.start, 0);
        let one = lexer.next_char();
        assert_eq!(Some('一'), one);
        assert_eq!(lexer.start, 3);
        let two = lexer.peek_char();
        assert_eq!(Some('二'), two);
        assert_eq!(lexer.start, 3);
    }

    #[test]
    fn next_ascii() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.start, 0);
        lexer.next_char();
        assert_eq!(lexer.start, 1);
        lexer.next_char();
        assert_eq!(lexer.start, 2);
        lexer.next_char();
        assert_eq!(lexer.start, 3);
    }

    #[test]
    fn next_utf8() {
        let mut lexer = Lexer::new("一二三");
        assert_eq!(lexer.start, 0);
        lexer.next_char();
        assert_eq!(lexer.start, 3);
        lexer.next_char();
        assert_eq!(lexer.start, 6);
        lexer.next_char();
        assert_eq!(lexer.start, 9);
    }
}
