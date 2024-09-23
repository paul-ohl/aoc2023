#![allow(unused)]

use crate::utils::{read_input_file, Matrix};

pub fn solve_part_1() -> i32 {
    let schema = read_input_file("day_03.txt");
    let mut valid_numbers: Vec<i32> = Vec::new();

    schema
        .clone()
        .lines()
        .enumerate()
        .for_each(|(y_coord, line)| {
            let mut start = 0;
            loop {
                let (idx, num): (Vec<usize>, String) = line
                    .chars()
                    .enumerate()
                    .skip(start)
                    .skip_while(|(_, c)| !c.is_numeric())
                    .take_while(|(_, c)| c.is_numeric())
                    .unzip();
                if num.is_empty() {
                    break;
                }
                let x_coord = idx[0];
                if has_symbol_near(&schema, x_coord, y_coord, num.len()) {
                    valid_numbers.push(num.parse().unwrap());
                }
                start = x_coord + num.len();
            }
        });
    valid_numbers.iter().sum()
}

fn has_symbol_near(schema: &str, x: usize, y: usize, num: usize) -> bool {
    let width = schema.lines().next().unwrap().len();
    let height = schema.lines().count();

    let min_x = if x == 0 { 0 } else { x - 1 };
    let max_x = if x + num >= width { width - 1 } else { x + num };
    let min_y = if y == 0 { 0 } else { y - 1 };
    let max_y = if y + 1 >= height { height - 1 } else { y + 1 };

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = schema.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if c != '.' && !c.is_numeric() {
                return true;
            }
        }
    }
    false
}

pub fn solve_part_2() -> i32 {
    let schema = read_input_file("day_03.txt");
    let matrix = Matrix::from(schema);

    let gears = matrix.find_all_coords('*');

    gears
        .into_iter()
        .map(|(x, y)| {
            let adjacent_nums = matrix.get_adjacent_if(x, y, |c| c.is_numeric());
            let mut nums = Vec::new();
            for i in 0..adjacent_nums.len() {
                let (current_x, current_y) = adjacent_nums[i];
                let (next_x, next_y) = adjacent_nums[(i + 1) % adjacent_nums.len()];
                if (current_y != next_y || next_x != current_x + 1) {
                    nums.push(adjacent_nums[i]);
                }
            }
            if nums.len() != 2 {
                return 0;
            }
            nums.iter()
                .map(|(x, y)| matrix.number_from_coord(*x, *y))
                .product::<i32>()
        })
        .sum()
}
