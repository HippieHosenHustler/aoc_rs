use crate::file_helpers::read_lines;

const ROLL_CHARACTER: char = '@';
const MAX_NEIGHBORS_FOR_ACCESIBLE_ROLL: u32 = 4;

pub fn solve(input_file: &str) {
    let lines = read_lines(input_file);

    let mut matrix = build_matrix(lines);

    let mut accessible_rolls = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == ROLL_CHARACTER
                && find_surrounding_character_occurrences(&matrix, row, col, ROLL_CHARACTER)
                    < MAX_NEIGHBORS_FOR_ACCESIBLE_ROLL
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
                    && find_surrounding_character_occurrences(&matrix, row, col, ROLL_CHARACTER)
                        < MAX_NEIGHBORS_FOR_ACCESIBLE_ROLL
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

fn find_surrounding_character_occurrences(
    matrix: &Vec<Vec<char>>,
    line_number: usize,
    col_number: usize,
    character: char,
) -> u32 {
    // ensure we do not run out of bounds even on an empty matrix
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return 0;
    }
    get_neighbors(
        line_number,
        col_number,
        matrix.len(),
        matrix[line_number].len(),
    )
    .into_iter()
    .filter(|(row_offset, col_offset)| {
        let neighbor_row = (line_number as i32 + row_offset) as usize;
        let neighbor_col = (col_number as i32 + col_offset) as usize;

        matrix[neighbor_row][neighbor_col] == character
    })
    .count() as u32
}

fn get_neighbors(row: usize, col: usize, max_rows: usize, max_cols: usize) -> Vec<(i32, i32)> {
    (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| dr != 0 || dc != 0) // Exclude center (0, 0)
        .filter(|&(dr, dc)| {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            new_row >= 0 && new_row < max_rows as i32 && new_col >= 0 && new_col < max_cols as i32
        })
        .collect()
}
