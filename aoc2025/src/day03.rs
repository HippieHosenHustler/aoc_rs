use shared::lines_to_vec;

pub fn solve(input: &str) -> (String, String) {
    let mut part1 = 0;
    let mut part2: i128 = 0;

    let lines = lines_to_vec(input);

    for line in lines {
        let line = line;
        part1 += calculate_maximum_joltage(&line);
        part2 += calculate_overdrive_joltage(&line);
    }

    (part1.to_string(), part2.to_string())
}

fn calculate_maximum_joltage(bank: &String) -> i32 {
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

fn calculate_overdrive_joltage(bank: &String) -> i128 {
    let batteries = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i128)
        .collect::<Vec<i128>>();

    let mut digits: Vec<i128> = vec![];

    let mut starting_index: usize = 0;

    for i in (0..=11).rev() {
        let mut current_largest = 0;
        let mut largest_index = starting_index;

        for (idx, battery) in batteries[starting_index..(batteries.len() - i)]
            .iter()
            .enumerate()
        {
            if battery > &current_largest {
                current_largest = battery.clone();
                largest_index = starting_index + idx;
            }
        }

        digits.push(current_largest);
        starting_index = largest_index + 1;
    }

    // combine all the digits into one number
    let mut result: i128 = 0;

    for i in 0..digits.len() {
        result += digits[i] * 10_i128.pow((digits.len() - 1 - i) as u32);
    }

    result
}
