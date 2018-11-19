use std::{iter, str};

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

    fn new(kind: TokenType, contents: &str) -> Self {
        Token::with_line(kind, contents, 1)
    }

    fn with_line(kind: TokenType, contents: &str, line: u32) -> Self {
        Token {
            kind: kind,
            contents: String::from(contents),
            line: line,
        }
    }
}

pub struct Scanner<'a> {
    contents: iter::Peekable<str::Chars<'a>>,
    buffer: String,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            contents: source.chars().peekable(),
            buffer: String::new(),
            line: 1,
        }
    }

    fn make_token(&self, kind: TokenType, contents: &str) -> Token {
        Token::with_line(kind, contents, self.line)
    }

    fn default_token(&self, kind: TokenType) -> Token {
        let tok = Token::from_type(kind);
        Token {
            line: self.line,
            ..tok
        }
    }

    fn word_starting_with(&mut self, ch: char) -> Token {
        self.buffer.push(ch);
        self.get_rest_of_word();
        match ch {
            'a' if self.buffer == "and" => self.default_token(TokenType::And),
            'c' if self.buffer == "class" => self.default_token(TokenType::Class),
            'e' if self.buffer == "else" => self.default_token(TokenType::Else),
            'f' if self.buffer == "false" => self.default_token(TokenType::False),
            'f' if self.buffer == "for" => self.default_token(TokenType::For),
            'f' if self.buffer == "fun" => self.default_token(TokenType::Fun),
            'i' if self.buffer == "if" => self.default_token(TokenType::If),
            'n' if self.buffer == "nil" => self.default_token(TokenType::Nil),
            'o' if self.buffer == "or" => self.default_token(TokenType::Or),
            'p' if self.buffer == "print" => self.default_token(TokenType::Print),
            'r' if self.buffer == "return" => self.default_token(TokenType::Return),
            's' if self.buffer == "super" => self.default_token(TokenType::Super),
            't' if self.buffer == "this" => self.default_token(TokenType::This),
            't' if self.buffer == "true" => self.default_token(TokenType::True),
            'v' if self.buffer == "var" => self.default_token(TokenType::Var),
            'w' if self.buffer == "while" => self.default_token(TokenType::While),
            _ => Token::with_line(TokenType::Identifier, &self.buffer, self.line),
        }
    }

    fn get_rest_of_number(&mut self) {
        let mut dot = false;
        let mut ch_ = self.contents.peek().map(|&c| c);
        while let Some(ch) = ch_ {
            match ch {
                '0'...'9' => self.buffer.push(ch),
                '.' if !dot => {
                    dot = true;
                    self.buffer.push(ch);
                }
                _ => break,
            }
            let _ = self.contents.next();
            ch_ = self.contents.peek().map(|&c| c);
        }
    }

    fn get_rest_of_word(&mut self) {
        let mut ch_ = self.contents.peek().map(|&c| c);
        while let Some(ch) = ch_ {
            match ch {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                    self.buffer.push(ch);
                }
                _ => break,
            }
            let _ = self.contents.next(); // the value we just pushed
            ch_ = self.contents.peek().map(|&c| c); // set up for next iteration
        }
    }
}

impl<'a> iter::Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        while let Some(ch) = self.contents.next() {
            match ch {
                '(' => return Some(self.default_token(TokenType::LeftParen)),
                ')' => return Some(self.default_token(TokenType::RightParen)),
                '{' => return Some(self.default_token(TokenType::LeftBrace)),
                '}' => return Some(self.default_token(TokenType::RightBrace)),
                ',' => return Some(self.default_token(TokenType::Comma)),
                '.' => return Some(self.default_token(TokenType::Dot)),
                '-' => return Some(self.default_token(TokenType::Minus)),
                '+' => return Some(self.default_token(TokenType::Plus)),
                ';' => return Some(self.default_token(TokenType::Semicolon)),
                '/' => return Some(self.default_token(TokenType::Slash)),
                '*' => return Some(self.default_token(TokenType::Star)),
                '!' => {
                    if let Some('=') = self.contents.peek() {
                        let _ = self.contents.next();
                        return Some(self.default_token(TokenType::BangEqual));
                    } else {
                        return Some(self.default_token(TokenType::Bang));
                    }
                }
                '=' => {
                    if let Some('=') = self.contents.peek() {
                        let _ = self.contents.next();
                        return Some(self.default_token(TokenType::EqualEqual));
                    } else {
                        return Some(self.default_token(TokenType::Equal));
                    }
                }
                '>' => {
                    if let Some('=') = self.contents.peek() {
                        let _ = self.contents.next();
                        return Some(self.default_token(TokenType::GreaterEqual));
                    } else {
                        return Some(self.default_token(TokenType::Greater));
                    }
                }
                '<' => {
                    if let Some('=') = self.contents.peek() {
                        let _ = self.contents.next();
                        return Some(self.default_token(TokenType::LessEqual));
                    } else {
                        return Some(self.default_token(TokenType::Less));
                    }
                }
                '"' => {
                    while let Some(c) = self.contents.next() {
                        if c == '"' {
                            break;
                        }
                        self.buffer.push(c);
                    }
                    return Some(self.make_token(TokenType::Str, &self.buffer));
                }
                'a'...'z' | 'A'...'Z' | '_' => return Some(self.word_starting_with(ch)),
                // zero should not start an integer
                '0' => {
                    self.buffer.push(ch);
                    self.get_rest_of_number();
                    return if self.buffer.len() == 1 || self.buffer.contains('.') {
                        Some(self.make_token(TokenType::Number, &self.buffer))
                    } else {
                        Some(self.make_token(TokenType::Error, "Integer must not begin with '0'"))
                    }
                }
                '1'...'9' => {
                    self.buffer.push(ch);
                    self.get_rest_of_number();
                    return Some(self.make_token(TokenType::Number, &self.buffer));
                }
                ' ' | '\t' => {} // skip quietly
                '\n' => {
                    self.line += 1;
                }
                _ => {
                    return Some(self.make_token(
                        TokenType::Error,
                        "Unexpected character encountered",
                    ))
                }
            }
        }
        None
    }
}

