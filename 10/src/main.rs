use anyhow::{anyhow, ensure, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&input_str)?;
    println!("Result 2: {}", result_2);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let mut trail_map = TopographicMap::from_str(input_str)?;

    trail_map.calculate_all_trailhead_scores();

    Ok(trail_map
        .trailhead_scores
        .iter()
        .filter_map(|(_, &score)| score)
        .sum())
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let mut trail_map = TopographicMap::from_str(input_str)?;

    trail_map.calculate_all_trailhead_ranks();

    Ok(trail_map
        .trailhead_ranks
        .iter()
        .filter_map(|(_, &rank)| rank)
        .sum())
}

struct TopographicMap {
    map: Vec<Vec<usize>>,
    num_rows: usize,
    num_cols: usize,
    trailheads: HashSet<(usize, usize)>,
    trailhead_scores: HashMap<(usize, usize), Option<usize>>,
    trailhead_ranks: HashMap<(usize, usize), Option<usize>>,
}

// const TRAILHEAD_LEVEL: usize = 0;
const TRAILEND_LEVEL: usize = 9;

impl TopographicMap {
    fn from_str(input_str: &str) -> Result<Self> {
        let map: Result<Vec<Vec<usize>>> = input_str
            .lines()
            .map(|line| -> Result<Vec<usize>> {
                line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| {
                        c.to_string()
                            .parse::<usize>()
                            .map_err(|_| anyhow!("Invalid input for digit {}", c))
                    })
                    .collect()
            })
            .collect();
        let map = map?;

        ensure!(!map.is_empty(), "Input string is empty");

        let num_rows = map.len();
        let num_cols = map[0].len();

        ensure!(
            map.iter().all(|row| row.len() == num_cols),
            "All rows must have the same length"
        );

        let trailheads: HashSet<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &level)| (x, y, level)))
            .filter_map(|(x, y, level)| if level == 0 { Some((x, y)) } else { None })
            .collect();

        let trailhead_scores: HashMap<(usize, usize), Option<usize>> = trailheads
            .iter()
            .map(|&trailhead| (trailhead, None))
            .collect();

        Ok(Self {
            map,
            num_rows,
            num_cols,
            trailheads,
            trailhead_ranks: trailhead_scores.clone(),
            trailhead_scores,
        })
    }


    // fn is_trailhead(&self, (x, y): (usize, usize)) -> bool {
    //     self.map[y][x] == TRAILHEAD_LEVEL
    // }

    fn is_trailend(&self, (x, y): (usize, usize)) -> bool {
        self.map[y][x] == TRAILEND_LEVEL
    }

    fn get_level(&self, (x, y): (usize, usize)) -> usize {
        self.map[y][x]
    }

    fn get_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let level = self.get_level((x, y));

        if level == TRAILEND_LEVEL {
            return vec![];
        }

        let mut neighbors = Vec::with_capacity(4);

        if x > 0 && self.get_level((x - 1, y)) == level + 1 {
            neighbors.push((x - 1, y));
        }
        if x < self.num_cols - 1 && self.get_level((x + 1, y)) == level + 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 && self.get_level((x, y - 1)) == level + 1 {
            neighbors.push((x, y - 1));
        }
        if y < self.num_rows - 1 && self.get_level((x, y + 1)) == level + 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn calculate_trailhead_score(&self, trailhead: (usize, usize)) -> usize {
        let mut visited = HashSet::new();
        let mut queue = vec![trailhead];
        let mut score = 0;

        while let Some(current) = queue.pop() {
            visited.insert(current);

            if self.is_trailend(current) {
                score += 1;
            }

            let neighbors = self.get_neighbors(current);
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }
        score
    }

    fn calculate_trailhead_rank(&self, trailhead: (usize, usize)) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([trailhead]);
        let mut score  = 0;

        while let Some(current) = queue.pop_front() {
            visited.insert(current);

            if self.is_trailend(current) {
                score += 1;
            }

            let neighbors = self.get_neighbors(current);
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        score
    }

    fn calculate_all_trailhead_scores(&mut self) {
        for trailhead in self.trailheads.iter() {
            let score = self.calculate_trailhead_score(*trailhead);
            self.trailhead_scores.insert(*trailhead, Some(score));
        }
    }

    fn calculate_all_trailhead_ranks(&mut self) {
        for trailhead in self.trailheads.iter() {
            let rank = self.calculate_trailhead_rank(*trailhead);
            self.trailhead_ranks.insert(*trailhead, Some(rank));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_input_str() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[fixture]
    fn expected_output_1() -> usize {
        36
    }

    #[fixture]
    fn expected_output_2() -> usize {
        81
    }

    #[fixture]
    fn expected_map() -> TopographicMap {
        TopographicMap {
            map: vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3], // 0
                vec![7, 8, 1, 2, 1, 8, 7, 4], // 1
                vec![8, 7, 4, 3, 0, 9, 6, 5], // 2
                vec![9, 6, 5, 4, 9, 8, 7, 4], // 3
                vec![4, 5, 6, 7, 8, 9, 0, 3], // 4
                vec![3, 2, 0, 1, 9, 0, 1, 2], // 5
                vec![0, 1, 3, 2, 9, 8, 0, 1], // 6
                vec![1, 0, 4, 5, 6, 7, 3, 2], // 7
            ],
            num_rows: 8,
            num_cols: 8,
            trailheads: HashSet::from([
                (2, 0),
                (4, 0),
                (4, 2),
                (6, 4),
                (2, 5),
                (5, 5),
                (0, 6),
                (6, 6),
                (1, 7),
            ]),
            trailhead_scores: HashMap::from([
                ((2, 0), None),
                ((4, 0), None),
                ((4, 2), None),
                ((6, 4), None),
                ((2, 5), None),
                ((5, 5), None),
                ((0, 6), None),
                ((6, 6), None),
                ((1, 7), None),
            ]),
            trailhead_ranks: HashMap::from([
                ((2, 0), None),
                ((4, 0), None),
                ((4, 2), None),
                ((6, 4), None),
                ((2, 5), None),
                ((5, 5), None),
                ((0, 6), None),
                ((6, 6), None),
                ((1, 7), None),
            ]),

        }
    }

    #[rstest]
    fn test_input_parsing(
        #[from(sample_input_str)] input: &str,
        #[from(expected_map)] expected: TopographicMap,
    ) {
        let result = TopographicMap::from_str(input).unwrap();
        assert_eq!(result.map, expected.map);
        assert_eq!(result.num_rows, expected.num_rows);
        assert_eq!(result.num_cols, expected.num_cols);
        assert_eq!(result.trailheads, expected.trailheads);
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
        #[from(sample_input_str)] input: &str,
        #[from(expected_output_2)] expected: usize,
    ) {
        let result = exercise_2(input).unwrap();
        assert_eq!(result, expected);
    }
}
