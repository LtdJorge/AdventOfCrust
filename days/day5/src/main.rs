mod lexer;
mod parser;
mod types;

use crate::{lexer::Token, types::OrderedNumeric};
use ahash::{HashMap, HashMapExt};
use chumsky::Parser;
use common::{get_input, InputType};
use logos::Logos;
use std::path::PathBuf;
use types::{HigherNumbers, LowerNumbers};

fn main() -> anyhow::Result<()> {
    let input_type = InputType::Input(PathBuf::from("./days/day5/input.txt"));
    let part1_result = solve_part_1(input_type.clone())?;
    println!("Part 1: {}", part1_result);

    Ok(())
}

fn solve_part_1(input_type: InputType) -> anyhow::Result<usize> {
    let input = get_input(input_type)?;
    let lexer = Token::lexer(input.as_str());
    let tokens = lexer.map(|lex| lex.unwrap()).collect::<Vec<_>>();
    let mut map = HashMap::<usize, (LowerNumbers, HigherNumbers)>::new();
    let parsed = parser::parser().parse(tokens.as_slice());
    let (ordering_rules, updates) = parsed.output().unwrap();

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
