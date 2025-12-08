#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

pub fn solve(input: &str) -> (String, String) {
    let ranges = build_ranges(input);
    let ingredients = build_ingredient_list(input);

    let part1 = find_amount_of_fresh_ingredients(&ranges, &ingredients);
    let part2 = find_all_fresh_ingredient_ids(&ranges);

    (part1.to_string(), part2.to_string())
}

fn find_amount_of_fresh_ingredients(ranges: &[Range], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|&&ingredient| {
            !ranges
                .iter()
                .any(|range| ingredient >= range.start && ingredient <= range.end)
        })
        .count()
}

fn find_all_fresh_ingredient_ids(ranges: &Vec<Range>) -> u64 {
    let mut merged_ranges = Vec::new();

    let mut current_start = ranges[0].start;
    let mut current_end = ranges[0].end;

    for i in 1..ranges.len() {
        let range = &ranges[i];

        match range.start.cmp(&(current_end + 1)) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                if range.end > current_end {
                    current_end = range.end;
                }
            }
            std::cmp::Ordering::Greater => {
                merged_ranges.push(Range {
                    start: current_start,
                    end: current_end,
                });
                current_start = range.start;
                current_end = range.end;
            }
        }
    }

    merged_ranges.push(Range {
        start: current_start,
        end: current_end,
    });

    merged_ranges.iter().map(|r| r.end - r.start + 1).sum()
}

fn build_ranges(input: &str) -> Vec<Range> {
    let empty_index = input.lines().position(|line| line.is_empty()).unwrap();

    let mut ranges: Vec<Range> = input
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

    ranges.sort_by_key(|r| r.start);
    ranges
}

fn build_ingredient_list(input: &str) -> Vec<u64> {
    let empty_index = input.lines().position(|line| line.is_empty()).unwrap();

    input
        .lines()
        .skip(empty_index + 1)
        .map(|line| line.trim().parse().unwrap())
        .collect()
}
