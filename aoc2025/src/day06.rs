use shared::lines_to_vec;

struct Problem {
    input: Vec<u16>,
    method: Method,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        match self.method {
            Method::ADDITION => add(&self.input),
            Method::MULTIPLICATION => multiply(&self.input),
        }
    }

    pub fn add_new_value(&mut self, value: u16) {
        self.input.push(value);
    }
}

#[derive(Clone)]
enum Method {
    ADDITION,
    MULTIPLICATION,
}

pub fn solve(input: &str) -> (String, String) {
    let lines = lines_to_vec(input);

    let part1 = build_problems(&lines)
        .iter()
        .map(|p| p.solve())
        .sum::<u64>();
    let part2 = build_cephalopod_problems(&lines)
        .iter()
        .map(|p| p.solve())
        .sum::<u64>();

    (part1.to_string(), part2.to_string())
}

fn build_problems(input: &Vec<String>) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    let numbers = input
        .iter()
        .take(input.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();

    let methods = input
        .last()
        .unwrap()
        .split_whitespace()
        .map(|part| match part {
            "+" => Method::ADDITION,
            "*" => Method::MULTIPLICATION,
            _ => panic!("Unknown method"),
        })
        .collect::<Vec<Method>>();

    for row in numbers {
        for i in 0..row.len() {
            match problems.get_mut(i) {
                Some(problem) => problem.add_new_value(row[i]),
                None => {
                    problems.push(Problem {
                        input: vec![row[i]],
                        method: methods[i].clone(),
                    });
                }
            }
        }
    }

    problems
}

fn build_cephalopod_problems(input: &Vec<String>) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut current_numbers: Vec<u16> = Vec::new();
    let mut current_method: Option<Method> = None;

    let max_len = input.iter().map(|line| line.len()).max().unwrap();

    for col_idx in (0..max_len).rev() {
        let mut number_chars: Vec<char> = Vec::new();
        let mut operator_char: Option<char> = None;
        let mut has_content = false;

        for (row_idx, line) in input.iter().enumerate() {
            if col_idx < line.len() {
                let ch = line.chars().nth(col_idx).unwrap();

                if ch != ' ' {
                    has_content = true;

                    if row_idx == input.len() - 1 {
                        operator_char = Some(ch);
                    } else {
                        number_chars.push(ch);
                    }
                }
            }
        }

        if !has_content {
            if !current_numbers.is_empty() && current_method.is_some() {
                problems.push(Problem {
                    input: current_numbers,
                    method: current_method.unwrap(),
                });
                current_numbers = Vec::new();
                current_method = None;
            }
        } else {
            if !number_chars.is_empty() {
                let number_str: String = number_chars.iter().collect();
                let number = number_str.parse::<u16>().unwrap();
                current_numbers.push(number);
            }

            if let Some(op) = operator_char {
                current_method = Some(match op {
                    '+' => Method::ADDITION,
                    '*' => Method::MULTIPLICATION,
                    _ => panic!("Unknown method"),
                });
            }
        }
    }

    if !current_numbers.is_empty() && current_method.is_some() {
        problems.push(Problem {
            input: current_numbers,
            method: current_method.unwrap(),
        });
    }

    problems
}

fn multiply(values: &[u16]) -> u64 {
    values.iter().map(|&v| v as u64).product()
}

fn add(values: &[u16]) -> u64 {
    values.iter().map(|&v| v as u64).sum()
}
