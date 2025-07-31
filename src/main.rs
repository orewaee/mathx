use std::collections::HashMap;
use std::io;

use clap::Parser;

use crate::{
    cli::Cli,
    lexer::{
        lexer::lex,
        token::{Kind, Token},
    },
    math::calc,
    queue::{list::ListQueue, queue::Queue},
    stack::{list::ListStack, stack::Stack},
};

mod cli;
mod lexer;
mod list;
mod math;
mod queue;
mod stack;

// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
// https://brilliant.org/wiki/shunting-yard-algorithm/

fn shunting_yard(input: String) -> ListQueue<Token> {
    let tokens = lex(input);
    let mut output_queue: ListQueue<Token> = ListQueue::new();
    let mut operator_stack: ListStack<Token> = ListStack::new();

    let priority = HashMap::from([
        (Kind::Plus, 1),
        (Kind::Minus, 1),
        (Kind::Mul, 2),
        (Kind::Div, 2),
    ]);

    for token in tokens {
        match token.kind {
            Kind::Number(_) => output_queue.push(token.clone()),
            Kind::Plus | Kind::Minus | Kind::Mul | Kind::Div => {
                while let Some(top) = operator_stack.peek() {
                    if priority.get(&top.kind).unwrap_or(&0)
                        >= priority.get(&token.kind).unwrap_or(&0)
                    {
                        if let Some(op) = operator_stack.pop() {
                            output_queue.push(op);
                        }
                    } else {
                        break;
                    }
                }

                operator_stack.push(token);
            }
            Kind::LeftParen => operator_stack.push(token),
            Kind::RightParen => {
                while let Some(top) = operator_stack.peek() {
                    if !matches!(top.kind, Kind::LeftParen) {
                        if let Some(op) = operator_stack.pop() {
                            output_queue.push(op);
                        }
                    } else {
                        break;
                    }
                }

                operator_stack.pop();
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue
}

fn main() {
    // let mut buf = String::new();

    // io::stdin()
    //     .read_line(&mut buf)
    //     .expect("failed to read expression");

    let cli = Cli::parse();
    match &cli.command {
        cli::Commands::Calc { exp } => {
            let exp = exp.clone();

            let mut output_queue = shunting_yard(exp);
            output_queue.println();

            match calc(&mut output_queue) {
                Ok(result) => println!("result: {}", result),
                Err(err) => println!("err: {}", err),
            }
        }
    }
}
