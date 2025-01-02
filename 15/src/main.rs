use anyhow::{anyhow, ensure, Result};
use std::{fs, mem::swap};

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _ => Err(anyhow!("Invalid character: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum WarehouseCell {
    Empty,
    Wall,
    Box,
    Robot,
}

impl WarehouseCell {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            _ => Err(anyhow!("Invalid character: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Warehouse {
    cells: Vec<Vec<WarehouseCell>>,
    width: usize,
    height: usize,
    robot_location: (usize, usize),
}

impl Warehouse {
    fn from_input_str(input_str: &str) -> Result<Self> {
        let mut robot_location: Option<(usize, usize)> = None;
        let cells: Result<Vec<Vec<WarehouseCell>>> = input_str
            .lines()
            .enumerate()
            .map(|(j, line)| -> Result<Vec<WarehouseCell>> {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let c = WarehouseCell::from_char(c)?;
                        if let WarehouseCell::Robot = c {
                            robot_location = Some((i, j));
                        }
                        Ok(c)
                    })
                    .collect()
            })
            .collect();
        let cells = cells?;
        let height = cells.len();
        let width = cells[0].len();

        ensure!(height > 0, "Warehouse must have at least one row");
        ensure!(width > 0, "Warehouse must have at least one column");
        ensure!(
            cells.iter().all(|row| row.len() == width),
            "All rows must have the same length"
        );
        ensure!(robot_location.is_some(), "Robot not found");
        ensure!(
            cells[0].iter().all(|cell| *cell == WarehouseCell::Wall),
            "First row must be a full wall",
        );
        ensure!(
            cells[height - 1]
                .iter()
                .all(|cell| *cell == WarehouseCell::Wall),
            "Last row must be a full wall",
        );
        ensure!(
            cells.iter().all(|row| row[0] == WarehouseCell::Wall),
            "First column must be a full wall",
        );
        ensure!(
            cells
                .iter()
                .all(|row| row[width - 1] == WarehouseCell::Wall),
            "Last column must be a full wall",
        );

        Ok(Self {
            width,
            height,
            cells,
            robot_location: robot_location.unwrap(),
        })
    }

    fn move_robot(&mut self, mv: &Move) -> Result<bool> {
        let (x_r, y_r) = self.robot_location;
        ensure!(
            x_r > 0 && x_r < self.width - 1 && y_r > 0 && y_r < self.height - 1,
            "Cannot move robot on the edge"
        );

        let (x_p, y_p) = match mv {
            Move::Up => (x_r, y_r - 1),
            Move::Down => (x_r, y_r + 1),
            Move::Left => (x_r - 1, y_r),
            Move::Right => (x_r + 1, y_r),
        };

        match &self.cells[y_p][x_p] {
            // Simplest case, the cell where we are moving to is empty.
            WarehouseCell::Empty => {
                self.cells[y_r][x_r] = WarehouseCell::Empty;
                self.cells[y_p][x_p] = WarehouseCell::Robot;
                self.robot_location = (x_p, y_p);
                Ok(true)
            }
            // Also simple, bump directly into a wall: no move but Ok.
            WarehouseCell::Wall => Ok(false),
            // This cannot be possible since there is only one robot.
            WarehouseCell::Robot => Err(anyhow!("Robot cannot move into another robot")),
            // Real problem: move into a box an try to push it...
            WarehouseCell::Box => match mv {
                Move::Up => {
                    // Find an empty cell or a wall above: cells[0..y_r][x_r]
                    // Notice that we are searching from the right.
                    // In other words, our search is y_r-1, y_r-2, ..., 0.
                    if let Some(y_c) = (0..y_r)
                        .map(|j| &self.cells[j][x_r])
                        .rposition(|c| *c != WarehouseCell::Box)
                    {
                        // If we found a wall first, then there is no move.
                        if self.cells[y_c][x_r] == WarehouseCell::Wall {
                            return Ok(false);
                        }
                        // Otherwise, we found an empty cell and can move all cells (including the robot) one cell above.
                        // We know the cell at (y_c, x_r) is empty and all cells with y_c < y <= y_r are non-empty.
                        // Here we are essentially swapping from the left cells[j][x_r] <-> cells[j+1][x_r].
                        // Given Rust's borrowing rules, we have to do some juggling, though...
                        // Alternatively, we could recurse here?
                        for j in y_c..y_r {
                            let (up, down) = self.cells.split_at_mut(j + 1);
                            swap(&mut up[j][x_r], &mut down[0][x_r]);
                        }
                        return Ok(true);
                    }
                    Ok(false)
                }
                Move::Down => {
                    // Find an empty cell or a wall below: cells[y_r+1..self.height][x_r]
                    // Notice that we are searching from the left.
                    // In other words, our search is y_r+1, y_r+2, ..., self.height - 1.
                    if let Some(y_c) = (y_r + 1..self.height)
                        .map(|j| &self.cells[j][x_r])
                        .position(|c| *c != WarehouseCell::Box)
                    {
                        // If we found a wall first, then there is no move.
                        if self.cells[y_c][x_r] == WarehouseCell::Wall {
                            return Ok(false);
                        }
                        // Otherwise, we found an empty cell and can move all cells (including the robot) once cell below.
                        // We know the cell at (y_c, x_r) is empty and all cells with y_r <= y < y_c are non-empty.
                        // Here we are essentially swapping from the right cells[j][x_r] <-> cells[j-1][x_r].
                        // Careful with the range since it is reversed and must not touch y_r.
                        for j in ((y_r + 1)..=y_c).rev() {
                            let (up, down) = self.cells.split_at_mut(j);
                            swap(&mut up[j - 1][x_r], &mut down[0][x_r]);
                        }
                        return Ok(true);
                    }
                    Ok(false)
                }
                Move::Left => {
                    // Find an empty cell or a wall to the left: cells[y_r][0..x_r]
                    // Notice that we are searching from the right.
                    // In other words, our search is x_r-1, x_r-2, ..., 0.
                    if let Some(x_c) = (0..x_r)
                        .map(|i| &self.cells[y_r][i])
                        .rposition(|c| *c != WarehouseCell::Box)
                    {
                        // If we found a wall first, then there is no move.
                        if self.cells[y_r][x_c] == WarehouseCell::Wall {
                            return Ok(false);
                        }
                        // Otherwise, we found an empty cell and can move all cells (including the robot) one cell to the left.
                        // We know the cell at (y_r, x_c) is empty and all cells with x_c < x <= x_r are non-empty.
                        // Here we are essentially swapping from the left cells[y_r][i] <-> cells[y_r][i+1].
                        // Given Rust's borrowing rules, we have to do some juggling, though...
                        // Alternatively, we could recurse here?
                        for i in x_c..x_r {
                            let (left, right) = self.cells[y_r].split_at_mut(i + 1);
                            swap(&mut left[i], &mut right[0]);
                        }
                        return Ok(true);
                    }
                    Ok(false)
                }
                Move::Right => {
                    // Find an empty cell or a wall to the right: cells[y_r][x_r+1..self.width]
                    // Notice that we are searching from the left.
                    // In other words, our search is x_r+1, x_r+2, ..., self.width - 1.
                    if let Some(x_c) = (x_r + 1..self.width)
                        .map(|i| &self.cells[y_r][i])
                        .position(|c| *c != WarehouseCell::Box)
                    {
                        // If we found a wall first, then there is no move.
                        if self.cells[y_r][x_c] == WarehouseCell::Wall {
                            return Ok(false);
                        }
                        // Otherwise, we found an empty cell and can move all cells (including the robot) one cell to the right.
                        // We know the cell at (y_r, x_c) is empty and all cells with x_r <= x < x_c are non-empty.
                        // Here we are essentially swapping from the right cells[y_r][i] <-> cells[y_r][i-1].
                        // Careful with the range since it is reversed and must not touch x_r.
                        for i in ((x_r + 1)..=x_c).rev() {
                            let (left, right) = self.cells[y_r].split_at_mut(i);
                            swap(&mut left[i - 1], &mut right[0]);
                        }
                        return Ok(true);
                    }
                    Ok(false)
                }
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn preprocessing(input_str: &str) -> Result<(Warehouse, Vec<Move>)> {
    let (warehouse_str, moves_str) = input_str
        .split_once("\n\n")
        .ok_or(anyhow!("Invalid input"))?;
    let warehouse = Warehouse::from_input_str(warehouse_str)?;
    let moves: Result<Vec<Move>> = moves_str
        .chars()
        .filter(|c| *c != '\n')
        .map(Move::from_char)
        .collect();
    let moves = moves?;
    Ok((warehouse, moves))
}

// fn exercise_1(input_str: &str) -> Result<usize> {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_input_str_simple() -> &'static str {
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<"
    }

    fn sample_states() -> Vec<Warehouse> {
        vec![
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
            "########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
            
        ]
    }

    #[fixture]
    fn sample_input_str() -> &'static str {
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    }

    #[fixture]
    fn expected_moves_simple() -> Vec<Move> {
        vec![
            Move::Left,
            Move::Up,
            Move::Up,
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Down,
            Move::Down,
            Move::Left,
            Move::Down,
            Move::Right,
            Move::Right,
            Move::Down,
            Move::Left,
            Move::Left,
        ]
    }

    #[fixture]
    fn expected_position_simple() -> (usize, usize) {
        (4, 4)
    }

    #[fixture]
    fn expected_warehouse() -> Warehouse {
        Warehouse {
            width: 8,
            height: 8,
            cells: vec![
                vec![WarehouseCell::Wall; 8],
                vec![
                    WarehouseCell::Wall,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Box,
                    WarehouseCell::Empty,
                    WarehouseCell::Box,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                ],
                vec![
                    WarehouseCell::Wall,
                    WarehouseCell::Wall,
                    WarehouseCell::Robot,
                    WarehouseCell::Empty,
                    WarehouseCell::Box,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                ],
                vec![
                    WarehouseCell::Wall,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Box,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                ],
                vec![
                    WarehouseCell::Wall,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                    WarehouseCell::Empty,
                    WarehouseCell::Box,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                ],
                vec![
                    WarehouseCell::Wall,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Box,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                ],
                vec![
                    WarehouseCell::Wall,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Empty,
                    WarehouseCell::Wall,
                ],
                vec![WarehouseCell::Wall; 8],
            ],
            robot_location: (2, 2),
        }
    }

    #[rstest]
    fn test_preprocessing(
        sample_input_str_simple: &str,
        expected_warehouse: Warehouse,
        expected_moves_simple: Vec<Move>,
    ) {
        let (warehouse, moves) = preprocessing(sample_input_str_simple).unwrap();
        assert_eq!(warehouse, expected_warehouse);
        assert_eq!(moves, expected_moves_simple);
    }

    #[rstest]
    fn test_move_left(expected_warehouse: Warehouse) {
        let mut warehouse = expected_warehouse.clone();
        assert_eq!(warehouse.move_robot(&Move::Left).unwrap(), false);
    }

    #[rstest]
    fn test_move_right(expected_warehouse: Warehouse) {
        let mut warehouse = expected_warehouse.clone();
        assert_eq!(warehouse.move_robot(&Move::Right).unwrap(), true);
    }

    #[rstest]
    fn test_move_up(expected_warehouse: Warehouse) {
        let mut warehouse = expected_warehouse.clone();
        assert_eq!(warehouse.move_robot(&Move::Up).unwrap(), true);
    }
    #[rstest]
    fn test_move_down(expected_warehouse: Warehouse) {
        let mut warehouse = expected_warehouse.clone();
        assert_eq!(warehouse.move_robot(&Move::Down).unwrap(), true);
    }
    #[rstest]
    fn test_simple_push(expected_warehouse: Warehouse) {
        let mut warehouse = expected_warehouse.clone();
        assert_eq!(warehouse.move_robot(&Move::Up).unwrap(), true);
        assert_eq!(warehouse.move_robot(&Move::Right).unwrap(), true);
    }

    #[rstest]
    fn test_simple_moves(
        expected_warehouse: Warehouse,
        expected_moves_simple: Vec<Move>,
        expected_position_simple: (usize, usize),
    ) {
        let mut warehouse = expected_warehouse.clone();
        for mv in expected_moves_simple.iter() {
            warehouse.move_robot(mv).unwrap();
        }
        assert_eq!(warehouse.robot_location, expected_position_simple);
    }
}
