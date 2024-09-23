pub fn solve_part_1() -> i32 {
    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    let input = crate::utils::read_input_file("day_02.txt");
    let mut current_game_number = 0;
    input.lines().fold(0, |sum, current_line| {
        current_game_number += 1;
        let result: Vec<bool> = current_line
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(';')
            .flat_map(|hand_content| {
                hand_content.trim().split(',').map(|color| {
                    let (num, color) = color.trim().split_once(' ').unwrap();
                    let num: i32 = num.parse().expect("Wrong format!");
                    match color {
                        "red" => num <= RED,
                        "green" => num <= GREEN,
                        "blue" => num <= BLUE,
                        _ => panic!("Color didn't match red, green or blue"),
                    }
                })
            })
            .collect();

        if result.contains(&false) {
            sum
        } else {
            sum + current_game_number
        }
    })
}
