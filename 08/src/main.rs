use anyhow::{ensure, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq)]
struct AntennaCell {
    antenna: Option<char>,
    coords: (usize, usize),
}

#[derive(Debug)]
struct AntennaGrid {
    cells: Vec<Vec<AntennaCell>>,
    n_rows: usize,
    n_cols: usize,
}

impl AntennaGrid {
    fn from_str(input_str: &str) -> Result<Self> {
        let n_rows = input_str.lines().count();
        let n_cols = input_str.lines().next().unwrap().chars().count();
        ensure!(n_rows > 0, "Empty input");
        ensure!(
            input_str.lines().all(|line| line.chars().count() == n_cols),
            "Inconsistent row length"
        );
        ensure!(n_rows == n_cols, "Non-square grid");

        let cells: Vec<Vec<AntennaCell>> = input_str
            .lines()
            .enumerate()
            .map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| match c {
                        '.' => AntennaCell {
                            antenna: None,
                            coords: (i, j),
                        },
                        c => AntennaCell {
                            antenna: Some(c),
                            coords: (i, j),
                        },
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            cells,
            n_rows,
            n_cols,
        })
    }
}

fn preprocessing(antenna_grid: &AntennaGrid) -> Result<HashMap<char, Vec<&AntennaCell>>> {
    let unique_antennas: HashSet<char> = antenna_grid
        .cells
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|cell| cell.antenna)
        .collect();

    ensure!(unique_antennas.len() > 1, "Not enough antennas");

    let antennas: HashMap<char, Vec<&AntennaCell>> = unique_antennas
        .iter()
        .map(|&antenna| {
            let cells: Vec<&AntennaCell> = antenna_grid
                .cells
                .iter()
                .flat_map(|row| row.iter())
                .filter(|cell| cell.antenna == Some(antenna))
                .collect();
            (antenna, cells)
        })
        .collect();

    Ok(antennas)
}

fn build_pairs<'a>(antennas: &'a HashMap<char, Vec<&'a AntennaCell>>) -> Result<HashMap<char, Vec<(&'a AntennaCell, &'a AntennaCell)>>> {
    let antenna_pairs: HashMap<char, Vec<(&AntennaCell, &AntennaCell)>> = antennas
        .iter()
        .filter_map(|(antenna, cells)| {
            if cells.len() < 2 {
                return None;
            }
            let cell_pairs: Vec<(&AntennaCell, &AntennaCell)> = (0..cells.len() - 1)
                .flat_map(|k| -> Vec<(&AntennaCell, &AntennaCell)> {
                    (k + 1..cells.len()).map(|l| (cells[k], cells[l])).collect()
                })
                .collect();
            Some((*antenna, cell_pairs))
        })
        .collect();

    ensure!(
        antenna_pairs.values().all(|pairs| !pairs.is_empty()),
        "No antenna pairs"
    );

    Ok(antenna_pairs)
}

enum Direction {
    //    (2)
    //    ^
    //   /
    // (1)
    NE,
    // (1)
    //   \
    //    v
    //    (2)
    SE,
    //    (1)
    //    /
    //   v
    // (2)
    SW,
    // (2)
    //   ^
    //    \
    //    (1)
    NW,
}

impl Direction {
    fn determine_direction(p_1: (usize, usize), p_2: (usize, usize)) -> Self {
        let (x_1, y_1) = p_1;
        let (x_2, y_2) = p_2;

        match (x_1 < x_2, y_1 < y_2) {
            (true, true) => Direction::NE,
            (true, false) => Direction::SE,
            (false, false) => Direction::SW,
            (false, true) => Direction::NW,
        }
    }
}

