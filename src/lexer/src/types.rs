use std::fmt::{self, Debug, Display, Formatter};

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(line: usize, col: usize, token_type: TokenType) -> Token {
        Token {
            literal: token_type.to_literal().clone(),
            token_type,
            line,
            col,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_literal(&self) -> &String {
        &self.literal
    }

    pub fn get_line(&self) -> &usize {
        &self.line
    }

    pub fn get_col(&self) -> &usize {
        &self.col
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s: String = String::new();
        s.push_str(format!("[LINE: {}, COL: {}] ", self.line, self.col).as_str());
        s.push_str(self.token_type.to_string().as_str());

        write!(f, "{}", s)
    }
}

impl TokenType {
    pub fn to_literal(&self) -> String {
        match &self {
            TokenType::Punctuation(p) => p.to_string(),
            TokenType::Operators(o) => o.to_string(),
            TokenType::Identifiers(i) => i.to_string(),
            TokenType::Type(t) => t.to_string(),
            TokenType::Bool(b) => b.to_string(),

            TokenType::Name(s) | TokenType::String(s) | TokenType::Number(s) => s.clone(),

            TokenType::Char(c) => c.to_string(),
            TokenType::EOF => String::from("EOF"),
        }
    }

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
                ":" => TokenType::Punctuation(PunctuationKind::COLON),
                "?" => TokenType::Punctuation(PunctuationKind::QUESTION),
                "-" => TokenType::Operators(OperatorKind::MINUS),
                "+" => TokenType::Operators(OperatorKind::PLUS),
                "/" => TokenType::Operators(OperatorKind::SLASH),
                "*" => TokenType::Operators(OperatorKind::STAR),
                "!" => TokenType::Operators(OperatorKind::BANG),
                "=" => TokenType::Operators(OperatorKind::EQUAL),
                ">" => TokenType::Operators(OperatorKind::GREATER),
                "<" => TokenType::Operators(OperatorKind::LESS),
                "|" => TokenType::Operators(OperatorKind::BAR),
                _ => return None,
            },
            2 => match token.as_str() {
                "!=" => TokenType::Operators(OperatorKind::BangEqual),
                "==" => TokenType::Operators(OperatorKind::EqualEqual),
                ">=" => TokenType::Operators(OperatorKind::GreaterEqual),
                "<=" => TokenType::Operators(OperatorKind::LessEqual),
                "=>" => TokenType::Operators(OperatorKind::ARROW),
                "->" => TokenType::Operators(OperatorKind::SingleArrow),
                ".." => TokenType::Operators(OperatorKind::SPREAD),
                ":=" => TokenType::Operators(OperatorKind::ColonEqual),
                "&&" => TokenType::Operators(OperatorKind::AND),
                "||" => TokenType::Operators(OperatorKind::OR),
                "if" => TokenType::Identifiers(IdentifierKind::IF),
                "on" => TokenType::Identifiers(IdentifierKind::ON),
                "fn" => TokenType::Identifiers(IdentifierKind::FN),
                _ => return None,
            },
            3 => match token.as_str() {
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
                "match" => TokenType::Identifiers(IdentifierKind::MATCH),
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    COLON,
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
            PunctuationKind::COLON => String::from(":"),
            PunctuationKind::QUESTION => String::from("?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    ARROW,
    SPREAD,
    ColonEqual,
    AND,
    OR,
    BAR,
    SingleArrow,
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
            OperatorKind::ARROW => String::from("=>"),
            OperatorKind::SPREAD => String::from(".."),
            OperatorKind::ColonEqual => String::from(":="),
            OperatorKind::AND => String::from("&&"),
            OperatorKind::OR => String::from("||"),
            OperatorKind::BAR => String::from("|"),
            OperatorKind::SingleArrow => String::from("->"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierKind {
    IF,
    ON,
    FN,
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
    MATCH,
}

impl IdentifierKind {
    pub fn to_string(&self) -> String {
        match &self {
            IdentifierKind::IF => String::from("if"),
            IdentifierKind::ON => String::from("on"),
            IdentifierKind::FN => String::from("fn"),
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
            IdentifierKind::MATCH => String::from("match"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
            TypeKind::CHAR => String::from("char"),
            TypeKind::BOOL => String::from("bool"),
        }
    }
}
