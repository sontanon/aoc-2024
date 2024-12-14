use anyhow::{anyhow, Result, ensure};
use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Guard {
    x: usize,
    y: usize,
    orientation: Orientation,
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let orientation = match self.orientation {
            Orientation::North => "^",
            Orientation::East => ">",
            Orientation::South => "v",
            Orientation::West => "<",
        };
        write!(f, "{}", orientation)
    }
}

#[derive(Clone, PartialEq)]
enum MazeCell {
    Obstacle,
    Empty,
}

impl Display for MazeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeCell::Obstacle => write!(f, "#"),
            MazeCell::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone)]
struct Maze {
    n_rows: usize,
    n_cols: usize,
    cells: Vec<Vec<MazeCell>>,
    guard: Guard,
    visited_locations: HashSet<(usize, usize)>,
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i == self.guard.y && j == self.guard.x {
                    write!(f, "{}", self.guard)?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn preprocessing(input_string: &str) -> Result<Maze> {
    let num_rows = input_string.lines().count();
    let num_cols = input_string.lines().next().ok_or_else(|| anyhow!("Input must contain at least one row."))?.chars().count();

    ensure!(
        num_rows > 0 && num_cols > 0,
        "Input must contain at least one row and one column."
    );

    let mut cells = vec![vec![MazeCell::Empty; num_cols]; num_rows];
    let mut visited_locations = HashSet::new();

    let mut guard = Guard {
        x: 0,
        y: 0,
        orientation: Orientation::North,
    };

    for (i, line) in input_string.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => cells[i][j] = MazeCell::Obstacle,
                '^' => {
                    guard.x = j;
                    guard.y = i;
                    guard.orientation = Orientation::North;
                }
                '>' => {
                    guard.x = j;
                    guard.y = i;
                    guard.orientation = Orientation::East;
                }
                'v' => {
                    guard.x = j;
                    guard.y = i;
                    guard.orientation = Orientation::South;
                }
                '<' => {
                    guard.x = j;
                    guard.y = i;
                    guard.orientation = Orientation::West;
                }
                _ => (),
            }
        }
    }
    visited_locations.insert((guard.x, guard.y));

    Ok(Maze {
        n_rows: num_rows,
        n_cols: num_cols,
        cells,
        guard,
        visited_locations,
    })
}

#[derive(Debug, Clone)]
enum GuardMove {
    TurnRight,
    MoveForward(usize, usize),
    ExitMaze,
}



impl Maze {
    fn move_guard(&mut self) -> GuardMove {
        let (x, y) = (self.guard.x, self.guard.y);
        match self.guard.orientation {
            Orientation::North => {
                if y == 0 {
                    return GuardMove::ExitMaze;
                }
                match self.cells[y - 1][x] {
                    MazeCell::Obstacle => GuardMove::TurnRight,
                    MazeCell::Empty => {
                        self.guard.y -= 1;
                        self.visited_locations.insert((x, y - 1));
                        GuardMove::MoveForward(x, y - 1)
                    }
                }
            }
            Orientation::East => {
                if x == self.n_cols - 1 {
                    return GuardMove::ExitMaze;
                }
                match self.cells[y][x + 1] {
                    MazeCell::Obstacle => GuardMove::TurnRight,
                    MazeCell::Empty => {
                        self.guard.x += 1;
                        self.visited_locations.insert((x + 1, y));
                        GuardMove::MoveForward(x + 1, y)
                    }
                }
            }
            Orientation::South => {
                if y == self.n_rows - 1 {
                    return GuardMove::ExitMaze;
                }
                match self.cells[y + 1][x] {
                    MazeCell::Obstacle => GuardMove::TurnRight,
                    MazeCell::Empty => {
                        self.guard.y += 1;
                        self.visited_locations.insert((x, y + 1));
                        GuardMove::MoveForward(x, y + 1)
                    }
                }
            }
            Orientation::West => {
                if x == 0 {
                    return GuardMove::ExitMaze;
                }
                match self.cells[y][x - 1] {
                    MazeCell::Obstacle => GuardMove::TurnRight,
                    MazeCell::Empty => {
                        self.guard.x -= 1;
                        self.visited_locations.insert((x - 1, y));
                        GuardMove::MoveForward(x - 1, y)
                    }
                }
            }
        }

    }

}

