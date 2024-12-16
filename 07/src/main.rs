use anyhow::{anyhow, ensure, Result};
use core::panic;
use std::fs;

// #[derive(Debug, Clone, PartialEq)]
// enum Operation {
//     Add,
//     Mult,
// }

#[derive(Debug, Clone, PartialEq)]
struct Calibration {
    result: usize,
    operands: Vec<usize>,
    // operations: Vec<Option<Operation>>,
    possible: bool,
}

fn calculate_bounds(operands: &[usize]) -> (usize, usize) {
    if operands.len() < 2 {
        panic!("Must contain at least two operands.");
    }
    // The operands slice is a sequence of non-negative integers.

    // The lower bound will usually be the sum of the operands, except when an operand is 1, in which case the smallest operation is to multiply by 1.
    // There is an edge case when the inital element is 1.
    let lower_bound = operands[1..].iter().fold(operands[0], |acc, &x| {
        if acc == 1 {
            x
        } else if x == 1 {
            acc
        } else {
            acc + x
        }
    });
    let upper_bound = operands[1..].iter().fold(operands[0], |acc, &x| {
        if acc == 1 || x == 1{
            acc + x
        } else {
            acc * x
        }
    });
    (lower_bound, upper_bound)
}

fn is_possible(
    result: usize,
    lower_bound: usize,
    upper_bound: usize,
    operands: &[usize],
) -> bool {
    // If the result is outside the bounds, it is impossible.
    if result < lower_bound || result > upper_bound {
        return false;
    }

    if result == lower_bound || result == upper_bound {
        return true;
    }

    let n = operands.len();

    // If there are only two operands and the result is not equal to either of them, it is impossible.
    if n <= 2 {
        return false;
    }

    // Check recursively.
    // Get the last element of the operands slice.
    let tail = operands[n - 1];

    // Edge case when the tail is 1.
    if tail == 1 {
        // (Last Operation is Multiplication) || (Last Operation is Addition)
        return is_possible( 
                result,
                lower_bound,
                upper_bound - 1,
                &operands[..n - 1],
            ) 
            // Last operation is addition.
            || is_possible(
                result - 1,
                lower_bound,
                upper_bound - 1,
                &operands[..n - 1],
            );
    }

    // The tail cannot be 1.
    // (Last Operation is Multiplication) || (Last Operation is Addition)
    ((result % tail == 0)
        // Last operation is multiplication.
        && is_possible(
            result / tail,
            lower_bound - tail,
            upper_bound / tail,
            &operands[..n - 1],
        ))
        // Last operation is addition.
        || is_possible(
            result - tail,
            lower_bound - tail,
            upper_bound / tail,
            &operands[..n - 1],
        )
}

fn concat_integers(lhs: usize, rhs: usize) -> usize {
    let mut n = rhs;
    let mut lhs = lhs;
    while n > 0 {
        lhs *= 10;
        n /= 10;
    }
    lhs + rhs
}


fn can_unconcat(lhs: usize, rhs: usize) -> bool {
    if lhs == rhs {
        return false;
    }
    let n = usize::ilog10(rhs) + 1;
    lhs % 10usize.pow(n) == rhs
}

fn unconcat(lhs: usize, rhs: usize) -> usize {
    let n = usize::ilog10(rhs) + 1;
    (lhs - rhs) / 10usize.pow(n)
}


