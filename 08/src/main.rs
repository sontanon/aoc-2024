use anyhow::{ensure, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
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
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
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

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let antenna_grid = AntennaGrid::from_str(&input_str)?;

    let unique_antennas: HashSet<char> = antenna_grid
        .cells
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|cell| cell.antenna)
        .collect();

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

    let antenna_pairs: HashMap<char, Vec<(&AntennaCell, &AntennaCell)>> = antennas
        .iter()
        .filter_map(|(antenna, cells)| {
            if cells.len() < 2 {
                return None;
            }
            let cell_pairs: Vec<(&AntennaCell, &AntennaCell)> = (0..cells.len() - 1)
                .flat_map(|i| -> Vec<(&AntennaCell, &AntennaCell)> {
                    (i + 1..cells.len()).map(|j| (cells[i], cells[j])).collect()
                })
                .collect();
            Some((*antenna, cell_pairs))
        })
        .collect();

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
                            if x < antenna_grid.n_rows && y < antenna_grid.n_cols {
                                Some((x, y))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let antinode_2 = match antinode_2 {
                        (Some(x), Some(y)) => {
                            if x < antenna_grid.n_rows && y < antenna_grid.n_cols {
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

    println!("{:?}", antinodes);
    println!("Number of antinodes: {}", antinodes.len());

    Ok(())
}
