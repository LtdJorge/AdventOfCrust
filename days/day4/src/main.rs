use common::{get_input, InputType};
use std::{path::PathBuf, sync::OnceLock};

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

pub fn look(direction: Direction, position: isize, count: isize, chars: &[char]) -> Option<char> {
    let (x, y) = one_dim_to_two(position);
    let row_len = *ROW_LEN.get_or_init(|| 9);
    match direction {
        Direction::Up => match y - count >= 0 {
            true => Some(chars[two_dim_to_one(x, y - count) as usize]),
            false => None,
        },
        Direction::Down => {
            let end = chars.len() - 1;
            let (_, end_pos_y) = one_dim_to_two(end as isize);

            match y + count <= end_pos_y {
                true => Some(chars[two_dim_to_one(x, y + count) as usize]),
                false => None,
            }
        }
        Direction::Left => match x - count >= 0 {
            true => Some(chars[two_dim_to_one(x - count, y) as usize]),
            false => None,
        },
        Direction::Right => match x + count <= row_len as isize - 1 {
            true => Some(chars[two_dim_to_one(x + count, y) as usize]),
            false => None,
        },
        Direction::UpLeft => match y - count >= 0 && x - count >= 0 {
            true => Some(chars[two_dim_to_one(x - count, y - count) as usize]),
            false => None,
        },
        Direction::UpRight => match y - count >= 0 && x + count <= row_len as isize - 1 {
            true => Some(chars[two_dim_to_one(x + count, y - count) as usize]),
            false => None,
        },
        Direction::DownLeft => {
            let end = chars.len() - 1;
            let (_, end_pos_y) = one_dim_to_two(end as isize);

            match y + count <= end_pos_y && x - count >= 0 {
                true => Some(chars[two_dim_to_one(x - count, y + count) as usize]),
                false => None,
            }
        }
        Direction::DownRight => {
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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
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
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    for (index, &(position, _)) in exes.iter().enumerate() {
        // Since Directions is Copy/Clone
        for direction in directions {
            if let Some(up_char) = look(direction, position as isize, 1, chars.as_slice()) {
                match up_char == 'M' {
                    true => match look(direction, position as isize, 2, chars.as_slice()) {
                        None => continue,
                        Some(up_char) => match up_char == 'A' {
                            true => match look(direction, position as isize, 3, chars.as_slice()) {
                                None => {}
                                Some(up_char) => match up_char == 'S' {
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

fn main() -> anyhow::Result<()> {
    let count = solve_part_1(InputType::Input(PathBuf::from("./days/day4/input.txt")))?;

    println!("{count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::solve_part_1;
    use common::InputType;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let count = solve_part_1(InputType::Test)?;

        println!("{count}");

        Ok(())
    }
}
