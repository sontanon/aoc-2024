use anyhow::{anyhow, ensure, Result};
use std::fs;

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let block_sizes: Result<Vec<usize>> = input_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| {
            c.to_string()
                .parse::<usize>()
                .map_err(|_| anyhow!("Invalid input"))
        })
        .collect();

    let mut blocks: Vec<Option<usize>> = block_sizes?
        .iter()
        .enumerate()
        .flat_map(|(i, &block_size)| -> Vec<Option<usize>> {
            if i % 2 == 0 {
                vec![Some(i / 2); block_size]
            } else {
                vec![None; block_size]
            }
        })
        .collect();

    // Find the first left index that is empty.
    let mut left = blocks.iter().position(|&x| x.is_none()).ok_or(anyhow!("Must have a left empty block"))?;
    // Find the first right index that is not empty.
    let mut right: usize = blocks.iter().rposition(|&x| x.is_some()).ok_or(anyhow!("Must have a right non-empty block"))?;

    while left < right {
        // Swap the empty block with the first non-empty block to the right.
        blocks.swap(left, right);

        // Update the new left index
        let new_left = blocks[left + 1..right].iter().position(|&x| x.is_none());
        let new_right = blocks[left + 1..right].iter().rposition(|&x| x.is_some());

        // Update the indices.
        match (new_left, new_right) {
            (Some(l), Some(r)) => {
                right = left + 1 + r;
                left = left + 1 + l;
            }
            _ => break,
        }
    }

    Ok(blocks
        .iter()
        .filter_map(|&x| x)
        .enumerate()
        .map(|(i, n)| i * n)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_input_str() -> &'static str {
        "2333133121414131402"
    }

    #[fixture]
    fn expected_output() -> usize {
        1928
    }

    #[rstest]
    fn test_exercise_1(
        #[from(sample_input_str)] input: &str,
        #[from(expected_output)] expected: usize,
    ) {
        let result = exercise_1(input).unwrap();
        assert_eq!(result, expected);
    }
}
