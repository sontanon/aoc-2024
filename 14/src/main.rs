use anyhow::{anyhow, ensure, Result};
use regex::Regex;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    println!(
        "Exercise 1: {}",
        exercise_1(&input_str, 101, 103, 100, false)?
    );

    println!(
        "Exercise 2: {}",
        exercise_2(&input_str, 101, 103, 1_000_000)?
    );

    Ok(())
}

fn exercise_1(
    input_str: &str,
    width: usize,
    height: usize,
    timesteps: usize,
    print: bool,
) -> Result<usize> {
    let mut map = Map::from_str(input_str, width, height)?;

    map.advance_timesteps(timesteps, print);
    let safety_score = map.calculate_guards_per_quadrant().iter().product();

    Ok(safety_score)
}


fn exercise_2(input_str: &str, width: usize, height: usize, max_iterations: usize) -> Result<usize> {
    let mut map = Map::from_str(input_str, width, height)?;

    for k in 0..max_iterations {
        if map.dense_frame() {
            println!("{}", map);
            return Ok(k);
        }
        map.advance_timesteps(1, false);
    }
    Err(anyhow!("No dense frame found after {} iterations", max_iterations))
}

#[derive(Debug, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    guards: Vec<Guard>,
}

fn move_cyclically(position: usize, velocity: isize, max: usize) -> usize {
    let position = position as isize;
    let max = max as isize;
    let new_position = ((position + velocity) % max + max) % max;
    new_position as usize
}

impl Map {
    fn new(width: usize, height: usize, guards: Vec<Guard>) -> Self {
        Self {
            width,
            height,
            guards,
        }
    }

    fn from_str(input_str: &str, width: usize, height: usize) -> Result<Self> {
        let guards: Result<Vec<Guard>> = input_str.lines().map(Guard::from_str).collect();
        let guards = guards?;
        Ok(Map::new(width, height, guards))
    }

    fn move_guards(&mut self) {
        for guard in &mut self.guards {
            let [p_x, p_y] = guard.position;
            let [v_x, v_y] = guard.velocity;
            guard.position = [
                move_cyclically(p_x, v_x, self.width),
                move_cyclically(p_y, v_y, self.height),
            ];
        }
    }

    fn advance_timesteps(&mut self, n: usize, print: bool) {
        if print {
            println!("{}", self);
        }
        for _ in 0..n {
            self.move_guards();
            if print {
                println!("{}", self);
            }
        }
    }

