use std::{fs, path::Path};
use regex::Regex;

fn count_occurrences(s: &str) -> usize {
    let pattern_1 = Regex::new("XMAS").unwrap();
    let pattern_2 = Regex::new("SAMX").unwrap();

    pattern_1.find_iter(s).count() + pattern_2.find_iter(s).count()
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

    let row_counts: usize = rows
        .iter()
        .map(|row| count_occurrences(row))
        .sum();
    let col_counts: usize = cols
        .iter()
        .map(|col| count_occurrences(col))
        .sum();
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

    Ok(())
}
