use lexer::types::{IdentifierKind, OperatorKind, Token, TokenType};
use std::{iter::Peekable, mem::discriminant, slice::Iter};

use crate::types::{Declaration, Literal, Node, Program};

pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>) -> Parser<'a> {
        Parser {
            iter: tokens.iter().peekable().to_owned(),
        }
    }

    pub fn print_tokens(&mut self) {
        while let Some(token) = self.iter.next() {
            println!("{token}");
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        // while let Some(t) = self.iter.next() {

        // }
        while self.iter.len() > 0 {
            if let Some(n) = self.next_node() {
                program.push_node(n);
            }
        }
        program
    }

    pub fn next_node(&mut self) -> Option<Node> {
        let t = match self.iter.next() {
            Some(token) => token,
            None => panic!("Error reading token"),
        };
        match &t.token_type {
            TokenType::Identifiers(IdentifierKind::LET) => self.parser_declaration(),
            TokenType::Number(n) => self.parse_num(n),
            TokenType::String(n) => {
                return Some(Node::Literal(Literal::String(n.clone())));
            }
            _ => None,
        }
    }

    pub fn eat(&mut self) -> Option<&Token> {
        self.iter.next()
    }

    pub fn expect_token(&mut self, expected: TokenType) -> (bool, Option<&Token>) {
        if let Some(next) = self.iter.next() {
            if discriminant(&expected) == discriminant(&next.token_type) {
                return (true, Some(next));
            }

            panic!("Error on [{}, {}]: Unexpected token.", next.line, next.col,);
        }

        (false, None)
    }

    pub fn check_semicolon(&mut self) -> bool {
        match self.expect_token(TokenType::Punctuation(
            lexer::types::PunctuationKind::SEMICOLON,
        )) {
            (s, _) => s,
        }
    }

    pub fn parser_declaration(&mut self) -> Option<Node> {
        let mut symbol = String::new();
        let (mut success, next_token) = self.expect_token(TokenType::Name(String::new()));
        if !success {
            return None;
        }
        if let TokenType::Name(n) = &next_token.unwrap().token_type {
            symbol = n.clone();
        }

        (success, _) = self.expect_token(TokenType::Operators(OperatorKind::EQUAL));
        if !success {
            return None;
        }


        // get value
        let value = match self.next_node() {
            Some(n) => n,
            None => return None,
        };


        if !self.check_semicolon() {
            panic!("Expected semicolon");
        }

        Some(Node::Declaration(Declaration {
            symbol,
            value: Box::new(value),
        }))
    }

    pub fn parse_num(&mut self, literal: &String) -> Option<Node> {
        if let Ok(f) = literal.parse::<f32>() {
            return Some(Node::Literal(Literal::Float(f)));
        }
        None
    }
}

pub fn build_ast() {}
