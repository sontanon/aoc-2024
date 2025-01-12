use anyhow::{anyhow, ensure, Result};
use std::{fs, str::FromStr};

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
struct State {
    x: usize,
    y: usize,
    can_cheat: bool,
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cells = vec![];
        let mut start = None;
        let mut goal = None;

        for (y, line) in s.lines().enumerate() {
            let mut row = vec![];
            for (x, cell) in line.chars().enumerate() {
                match cell {
                    'S' => {
                        if start.is_some() {
                            return Err(anyhow!("Multiple starts"));
                        }
                        start = Some((x, y));
                        row.push(Cell::Empty);
                    }
                    'E' => {
                        if goal.is_some() {
                            return Err(anyhow!("Multiple goals"));
                        }
                        goal = Some((x, y));
                        row.push(Cell::Empty);
                    }
                    '.' => row.push(Cell::Empty),
                    '#' => row.push(Cell::Blocked),
                    _ => return Err(anyhow!("Invalid cell: {}", cell)),
                }
            }
            cells.push(row);
        }

        let (cols, rows) = (cells[0].len(), cells.len());
        ensure!(cols > 0, "Maze must have at least one column");
        ensure!(rows > 0, "Maze must have at least one row");
        ensure!(
            cells.iter().all(|row| row.len() == cols),
            "All rows must have the same length"
        );
        let (start, goal) = match (start, goal) {
            (Some(start), Some(goal)) => (start, goal),
            _ => return Err(anyhow!("Missing start or goal")),
        };

        Ok(Self {
            cells,
            rows,
            cols,
            start,
            goal,
        })
    }
}

impl Maze {
    fn goal_test(&self, state: &State) -> bool {
        state.x == self.goal.0 && state.y == self.goal.1
    }

    fn successors(&self, state: &State, cheats_enabled: bool) -> Vec<State> {
        let mut successors = vec![];
        let (x, y) = (state.x, state.y);

        if x > 0 && self.cells[y][x - 1] == Cell::Empty {
            successors.push(State {
                x: x - 1,
                y,
                can_cheat: state.can_cheat,
            });
        }

        if x < self.cols - 1 && self.cells[y][x + 1] == Cell::Empty {
            successors.push(State {
                x: x + 1,
                y,
                can_cheat: state.can_cheat,
            });
        }

        if y > 0 && self.cells[y - 1][x] == Cell::Empty {
            successors.push(State {
                x,
                y: y - 1,
                can_cheat: state.can_cheat,
            });
        }

        if y < self.rows - 1 && self.cells[y + 1][x] == Cell::Empty {
            successors.push(State {
                x,
                y: y + 1,
                can_cheat: state.can_cheat,
            });
        }

        if cheats_enabled && state.can_cheat {
            if x > 0 && self.cells[y][x - 1] == Cell::Blocked {
                successors.push(State {
                    x: x - 1,
                    y,
                    can_cheat: false,
                });
            }
            if x < self.cols - 1 && self.cells[y][x + 1] == Cell::Blocked {
                successors.push(State {
                    x: x + 1,
                    y,
                    can_cheat: false,
                });
            }
            if y > 0 && self.cells[y - 1][x] == Cell::Blocked {
                successors.push(State {
                    x,
                    y: y - 1,
                    can_cheat: false,
                });
            }
            if y < self.rows - 1 && self.cells[y + 1][x] == Cell::Blocked {
                successors.push(State {
                    x,
                    y: y + 1,
                    can_cheat: false,
                });
            }
        }

        successors
    }

    fn no_cheat_length(&self) -> Result<usize> {
        let successors = |state: &State| self.successors(state, false);
        let goal_test = |state: &State| self.goal_test(state);
        bfs(
            State {
                x: self.start.0,
                y: self.start.1,
                can_cheat: true,
            },
            goal_test,
            successors,
        )
        .map(|solution| solution.node_to_path().len() - 1)
        .ok_or_else(|| anyhow!("No solution found"))
    }
}


fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    // println!("{}", exercise_1(&input)?);

    Ok(())
}