fn exercise_2(antenna_pairs: &HashMap<char, Vec<(&AntennaCell, &AntennaCell)>>, bounds: (usize, usize)) -> Result<usize> {
    let (n_rows, n_cols) = bounds;

    let antinodes: HashSet<(usize, usize)> = antenna_pairs
        .iter()
        .flat_map(|(_, pairs)| -> Vec<(usize, usize)> {
            pairs
                .iter()
                .flat_map(|(cell_1, cell_2)| -> Vec<(usize, usize)> {
                    let (x_1, y_1) = cell_1.coords;
                    let (x_2, y_2) = cell_2.coords;
                    let mut antinodes = Vec::with_capacity(n_rows);
                    // Add the two pairs as they are automatic antinodes
                    antinodes.push((x_1, y_1));
                    antinodes.push((x_2, y_2));

                    // Determine the direction from the first to the second cell
                    let direction = Direction::determine_direction((x_1, y_1), (x_2, y_2));
                    // Push the elements in the direction of the pair.
                    // First from 1 back and then from 2 forward.
                    match direction {
                        Direction::NE => {
                            let delta_x = x_2 - x_1;
                            let delta_y = y_2 - y_1;
                            let (mut x_i, mut y_i) = (x_1, y_1);
                            while x_i >= delta_x && y_i >= delta_y {
                                x_i -= delta_x;
                                y_i -= delta_y;
                                antinodes.push((x_i, y_i));
                            }
                            let (mut x_i, mut y_i) = (x_2, y_2);
                            while x_i + delta_x < n_cols && y_i + delta_y < n_rows {
                                x_i += delta_x;
                                y_i += delta_y;
                                antinodes.push((x_i, y_i));
                            }
                        }
                        Direction::SE => {
                            let delta_x = x_2 - x_1;
                            let delta_y = y_1 - y_2;
                            let (mut x_i, mut y_i) = (x_1, y_1);
                            while x_i >= delta_x && y_i + delta_y < n_rows {
                                x_i -= delta_x;
                                y_i += delta_y;
                                antinodes.push((x_i, y_i));
                            }
                            let (mut x_i, mut y_i) = (x_2, y_2);
                            while x_i + delta_x < n_cols && y_i >= delta_y {
                                x_i += delta_x;
                                y_i -= delta_y;
                                antinodes.push((x_i, y_i));
                            }
                        }
                        Direction::SW => {
                            let delta_x = x_1 - x_2;
                            let delta_y = y_1 - y_2;
                            let (mut x_i, mut y_i) = (x_1, y_1);
                            while x_i + delta_x < n_cols && y_i + delta_y < n_rows {
                                x_i += delta_x;
                                y_i += delta_y;
                                antinodes.push((x_i, y_i));
                            }
                            let (mut x_i, mut y_i) = (x_2, y_2);
                            while x_i >= delta_x && y_i >= delta_y {
                                x_i -= delta_x;
                                y_i -= delta_y;
                                antinodes.push((x_i, y_i));
                            }
                        }
                        Direction::NW => {
                            let delta_x = x_1 - x_2;
                            let delta_y = y_2 - y_1;
                            let (mut x_i, mut y_i) = (x_1, y_1);
                            while x_i + delta_x < n_cols && y_i >= delta_y {
                                x_i += delta_x;
                                y_i -= delta_y;
                                antinodes.push((x_i, y_i));
                            }
                            let (mut x_i, mut y_i) = (x_2, y_2);
                            while x_i >= delta_x && y_i + delta_y < n_cols {
                                x_i -= delta_x;
                                y_i += delta_y;
                                antinodes.push((x_i, y_i));
                            }
                        }
                    }
                    antinodes
                })
                .collect()
        })
        .collect();

    Ok(antinodes.len())
}

