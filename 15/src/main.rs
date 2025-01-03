use anyhow::{anyhow, ensure, Result};
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    println!("Exercise 1: {}", exercise_1(&input_str)?);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let (mut warehouse, moves) = preprocessing(input_str)?;
    for mv in &moves {
        warehouse.move_robot(mv)?;
    }
    Ok(warehouse.calculate_score())
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let (mut warehouse, moves) = preprocessing_big(input_str)?;
    for mv in &moves {
        warehouse.move_robot(mv)?;
    }
    Ok(warehouse.calculate_score())
}

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
enum WarehouseCell {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, PartialEq, Clone)]
enum BigWarehouseCell {
    Empty,
    Wall,
    LeftBox,
    RightBox,
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

impl From<WarehouseCell> for char {
    fn from(val: WarehouseCell) -> Self {
        match val {
            WarehouseCell::Empty => '.',
            WarehouseCell::Wall => '#',
            WarehouseCell::Box => 'O',
            WarehouseCell::Robot => '@',
        }
    }
}

impl BigWarehouseCell {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            '[' => Ok(Self::LeftBox),
            ']' => Ok(Self::RightBox),
            '@' => Ok(Self::Robot),
            _ => Err(anyhow!("Invalid character: {}", c)),
        }
    }
}

impl From<BigWarehouseCell> for char {
    fn from(val: BigWarehouseCell) -> Self {
        match val {
            BigWarehouseCell::Empty => '.',
            BigWarehouseCell::Wall => '#',
            BigWarehouseCell::LeftBox => '[',
            BigWarehouseCell::RightBox => ']',
            BigWarehouseCell::Robot => '@',
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

impl FromStr for Warehouse {
    type Err = anyhow::Error;

    fn from_str(input_str: &str) -> Result<Self> {
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
}

impl Warehouse {
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
        (0..self.height)
            .flat_map(|j| {
                (0..self.width).filter_map(move |i| {
                    if self.cells[j][i] == WarehouseCell::Box {
                        return Some((i, j));
                    }
                    None
                })
            })
            .map(|(i, j)| 100 * j + i)
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BigWarehouse {
    cells: Vec<Vec<BigWarehouseCell>>,
    width: usize,
    height: usize,
    robot_location: (usize, usize),
}

impl FromStr for BigWarehouse {
    type Err = anyhow::Error;

    fn from_str(input_str: &str) -> Result<Self> {
        let mut robot_location: Option<(usize, usize)> = None;
        let cells: Result<Vec<Vec<BigWarehouseCell>>> = input_str
            .lines()
            .enumerate()
            .map(|(j, line)| -> Result<Vec<BigWarehouseCell>> {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let c = BigWarehouseCell::from_char(c)?;
                        if let BigWarehouseCell::Robot = c {
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
        ensure!(width % 2 == 0, "Width must be even");
        ensure!(
            cells.iter().all(|row| row.len() == width),
            "All rows must have the same length"
        );
        ensure!(robot_location.is_some(), "Robot not found");
        ensure!(
            cells[0].iter().all(|cell| *cell == BigWarehouseCell::Wall),
            "First row must be a full wall",
        );
        ensure!(
            cells[height - 1]
                .iter()
                .all(|cell| *cell == BigWarehouseCell::Wall),
            "Last row must be a full wall",
        );
        ensure!(
            cells
                .iter()
                .all(|row| row[0] == BigWarehouseCell::Wall && row[1] == BigWarehouseCell::Wall),
            "First and second columns must be full walls",
        );
        ensure!(
            cells
                .iter()
                .all(|row| row[width - 1] == BigWarehouseCell::Wall
                    && row[width - 2] == BigWarehouseCell::Wall),
            "Last and second to last columns must be full walls",
        );

        ensure!(
            cells.iter().all(|row| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| *c == &BigWarehouseCell::LeftBox)
                    .all(|(i, _)| row[i + 1] == BigWarehouseCell::RightBox)
            }),
            "Boxes must be in pairs"
        );

        Ok(Self {
            width,
            height,
            cells,
            robot_location: robot_location.unwrap(),
        })
    }
}

impl BigWarehouse {
    fn from_regular_warehouse_str(input_str: &str) -> Result<Self> {
        let normal_warehouse = Warehouse::from_str(input_str)?;

        let cells: Vec<Vec<BigWarehouseCell>> = normal_warehouse
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|cell| match cell {
                        WarehouseCell::Empty => [BigWarehouseCell::Empty, BigWarehouseCell::Empty],
                        WarehouseCell::Wall => [BigWarehouseCell::Wall, BigWarehouseCell::Wall],
                        WarehouseCell::Box => {
                            [BigWarehouseCell::LeftBox, BigWarehouseCell::RightBox]
                        }
                        WarehouseCell::Robot => [BigWarehouseCell::Robot, BigWarehouseCell::Empty],
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            width: normal_warehouse.width * 2,
            height: normal_warehouse.height,
            cells,
            robot_location: (
                normal_warehouse.robot_location.0 * 2,
                normal_warehouse.robot_location.1,
            ),
        })
    }

    fn move_robot(&mut self, mv: &Move) -> Result<bool> {
        let (x_r, y_r) = self.robot_location;
        ensure!(
            x_r > 1 && x_r < self.width - 2 && y_r > 0 && y_r < self.height - 1,
            "Cannot move robot on the edge"
        );

        let (x_p, y_p) = match mv {
            Move::Up => (x_r, y_r - 1),
            Move::Down => (x_r, y_r + 1),
            Move::Left => (x_r - 1, y_r),
            Move::Right => (x_r + 1, y_r),
        };

        match &self.cells[y_p][x_p] {
            // First cases are the same as the normal warehouse.
            BigWarehouseCell::Empty => {
                self.cells[y_r][x_r] = BigWarehouseCell::Empty;
                self.cells[y_p][x_p] = BigWarehouseCell::Robot;
                self.robot_location = (x_p, y_p);
                Ok(true)
            }
            BigWarehouseCell::Wall => Ok(false),
            BigWarehouseCell::Robot => Err(anyhow!("Robot cannot move into another robot")),
            // For this case, we may cascade multiple movements due to overlap.
            // A recursive approach is probably best.
            _ => {
                // The horizontal movements are simpler.
                if mv == &Move::Left || mv == &Move::Right {
                    let (x_c, y_c) = (
                        match mv {
                            Move::Right => (x_p + 1..self.width)
                                .map(|i| &self.cells[y_r][i])
                                .position(|c| {
                                    *c != BigWarehouseCell::LeftBox
                                        || *c != BigWarehouseCell::RightBox
                                })
                                .map(|x| x + x_p + 1),
                            Move::Left => (0..x_p).map(|i| &self.cells[y_r][i]).rposition(|c| {
                                *c != BigWarehouseCell::RightBox || *c != BigWarehouseCell::LeftBox
                            }),
                            _ => unreachable!(),
                        },
                        y_r,
                    );
                    if let Some(x_c) = x_c {
                        if self.cells[y_c][x_c] == BigWarehouseCell::Wall {
                            return Ok(false);
                        }
                        // Move is not a simple assignment, rather we need to overwrite the slice using a rotation.
                        if mv == &Move::Right {
                            self.cells[y_r][x_r..=x_c].rotate_right(1);
                        } else {
                            self.cells[y_r][x_c..=x_r].rotate_left(1);
                        }
                        self.robot_location = (x_p, y_p);
                        return Ok(true)
                    }
                    Ok(false)
                }
                // Vertical movements are more complex.
                match &self.cells[y_p][x_p] {
                    BigWarehouseCell::LeftBox => {
                        // Recursive call.
                        // First assume the movement will be possible by setting the robot location to the candidate.
                        self.cells[y_p][x_p] = BigWarehouseCell::Robot;
                        self.cells[y_r][x_r] = BigWarehouseCell::Empty;
                        self.robot_location = (x_p, y_p);
                        // Attempt to move the left box.
                        let push_left_box = self.move_robot(mv)?;
                        // If the left box was not moved, reset to previous state and return early.
                        if !push_left_box {
                            self.cells[y_p][x_p] = BigWarehouseCell::LeftBox;
                            self.cells[y_r][x_r] = BigWarehouseCell::Robot;
                            self.robot_location = (x_r, y_r);
                            return Ok(false);
                        }
                        // Otherwise, attempt to move the right box now by imagining a robot at the right box.
                        self.cells[y_p][x_p]
                        self.cells[y_p][x_p + 1] = BigWarehouseCell::Robot;
                        todo!()
                    }
                    BigWarehouseCell::RightBox => todo!(),
                    _ => unreachable!()
                }
            }
        }
    }

    fn calculate_score(&self) -> usize {
        (0..self.height)
            .flat_map(|j| {
                (0..self.width).filter_map(move |i| {
                    if self.cells[j][i] == BigWarehouseCell::LeftBox {
                        return Some((i, j));
                    }
                    None
                })
            })
            .map(|(i, j)| 100 * j + i)
            .sum()
    }
}

fn preprocessing(input_str: &str) -> Result<(Warehouse, Vec<Move>)> {
    let (warehouse_str, moves_str) = input_str
        .split_once("\n\n")
        .ok_or(anyhow!("Invalid input"))?;
    let warehouse = Warehouse::from_str(warehouse_str)?;
    let moves: Result<Vec<Move>> = moves_str
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(anyhow!("Invalid move character: {}", c)),
        })
        .collect();
    let moves = moves?;
    Ok((warehouse, moves))
}

fn preprocessing_big(input_str: &str) -> Result<(BigWarehouse, Vec<Move>)> {
    let (warehouse_str, moves_str) = input_str
        .split_once("\n\n")
        .ok_or(anyhow!("Invalid input"))?;
    let warehouse = BigWarehouse::from_regular_warehouse_str(warehouse_str)?;
    let moves: Result<Vec<Move>> = moves_str
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(anyhow!("Invalid move character: {}", c)),
        })
        .collect();
    let moves = moves?;
    Ok((warehouse, moves))
}

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

impl Display for BigWarehouse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(
                    f,
                    "{}",
                    match cell {
                        BigWarehouseCell::Empty => '.',
                        BigWarehouseCell::Wall => '#',
                        BigWarehouseCell::LeftBox => '[',
                        BigWarehouseCell::RightBox => ']',
                        BigWarehouseCell::Robot => '@',
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
        let c: char = match self {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}

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
    fn sample_input_str_big_simple() -> &'static str {
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
    }

    #[fixture]
    fn sample_big_warehouse_str() -> &'static str {
        "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############"
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
    fn test_processing_big(sample_input_str_big_simple: &str, sample_big_warehouse_str: &str) {
        let (big_warehouse, _) = preprocessing_big(sample_input_str_big_simple).unwrap();
        assert_eq!(
            big_warehouse,
            BigWarehouse::from_str(sample_big_warehouse_str).unwrap()
        );
    }

    #[rstest]
    fn test_big_score() {
        let big_warehouse = BigWarehouse::from_str(
            "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################",
        )
        .unwrap();
        assert_eq!(big_warehouse.calculate_score(), 9021);
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
    fn test_exercise_1(sample_input_str_simple: &str, sample_input_str: &str) {
        assert_eq!(exercise_1(sample_input_str_simple).unwrap(), 2028,);

        assert_eq!(exercise_1(sample_input_str).unwrap(), 10092,);
    }
}
