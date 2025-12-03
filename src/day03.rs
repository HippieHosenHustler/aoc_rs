use crate::file_helpers::read_lines;

pub fn solve(input_file: &str) {
    let mut total_joltage = 0;

    let lines = read_lines(input_file).expect("Could not read lines from file");

    for line in lines {
        let line = line.expect("Could not read line from file");
        total_joltage += calculate_maximum_joltage(line);
    }

    println!("Total maximum joltage (part 1 solution): {}", total_joltage);
}

fn calculate_maximum_joltage(bank: String) -> i32 {
    let batteries = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut first_val = 0;
    let mut second_val = 0;
    let mut first_index = 0;

    for battery in &batteries[0..batteries.len() - 1] {
        if battery > &first_val {
            first_val = battery.clone();
            first_index = batteries.iter().position(|&x| x == *battery).unwrap();
        }
    }

    for battery in &batteries[first_index + 1..] {
        if battery > &second_val {
            second_val = battery.clone();
        }
    }

    //combine the two digits into one number
    return first_val * 10 + second_val;
}
