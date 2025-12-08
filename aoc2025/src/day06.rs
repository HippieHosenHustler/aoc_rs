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
    let problems = build_problems(&lines);

    let part1 = problems.iter().map(|p| p.solve()).sum::<u64>();

    (part1.to_string(), "".to_string())
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

fn multiply(values: &[u16]) -> u64 {
    values.iter().map(|&v| v as u64).product()
}

fn add(values: &[u16]) -> u64 {
    values.iter().map(|&v| v as u64).sum()
}