impl<'a> iter::FusedIterator for Scanner<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    fn token_sequence_test(scanner: &mut Scanner, tokens: Vec<Token>) {
        for token in tokens {
            assert_eq!(scanner.next(), Some(token));
        }
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_recognize_single_token() {
        let mut scanner = Scanner::new("=");
        token_sequence_test(&mut scanner, vec![Token::from_type(TokenType::Equal)]);
    }

    #[test]
    fn scanner_can_recognize_multiple_tokens() {
        let mut scanner = Scanner::new("><");
        token_sequence_test(
            &mut scanner,
            vec![
                Token::from_type(TokenType::Greater),
                Token::from_type(TokenType::Less),
            ],
        )
    }

    #[test]
    fn scanner_can_skip_whitespace() {
        let mut scanner = Scanner::new("! !");
        token_sequence_test(
            &mut scanner,
            vec![
                Token::from_type(TokenType::Bang),
                Token::from_type(TokenType::Bang),
            ],
        );
    }

    #[test]
    fn scanner_can_recognize_all_simple_tokens() {
        let mut scanner = Scanner::new(">= <= > < = == - + * ! . , ; () {}");
        token_sequence_test(
            &mut scanner,
            vec![
                Token::from_type(TokenType::GreaterEqual),
                Token::from_type(TokenType::LessEqual),
                Token::from_type(TokenType::Greater),
                Token::from_type(TokenType::Less),
                Token::from_type(TokenType::Equal),
                Token::from_type(TokenType::EqualEqual),
                Token::from_type(TokenType::Minus),
                Token::from_type(TokenType::Plus),
                Token::from_type(TokenType::Star),
                Token::from_type(TokenType::Bang),
                Token::from_type(TokenType::Dot),
                Token::from_type(TokenType::Comma),
                Token::from_type(TokenType::Semicolon),
                Token::from_type(TokenType::LeftParen),
                Token::from_type(TokenType::RightParen),
                Token::from_type(TokenType::LeftBrace),
                Token::from_type(TokenType::RightBrace),
            ],
        );
    }

    #[test]
    fn scanner_can_recognize_string_literal() {
        let mut scanner = Scanner::new("\"hello\"");
        token_sequence_test(&mut scanner, vec![Token::new(TokenType::Str, "hello")]);
    }

    #[test]
    fn scanner_can_recognize_identifier() {
        let mut scanner = Scanner::new("id");
        token_sequence_test(&mut scanner, vec![Token::new(TokenType::Identifier, "id")]);
    }

    #[test]
    fn scanner_can_recognize_true() {
        let mut scanner = Scanner::new("true");
        token_sequence_test(&mut scanner, vec![Token::from_type(TokenType::True)]);
    }

    #[test]
    fn scanner_can_recognize_super() {
        let mut scanner = Scanner::new("super");
        token_sequence_test(&mut scanner, vec![Token::from_type(TokenType::Super)]);
    }

    #[test]
    fn scanner_does_not_recognize_super_inside_superb() {
        let mut scanner = Scanner::new("superb");
        token_sequence_test(
            &mut scanner,
            vec![Token::new(TokenType::Identifier, "superb")],
        );
    }

    #[test]
    fn scanner_can_recognize_all_keywords() {
        let mut scanner = Scanner::new(
            "and class else false fun for if nil or print return super this true var while",
        );
        token_sequence_test(
            &mut scanner,
            vec![
                Token::from_type(TokenType::And),
                Token::from_type(TokenType::Class),
                Token::from_type(TokenType::Else),
                Token::from_type(TokenType::False),
                Token::from_type(TokenType::Fun),
                Token::from_type(TokenType::For),
                Token::from_type(TokenType::If),
                Token::from_type(TokenType::Nil),
                Token::from_type(TokenType::Or),
                Token::from_type(TokenType::Print),
                Token::from_type(TokenType::Return),
                Token::from_type(TokenType::Super),
                Token::from_type(TokenType::This),
                Token::from_type(TokenType::True),
                Token::from_type(TokenType::Var),
                Token::from_type(TokenType::While),
            ],
        )
    }

    #[test]
    fn scanner_can_recognize_numbers() {
        let mut scanner = Scanner::new("12345");
        token_sequence_test(&mut scanner, vec![Token::new(TokenType::Number, "12345")]);
    }

    #[test]
    fn scanner_can_recognize_floating_point_numbers() {
        let mut scanner = Scanner::new("987.654");
        token_sequence_test(&mut scanner, vec![Token::new(TokenType::Number, "987.654")]);
    }

    #[test]
    fn scanner_fails_on_zero() {
        let mut scanner = Scanner::new("0");
        let token = scanner.next().unwrap();
        assert_eq!(token.kind, TokenType::Error);
        assert_eq!(scanner.next(), None);
    }
}