    fn calculate_guards_per_quadrant(&self) -> [usize; 4] {
        let nw_guards = self
            .guards
            .iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::NorthWest)
            .count();
        let ne_guards = self
            .guards
            .iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::NorthEast)
            .count();
        let se_guards = self
            .guards
            .iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::SouthEast)
            .count();
        let sw_guards = self
            .guards
            .iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::SouthWest)
            .count();

        [nw_guards, ne_guards, se_guards, sw_guards]
    }

    fn dense_row(&self, j: usize) -> bool {
        let columns: HashSet<usize> = HashSet::from_iter(self.guards.iter().filter_map(|g| {
            if g.position[1] == j {
                Some(g.position[0])
            } else {
                None
            }
        }));
        columns.len() >= 30
    }

    fn dense_column(&self, i: usize) -> bool {
        let rows: HashSet<usize> = HashSet::from_iter(self.guards.iter().filter_map(|g| {
            if g.position[0] == i {
                Some(g.position[1])
            } else {
                None
            }
        }));
        rows.len() >= 30
    }

    fn dense_frame(&self) -> bool {
        (0..self.width).any(|i| self.dense_column(i)) && (0..self.height).any(|j| self.dense_row(j))
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut map = vec![vec!['.'; self.width + 2]; self.height + 2];
        map[0] = vec!['-'; self.width + 2];
        map[self.height + 1] = vec!['-'; self.width + 2];
        for row in map.iter_mut().skip(1).take(self.height) {
            row[0] = '|';
            row[self.width + 1] = '|';
        }

        for guard in &self.guards {
            let [x, y] = guard.position;
            match map[y + 1][x + 1] {
                '.' => map[y + 1][x + 1] = '1',
                '*' => map[y + 1][x + 1] = '*',
                c => {
                    let n = c.to_digit(10).unwrap();
                    if n == 9 {
                        map[y + 1][x + 1] = '*';
                    } else {
                        map[y + 1][x + 1] = char::from_digit(n + 1, 10).unwrap();
                    }
                }
            }
        }

        for row in map {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Quadrant {
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

#[derive(Debug, PartialEq)]
struct Guard {
    position: [usize; 2],
    velocity: [isize; 2],
}

impl Guard {
    fn new(position: [usize; 2], velocity: [isize; 2]) -> Self {
        Self { position, velocity }
    }

    fn from_str(input_str: &str) -> Result<Self> {
        let number_pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

        let numbers: Result<Vec<isize>> = number_pattern
            .captures_iter(input_str)
            .flat_map(|c| {
                vec![
                    c[1].parse().map_err(|_| anyhow!("Integer parse error")),
                    c[2].parse().map_err(|_| anyhow!("Integer parse error")),
                    c[3].parse().map_err(|_| anyhow!("Integer parse error")),
                    c[4].parse().map_err(|_| anyhow!("Integer parse error")),
                ]
            })
            .collect();

        let numbers = numbers?;

        ensure!(numbers.len() == 4, "Expected 4 numbers in the input string");
        ensure!(
            numbers[0] >= 0 && numbers[1] >= 0,
            "Position values must be non-negative"
        );

        Ok(Self {
            position: [numbers[0] as usize, numbers[1] as usize],
            velocity: [numbers[2], numbers[3]],
        })
    }

    fn assign_quadrant(&self, width: usize, height: usize) -> Option<Quadrant> {
        let (mid_width, mid_height) = (width / 2, height / 2);
        let [x, y] = self.position;
        if x == mid_width || y == mid_height {
            return None;
        }

        if x < mid_width && y < mid_height {
            Some(Quadrant::NorthWest)
        } else if y < mid_height {
            Some(Quadrant::NorthEast)
        } else if x > mid_width && y > mid_height {
            Some(Quadrant::SouthEast)
        } else {
            Some(Quadrant::SouthWest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_line() -> &'static str {
        "p=0,4 v=3,-3"
    }

    #[fixture]
    fn expected_guard() -> Guard {
        Guard::new([0, 4], [3, -3])
    }

    #[rstest]
    fn test_from_str(sample_line: &str, expected_guard: Guard) {
        assert_eq!(Guard::from_str(sample_line).unwrap(), expected_guard)
    }

    #[fixture]
    fn sample_input() -> &'static str {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    }

    #[fixture]
    fn expected_guards() -> Vec<Guard> {
        vec![
            Guard::new([0, 4], [3, -3]),
            Guard::new([6, 3], [-1, -3]),
            Guard::new([10, 3], [-1, 2]),
            Guard::new([2, 0], [2, -1]),
            Guard::new([0, 0], [1, 3]),
            Guard::new([3, 0], [-2, -2]),
            Guard::new([7, 6], [-1, -3]),
            Guard::new([3, 0], [-1, -2]),
            Guard::new([9, 3], [2, 3]),
            Guard::new([7, 3], [-1, 2]),
            Guard::new([2, 4], [2, -3]),
            Guard::new([9, 5], [-3, -3]),
        ]
    }

    #[rstest]
    fn test_preprocessing(sample_input: &str, expected_guards: Vec<Guard>) {
        let guards: Result<Vec<Guard>> = sample_input
            .lines()
            .map(|line| Guard::from_str(line))
            .collect();
        assert_eq!(guards.unwrap(), expected_guards);
    }

    #[rstest]
    fn test_exercise_1(sample_input: &str) {
        assert_eq!(exercise_1(sample_input, 11, 7, 100, true).unwrap(), 12);
    }
}