fn exercise_1(maze: &mut Maze) -> usize {
    loop {
        let mv = maze.move_guard();
        match mv {
            GuardMove::TurnRight => {
                match maze.guard.orientation {
                    Orientation::North => maze.guard.orientation = Orientation::East,
                    Orientation::East => maze.guard.orientation = Orientation::South,
                    Orientation::South => maze.guard.orientation = Orientation::West,
                    Orientation::West => maze.guard.orientation = Orientation::North,
                }
            }
            GuardMove::MoveForward(_x, _y) => {
                // println!("Guard moved to ({}, {})", x, y);
            }
            GuardMove::ExitMaze => {
                println!("Guard exited the maze.");
                break;
            }
        }
    }

    maze.visited_locations.len()
}

fn restore_maze(maze: &mut Maze, starting_pos: (usize, usize, Orientation), old_obstacle: (usize, usize)) {
    maze.guard.x = starting_pos.0;
    maze.guard.y = starting_pos.1;
    maze.guard.orientation = starting_pos.2;
    maze.cells[old_obstacle.1][old_obstacle.0] = MazeCell::Empty;
}

fn break_condition(maze: &Maze, visited_locations_and_orientations: &HashSet<(usize, usize, Orientation)>) -> bool {
    let (x, y, orientation) = (maze.guard.x, maze.guard.y, maze.guard.orientation);
    visited_locations_and_orientations.contains(&(x, y, orientation))
}

fn exercise_2(maze: &mut Maze) -> usize {
    let starting_pos = (maze.guard.x, maze.guard.y, maze.guard.orientation);
    let mut positions_with_no_exit = 0;

    // Place a new obstacle in each cell of the maze and try to exit the maze.
    for x in 0..maze.n_cols {
        for y in 0..maze.n_rows {
            // Cannot place an obstacle in the guard starting location.
            if (x, y) == (starting_pos.0, starting_pos.1) {
                continue;
            }
            // Cannot place an obstacle if there is already an obstacle.
            if maze.cells[y][x] == MazeCell::Obstacle {
                continue;
            }
            maze.cells[y][x] = MazeCell::Obstacle;

            let mut visited_locations_and_orientations: HashSet<(usize, usize, Orientation)> = HashSet::new();
            visited_locations_and_orientations.insert(starting_pos);

            loop {
                let mv = maze.move_guard(); 
                match mv {
                    GuardMove::TurnRight => {
                        match maze.guard.orientation {
                            Orientation::North => maze.guard.orientation = Orientation::East,
                            Orientation::East => maze.guard.orientation = Orientation::South,
                            Orientation::South => maze.guard.orientation = Orientation::West,
                            Orientation::West => maze.guard.orientation = Orientation::North,
                        }
                        if break_condition(maze, &visited_locations_and_orientations) {
                            positions_with_no_exit += 1;
                            restore_maze(maze, starting_pos, (x, y));
                            break;
                        } 
                        visited_locations_and_orientations.insert((maze.guard.x, maze.guard.y, maze.guard.orientation));
                    }
                    GuardMove::MoveForward(_, _) => {
                        // println!("Guard moved to ({}, {})", x, y);
                        if break_condition(maze, &visited_locations_and_orientations) {
                            positions_with_no_exit += 1;
                            restore_maze(maze, starting_pos, (x, y));
                            break;
                        } else {
                            visited_locations_and_orientations.insert((x, y, maze.guard.orientation));
                        }
                    }
                    GuardMove::ExitMaze => {
                        // println!("Guard exited the maze.");
                        restore_maze(maze, starting_pos, (x, y));
                        break;
                    }
                }
            }
        }
    }

    positions_with_no_exit
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string("input.txt")?;
    let maze = preprocessing(&input_string)?;

    println!("{}", maze);
    println!("Guard: {:?}", maze.guard);

    let result_1 = exercise_1(&mut maze.clone());
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&mut maze.clone());
    println!("Result 2: {}", result_2);

    Ok(())
}
