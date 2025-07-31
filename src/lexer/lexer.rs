use std::vec;

use crate::lexer::token::{Kind, Token};

#[derive(Debug, PartialEq, Eq)]
enum NumberPart {
    Integer,
    Fractional,
}

pub fn lex(input: String) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let size = chars.len();

    let mut tokens: Vec<Token> = vec![];
    let mut pos: usize = 0;
    while pos < size {
        let curr = chars[pos];

        if curr.is_digit(10) {
            let mut part = NumberPart::Integer;

            let start = pos;
            let mut integer = 0;

            while pos < size && chars[pos].is_digit(10) {
                integer *= 10;
                integer += chars[pos] as i32 - '0' as i32;
                pos += 1;
            }

            if pos < size && chars[pos] == '.' {
                part = NumberPart::Fractional;
                pos += 1;
            }

            if part == NumberPart::Integer {
                let new_token = Token {
                    kind: Kind::Number(integer),
                    start: start,
                    end: pos,
                };
                tokens.push(new_token);
                continue;
            }

            let mut fractional = 0;
            while pos < size && chars[pos].is_digit(10) {
                fractional *= 10;
                fractional += chars[pos] as i32 - '0' as i32;
                pos += 1;
            }

            let new_token = Token {
                kind: Kind::Float(integer, fractional),
                start: start,
                end: pos,
            };
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
            _ => None,
        };

        if let Some(kind) = kind {
            tokens.push(Token {
                kind: kind,
                start: pos,
                end: pos,
            });
        }

        pos += 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right() {
        let input = String::from("12.3");
        let result = lex(input);
        assert_eq!(result[0].kind, Kind::Float(12, 3));
    }

    #[test]
    fn wrong() {
        let input = String::from("12.03");
        let result = lex(input);
        assert_eq!(result[0].kind, Kind::Float(12, 3));
    }
}
