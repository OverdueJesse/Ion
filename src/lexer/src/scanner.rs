use crate::types::{Token, TokenType};
use error::types::{CompilerError, ErrorTypes};
use std::iter::Peekable;
use std::str::Chars;

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

    pub fn throw_scanner_error(
        &self,
        error_type: ErrorTypes,
        message: &str,
        col_adj: Option<usize>,
    ) {
        CompilerError::throw_new(
            self.line,
            self.col - col_adj.unwrap_or_else(|| 0),
            message,
            error_type,
            None,
        );
    }

    pub fn scan_tokens(&mut self) {
        while let Some(t) = self.next_token() {
            // push token t
            let _ = &self.tokens.push(t);
        }

        let _ = &self
            .tokens
            .push(Token::new(self.line, self.col, TokenType::EOF));
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }

        let mut c: char = '\0';
        let mut token_type: Option<TokenType>;

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
                '#' => {
                    c = '\0';
                    self.absorb_block_comment();
                    continue;
                }
                _ => break,
            };
        }

        if c == '\0' {
            return None;
        }

        token_type = self.parse_singleton(c);

        if c.is_digit(10) {
            // check Number
            token_type = Some(self.parse_number(c));
        } else if c == '"' {
            // check String
            token_type = Self::parse_string(self);
        } else if c == '\'' {
            token_type = Self::parse_char(self);
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
                Some(Token::new(self.line, self.col - t.to_literal().len(), t))
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

    fn parse_double(&mut self, current: char) -> Option<TokenType> {
        let mut token_type: Option<TokenType> = None;
        match self.source.peek() {
            Some(nc) => {
                let mut double = String::from(current);
                double.push_str(nc.to_string().as_str());
                token_type = TokenType::new(&double);

                if token_type.is_some() {
                    self.advance_cursor();
                }
            }
            None => {}
        }

        token_type
    }

    fn parse_number(&mut self, current_char: char) -> TokenType {
        // peek at next value and continue stacking number string
        let mut s = String::from(current_char);

        while let Some(c) = self.source.peek() {
            let ch = c.clone();
            let dot: char;

            if ch == '.' {
                dot = self.advance_cursor().unwrap();
                match self.parse_double(dot) {
                    Some(t) => {
                        let _ = &self.tokens.push(Token::new(
                            self.line,
                            self.col - s.len(),
                            TokenType::Number(s.clone()),
                        ));

                        return t;
                    }
                    None => {}
                }
            }

            if ch.is_digit(10) || ch == '.' {
                s.push_str(self.advance_cursor().unwrap().to_string().as_str());
            } else {
                break;
            }
        }

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
        }

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
        }

        Some(TokenType::String(s.clone()))
    }

    fn parse_token(&mut self, current_char: char) -> (Option<TokenType>, String) {
        let mut s = String::from(current_char);
        let mut token_type: Option<TokenType> = None;
        let mut longest_match: usize = 0;

        while let Some(c) = self.source.peek() {
            let next_char = c.clone();

            let keyword_match = TokenType::new(&s);
            if let Some(t) = keyword_match {
                if s.len() >= longest_match {
                    token_type = Some(t);
                    longest_match = s.len();
                }
            }

            let name_match = Some(TokenType::Name(s.clone()));
            if s.len() > longest_match {
                token_type = name_match;
                longest_match = s.len();
            }

            // try to match next char alone
            let next_char_type = TokenType::new(&next_char.to_string());
            if Self::stop_char(&next_char) || next_char_type.is_some() {
                break;
            }

            s.push(self.advance_cursor().unwrap());
        }

        if self.source.peek().is_none() {
            if let Some(t) = TokenType::new(&s) {
                token_type = Some(t);
            } else {
                token_type = Some(TokenType::Name(s.clone()));
            }
        }

        (token_type, s)
    }

    fn is_double(c: &char) -> bool {
        match c {
            '!' | '=' | '>' | '<' | '.' | ':' | '|' | '&' | '-' => true,
            _ => false,
        }
    }

    fn parse_singleton(&mut self, c: char) -> Option<TokenType> {
        let mut double = None;
        let single = TokenType::new(&c.to_string());

        if Self::is_double(&c) {
            double = self.parse_double(c);
        }

        if double.is_some() {
            return double;
        } else if single.is_some() {
            return single;
        }

        None
    }

    fn stop_char(c: &char) -> bool {
        match c {
            '\n' | '\r' | ' ' | '.' => true,
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
        }
    }

    fn absorb_block_comment(&mut self) {
        while let Some(c) = self.source.peek() {
            if c == &'#' {
                let _ = self.advance_cursor();
                break;
            } else {
                self.advance_cursor();
            }
        }
    }
}
