use shared::read_lines;

const ROLL_CHARACTER: char = '@';
const MAX_NEIGHBORS_FOR_ACCESIBLE_ROLL: usize = 4;

pub fn solve(input_file: &str) {
    let lines = read_lines(input_file);
    let matrix = build_matrix(lines);

    let part1 = count_accessible_rolls(&matrix);
    println!("Part 1: {}", part1);

    let part2 = count_accessible_rolls_with_chain_reaction(matrix);
    println!("Part 2: {}", part2);
}

fn count_accessible_rolls(matrix: &[Vec<char>]) -> usize {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, &ch)| (row, col, ch))
        })
        .filter(|&(row, col, ch)| {
            ch == ROLL_CHARACTER
                && find_surrounding_character_occurrences(matrix, row, col, ROLL_CHARACTER)
                    < MAX_NEIGHBORS_FOR_ACCESIBLE_ROLL
        })
        .count()
}

fn count_accessible_rolls_with_chain_reaction(mut matrix: Vec<Vec<char>>) -> usize {
    let mut total = 0;

    loop {
        let removed = remove_accessible_rolls(&mut matrix);
        if removed == 0 {
            break;
        }
        total += removed;
    }

    total
}

fn remove_accessible_rolls(matrix: &mut [Vec<char>]) -> usize {
    let mut to_remove = Vec::new();

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == ROLL_CHARACTER
                && find_surrounding_character_occurrences(matrix, row, col, ROLL_CHARACTER)
                    < MAX_NEIGHBORS_FOR_ACCESIBLE_ROLL
            {
                to_remove.push((row, col));
            }
        }
    }

    let count = to_remove.len();
    for (row, col) in to_remove {
        matrix[row][col] = '.';
    }
    count
}

fn build_matrix(file_contents: Vec<String>) -> Vec<Vec<char>> {
    file_contents
        .iter()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_surrounding_character_occurrences(
    matrix: &[Vec<char>],
    row: usize,
    col: usize,
    character: char,
) -> usize {
    // ensure we do not run out of bounds even on an empty matrix
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return 0;
    }
    get_neighbors(row, col, matrix.len(), matrix[row].len())
        .filter(|(row_offset, col_offset)| {
            let neighbor_row = row.wrapping_add_signed(*row_offset as isize);
            let neighbor_col = col.wrapping_add_signed(*col_offset as isize);

            matrix
                .get(neighbor_row)
                .and_then(|row| row.get(neighbor_col))
                .map_or(false, |&c| c == character)
        })
        .count()
}

fn get_neighbors(
    row: usize,
    col: usize,
    max_rows: usize,
    max_cols: usize,
) -> impl Iterator<Item = (i32, i32)> {
    (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| dr != 0 || dc != 0) // Exclude center (0, 0)
        .filter(move |&(dr, dc)| {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            new_row >= 0 && new_row < max_rows as i32 && new_col >= 0 && new_col < max_cols as i32
        })
}
