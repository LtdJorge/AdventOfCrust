mod lexer;
mod parser;
mod types;

use crate::{
    lexer::Token,
    types::{HigherNumbers, LowerNumbers, OrderedNumeric, Update},
};
use ahash::{HashMap, HashMapExt};
use chumsky::Parser;
use common::{get_input, InputType};
use logos::Logos;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let input_type = InputType::Input(PathBuf::from("./days/day5/input.txt"));
    let part1_result = solve_part_1(input_type.clone())?;
    println!("Part 1: {}", part1_result);
    let part2_result = solve_part_2(input_type)?;
    println!("Part 2: {}", part2_result);

    Ok(())
}

pub type OrderingMap = HashMap<usize, (LowerNumbers, HigherNumbers)>;

fn common_part(input_type: InputType) -> anyhow::Result<(Vec<Update>, OrderingMap)> {
    let input = get_input(input_type)?;
    let lexer = Token::lexer(input.as_str());
    let tokens = lexer.map(|lex| lex.unwrap()).collect::<Vec<_>>();
    let parsed = parser::parser().parse(tokens.as_slice());
    let (output, errors) = parsed.into_output_errors();

    if !errors.is_empty() {
        anyhow::bail!("Parsing errors: {:?}", errors);
    }

    let (ordering_rules, updates) = output.ok_or(anyhow::anyhow!("No output"))?;

    let mut map = HashMap::<usize, (LowerNumbers, HigherNumbers)>::new();

    // Fill up the hashmap
    ordering_rules.iter().for_each(|rule| {
        let left = rule.left;
        let right = rule.right;

        if let Some((_lower, higher)) = map.get_mut(&left) {
            higher.push(right);
        } else {
            map.insert(left, (vec![], vec![right]));
        };

        if let Some((lower, _higher)) = map.get_mut(&right) {
            lower.push(left);
        } else {
            map.insert(right, (vec![left], vec![]));
        }
    });

    Ok((updates, map))
}

fn solve_part_1(input_type: InputType) -> anyhow::Result<usize> {
    let (updates, map) = common_part(input_type)?;

    let result = updates
        .iter()
        // Just for debugging
        .enumerate()
        .filter(|(_index, update)| {
            let numerics = update
                .list
                .iter()
                .map(|item| {
                    let numeric = map.get(item).unwrap();
                    OrderedNumeric::new(*item, numeric)
                })
                .collect::<Vec<_>>();

            numerics.is_sorted()
        })
        .map(|(_, update)| {
            let middle = update.list.len() / 2;
            update.list[middle]
        })
        .sum::<usize>();

    Ok(result)
}

fn solve_part_2(input_type: InputType) -> anyhow::Result<usize> {
    let (mut updates, map) = common_part(input_type)?;

    let result = updates
        .iter_mut()
        .filter_map(|update| {
            let mut numerics = update
                .list
                .iter()
                .map(|item| {
                    let numeric = map.get(item).unwrap();
                    OrderedNumeric::new(*item, numeric)
                })
                .collect::<Vec<_>>();

            if !numerics.is_sorted() {
                numerics.sort_unstable();
                update.list = numerics
                    .iter()
                    .map(|ordered_numeric| ordered_numeric.numeric)
                    .collect();

                Some(update)
            } else {
                None
            }
        })
        .map(|update| {
            let middle = update.list.len() / 2;
            update.list[middle]
        })
        .sum::<usize>();

    Ok(result)
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
