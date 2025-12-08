use shared::lines_to_vec;

pub fn solve(input: &str) -> (String, String) {
    let lines = lines_to_vec(input);

    let part1 = count_splits(&lines);

    (part1.to_string(), String::from("N/A"))
}

fn count_splits(lines: &Vec<String>) -> usize {
    let mut beam_positions = Vec::new();
    let mut split_events = 0;

    lines
        .first()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            if c == 'S' {
                beam_positions.push(i);
            }
        });

    lines.iter().skip(1).for_each(|line| {
        let chars = line.chars();

        beam_positions.clone().iter().for_each(|&pos| {
            if chars.clone().nth(pos) == Some('^') {
                split_events += 1;

                if beam_positions.contains(&(pos - 1)) == false {
                    beam_positions.push(pos - 1);
                }
                beam_positions.remove(beam_positions.iter().position(|&i| i == pos).unwrap());
                if beam_positions.contains(&(pos + 1)) == false {
                    beam_positions.push(pos + 1);
                }
            }
        });
    });

    split_events
}
