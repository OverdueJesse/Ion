use std::fmt::{self, Debug, Display, Formatter};

const TOKEN_MAX_LEN: i32 = 6;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
    col: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize, col: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            col,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn get_literal(&self) -> &String {
        &self.literal
    }

    pub fn get_line(&self) -> &usize {
        &self.line
    }

    pub fn get_col(&self) -> &usize { &self.col }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s: String = String::new();
        s.push_str(format!("[LINE: {}, ", self.line).as_str());
        s.push_str(format!("COL: {}] ", self.col).as_str());
        s.push_str(self.token_type.to_string().as_str());
        s.push_str(&self.literal);

        write!(f, "{}", s)
    }
}


impl TokenType {
    pub fn new(token: &String) -> Option<TokenType> {
        let t = match token.len() {
            1 => match token.as_str() {
                "(" => TokenType::Punctuation(PunctuationKind::LPAREN),
                ")" => TokenType::Punctuation(PunctuationKind::RPAREN),
                "[" => TokenType::Punctuation(PunctuationKind::LBRACE),
                "]" => TokenType::Punctuation(PunctuationKind::RBRACE),
                "{" => TokenType::Punctuation(PunctuationKind::LCURL),
                "}" => TokenType::Punctuation(PunctuationKind::RCURL),
                "," => TokenType::Punctuation(PunctuationKind::COMMA),
                "." => TokenType::Punctuation(PunctuationKind::DOT),
                ";" => TokenType::Punctuation(PunctuationKind::SEMICOLON),
                "?" => TokenType::Punctuation(PunctuationKind::QUESTION),
                "-" => TokenType::Operators(OperatorKind::MINUS),
                "+" => TokenType::Operators(OperatorKind::PLUS),
                "/" => TokenType::Operators(OperatorKind::SLASH),
                "*" => TokenType::Operators(OperatorKind::STAR),
                "!" => TokenType::Operators(OperatorKind::BANG),
                "=" => TokenType::Operators(OperatorKind::EQUAL),
                ">" => TokenType::Operators(OperatorKind::GREATER),
                "<" => TokenType::Operators(OperatorKind::LESS),
                _ => return None,
            },
            2 => match token.as_str() {
                "!=" => TokenType::Operators(OperatorKind::BangEqual),
                "==" => TokenType::Operators(OperatorKind::EqualEqual),
                ">=" => TokenType::Operators(OperatorKind::GreaterEqual),
                "<=" => TokenType::Operators(OperatorKind::LessEqual),
                "if" => TokenType::Identifiers(IdentifierKind::IF),
                "or" => TokenType::Identifiers(IdentifierKind::OR),
                "on" => TokenType::Identifiers(IdentifierKind::ON),
                "fn" => TokenType::Identifiers(IdentifierKind::FN),
                _ => return None
            },
            3 => match token.as_str() {
                "and" => TokenType::Identifiers(IdentifierKind::AND),
                "for" => TokenType::Identifiers(IdentifierKind::FOR),
                "let" => TokenType::Identifiers(IdentifierKind::LET),
                "nil" => TokenType::Identifiers(IdentifierKind::NIL),
                _ => return None,
            },
            4 => match token.as_str() {
                "else" => TokenType::Identifiers(IdentifierKind::ELSE),
                "this" => TokenType::Identifiers(IdentifierKind::THIS),
                "enum" => TokenType::Identifiers(IdentifierKind::ENUM),
                "impl" => TokenType::Identifiers(IdentifierKind::IMPL),
                "bool" => TokenType::Type(TypeKind::BOOL),
                "char" => TokenType::Type(TypeKind::CHAR),
                "true" => TokenType::Bool(true),
                _ => return None,
            },
            5 => match token.as_str() {
                "super" => TokenType::Identifiers(IdentifierKind::SUPER),
                "while" => TokenType::Identifiers(IdentifierKind::WHILE),
                "false" => TokenType::Bool(false),
                _ => return None,
            },
            6 => match token.as_str() {
                "struct" => TokenType::Identifiers(IdentifierKind::STRUCT),
                "return" => TokenType::Identifiers(IdentifierKind::RETURN),
                "shared" => TokenType::Identifiers(IdentifierKind::SHARED),
                "number" => TokenType::Type(TypeKind::NUMBER),
                "string" => TokenType::Type(TypeKind::STRING),
                _ => return None,
            },
            _ => return None,
        };

        Some(t)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Matched here
    Punctuation(PunctuationKind),
    Operators(OperatorKind),
    Identifiers(IdentifierKind),
    Type(TypeKind),

    // Matched elsewhere
    Name(String),
    Char(char),
    Number(String),
    Bool(bool),
    String(String),

    EOF,
}

#[derive(Debug, Clone)]
pub enum PunctuationKind {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LCURL,
    RCURL,
    COMMA,
    DOT,
    SEMICOLON,
    QUESTION,
}

impl PunctuationKind {
    pub fn to_string(&self) -> String {
        match &self {
            PunctuationKind::LPAREN => String::from("("),
            PunctuationKind::RPAREN => String::from(")"),
            PunctuationKind::LBRACE => String::from("["),
            PunctuationKind::RBRACE => String::from("]"),
            PunctuationKind::LCURL => String::from("{"),
            PunctuationKind::RCURL => String::from("}"),
            PunctuationKind::COMMA => String::from(","),
            PunctuationKind::DOT => String::from("."),
            PunctuationKind::SEMICOLON => String::from(","),
            PunctuationKind::QUESTION => String::from("?"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperatorKind {
    SLASH,
    STAR,
    PLUS,
    MINUS,
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
}

impl OperatorKind {
    pub fn to_string(&self) -> String {
        match &self {
            OperatorKind::MINUS => String::from("-"),
            OperatorKind::PLUS => String::from("+"),
            OperatorKind::SLASH => String::from("/"),
            OperatorKind::STAR => String::from("*"),
            OperatorKind::BANG => String::from("!"),
            OperatorKind::EQUAL => String::from("="),
            OperatorKind::GREATER => String::from(">"),
            OperatorKind::LESS => String::from("<"),
            OperatorKind::BangEqual => String::from("!="),
            OperatorKind::EqualEqual => String::from("=="),
            OperatorKind::GreaterEqual => String::from(">="),
            OperatorKind::LessEqual => String::from("<="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IdentifierKind {
    IF,
    OR,
    ON,
    FN,
    AND,
    FOR,
    LET,
    NIL,
    ELSE,
    THIS,
    STRUCT,
    SUPER,
    WHILE,
    RETURN,
    ENUM,
    IMPL,
    SHARED,
}

impl IdentifierKind {
    pub fn to_string(&self) -> String {
        match &self {
            IdentifierKind::IF => String::from("if"),
            IdentifierKind::OR => String::from("or"),
            IdentifierKind::ON => String::from("on"),
            IdentifierKind::FN => String::from("fn"),
            IdentifierKind::AND => String::from("and"),
            IdentifierKind::FOR => String::from("for"),
            IdentifierKind::LET => String::from("let"),
            IdentifierKind::NIL => String::from("nil"),
            IdentifierKind::ELSE => String::from("else"),
            IdentifierKind::THIS => String::from("this"),
            IdentifierKind::STRUCT => String::from("struct"),
            IdentifierKind::SUPER => String::from("super"),
            IdentifierKind::WHILE => String::from("while"),
            IdentifierKind::RETURN => String::from("return"),
            IdentifierKind::ENUM => String::from("enum"),
            IdentifierKind::IMPL => String::from("impl"),
            IdentifierKind::SHARED => String::from("shared"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    STRING,
    NUMBER,
    CHAR,
    BOOL,
}

impl TypeKind {
    pub fn to_string(&self) -> String {
        match &self {
            TypeKind::STRING => String::from("string"),
            TypeKind::NUMBER => String::from("number"),
            TypeKind::CHAR   => String::from("char"),
            TypeKind::BOOL   => String::from("bool"),
        }
    }
}