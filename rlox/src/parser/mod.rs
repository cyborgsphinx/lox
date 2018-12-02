use self::scanner::Scanner;
use self::scanner::token::Token;

use std::collections::HashMap;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }
}
