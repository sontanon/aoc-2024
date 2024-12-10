use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn valid_level(level: &[i32]) -> bool {
    // If the slice has less than 2 elements, it is an automatic success.
    if level.len() < 2 {
        return true;
    }

    let ascending = level[0] < level[1];

    level.windows(2).all(|pair| {
        let i = pair[0];
        let j = pair[1];
        valid_condition(i, j, ascending)
    })
}

fn valid_condition(i: i32, j: i32, ascending: bool) -> bool {
    if ascending {
        (i < j) && (j - i) <= 3
    } else {
        (i > j) && (i - j) <= 3
    }
}

fn exercise_1(path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let valid_levels: Result<Vec<bool>, _> = buffered
        .lines()
        .map(|line| -> Result<bool, Box<dyn std::error::Error>> {
            let line = line?;
            let level: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<_, _>>()?;
            Ok(valid_level(&level))
        })
        .collect();

    Ok(valid_levels?.iter().filter(|&&valid| valid).count() as i32)
}

fn exercise_2(path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let levels: Result<Vec<Vec<i32>>, _> = buffered
        .lines()
        .map(|line| -> Result<Vec<i32>, Box<dyn std::error::Error>> {
            let line = line?;
            let level: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<_, _>>()?;
            Ok(level)
        })
        .collect();

    let exploded_levels: Vec<Vec<Vec<i32>>> = levels?
        .iter()
        .map(|level| -> Vec<Vec<i32>> {
            (0..level.len())
                .map(|skip_idx| -> Vec<i32> {
                    level
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != skip_idx)
                        .map(|(_, &x)| x)
                        .collect()
                })
                .collect()
        })
        .collect();

    let valid_levels: Vec<bool> = exploded_levels
        .iter()
        .map(|exploded_level| -> bool {
            exploded_level
                .iter()
                .any(|level| -> bool { valid_level(level) })
        })
        .collect();

    Ok(valid_levels.iter().filter(|&&valid| valid).count() as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("input.txt");

    let sum_1 = exercise_1(path)?;
    let sum_2 = exercise_2(path)?;

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);

    Ok(())
}
