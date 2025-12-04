use std::fs;

pub fn solve(input_file: &str) {
    let file_contents = fs::read_to_string(input_file).expect("Could not read file");
    let mut sum_invalid: i64 = 0;
    let mut sum_invalid_2: i64 = 0;

    let ranges = split_into_ranges(&file_contents);

    ranges.iter().for_each(|range| {
        let invalid_ids = range.find_invalid_ids();
        invalid_ids.iter().for_each(|id| sum_invalid += id);
    });

    println!(
        "Sum of all invalid product IDs (part 1 solution): {}",
        sum_invalid
    );

    ranges.iter().for_each(|range| {
        let invalid_ids = range.find_invalid_ids_2();
        invalid_ids.iter().for_each(|id| sum_invalid_2 += id);
    });

    println!(
        "Sum of all invalid product IDs (part 2 solution): {}",
        sum_invalid_2
    );
}

#[derive(Debug)]
struct ProductRange {
    start: i64,
    end: i64,
}

impl ProductRange {
    fn find_invalid_ids(&self) -> Vec<i64> {
        let mut invalid_ids = Vec::new();
        for id in self.start..=self.end {
            if !is_valid_product_id(id) {
                invalid_ids.push(id);
            }
        }
        invalid_ids
    }

    fn find_invalid_ids_2(&self) -> Vec<i64> {
        let mut invalid_ids = Vec::new();
        for id in self.start..=self.end {
            if !is_valid_product_id_2(id) {
                invalid_ids.push(id);
            }
        }

        invalid_ids
    }
}

fn split_into_ranges(s: &str) -> Vec<ProductRange> {
    s.trim()
        .split(',')
        .map(|line| {
            let (start_str, end_str) = line.split_at(line.find('-').unwrap());
            let start: i64 = start_str.parse().expect("Invalid start number");
            let end: i64 = end_str[1..].parse().expect("Invalid end number");
            ProductRange { start, end }
        })
        .collect()
}

fn is_valid_product_id(id: i64) -> bool {
    let digits = split_into_digits(id);

    let len = digits.len();

    if len == 1 || len % 2 != 0 {
        return true;
    }

    for i in 0..(len / 2) {
        if digits[i] != digits[(len / 2) + i] {
            return true;
        }
    }

    false
}

fn is_valid_product_id_2(id: i64) -> bool {
    let digits = split_into_digits(id);

    let len = digits.len();

    if len == 1 {
        return true;
    }

    for i in 2..=len {
        if len % i != 0 {
            continue;
        }

        // split digits into i equal parts and compare
        let chunk_size = len / i;
        let chunks: Vec<&[u32]> = digits.chunks(chunk_size).collect();

        let mut all_equal = true;

        for chunk in &chunks[1..] {
            if chunk != &chunks[0] {
                all_equal = false;
            }
        }

        if all_equal {
            return false;
        }
    }

    true
}

fn split_into_digits(num: i64) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
