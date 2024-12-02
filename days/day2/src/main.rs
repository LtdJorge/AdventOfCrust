use common::{get_input, InputType};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    error::ErrorKind,
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::{
    cmp::{Ordering, PartialEq},
    path::PathBuf,
};

fn line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, sequence) = terminated(many1(terminated(digit1, space0)), tag("\n"))(input)?;
    let mut new_sequence = Vec::with_capacity(sequence.len());
    for entry in sequence {
        let num = entry
            .parse::<u32>()
            .map_err(|_| nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Digit)))?;
        new_sequence.push(num);
    }

    Ok((input, new_sequence))
}

#[derive(Debug, PartialEq)]
pub enum LevelSlope {
    Increasing,
    Decreasing,
}

pub struct State {
    previous: Option<u32>,
    slope: Option<LevelSlope>,
}

fn solve_part_1(input_type: InputType) -> anyhow::Result<usize> {
    let input = get_input(input_type)?;
    let (_, lines) = many1(line)(input.leak())?;

    let res = lines
        .into_iter()
        .filter(|line| {
            let length = line.len();

            let res = line
                .iter()
                .scan(
                    State {
                        previous: None,
                        slope: None,
                    },
                    |state, entry| {
                        match state.previous {
                            // First iteration
                            None => {
                                state.previous = Some(*entry);
                                Some(())
                            }
                            Some(previous) => {
                                state.previous = Some(*entry);
                                let entry = *entry;

                                match &state.slope {
                                    // Second iteration
                                    None => match previous.cmp(&entry) {
                                        Ordering::Less => {
                                            state.slope = Some(LevelSlope::Increasing)
                                        }
                                        Ordering::Greater => {
                                            state.slope = Some(LevelSlope::Decreasing)
                                        }
                                        Ordering::Equal => {
                                            return None;
                                        }
                                    },

                                    // All other iterations
                                    Some(slope) => match previous.cmp(&entry) {
                                        Ordering::Less => {
                                            if *slope != LevelSlope::Increasing {
                                                return None;
                                            }
                                        }
                                        Ordering::Greater => {
                                            if *slope != LevelSlope::Decreasing {
                                                return None;
                                            }
                                        }
                                        Ordering::Equal => {
                                            return None;
                                        }
                                    },
                                }
                                let distance = previous.abs_diff(entry);
                                match distance > 0 && distance <= 3 {
                                    true => Some(()),
                                    false => None,
                                }
                            }
                        }
                    },
                )
                .count();
            length == res
        })
        .count();

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    let input_type = InputType::Input(PathBuf::from("./days/day2/input.txt"));
    let part1_result = solve_part_1(input_type.clone())?;
    println!("Part 1: {}", part1_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::solve_part_1;
    use common::InputType;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let res = solve_part_1(InputType::Test)?;
        println!("{res}");
        Ok(())
    }
}
