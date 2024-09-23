use std::char;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve_part_1() -> i32 {
    let input = crate::utils::read_input_file("day_01.txt");

    input.lines().fold(0, |sum, current_line| {
        let first_num = current_line
            .chars()
            .find(|c| c.is_numeric())
            .expect("No numeric value found");
        let last_num = current_line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .expect("No numeric value found");

        combine_to_num(first_num, last_num) + sum
    })
}

fn combine_to_num(decimal: char, unit: char) -> i32 {
    let number = format!("{decimal}{unit}");
    number
        .parse()
        .expect("Characters entered were not parsable into i32")
}

pub fn solve_part_2() -> i32 {
    let input = crate::utils::read_input_file("day_01.txt");

    input.lines().fold(0, |sum, current_line| {
        let first_num = get_first_number(current_line);
        let last_num = get_last_number(current_line);
        sum + first_num * 10 + last_num
    })
}

fn get_first_number(original_line: &str) -> i32 {
    let mut line = original_line;

    loop {
        for (i, number) in NUMBERS.iter().enumerate() {
            if line.starts_with(number) || line.starts_with(char::from_digit(i as u32, 10).unwrap())
            {
                return i as i32;
            }
        }

        line = &line[1..];
        if line.is_empty() {
            panic!("No number found in line '{}'", original_line);
        }
    }
}

fn get_last_number(line: &str) -> i32 {
    let mut line = line;

    loop {
        for (i, number) in NUMBERS.iter().enumerate() {
            if line.ends_with(number) || line.ends_with(char::from_digit(i as u32, 10).unwrap()) {
                return i as i32;
            }
        }

        line = &line[0..line.len() - 1];
        if line.is_empty() {
            panic!("No number found in line");
        }
    }
}
