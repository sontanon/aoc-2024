use regex::Regex;
use std::{fs, path::Path};

fn count_occurrences(s: &str) -> usize {
    let pattern_1 = Regex::new("XMAS").unwrap();
    let pattern_2 = Regex::new("SAMX").unwrap();

    pattern_1.find_iter(s).count() + pattern_2.find_iter(s).count()
}

fn char_to_array(c: char) -> [u8; 3] {
    match c {
        // 'X' => [0, 0, 0],
        'M' => [1, 0, 0],
        'A' => [0, 1, 0],
        'S' => [0, 0, 1],
        _ => [0, 0, 0],
    }
}

// M . M
// . A .
// S . S
const FILTER_1: [[[u8; 3]; 3]; 3] = [
    [[1, 0, 0], [0, 0, 0], [1, 0, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 0, 1], [0, 0, 0], [0, 0, 1]],
];

// M . S
// . A .
// M . S
const FILTER_2: [[[u8; 3]; 3]; 3] = [
    [[1, 0, 0], [0, 0, 0], [0, 0, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[1, 0, 0], [0, 0, 0], [0, 0, 1]],
];

// S . S
// . A .
// M . M
const FILTER_3: [[[u8; 3]; 3]; 3] = [
    [[0, 0, 1], [0, 0, 0], [0, 0, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[1, 0, 0], [0, 0, 0], [1, 0, 0]],
];

// S . M
// . A .
// S . M
const FILTER_4: [[[u8; 3]; 3]; 3] = [
    [[0, 0, 1], [0, 0, 0], [1, 0, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 0, 1], [0, 0, 0], [1, 0, 0]],
];

fn convolve(matrix: &Vec<Vec<[u8; 3]>>, filter: &[[[u8; 3]; 3]; 3]) -> Vec<Vec<bool>> {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    (1..n_rows - 1)
        .map(|i| -> Vec<bool> {
            (1..n_cols - 1)
                .map(|j| -> bool {
                    let mut sum = 0;
                    for w in 0..3 {
                        for h in 0..3 {
                            for c in 0..3 {
                                sum += matrix[i - 1 + w][j - 1 + h][c] * filter[w][h][c];
                            }
                        }
                    }
                    sum == 5
                })
                .collect()
        })
        .collect()
}

fn exercise_2(input_string: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let matrix: Vec<Vec<[u8; 3]>> = input_string
        .split("\n")
        .filter(|&row| !row.is_empty())
        .map(|row| -> Vec<[u8; 3]> { row.chars().map(|c| char_to_array(c)).collect() })
        .collect();

    let convolution_1 = convolve(&matrix, &FILTER_1);
    let convolution_2 = convolve(&matrix, &FILTER_2);
    let convolution_3 = convolve(&matrix, &FILTER_3);
    let convolution_4 = convolve(&matrix, &FILTER_4);

    let sum = convolution_1
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum::<usize>()
        + convolution_2
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum::<usize>()
        + convolution_3
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum::<usize>()
        + convolution_4
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum::<usize>();

    Ok(sum)
}

fn exercise_1(input_string: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let rows: Vec<Vec<char>> = input_string
        .split("\n")
        .filter(|&row| !row.is_empty())
        .map(|row| row.chars().collect())
        .collect();

    let n_rows = rows.len();
    let n_cols = rows[0].len();

    assert_eq!(n_rows, n_cols);
    assert!(rows.iter().all(|row| row.len() == n_cols));
    let n = n_rows;

    let cols: Vec<Vec<char>> = (0..n)
        .map(|i| rows.iter().map(|row| row[i]).collect())
        .collect();

    let diags_left_right: Vec<Vec<char>> = (0..2 * n - 1)
        .map(|k| -> Vec<char> {
            let left_bound = usize::max(0, usize::saturating_sub(n, k + 1));
            let right_bound = usize::min(n, 2 * n - (1 + k));
            (left_bound..right_bound)
                .map(|i| -> char { rows[i][(i + k + 1) - n] })
                .collect()
        })
        .collect();

    let diags_right_left: Vec<Vec<char>> = (0..2 * n - 1)
        .map(|k| -> Vec<char> {
            let left_bound = usize::max(0, usize::saturating_sub(n, k + 1));
            let right_bound = usize::min(n, 2 * n - (1 + k));
            (left_bound..right_bound)
                .map(|i| -> char { rows[i][2 * (n - 1) - (i + k)] })
                .collect()
        })
        .collect();

    let rows: Vec<String> = rows
        .into_iter()
        .map(|v_c| -> String { v_c.iter().collect() })
        .collect();

    let cols: Vec<String> = cols
        .into_iter()
        .map(|v_c| -> String { v_c.iter().collect() })
        .collect();

    let diags_left_right: Vec<String> = diags_left_right
        .into_iter()
        .map(|v_c| -> String { v_c.iter().collect() })
        .collect();

    let diags_right_left: Vec<String> = diags_right_left
        .into_iter()
        .map(|v_c| -> String { v_c.iter().collect() })
        .collect();

    let row_counts: usize = rows.iter().map(|row| count_occurrences(row)).sum();
    let col_counts: usize = cols.iter().map(|col| count_occurrences(col)).sum();
    let diag_left_right_counts: usize = diags_left_right
        .iter()
        .map(|diag| count_occurrences(diag))
        .sum();
    let diag_right_left_counts: usize = diags_right_left
        .iter()
        .map(|diag| count_occurrences(diag))
        .sum();

    Ok(row_counts + col_counts + diag_left_right_counts + diag_right_left_counts)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_string = fs::read_to_string(Path::new("input.txt"))?;

    let sum_1 = exercise_1(&input_string)?;
    println!("Sum 1: {sum_1}");

    let sum_2 = exercise_2(&input_string)?;
    println!("Sum 2: {sum_2}");

    Ok(())
}
