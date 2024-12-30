use anyhow::{anyhow, ensure, Result};
use std::fs;

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&input_str)?;
    println!("Result 2: {}", result_2);

    Ok(())
}

fn get_block_sizes_vector(input_str: &str) -> Result<Vec<usize>> {
    ensure!(!input_str.is_empty(), "Input string is empty");

    let block_sizes: Result<Vec<usize>> = input_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| {
            c.to_string()
                .parse::<usize>()
                .map_err(|_| anyhow!("Invalid input for digit {}", c))
        })
        .collect();
    let block_sizes = block_sizes?;

    ensure!(!block_sizes.is_empty(), "No block sizes found");

    Ok(block_sizes)
}

/// Careful with the indices: they are inclusive.
fn get_right_block_indices(blocks: &[Option<usize>]) -> Option<(usize, usize)> {
    let right_end = blocks.iter().rposition(|&x| x.is_some())?;

    let right_id = blocks[right_end]?;

    let right_start = blocks[..right_end].iter().rposition(|&x| match x {
        None => true,
        Some(n) => n != right_id,
    })? + 1;

    Some((right_start, right_end))
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let block_sizes = get_block_sizes_vector(input_str)?;

    let mut blocks: Vec<Option<usize>> = block_sizes
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

    // Find the first block on the right that is non empty.
    let (mut right_start, mut right_end) = get_right_block_indices(&blocks[..])
        .ok_or(anyhow!("Must have an initial right non-empty block"))?;

    loop {
        let right_slice_size = right_end - right_start + 1;

        // Search for the first left block that is empty and has enough space to fit the right slice.
        if let Some(left_start) = blocks[..right_start]
            .windows(right_slice_size)
            .position(|window| window.iter().all(|x| x.is_none()))
        {
            // We are guaranteed to have a left_end given the closure above.
            let left_end = left_start + right_slice_size - 1;

            // Get the two mutable slices to swap.
            // This requires using `split_at_mut` and then reducing the slices to the correct size.
            let (left_slice, right_slice) = {
                let (left_super_slice, right_super_slice) = blocks.split_at_mut(right_start);
                (
                    &mut left_super_slice[left_start..=left_end],
                    &mut right_super_slice[..right_slice_size],
                )
            };
            // Swap the slices.
            left_slice.swap_with_slice(right_slice);
        }

        // Find the new right start: reduce the slice we are searching over
        match get_right_block_indices(&blocks[..right_start]) {
            Some((new_right_start, new_right_end)) => {
                right_start = new_right_start;
                right_end = new_right_end;
            }
            None => break,
        }
    }

    Ok(blocks
        .iter()
        .enumerate()
        .map(|(i, n)| match n {
            Some(n) => i * n,
            None => 0,
        })
        .sum())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let block_sizes = get_block_sizes_vector(input_str)?;

    let mut blocks: Vec<Option<usize>> = block_sizes
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
    let mut left = blocks
        .iter()
        .position(|&x| x.is_none())
        .ok_or(anyhow!("Must have a left empty block"))?;
    // Find the first right index that is not empty.
    let mut right: usize = blocks
        .iter()
        .rposition(|&x| x.is_some())
        .ok_or(anyhow!("Must have a right non-empty block"))?;

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
    fn expected_output_1() -> usize {
        1928
    }

    #[fixture]
    fn expected_output_2() -> usize {
        2858
    }

    #[rstest]
    fn test_exercise_1(
        #[from(sample_input_str)] input: &str,
        #[from(expected_output_1)] expected: usize,
    ) {
        let result = exercise_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    fn test_exercise_2(
        #[from(sample_input_str)] input_str: &str,
        #[from(expected_output_2)] expected: usize,
    ) {
        let result = exercise_2(input_str).unwrap();
        assert_eq!(result, expected);
    }
}
