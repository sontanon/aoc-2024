use anyhow::{ensure, Result};
use regex::Regex;
use std::fs;

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    println!("Exercise 1: {}", exercise_1(&input_str, 101, 103, 100)?);

    Ok(())
}

fn exercise_1(input_str: &str, width: usize, height: usize, timesteps: usize) -> Result<usize> {
    let guards: Result<Vec<Guard>> = input_str
        .lines()
        .map(|line| Guard::from_str(line))
        .collect();
    let guards = guards?;
    let mut map = Map::new(width, height, guards);

    map.advance_timesteps(timesteps);
    let safety_score = map.calculate_guards_per_quadrant()
        .iter()
        .product();

    Ok(safety_score)
}

#[derive(Debug, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    guards: Vec<Guard>,
}

fn move_cyclically(position: usize, velocity: isize, max: usize) -> usize {
    let mut w_position = position;
    if velocity < 0 {
        while (w_position as isize) < -velocity {
            w_position += max;
        }
        w_position + velocity as usize
    } else {
        (w_position + velocity as usize) % max
    }
}

impl Map {
    fn new(width: usize, height: usize, guards: Vec<Guard>) -> Self {
        Self {
            width,
            height,
            guards,
        }
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

    fn advance_timesteps(&mut self, n: usize) {
        for _ in 0..n {
            self.move_guards();
        }
    }

    fn calculate_guards_per_quadrant(&self) -> [usize; 4] {
        let nw_guards = self.guards.iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::NorthWest)
            .count();
        let ne_guards = self.guards.iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::NorthEast)
            .count();
        let se_guards = self.guards.iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::SouthEast)
            .count();
        let sw_guards = self.guards.iter()
            .flat_map(|g| g.assign_quadrant(self.width, self.height))
            .filter(|q| *q == Quadrant::SouthWest)
            .count();

        [nw_guards, ne_guards, se_guards, sw_guards]
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
        Self {position, velocity}
    }

    fn from_str(input_str: &str) -> Result<Self> {
        let number_pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

        let numbers: Vec<isize> = number_pattern
            .captures_iter(input_str)
            .map(|c| isizc.extract())
            .collect();

        ensure!(numbers.len() == 4, "Invalid input string");

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
        "p=0,4, v=3,-3"
    }

    #[fixture]
    fn expected_guard() -> Guard {
        Guard::new([0, 4], [3, -3])
    }

    #[rstest]
    fn test_from_str(
        sample_line: &str, 
        expected_guard: Guard,
    ) {
        assert_eq!(
            Guard::from_str(sample_line).unwrap(),
            expected_guard
        )
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
    fn test_preprocessing(
        sample_input: &str,
        expected_guards: Vec<Guard>, 
    ) {
        let guards: Result<Vec<Guard>> = sample_input.lines()
            .map(|line| Guard::from_str(line))
            .collect();
        assert_eq!(guards.unwrap(), expected_guards);
    }

}