fn exercise_1(antenna_pairs: &HashMap<char, Vec<(&AntennaCell, &AntennaCell)>>, bounds: (usize, usize)) -> Result<usize> {
    let (n_rows, n_cols) = bounds;

    let antinodes: HashSet<(usize, usize)> = antenna_pairs
        .iter()
        .flat_map(|(_, pairs)| -> Vec<(usize, usize)> {
            pairs
                .iter()
                .flat_map(|(cell_1, cell_2)| -> [Option<(usize, usize)>; 2] {
                    let (x_1, y_1) = cell_1.coords;
                    let (x_2, y_2) = cell_2.coords;
                    let antinode_1: (Option<usize>, Option<usize>) = (
                        usize::checked_sub(2 * x_1, x_2),
                        usize::checked_sub(2 * y_1, y_2),
                    );
                    let antinode_2: (Option<usize>, Option<usize>) = (
                        usize::checked_sub(2 * x_2, x_1),
                        usize::checked_sub(2 * y_2, y_1),
                    );
                    let antinode_1 = match antinode_1 {
                        (Some(x), Some(y)) => {
                            if x < n_rows && y < n_cols {
                                Some((x, y))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let antinode_2 = match antinode_2 {
                        (Some(x), Some(y)) => {
                            if x < n_rows && y < n_cols {
                                Some((x, y))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    [antinode_1, antinode_2]
                })
                .flatten()
                .collect()
        })
        .collect();

    Ok(antinodes.len())
}

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let antenna_grid = AntennaGrid::from_str(&input_str)?;
    let antennas = preprocessing(&antenna_grid)?;
    let antenna_pairs = build_pairs(&antennas)?;

    let result_1 = exercise_1(&antenna_pairs, (antenna_grid.n_rows, antenna_grid.n_cols))?;
    println!("Result: {}", result_1);

    let result_2 = exercise_2(&antenna_pairs, (antenna_grid.n_rows, antenna_grid.n_cols))?;
    println!("Result: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[fixture]
    fn sample_input_string() -> &'static str {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    #[fixture] 
    fn sample_antennas() -> HashMap<char, Vec<&'static AntennaCell>> {
        HashMap::from([
            ('0', vec![
                &AntennaCell { antenna: Some('0'), coords: (8, 1) },
                &AntennaCell { antenna: Some('0'), coords: (5, 2) },
                &AntennaCell { antenna: Some('0'), coords: (7, 3) },
                &AntennaCell { antenna: Some('0'), coords: (4, 4) },
            ]),
            ('A', vec![
                &AntennaCell { antenna: Some('A'), coords: (6, 5) },
                &AntennaCell { antenna: Some('A'), coords: (8, 8) },
                &AntennaCell { antenna: Some('A'), coords: (9, 9) },
            ]),
        ])
    }

    #[fixture]
    fn sample_antenna_pairs() -> HashMap<char, Vec<(&'static AntennaCell, &'static AntennaCell)>> {
        HashMap::from([
            ('0', vec![
                (&AntennaCell { antenna: Some('0'), coords: (8, 1) }, &AntennaCell { antenna: Some('0'), coords: (5, 2) }),
                (&AntennaCell { antenna: Some('0'), coords: (8, 1) }, &AntennaCell { antenna: Some('0'), coords: (7, 3) }),
                (&AntennaCell { antenna: Some('0'), coords: (8, 1) }, &AntennaCell { antenna: Some('0'), coords: (4, 4) }),
                (&AntennaCell { antenna: Some('0'), coords: (5, 2) }, &AntennaCell { antenna: Some('0'), coords: (7, 3) }),
                (&AntennaCell { antenna: Some('0'), coords: (5, 2) }, &AntennaCell { antenna: Some('0'), coords: (4, 4) }),
                (&AntennaCell { antenna: Some('0'), coords: (7, 3) }, &AntennaCell { antenna: Some('0'), coords: (4, 4) }),
            ]),
            ('A', vec![
                (&AntennaCell { antenna: Some('A'), coords: (6, 5) }, &AntennaCell { antenna: Some('A'), coords: (8, 8) }),
                (&AntennaCell { antenna: Some('A'), coords: (6, 5) }, &AntennaCell { antenna: Some('A'), coords: (9, 9) }),
                (&AntennaCell { antenna: Some('A'), coords: (8, 8) }, &AntennaCell { antenna: Some('A'), coords: (9, 9) }),
            ]),
        ])
    }

    #[rstest]
    fn test_preprocessing(#[from(sample_input_string)] input_str: &'static str, #[from(sample_antennas)] expected_antennas: HashMap<char, Vec<&'static AntennaCell>>, #[from(sample_antenna_pairs)] expected_antenna_pairs: HashMap<char, Vec<(&'static AntennaCell, &'static AntennaCell)>>) {
        let antenna_grid = AntennaGrid::from_str(input_str).unwrap();
        let antennas = preprocessing(&antenna_grid).unwrap();
        assert_eq!(antennas, expected_antennas);

        let antenna_pairs = build_pairs(&antennas).unwrap();
        assert_eq!(antenna_pairs, expected_antenna_pairs);
    }

    #[rstest]
    fn test_exercise_1(#[from(sample_antenna_pairs)] antenna_pairs: HashMap<char, Vec<(&AntennaCell, &AntennaCell)>>) {
        let result = exercise_1(&antenna_pairs, (12, 12)).unwrap();
        assert_eq!(result, 14);
    }

    #[rstest]
    fn test_exercise_2(#[from(sample_antenna_pairs)] antenna_pairs: HashMap<char, Vec<(&AntennaCell, &AntennaCell)>>) {
        let result = exercise_2(&antenna_pairs, (12, 12)).unwrap();
        assert_eq!(result, 34);
    }

}
