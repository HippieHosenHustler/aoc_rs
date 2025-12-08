use std::collections::{HashMap, HashSet};

use shared::lines_to_vec;

pub fn solve(input: &str) -> (String, String) {
    let lines = lines_to_vec(input);

    let starting_position = find_starting_position(lines.first().expect("Expected to have lines"));

    let part1 = count_splits(&lines, &starting_position);

    let mut memo = HashMap::new();
    let part2 = count_realities(&lines, &starting_position, &1, &mut memo);

    (part1.to_string(), part2.to_string())
}

fn count_splits(lines: &Vec<String>, starting_position: &usize) -> usize {
    let mut beam_positions = HashSet::new();
    let mut split_events = 0;

    beam_positions.insert(*starting_position);

    lines.iter().skip(1).for_each(|line| {
        let chars: Vec<char> = line.chars().collect();

        let positions_to_process: Vec<_> = beam_positions.drain().collect();
        for pos in positions_to_process {
            if chars[pos] == '^' {
                split_events += 1;
                if pos > 0 {
                    beam_positions.insert(pos - 1);
                }
                if pos < chars.len() - 1 {
                    beam_positions.insert(pos + 1);
                }
            } else {
                beam_positions.insert(pos);
            }
        }
    });

    split_events
}

fn count_realities(
    lines: &Vec<String>,
    starting_position: &usize,
    row: &usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let key = (*starting_position, *row);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    for (offset, line) in lines.iter().skip(*row).enumerate() {
        let current_row = *row + offset;
        let chars = line.chars().collect::<Vec<char>>();
        let char = chars[*starting_position];

        if char == '^' {
            let mut total_count = 0;
            // Count paths going left
            if *starting_position > 0 {
                total_count +=
                    count_realities(lines, &(starting_position - 1), &(current_row + 1), memo);
            }
            // Count paths going right
            if *starting_position < line.len() - 1 {
                total_count +=
                    count_realities(lines, &(starting_position + 1), &(current_row + 1), memo);
            }

            memo.insert(key, total_count);
            return total_count;
        }
    }

    1
}

fn find_starting_position(first_line: &String) -> usize {
    first_line
        .chars()
        .position(|p| p == 'S')
        .expect("Expected 'S' in first line")
}
