use crate::{
    lexer::token::{Kind, Token},
    queue::{list::ListQueue, queue::Queue},
    stack::{list::ListStack, stack::Stack},
};

pub fn calc(output_queue: &mut ListQueue<Token>) -> Result<f64, String> {
    let mut nums: ListStack<f64> = ListStack::new();

    while let Some(token) = output_queue.pop() {
        match token.kind {
            Kind::Number(num) => {
                nums.push(num as f64);
            }
            Kind::Float(integer, fractional, scale) => {
                let mut value = integer as f64;
                let mut temp = fractional;
                let mut scale = scale;
                while temp > 0 {
                    value += 0.1_f64.powi(scale) * (temp % 10) as f64;
                    scale -= 1;
                    temp /= 10;
                }

                nums.push(value);
            }
            Kind::Plus | Kind::Minus | Kind::Mul | Kind::Div => {
                let a = nums.pop().ok_or("missing operand")?;
                let b = nums.pop().ok_or("missing operand")?;

                let result = match token.kind {
                    Kind::Plus => b + a,
                    Kind::Minus => b - a,
                    Kind::Mul => b * a,
                    Kind::Div => {
                        if a == 0.0 {
                            return Err("division by zero".to_string());
                        }
                        b / a
                    }
                    _ => return Err(format!("invalid operator: {:?}", token.kind)),
                };
                nums.push(result);
            }
            _ => return Err(format!("invalid token: {:?}", token.kind)),
        }
    }

    nums.pop()
        .ok_or("no result: empty stack".to_string())
        .and_then(|result| {
            if nums.len() == 0 {
                Ok(result)
            } else {
                Err("invalid expression: too many operands".to_string())
            }
        })
}
