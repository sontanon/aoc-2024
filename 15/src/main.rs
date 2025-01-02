use anyhow::{anyhow, ensure, Result};
use std::fs;

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
            WarehouseCell::Box => {
                // Get the candidate of where the box(es) may be moved to.
                let (x_c, y_c) = match mv {
                    Move::Up => (
                        Some(x_r),
                        (0..y_p)
                            .map(|j| &self.cells[j][x_r])
                            .rposition(|c| *c != WarehouseCell::Box),
                    ),
                    Move::Down => (
                        Some(x_r),
                        (y_p + 1..self.height)
                            .map(|j| &self.cells[j][x_r])
                            .position(|c| *c != WarehouseCell::Box)
                            .map(|y| y_p + 1 + y),
                    ),
                    Move::Right => (
                        (x_p + 1..self.width)
                            .map(|i| &self.cells[y_r][i])
                            .position(|c| *c != WarehouseCell::Box)
                            .map(|x| x + x_p + 1),
                        Some(y_r),
                    ),
                    Move::Left => (
                        (0..x_p)
                            .map(|i| &self.cells[y_r][i])
                            .rposition(|c| *c != WarehouseCell::Box),
                        Some(y_r),
                    ),
                };
                match (x_c, y_c) {
                    (None, _) => Ok(false),
                    (_, None) => Ok(false),
                    (Some(x_c), Some(y_c)) => {
                        // Candidate cell was a wall, cannot move.
                        if self.cells[y_c][x_c] == WarehouseCell::Wall {
                            return Ok(false);
                        }
                        // Otherwise, make the move.
                        self.cells[y_c][x_c] = WarehouseCell::Box;
                        self.cells[y_p][x_p] = WarehouseCell::Robot;
                        self.cells[y_r][x_r] = WarehouseCell::Empty;
                        self.robot_location = (x_p, y_p);
                        Ok(true)
                    }
                }
            }
        }
    }

    fn calculate_score(&self) -> usize {
        (0..self.height).flat_map(
            |j| (0..self.width)
                .filter_map(
                    move |i| {
                        if self.cells[j][i] == WarehouseCell::Box {
                            return Some((i, j))
                        }
                        None
                    }
                )
        )
        .map(
            |(i, j)| 100 * j + i
        )
        .sum()
    }
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let (mut warehouse, moves) = preprocessing(input_str)?;
    for mv in &moves {
        warehouse.move_robot(mv)?;
    }
    Ok(warehouse.calculate_score())
}

fn main() -> Result<()> {

    let input_str = fs::read_to_string("input.txt")?;

    println!(
        "Exercise 1: {}",
        exercise_1(&input_str)?
    );

    Ok(())
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

use std::fmt::{self, Display, Formatter};
impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(
                    f,
                    "{}",
                    match cell {
                        WarehouseCell::Empty => '.',
                        WarehouseCell::Wall => '#',
                        WarehouseCell::Box => 'O',
                        WarehouseCell::Robot => '@',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Move::Down => 'v',
            Move::Up => '^',
            Move::Left => '<',
            Move::Right => '>',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
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
    fn test_simple_moves(
        expected_warehouse: Warehouse,
        expected_moves_simple: Vec<Move>,
        expected_position_simple: (usize, usize),
    ) {
        let mut warehouse = expected_warehouse.clone();
        println!("Initial configuration:\n{}", warehouse);
        for (k, mv) in expected_moves_simple.iter().enumerate() {
            warehouse.move_robot(mv).unwrap();
            println!("Configuration after move {} {}:\n{}", k + 1, mv, warehouse);
        }
        assert_eq!(warehouse.robot_location, expected_position_simple);
    }

    #[rstest]
    fn test_exercise_1(
        sample_input_str_simple: &str,
        sample_input_str: &str,
    ) {
        assert_eq!(
            exercise_1(sample_input_str_simple).unwrap(),
            2028,
        );

        assert_eq!(
            exercise_1(sample_input_str).unwrap(),
            10092,
        );
    }
}
