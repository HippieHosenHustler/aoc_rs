use crate::file_helpers::read_lines;

const ROLL_CHARACTER: char = '@';

pub fn solve(input_file: &str) {
    let lines = read_lines(input_file);

    let mut matrix = build_matrix(lines);

    let mut accessible_rolls = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == ROLL_CHARACTER
                && find_surrounding_character_occurences(matrix.clone(), row, col, ROLL_CHARACTER)
                    < 4
            {
                accessible_rolls += 1;
            }
        }
    }

    println!(
        "Total accessible rolls (part 1 solution): {}",
        accessible_rolls
    );

    let mut accessible_rolls_total = 0;

    while accessible_rolls > 0 {
        accessible_rolls = 0;
        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                if matrix[row][col] == ROLL_CHARACTER
                    && find_surrounding_character_occurences(
                        matrix.clone(),
                        row,
                        col,
                        ROLL_CHARACTER,
                    ) < 4
                {
                    accessible_rolls += 1;
                    matrix[row][col] = '.';
                }
            }
        }
        accessible_rolls_total += accessible_rolls;
    }

    println!(
        "Total accessible rolls including chain reactions (part 2 solution): {}",
        accessible_rolls_total
    );
}

fn build_matrix(file_contents: Vec<String>) -> Vec<Vec<char>> {
    file_contents
        .into_iter()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn find_surrounding_character_occurences(
    matrix: Vec<Vec<char>>,
    line_number: usize,
    col_number: usize,
    character: char,
) -> u32 {
    get_neighbors(line_number, col_number, matrix.len(), matrix[0].len())
        .into_iter()
        .filter(|(dx, dy)| {
            let neighbor_row = (line_number as i32 + dx) as usize;
            let neighbor_col = (col_number as i32 + dy) as usize;

            matrix[neighbor_row][neighbor_col] == character
        })
        .collect::<Vec<(i32, i32)>>()
        .len()
        .try_into()
        .unwrap()
}

fn get_neighbors(row: usize, col: usize, max_rows: usize, max_cols: usize) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    // Prior row
    if row > 0 {
        if col > 0 {
            result.push((-1, -1))
        }
        result.push((-1, 0));
        if col != max_cols - 1 {
            result.push((-1, 1));
        }
    }

    // Current row
    if col > 0 {
        result.push((0, -1));
    }
    if col != max_cols - 1 {
        result.push((0, 1));
    }

    // Next row
    if row != max_rows - 1 {
        if col > 0 {
            result.push((1, -1))
        }
        result.push((1, 0));
        if col != max_cols - 1 {
            result.push((1, 1));
        }
    }

    result
}
