use common::{get_input, InputType};
use logos::{Lexer, Logos};
use std::{num::ParseIntError, path::PathBuf};

#[derive(Clone, Debug, Logos)]
#[logos(error = CustomError)]
pub enum Token {
    #[regex("mul\\([0-9]{1,3},[0-9]{1,3}\\)", mul)]
    Mul((u32, u32)),
    #[regex(r#"[\u0000-\u007F]"#, |lex| String::from(lex.slice()))]
    Garbage(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomError {
    message: String,
}

impl From<ParseIntError> for CustomError {
    fn from(_value: ParseIntError) -> Self {
        CustomError {
            message: String::from("ParseIntError"),
        }
    }
}

fn mul(lex: &mut Lexer<Token>) -> Result<(u32, u32), ParseIntError> {
    let end = lex.slice().len();
    let nums = &lex.slice()[4..end - 1];
    let pair = nums.split(',').collect::<Vec<_>>();
    let num1 = pair[0].parse()?;
    let num2 = pair[1].parse()?;
    Ok((num1, num2))
}

fn solve_part_1(input_type: InputType) -> anyhow::Result<u32> {
    let input = get_input(input_type)?;
    let lexer = Token::lexer(input.as_str());
    let tokens = lexer.map(|lex| lex.unwrap()).collect::<Vec<_>>();
    Ok(tokens
        .into_iter()
        .filter_map(|token| match token {
            Token::Mul((left, right)) => Some(left * right),
            Token::Garbage(_) => None,
        })
        .sum())
}

fn main() -> anyhow::Result<()> {
    let res = solve_part_1(InputType::Input(PathBuf::from("./days/day3/input.txt")))?;
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::solve_part_1;
    use common::InputType;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let res = solve_part_1(InputType::Test)?;
        println!("{}", res);
        Ok(())
    }
}
