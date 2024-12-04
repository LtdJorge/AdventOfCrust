use common::{get_input, InputType};
use std::{ops::AddAssign, path::PathBuf, sync::OnceLock};

static ROW_LEN: OnceLock<usize> = OnceLock::new();

pub fn one_dim_to_two(position: isize) -> (isize, isize) {
    let row_len = *ROW_LEN.get_or_init(|| 9);
    let x = position % row_len as isize;
    let y = position / row_len as isize;
    (x, y)
}

pub fn two_dim_to_one(x: isize, y: isize) -> isize {
    let row_len = *ROW_LEN.get_or_init(|| 9);
    y * row_len as isize + x
}

pub fn look(direction: Direction1, position: isize, count: isize, chars: &[char]) -> Option<char> {
    let (x, y) = one_dim_to_two(position);
    let row_len = *ROW_LEN.get_or_init(|| 9);
    match direction {
        Direction1::Up => match y - count >= 0 {
            true => Some(chars[two_dim_to_one(x, y - count) as usize]),
            false => None,
        },
        Direction1::Down => {
            let end = chars.len() - 1;
            let (_, end_pos_y) = one_dim_to_two(end as isize);

            match y + count <= end_pos_y {
                true => Some(chars[two_dim_to_one(x, y + count) as usize]),
                false => None,
            }
        }
        Direction1::Left => match x - count >= 0 {
            true => Some(chars[two_dim_to_one(x - count, y) as usize]),
            false => None,
        },
        Direction1::Right => match x + count <= row_len as isize - 1 {
            true => Some(chars[two_dim_to_one(x + count, y) as usize]),
            false => None,
        },
        Direction1::UpLeft => match y - count >= 0 && x - count >= 0 {
            true => Some(chars[two_dim_to_one(x - count, y - count) as usize]),
            false => None,
        },
        Direction1::UpRight => match y - count >= 0 && x + count <= row_len as isize - 1 {
            true => Some(chars[two_dim_to_one(x + count, y - count) as usize]),
            false => None,
        },
        Direction1::DownLeft => {
            let end = chars.len() - 1;
            let (_, end_pos_y) = one_dim_to_two(end as isize);

            match y + count <= end_pos_y && x - count >= 0 {
                true => Some(chars[two_dim_to_one(x - count, y + count) as usize]),
                false => None,
            }
        }
        Direction1::DownRight => {
            let end = chars.len() - 1;
            let (_, end_pos_y) = one_dim_to_two(end as isize);

            match y + count <= end_pos_y && x + count <= row_len as isize - 1 {
                true => Some(chars[two_dim_to_one(x + count, y + count) as usize]),
                false => None,
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction1 {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Clone, Copy)]
pub enum Direction2 {
    LtrUpDown,
    LtrDownUp,
    RtlUpDown,
    RtlDownUp,
}

fn solve_part_1(input_type: InputType) -> anyhow::Result<usize> {
    let input = get_input(input_type)?;
    let len = input.find('\n').ok_or(anyhow::anyhow!("\\n not found"))?;
    let input = input.lines().collect::<String>();
    ROW_LEN.get_or_init(|| len);
    let chars = input.chars().collect::<Vec<_>>();
    let exes = chars
        .iter()
        .enumerate()
        .filter(|(_, &char)| char == 'X')
        .collect::<Vec<_>>();

    let mut counters: Vec<(usize, usize)> = vec![(0, 0); exes.len()];

    let directions = [
        Direction1::Up,
        Direction1::Down,
        Direction1::Left,
        Direction1::Right,
        Direction1::UpLeft,
        Direction1::UpRight,
        Direction1::DownLeft,
        Direction1::DownRight,
    ];

    for (index, &(position, _)) in exes.iter().enumerate() {
        // Since Directions is Copy/Clone
        for direction in directions {
            if let Some(found_char) = look(direction, position as isize, 1, chars.as_slice()) {
                match found_char == 'M' {
                    true => match look(direction, position as isize, 2, chars.as_slice()) {
                        None => continue,
                        Some(found_char) => match found_char == 'A' {
                            true => match look(direction, position as isize, 3, chars.as_slice()) {
                                None => {}
                                Some(found_char) => match found_char == 'S' {
                                    true => {
                                        let mut counter = counters[index];
                                        counter.0 = position;
                                        counter.1 += 1;
                                        counters[index] = counter;
                                    }
                                    false => continue,
                                },
                            },
                            false => continue,
                        },
                    },
                    false => continue,
                }
            }
        }
    }
    let counted: usize = counters.iter().map(|(_, count)| *count).sum();

    Ok(counted)
}

fn compute_diag(
    dir_first: Direction1,
    dir_second: Direction1,
    position: usize,
    chars: &[char],
    counter: &mut usize,
) {
    if let Some(found_char) = look(dir_first, position as isize, 1, chars) {
        match found_char == 'M' {
            true => {
                if let Some(found_char) = look(dir_second, position as isize, 1, chars) {
                    match found_char == 'S' {
                        true => {
                            counter.add_assign(1);
                        }
                        false => {}
                    }
                }
            }
            false => {}
        }
    }
}

fn solve_part_2(input_type: InputType) -> anyhow::Result<usize> {
    let input = get_input(input_type)?;
    let len = input.find('\n').ok_or(anyhow::anyhow!("\\n not found"))?;
    let input = input.lines().collect::<String>();
    ROW_LEN.get_or_init(|| len);
    let chars = input.chars().collect::<Vec<_>>();
    let letter_as = chars
        .iter()
        .enumerate()
        .filter(|(_, &char)| char == 'A')
        .collect::<Vec<_>>();
    let mut counters: Vec<usize> = vec![0; letter_as.len()];

    for (index, &(position, _)) in letter_as.iter().enumerate() {
        let mut local_counter = 0_usize;
        compute_diag(
            Direction1::UpLeft,
            Direction1::DownRight,
            position,
            chars.as_slice(),
            &mut local_counter,
        );
        compute_diag(
            Direction1::DownLeft,
            Direction1::UpRight,
            position,
            chars.as_slice(),
            &mut local_counter,
        );
        compute_diag(
            Direction1::UpRight,
            Direction1::DownLeft,
            position,
            chars.as_slice(),
            &mut local_counter,
        );
        compute_diag(
            Direction1::DownRight,
            Direction1::UpLeft,
            position,
            chars.as_slice(),
            &mut local_counter,
        );

        if local_counter >= 2 {
            counters[index].add_assign(1);
        }
    }

    let counted: usize = counters.iter().copied().sum();

    Ok(counted)
}

fn main() -> anyhow::Result<()> {
    let input_type = InputType::Input(PathBuf::from("./days/day4/input.txt"));
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
        let count = solve_part_1(InputType::Test)?;

        println!("{count}");

        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let count = solve_part_2(InputType::Test)?;

        println!("{count}");

        Ok(())
    }
}
