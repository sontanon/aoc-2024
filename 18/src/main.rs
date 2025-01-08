use anyhow::{anyhow, ensure, Result};
use std::fs;

mod generic_search;
use generic_search::{bfs, Node};

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Empty,
    Blocked,
}

struct Maze {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
    start: (usize, usize),
    goal: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, std::hash::Hash)]
struct State((usize, usize));

fn exercise_2(input_str: &str, rows: usize, cols: usize) -> Result<(usize, usize)> {
    let start = (0, 0);
    let goal = (cols - 1, rows - 1);

    let cells = vec![vec![Cell::Empty; cols]; rows];

    let mut maze = Maze {
        cells,
        rows,
        cols,
        start,
        goal,
    };

    let blocks: Result<Vec<(usize, usize)>> = input_str
        .lines()
        .map(|line| {
            if let Some((x, y)) = line.split_once(',') {
                let x = x.parse::<usize>()?;
                let y = y.parse::<usize>()?;
                ensure!(x < cols, "x out of bounds");
                ensure!(y < rows, "y out of bounds");
                Ok((x, y))
            } else {
                Err(anyhow!("Invalid line: {}", line))
            }
        })
        .collect();

    let blocks = blocks?;
    for (x, y) in blocks.into_iter() {
        maze.cells[y][x] = Cell::Blocked;
        if maze.bfs().is_none() {
            return Ok((x, y))
        }
    }
    Err(anyhow!("Expected to find a blocked path."))
}

impl Maze {
    fn new(input_str: &str, rows: usize, cols: usize, bytes: usize) -> Result<Self> {
        let start = (0, 0);
        let goal = (cols - 1, rows - 1);

        let mut cells = vec![vec![Cell::Empty; cols]; rows];

        for line in
            input_str.lines().enumerate().filter_map(
                |(i, line)| {
                    if i < bytes {
                        Some(line)
                    } else {
                        None
                    }
                },
            )
        {
            if let Some((x, y)) = line.split_once(',') {
                let x = x.parse::<usize>()?;
                let y = y.parse::<usize>()?;
                ensure!(x < cols, "x out of bounds");
                ensure!(y < rows, "y out of bounds");
                cells[y][x] = Cell::Blocked;
            } else {
                return Err(anyhow!("Invalid line: {}", line));
            }
        }

        Ok(Self {
            cells,
            rows,
            cols,
            start,
            goal,
        })
    }

    fn goal_test(&self, state: &State) -> bool {
        state.0 == self.goal
    }

    fn successors(&self, state: &State) -> Vec<State> {
        let mut successors = Vec::new();
        let &(x, y) = &state.0;

        if x > 0 && self.cells[y][x - 1] == Cell::Empty {
            successors.push(State((x - 1, y)));
        }

        if x < self.cols - 1 && self.cells[y][x + 1] == Cell::Empty {
            successors.push(State((x + 1, y)));
        }

        if y > 0 && self.cells[y - 1][x] == Cell::Empty {
            successors.push(State((x, y - 1)));
        }

        if y < self.rows - 1 && self.cells[y + 1][x] == Cell::Empty {
            successors.push(State((x, y + 1)));
        }

        successors
    }

    fn bfs(&self) -> Option<Node<State>> {
        let successors = |state: &State| self.successors(state);
        let goal_test = |state: &State| self.goal_test(state);
        bfs(State(self.start), goal_test, successors)
    }

    fn bfs_path_length(&self) -> Option<usize> {
        self.bfs().map(|solution| solution.node_to_path().len() - 1)
    }
}

fn exercise_1(input_str: &str, rows: usize, cols: usize, bytes: usize) -> Result<usize> {
    let maze = Maze::new(input_str, rows, cols, bytes)?;
    maze.bfs_path_length()
        .ok_or_else(|| anyhow!("No solution found"))
}

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str, 71, 71, 1024)?;
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&input_str, 71, 71)?;
    println!("Result 2: {:?}", result_2);

    Ok(())
}
