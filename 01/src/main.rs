use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn exercise_1(path: &Path) -> Result<i32, io::Error> {
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        let mut numbers = line.split_whitespace();

        if let (Some(num_1), Some(num_2)) = (numbers.next(), numbers.next()) {
            if let (Ok(n_1), Ok(n_2)) = (num_1.parse::<i32>(), num_2.parse::<i32>()) {
                left_list.push(n_1);
                right_list.push(n_2);
            }
        }
    }

    left_list.sort();
    right_list.sort();

    Ok(left_list
        .iter()
        .zip(right_list.iter())
        .map(|(i, j)| i32::abs(i - j))
        .sum())
}

fn exercise_2(path: &Path) -> Result<i32, io::Error> {
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let mut left_list = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for line in buffered.lines() {
        let line = line?;
        let mut numbers = line.split_whitespace();

        if let (Some(num_1), Some(num_2)) = (numbers.next(), numbers.next()) {
            if let (Ok(n_1), Ok(n_2)) = (num_1.parse::<i32>(), num_2.parse::<i32>()) {
                left_list.push(n_1);
                let right_entry = right_map.entry(n_2).or_insert(0);
                *right_entry += 1;
            }
        }
    }

    Ok(left_list
        .iter()
        .map(|i| i * right_map.get(i).unwrap_or(&0))
        .sum())
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");

    let sum_1 = exercise_1(path)?;
    let sum_2 = exercise_2(path)?;

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);

    Ok(())
}
