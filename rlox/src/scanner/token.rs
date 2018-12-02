#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier,
    Str,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub contents: String,
    pub line: u32,
}

impl Token {
    pub fn from_type(kind: TokenType) -> Self {
        match kind {
            TokenType::LeftParen => Token::new(TokenType::LeftParen, "("),
            TokenType::RightParen => Token::new(TokenType::RightParen, ")"),
            TokenType::LeftBrace => Token::new(TokenType::LeftBrace, "{"),
            TokenType::RightBrace => Token::new(TokenType::RightBrace, "}"),

            TokenType::Comma => Token::new(TokenType::Comma, ","),
            TokenType::Dot => Token::new(TokenType::Dot, "."),
            TokenType::Minus => Token::new(TokenType::Minus, "-"),
            TokenType::Plus => Token::new(TokenType::Plus, "+"),
            TokenType::Semicolon => Token::new(TokenType::Semicolon, ";"),
            TokenType::Slash => Token::new(TokenType::Slash, "/"),
            TokenType::Star => Token::new(TokenType::Star, "*"),

            TokenType::Bang => Token::new(TokenType::Bang, "!"),
            TokenType::BangEqual => Token::new(TokenType::BangEqual, "!="),
            TokenType::Equal => Token::new(TokenType::Equal, "="),
            TokenType::EqualEqual => Token::new(TokenType::EqualEqual, "=="),
            TokenType::Greater => Token::new(TokenType::Greater, ">"),
            TokenType::GreaterEqual => Token::new(TokenType::GreaterEqual, ">="),
            TokenType::Less => Token::new(TokenType::Less, "<"),
            TokenType::LessEqual => Token::new(TokenType::LessEqual, "<="),

            TokenType::Identifier => Token::new(TokenType::Identifier, "i"),
            TokenType::Str => Token::new(TokenType::Str, ""),
            TokenType::Number => Token::new(TokenType::Number, "0"),

            TokenType::And => Token::new(TokenType::And, "and"),
            TokenType::Class => Token::new(TokenType::Class, "class"),
            TokenType::Else => Token::new(TokenType::Else, "else"),
            TokenType::False => Token::new(TokenType::False, "false"),
            TokenType::Fun => Token::new(TokenType::Fun, "fun"),
            TokenType::For => Token::new(TokenType::For, "for"),
            TokenType::If => Token::new(TokenType::If, "if"),
            TokenType::Nil => Token::new(TokenType::Nil, "nil"),
            TokenType::Or => Token::new(TokenType::Or, "or"),
            TokenType::Print => Token::new(TokenType::Print, "print"),
            TokenType::Return => Token::new(TokenType::Return, "return"),
            TokenType::Super => Token::new(TokenType::Super, "super"),
            TokenType::This => Token::new(TokenType::This, "this"),
            TokenType::True => Token::new(TokenType::True, "true"),
            TokenType::Var => Token::new(TokenType::Var, "var"),
            TokenType::While => Token::new(TokenType::While, "while"),

            TokenType::Error => Token::new(TokenType::Error, "error"),
        }
    }

    pub fn new(kind: TokenType, contents: &str) -> Self {
        Token::with_line(kind, contents, 1)
    }

    pub fn with_line(kind: TokenType, contents: &str, line: u32) -> Self {
        Token {
            kind: kind,
            contents: String::from(contents),
            line: line,
        }
    }
}


