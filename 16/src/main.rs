use anyhow::{anyhow, ensure, Result};
use std::{
    collections::{BinaryHeap, HashSet, LinkedList, VecDeque},
    fs,
    str::FromStr,
};

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let result_1 = exercise_1(&input_str)?;
    println!("Exercise 1: {}", result_1);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let mut maze = Maze::from_str(input_str)?;
    todo!()
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
    end: (usize, usize),
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut end = None;
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
                                ensure!(end.is_none(), "Multiple end cells");
                                end = Some((x, y));
                            }
                            _ => {}
                        }
                        Ok(cell)
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        ensure!(start.is_some(), "No start cell");
        ensure!(end.is_some(), "No end cell");
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
            end: end.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Self::Up => *other == Self::Down,
            Self::Down => *other == Self::Up,
            Self::Right => *other == Self::Left,
            Self::Left => *other == Self::Right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State(Orientation, (usize, usize));

impl State {
    fn x(&self) -> usize {
        self.1.0
    }

    fn y(&self) -> usize {
        self.1.1
    } 

    fn orientation(&self) -> &Orientation {
        &self.0
    }
    
    fn cost(&self, other: &Self) -> Result<usize> {
        if usize::abs_diff(self.x(), other.x()) > 1 || usize::abs_diff(self.y(), other.y()) > 1 {
            return Err(anyhow!("Impossible to reach states"))
        }

        if self.orientation().is_opposite(other.orientation()) {
            return Err(anyhow!("Cannot flip orientation"))
        }

        if self.orientation() == other.orientation() {
            Ok(1)
        } else {
            // Otherwise, must have rotated 90 degrees.
            Ok(1_000)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    state: State,
    parent: Option<Box<Node>>,
    cost: usize,
    heuristic: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.cost + self.heuristic).partial_cmp(&(other.cost + &other.heuristic))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost + self.heuristic).cmp(&(other.cost + &other.heuristic))
    }
}


impl Node {
    fn new(state: State, parent: Option<&Node>, cost: usize, heuristic: usize) -> Self {
        Node {
            state,
            parent: parent.map(|p| Box::new(p.to_owned())),
            cost,
            heuristic,
        }
    }
}

impl Maze {
    fn astar_paths(&self) -> Option<Node> {
        let mut frontier = BinaryHeap::from([
            Node::new(State(Orientation::Up, self.start), None, 0, 0),
            Node::new(State(Orientation::Right, self.start), None, 0, 0),
            Node::new(State(Orientation::Down, self.start), None, 0, 0),
            Node::new(State(Orientation::Left, self.start), None, 0, 0),
        ]);
        let mut explored = HashSet::from([self.start]);
        let mut paths: Vec<Node> = Vec::new();

        while let Some(current_node) = frontier.pop_front() {
            let (x, y) = current_node.state;
            if self.is_goal(x, y) {
                return Some(current_node);
            }

            self.get_successors(x, y).into_iter().for_each(|c| {
                if explored.contains(&c) {
                    ()
                }
                explored.insert(c.clone());
                frontier.push_back(Node::new(c, Some(&current_node)));
            });
        }
        None
    }

    fn is_goal(&self, x: usize, y: usize) -> bool {
        (x, y) == self.end
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn get_successors(&self, state: State) -> Vec<State> {
        let (o, (x, y)) = (state.0, state.1);
        let mut successors = vec![];
        if x > 0 && self.get_cell(x - 1, y).is_passable() && o != Orientation::Right {
            successors.push(State(Orientation::Left, (x - 1, y)));
        }
        if y > 0 && self.get_cell(x, y - 1).is_passable() && o != Orientation::Down {
            successors.push(State(Orientation::Up, (x, y - 1)));
        }
        if x < self.cells[0].len() - 1
            && self.get_cell(x + 1, y).is_passable()
            && o != Orientation::Left
        {
            successors.push(State(Orientation::Right, (x + 1, y)));
        }
        if y < self.cells.len() - 1
            && self.get_cell(x, y + 1).is_passable()
            && o != Orientation::Down
        {
            successors.push(State(Orientation::Up, (x, y + 1)));
        }
        successors
    }
}

enum Move {
    Forward,
    RotateLeft,
    RotateRight,
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
