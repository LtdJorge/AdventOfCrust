use common::{get_input, InputType};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::map_res,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use std::path::PathBuf;

fn pair(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (left, _, right)) = tuple((
        map_res(digit1, |a: &str| a.parse::<i32>()),
        space0,
        map_res(digit1, |a: &str| a.parse::<i32>()),
    ))(input)?;
    Ok((input, (left, right)))
}

fn line(input: &str) -> IResult<&str, (i32, i32)> {
    terminated(pair, tag("\n"))(input)
}

fn tokens(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(line)(input)
}

fn solve_part_1(input_type: InputType) -> anyhow::Result<u32> {
    let input = get_input(input_type)?;
    let input: &'static str = input.leak();
    let (_, tokens) = tokens(input)?;

    let (mut left_tokens, mut right_tokens): (Vec<_>, Vec<_>) = tokens.into_iter().unzip();

    left_tokens.sort_unstable_by(Ord::cmp);
    right_tokens.sort_unstable_by(Ord::cmp);

    let result: u32 = left_tokens
        .into_iter()
        .zip(right_tokens)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    Ok(result)
}

fn solve_part_2(input_type: InputType) -> anyhow::Result<usize> {
    let input = get_input(input_type)?;
    let input: &'static str = input.leak();
    let (_, tokens) = tokens(input)?;

    let (left_tokens, right_tokens): (Vec<_>, Vec<_>) = tokens.into_iter().unzip();

    let result: usize = left_tokens
        .into_iter()
        .map(|token| {
            let count = right_tokens
                .iter()
                .filter(|&&other_token| other_token == token)
                .count();
            count * token as usize
        })
        .sum();

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let input_type = InputType::Input(PathBuf::from("./days/day1/input.txt"));
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
        let res = solve_part_1(InputType::Test);
        println!("{}", res?);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let res = solve_part_2(InputType::Test);
        println!("{}", res?);
        Ok(())
    }
}
