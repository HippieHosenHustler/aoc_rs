struct Range {
    start: u64,
    end: u64,
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = find_amount_of_fresh_ingredients(input);

    (part1.to_string(), "".to_string())
}

fn find_amount_of_fresh_ingredients(input: &str) -> u32 {
    let empty_index = input.lines().position(|line| line.is_empty()).unwrap();

    let ranges: Vec<Range> = input
        .lines()
        .take(empty_index)
        .map(|line| {
            let parts: Vec<&str> = line.trim().split('-').collect();
            Range {
                start: parts[0].parse().unwrap(),
                end: parts[1].parse().unwrap(),
            }
        })
        .collect();

    println!("Number of ranges: {}", ranges.len());

    input
        .lines()
        .skip(empty_index + 1)
        .map(|line| {
            let ingredient: u64 = line.trim().parse().unwrap();
            if ranges
                .iter()
                .any(|range| ingredient >= range.start && ingredient <= range.end)
            {
                1
            } else {
                0
            }
        })
        .sum()
}