fn is_possible_expanded(
    result: usize,
    operands: &[usize],
) -> bool {
    let n = operands.len();
    
    if n == 2 {
        let s = operands[0] + operands[1];
        let m = operands[0] * operands[1];
        let c = concat_integers(operands[0], operands[1]);

        return result == s || result == m || result == c;
    }

    let tail = operands[n - 1];

    (result > tail && is_possible_expanded(result - tail, &operands[..n - 1]))
        || (result % tail == 0 && is_possible_expanded(result / tail, &operands[..n - 1]))
        || (can_unconcat(result, tail) && is_possible_expanded(unconcat(result, tail), &operands[..n - 1]))
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let all_calibrations: Result<Vec<Calibration>> = input_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| -> Result<Calibration> {
            let (result, operands) = line
                .split_once(":")
                .ok_or_else(|| anyhow!("Invalid line"))?;
            let result = result.trim().parse::<usize>()?;
            let operands: Result<Vec<usize>> = operands
                .split_whitespace()
                .map(|operand| -> Result<usize> {
                    operand
                        .parse::<usize>()
                        .map_err(|_| anyhow!("Integer parsing error"))
                })
                .collect();
            let operands = operands?;
            ensure!(operands.len() >= 2, "Must contain at least two operands.");
            let (lower_bound, upper_bound) = calculate_bounds(&operands[..]);

            let possible = is_possible(result, lower_bound, upper_bound, &operands[..]);

            Ok(Calibration {
                result,
                operands,
                possible,
            })
        })
        .collect();
    let all_calibrations = all_calibrations?;
    println!("All calibrations: {:?}", all_calibrations.len());

    // For each calibration, determine if it is possible, store its result, and filter-out.
    let possible_calibrations: Vec<&Calibration> = all_calibrations
        .iter()
        .filter(|calibration| calibration.possible)
        .collect();
    println!("Possible calibrations: {:?}", possible_calibrations.len());

    Ok(possible_calibrations
        .iter()
        .map(|calibration| calibration.result)
        .sum())
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let all_calibrations: Result<Vec<Calibration>> = input_str
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| -> Result<Calibration> {
        let (result, operands) = line
            .split_once(":")
            .ok_or_else(|| anyhow!("Invalid line"))?;
        let result = result.trim().parse::<usize>()?;
        let operands: Result<Vec<usize>> = operands
            .split_whitespace()
            .map(|operand| -> Result<usize> {
                operand
                    .parse::<usize>()
                    .map_err(|_| anyhow!("Integer parsing error"))
            })
            .collect();
        let operands = operands?;
        ensure!(operands.len() >= 2, "Must contain at least two operands.");

        let possible = is_possible_expanded(result, &operands[..]);

        Ok(Calibration {
            result,
            operands,
            possible,
        })
    })
    .collect();
let all_calibrations = all_calibrations?;
println!("All calibrations: {:?}", all_calibrations.len());

// For each calibration, determine if it is possible, store its result, and filter-out.
let possible_calibrations: Vec<&Calibration> = all_calibrations
    .iter()
    .filter(|calibration| calibration.possible)
    .collect();
println!("Possible calibrations: {:?}", possible_calibrations.len());

Ok(possible_calibrations
    .iter()
    .map(|calibration| calibration.result)
    .sum())
}

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&input_str)?;
    println!("Result 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(12345, 2345, true)]
    #[case(1237089, 7089, true)]
    #[case(12345, 234, false)]
    #[case(1, 1, true)]
    #[case(123, 123, true)]
    fn test_can_unconcat(#[case] lhs: usize, #[case] rhs: usize, #[case] expected: bool) {
        assert_eq!(can_unconcat(lhs, rhs), expected);
    }

    #[rstest]
    #[case(12345, 2345, 1)]
    #[case(1237089, 7089, 123)]
    #[case(123, 123, 0)]
    fn test_unconcat(#[case] lhs: usize, #[case] rhs: usize, #[case] expected: usize) {
        assert_eq!(unconcat(lhs, rhs), expected);
    }

    #[rstest]
    #[case(&[2, 1, 3, 4, 1], 9, 37)]
    #[case(&[2, 1, 1, 3], 5, 12)]
    #[case(&[1, 1, 1], 1, 3)]
    #[case(&[1, 2, 3], 5, 9)]
    #[case(&[10, 19], 29, 190)]
    fn test_calculate_bounds(
        #[case] operands: &[usize],
        #[case] lower: usize,
        #[case] upper: usize,
    ) {
        let (lower_bound, upper_bound) = calculate_bounds(operands);
        assert_eq!(lower_bound, lower);
        assert_eq!(upper_bound, upper);
    }

    #[rstest]
    #[case(190, &[10, 19], true)]
    #[case(3267, &[81, 40, 27], true)]
    #[case(83, &[17, 15], false)]
    #[case(156, &[15, 6], false)]
    #[case(7290, &[6, 8, 6, 15], false)]
    #[case(161011, &[16, 10, 13], false)]
    #[case(192, &[17, 8, 14], false)]
    #[case(21037, &[9, 7, 18, 13], false)]
    #[case(292, &[11, 6, 16, 20], true)]
    fn test_is_possible(#[case] result: usize, #[case] operands: &[usize], #[case] expected: bool) {
        let lower_bound: usize = operands.iter().sum();
        let upper_bound: usize = operands.iter().product();
        assert_eq!(
            is_possible(result, lower_bound, upper_bound, operands),
            expected
        );
    }

    #[fixture]
    fn sample_input_string() -> &'static str {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[rstest]
    fn test_exercise_1(#[from(sample_input_string)] input: &str) {
        let result = exercise_1(&input);
        assert!(result.is_ok_and(|x| x == 3749));
    }
}
