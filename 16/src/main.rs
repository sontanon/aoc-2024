use anyhow::{anyhow, ensure, Result};
use std::{
    fs,
    fmt::Display,
    str::FromStr,
};

mod generic_search;
use generic_search::{astar, Node};

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let result_1 = exercise_1(&input_str)?;
    println!("Exercise 1: {}", result_1);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let maze = Maze::from_str(input_str)?;
    maze.astar_path()
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
}

impl Cell {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            _ => Err(anyhow!("Invalid cell character: {}", c)),
        }
    }

    fn is_passable(&self) -> bool {
        !matches!(self, Self::Wall)
    }
}

#[derive(Debug, PartialEq)]
struct Maze {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
    start: (usize, usize),
    goal: (usize, usize),
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut goal = None;
        let cells = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let cell = Cell::from_char(c)?;
                        match cell {
                            Cell::Start => {
                                ensure!(start.is_none(), "Multiple start cells");
                                start = Some((x, y));
                            }
                            Cell::End => {
                                ensure!(goal.is_none(), "Multiple goal cells");
                                goal = Some((x, y));
                            }
                            _ => {}
                        }
                        Ok(cell)
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        ensure!(start.is_some(), "No start cell");
        ensure!(goal.is_some(), "No goal cell");
        let rows = cells.len();
        let cols = cells[0].len();
        ensure!(
            cells.iter().all(|row| row.len() == cols),
            "Inconsistent row length"
        );

        Ok(Maze {
            cells,
            rows,
            cols,
            start: start.unwrap(),
            goal: goal.unwrap(),
        })
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                let c = match cell {
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Start => 'S',
                    Cell::End => 'E',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, std::hash::Hash)]
enum Orientation {
    North,
    South,
    West,
    East,
}

impl Orientation {
    fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Self::North => *other == Self::South,
            Self::South => *other == Self::North,
            Self::East => *other == Self::West,
            Self::West => *other == Self::East,
        }
    }

    fn num_rotations(&self, other: &Self) -> usize {
        if self == other {
            return 0;
        }

        if self.is_opposite(other) {
            return 2;
        }
        1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, std::hash::Hash)]
struct State(Orientation, (usize, usize));

impl State {
    fn x(&self) -> usize {
        self.1 .0
    }

    fn y(&self) -> usize {
        self.1 .1
    }

    fn orientation(&self) -> &Orientation {
        &self.0
    }

    fn movement_cost(&self, other: &Self) -> Result<usize> {
        if usize::abs_diff(self.x(), other.x()) > 1 || usize::abs_diff(self.y(), other.y()) > 1 {
            return Err(anyhow!("Impossible to reach states"));
        }

        if self.orientation().is_opposite(other.orientation()) {
            return Err(anyhow!("Cannot flip orientation"));
        }

        if self.orientation() == other.orientation() {
            Ok(1)
        } else {
            // Otherwise, must have rotated 90 degrees _and then moved_ forward.
            Ok(1_001)
        }
    }

    fn manhattan_heuristic(&self, (x_n, y_n): (usize, usize)) -> usize {
        usize::abs_diff(self.x(), x_n) + usize::abs_diff(self.y(), y_n)
    }

    // fn rotations_heuristic(&self, (x_n, y_n): (usize, usize)) -> usize {
    //     let (x, y) = (self.x(), self.y());
    //     match (x_n.cmp(&x), y_n.cmp(&y)) {
    //         (Ordering::Equal, Ordering::Equal) => panic!("Should not calculate heuristic on equal cells"),
    //         // Final orientation will 
    //         (Ordering::Equal, Ordering::Greater) => y_n - y + 1_000 * self.orientation().num_rotations(&Orientation::Down),
    //         (Ordering::Equal, Ordering::Less) => y - y_n + 1_000 * self.orientation().num_rotations(&Orientation::Up),
    //         (Ordering::Greater, Ordering::Equal) => x_n - x + 1_000 * self.orientation().num_rotations(&Orientation::Right),
    //         (Ordering::Less, Ordering::Equal) => x - x_n + 1_000 * self.orientation().num_rotations(&Orientation::Left),
    //         _ => todo!(),
    //     }
    // }
}

impl Maze {
    fn astar_path(&self) -> Result<usize> {
        // Define closures.
        let successors = |state: &State| self.successors(state);
        let goal_test = |state: &State| self.goal_test(state.x(), state.y());
        let heuristic = |state: &State| state.manhattan_heuristic(self.goal);
        let cost = |parent: &Node<State>, child: &State| {
            parent.get_cost() + parent.get_state().movement_cost(child).unwrap()
        };

        if let Some(node) = astar(State(Orientation::East, self.start), goal_test, successors, heuristic, cost) {
            // let path = node.node_to_path();
            // println!("Path: {:?}", path);
            return Ok(node.get_cost());
        }

        Err(anyhow!("No path found"))
    }

    fn goal_test(&self, x: usize, y: usize) -> bool {
        (x, y) == self.goal
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn successors(&self, state: &State) -> Vec<State> {
        let (o, (x, y)) = (&state.0, state.1);
        let mut successors = vec![];
        if x > 1 && self.get_cell(x - 1, y).is_passable() && *o != Orientation::East {
            successors.push(State(Orientation::West, (x - 1, y)));
        }
        if y > 1 && self.get_cell(x, y - 1).is_passable() && *o != Orientation::South {
            successors.push(State(Orientation::North, (x, y - 1)));
        }
        if x < self.cells[0].len() - 2
            && self.get_cell(x + 1, y).is_passable()
            && *o != Orientation::West
        {
            successors.push(State(Orientation::East, (x + 1, y)));
        }
        if y < self.cells.len() - 2
            && self.get_cell(x, y + 1).is_passable()
            && *o != Orientation::North
        {
            successors.push(State(Orientation::South, (x, y + 1)));
        }
        successors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        7036
    )]
    #[case(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        11048
    )]
    fn test_solution(#[case] input: &str, #[case] expected: usize) {
        let result = exercise_1(input).unwrap();
        assert_eq!(result, expected);
    }
}
