use std::iter::Peekable;
use std::str::Chars;
use error::types::{CompilerError, ErrorTypes};
use crate::types::{Token, TokenType};

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    source_len: usize,
    pub tokens: Vec<Token>,
    current: usize,
    line: usize,
    col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Scanner<'a> {
        Scanner {
            source_len: source.len().to_owned(),
            source: source.chars().peekable().to_owned(),
            tokens: vec![],
            current: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn throw_scanner_error(&self, error_type: ErrorTypes, message: &str, col_adj: Option<usize>) {
        CompilerError::throw_new(
            self.line,
            self.col - col_adj.unwrap_or_else(|| 0),
            message,
            error_type, None,
        );
    }

    pub fn scan_tokens(&mut self) {
        while let Some(t) = self.next_token() {
            // push token t
            let _ = &self.tokens.push(t);
        }

        let _ = &self.tokens.push(
            Token::new(
                self.line,
                self.col,
                TokenType::EOF,
            )
        );
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }

        let mut c: char = '\0';
        let mut token_type: Option<TokenType> = None;

        // eat spaces and special characters
        while !self.is_at_end() {
            c = match self.advance_cursor() {
                Some(ch) => ch,
                None => return None,
            };

            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    c = '\0';
                    continue;
                }
                '/' => {
                    if self.is_line_comment(c) {
                        c = '\0';
                        self.absorb_line_comment();
                        continue;
                    } else {
                        break;
                    }
                }
                _ => break,
            };
        }

        if c == '\0' {
            return None;
        }

        if c.is_digit(10) { // check Number
            token_type = Some(self.number(c));
        } else if c == '"' { // check String
            token_type = Self::parse_string(self);
        } else if c == '\'' {
            token_type = Self::parse_char(self);
        } else if Self::possible_double(c) {
            match self.source.peek() {
                Some(nc) => {
                    // parse double
                    let mut double = String::from(c);
                    double.push_str(nc.to_string().as_str());
                    token_type = TokenType::new(&double);

                    if token_type.is_some() { self.advance_cursor(); }
                }
                None => {}
            };
        }

        if token_type.is_none() {
            // try to parse into token
            let eaten_string: String;
            (token_type, eaten_string) = Self::parse_token(self, c);

            if token_type.is_none() {
                // token was not found in enum
                // leftover string (eaten_string) is a Name
                token_type = Some(TokenType::Name(eaten_string));
            }
        }

        match token_type {
            Some(t) => {
                // create and push Token
                Some(
                    Token::new(
                        self.line,
                        self.col - t.to_literal().len(),
                        t,
                    )
                )
            }
            None => None,
        }
    }

    pub fn is_at_end(&self) -> bool {
        &self.current >= &self.source_len
    }

    fn advance_cursor(&mut self) -> Option<char> {
        self.current += 1;
        self.col += 1;

        let next = self.source.next();
        match next {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                }
            }
            None => {}
        }

        next
    }

    fn number(&mut self, current_char: char) -> TokenType {
        // peek at next value and continue stacking number string
        let mut s = String::from(current_char);

        while let Some(c) = self.source.peek() {
            // while peek is digit, iter and push to string
            if c.is_digit(10) || c == &'.' {
                s.push_str(self.advance_cursor().unwrap().to_string().as_str());
            } else {
                break;
            }
        };

        TokenType::Number(s.clone())
    }

    fn parse_char(&mut self) -> Option<TokenType> {
        let mut s = String::new();

        while let Some(c) = self.source.peek() {
            if c == &'\'' {
                // end of string
                //consume end quote
                let _ = self.advance_cursor();
                break;
            } else {
                s.push_str(self.advance_cursor().unwrap().to_string().as_str());
            }
        };

        if s.len() == 1 {
            Some(TokenType::Char(s.chars().nth(0).unwrap()))
        } else {
            self.throw_scanner_error(
                ErrorTypes::TokenizationError,
                "Unexpected character",
                Some(s.len()),
            );
            None
        }
    }

    fn parse_string(&mut self) -> Option<TokenType> {
        let mut s = String::new();

        while let Some(c) = self.source.peek() {
            if c == &'"' {
                // end of string
                //consume end quote
                let _ = self.advance_cursor();
                break;
            } else {
                s.push_str(self.advance_cursor().unwrap().to_string().as_str());
            }
        };

        Some(TokenType::String(s.clone()))
    }

    fn parse_token(&mut self, current_char: char)
                   -> (Option<TokenType>, String)
    {
        let mut s = String::from(current_char);
        let mut token_type: Option<TokenType> = None;

        // try to initially match current_char with TokenType
        match TokenType::new(&s) {
            Some(t) => {
                // found a token
                return (Some(t), s);
            }
            None => {
                // not a token so we do nothing
            }
        };


        // BUG: so we use peek, but this results in returning (None, s) if the last char is EOF
        // Solved by initially trying to match, which isn't horribly inefficient, but not pretty.
        while let Some(c) = self.source.peek() {
            let mut match_string: String = s.clone();
            match_string.push_str(c.to_string().as_str());
            // try to match s with TokenType
            match TokenType::new(&match_string) {
                Some(t) => {
                    // found a token
                    token_type = Some(t);
                    self.advance_cursor();
                    break;
                }
                None => {
                    // not a token so we do nothing
                }
            };

            // absorb spaces and chars
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance_cursor();
                    break;
                }
                '\n' => {
                    self.advance_cursor();
                    break;
                }
                _ => {}
            }

            // try to convert next char to TokenType
            match TokenType::new(&c.to_string()) {
                Some(_t) => {
                    // next char is a valid token, thus we break and return
                    break;
                }
                None => {
                    // next char invalid TokenType so we iter
                    s.push_str(self.advance_cursor().unwrap().to_string().as_str());
                }
            };
        };

        (token_type, s)
    }

    fn possible_double(c: char) -> bool {
        match c {
            '=' | '!' | '>' | '<' | '/' => true,
            _ => false,
        }
    }

    fn is_line_comment(&mut self, current_char: char) -> bool {
        let mut s = String::from(current_char);
        let next_char = self.source.peek().unwrap_or_else(|| &'\0');
        s.push_str(next_char.to_string().as_str());

        s == String::from("//")
    }

    fn absorb_line_comment(&mut self) {
        while let Some(c) = self.source.peek() {
            if c == &'\n' || c == &'\r' {
                // line comment finished
                break;
            }

            self.advance_cursor();
        };
    }
}