use std::vec;

use crate::lexer::token::{Token, Kind};

pub fn lex(input: String) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let size = chars.len();

    let mut tokens: Vec<Token> = vec![];
    let mut pos: usize = 0;
    while pos < size {
        let curr = chars[pos];

        if curr.is_digit(10) {
            let start = pos;
            let mut num = 0;
            while pos < size && chars[pos].is_digit(10) {
                num *= 10;
                num += chars[pos] as i32 - '0' as i32;
                pos += 1;
            }

            let new_token = Token { kind: Kind::Number(num), start: start, end: pos };
            tokens.push(new_token);
            continue;
        }

        
        let kind = match curr {
            '+' => Some(Kind::Plus),
            '-' => Some(Kind::Minus),
            '*' => Some(Kind::Mul),
            '/' => Some(Kind::Div),
            '(' => Some(Kind::LeftParen),
            ')' => Some(Kind::RightParen),
            _ => None
        };

        if let Some(kind) = kind {
            tokens.push(Token { kind: kind, start: pos, end: pos });
        }

        pos += 1;
    }

    tokens
}
