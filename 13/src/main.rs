use anyhow::{ensure, Result};
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct System {
    // matrix[0][0] * x[0] + matrix[0][1] * x[1] = b[0]
    // matrix[1][0] * x[0] + matrix[1][1] * x[1] = b[1]
    // a = matrix[0][0], b = matrix[0][1], c = matrix[1][0], d = matrix[1][1]
    matrix: [[usize; 2]; 2],
    b: [usize; 2],
    x: Option<[usize; 2]>,
}

impl System {
    fn new(matrix: [[usize; 2]; 2], b: [usize; 2]) -> Self {
        Self { matrix, b, x: None }
    }

    fn from_str(input_str: &str) -> Result<Self> {
        let number_pattern = Regex::new(r"[XY][+=](\d+)")?;

        let numbers: Vec<usize> = number_pattern
            .captures_iter(input_str)
            .filter_map(|cap| cap[1].parse().ok())
            .collect();

        ensure!(numbers.len() == 6, "Invalid input string");

        Ok(Self::new(
            [[numbers[0], numbers[2]], [numbers[1], numbers[3]]],
            [numbers[4], numbers[5]],
        ))
    }

    fn from_str_added(input_str: &str) -> Result<Self> {
        let mut system = Self::from_str(input_str)?;

        system.b[0] += 10_000_000_000_000;
        system.b[1] += 10_000_000_000_000;

        Ok(system)
    }

    fn det(&self) -> isize {
        // a * d - b * c
        (self.matrix[0][0] * self.matrix[1][1]) as isize
            - (self.matrix[0][1] * self.matrix[1][0]) as isize
    }

    fn solve(&mut self) -> bool {
        let det = self.det();
        if det == 0 {
            return false;
        }

        // Calculate the solution.
        // x[0] = (d * b[0] - b * b[1]) / det
        // x[1] = (a * b[1] - c * b[0]) / det
        let mut x_p = [
            (self.matrix[1][1] * self.b[0]) as isize - (self.matrix[0][1] * self.b[1]) as isize,
            (self.matrix[0][0] * self.b[1]) as isize - (self.matrix[1][0] * self.b[0]) as isize,
        ];

        // Check if the solution is an integer.
        if x_p[0] % det != 0 || x_p[1] % det != 0 {
            return false;
        }

        x_p[0] /= det;
        x_p[1] /= det;

        // Check if the solution is positive.
        if x_p[0] < 0 || x_p[1] < 0 {
            return false;
        }

        // Set a successful solution.
        self.x = Some([x_p[0] as usize, x_p[1] as usize]);
        true
    }

    fn calculate_cost(&self, threshold: Option<usize>) -> Option<usize> {
        if let Some(x) = self.x {
            if let Some(t) = threshold {
                if x[0] > t || x[1] > t {
                    None
                } else {
                    Some(x[0] * 3 + x[1])
                }
            } else {
                Some(x[0] * 3 + x[1])
            }
        } else {
            None
        }
    }
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let systems: Result<Vec<System>> = input_str
        .split("\n\n")
        .map(|s| -> Result<System> { System::from_str(s) })
        .collect();
    let mut systems = systems?;

    let scores: Vec<usize> = systems
        .iter_mut()
        .flat_map(|s| {
            if !s.solve() {
                return None;
            }
            s.calculate_cost(Some(100))
        })
        .collect();

    Ok(scores.iter().sum())
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let systems: Result<Vec<System>> = input_str
        .split("\n\n")
        .map(|s| -> Result<System> { System::from_str_added(s) })
        .collect();
    let mut systems = systems?;

    let scores: Vec<usize> = systems
        .iter_mut()
        .flat_map(|s| {
            if !s.solve() {
                return None;
            }
            s.calculate_cost(None)
        })
        .collect();

    Ok(scores.iter().sum())
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

    #[fixture]
    fn sample_input_str() -> &'static str {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"
    }

    #[fixture]
    fn expected_input() -> System {
        System::new([[94, 22], [34, 67]], [8400, 5400])
    }

    #[fixture]
    fn expected_output() -> usize {
        280
    }

    #[rstest]
    fn test_system_from_str(sample_input_str: &str, expected_input: System) {
        let system = System::from_str(sample_input_str).unwrap();
        assert_eq!(system, expected_input);
    }

    #[rstest]
    fn test_cost_calculation(expected_input: System, expected_output: usize) {
        let mut system = expected_input;
        system.solve();
        assert_eq!(system.calculate_cost(Some(100)), Some(expected_output));
    }
}
