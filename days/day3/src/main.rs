mod lexer;
mod parser;

use crate::{
    lexer::Token,
    parser::{ParserState, ParserStatus},
};
use anyhow::anyhow;
use chumsky::Parser;
use common::{get_input, InputType};
use logos::Logos;
use std::path::PathBuf;

fn solve_part_1(input_type: InputType) -> anyhow::Result<u32> {
    let input = get_input(input_type)?;
    let lexer = Token::lexer(input.as_str());
    let tokens = lexer.map(|lex| lex.unwrap()).collect::<Vec<_>>();
    Ok(tokens
        .into_iter()
        .filter_map(|token| match token {
            Token::Mul((left, right)) => Some(left * right),
            _ => None,
        })
        .sum())
}

fn solve_part_2(input_type: InputType) -> anyhow::Result<u32> {
    let input = get_input(input_type)?;
    let lexer = Token::lexer(input.as_str());
    let tokens = lexer
        .map(|lex| lex.unwrap())
        .filter(|token| {
            #[cfg(feature = "debug")]
            {
                !matches!(token, Token::Garbage(_))
            }
            #[cfg(not(feature = "debug"))]
            {
                !matches!(token, Token::Garbage)
            }
        })
        .collect::<Vec<_>>();
    let mut state = ParserState {
        status: ParserStatus::Enabled,
    };
    let (values, _) = parser::parser()
        .parse_with_state(tokens.as_slice(), &mut state)
        .into_output_errors();
    let values = values.ok_or(anyhow!("Parser didn't return any values"))?;

    Ok(values.into_iter().sum())
}

fn main() -> anyhow::Result<()> {
    let input_type = InputType::Input(PathBuf::from("./days/day3/input.txt"));
    let part1_result = solve_part_1(input_type.clone())?;
    println!("Part 1: {}", part1_result);
    let part2_result = solve_part_2(input_type)?;
    println!("Part 2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};
    use common::InputType;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let res = solve_part_1(InputType::Test)?;
        println!("{}", res);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let res = solve_part_2(InputType::Test)?;
        println!("{}", res);
        Ok(())
    }
}
