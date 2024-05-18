use lexer::types::{IdentifierKind, Token, TokenType};
use std::{iter::Peekable, slice::Iter};

use crate::types::{Node, Program};

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
        println!("printtokens:");
        while let Some(token) = self.iter.next() {
            println!("{token}");
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut program = Program::new();
        while let Some(t) = self.iter.next() {
           let n: Option<Node> = match t.token_type {
                TokenType::Identifiers(IdentifierKind::LET) => Some(self.parser_assignment(t)),
                _ => None
            }; 
            if let Some(node) = n {
                program.push_node(node);
            }
        }
       program 
    }

    pub fn eat(&mut self) -> Option<&Token> {
        self.iter.next()
    }

    pub fn expect(&mut self, expected: TokenType) -> (bool, Option<&Token>) {
        if let Some(next) = self.iter.next() {
            if next.token_type == expected {
                return (true, Some(next));
            }
        }

        return (false, None);
    }

    pub fn parser_assignment(&self, token: &Token) -> Node {
        println!("{token}");
        Node::Literal(crate::types::Literal::String(String::from("Hello World")))
    }
}

pub fn build_ast() {}
