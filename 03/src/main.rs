use regex::Regex;
use std::fs::{self};
use std::path::Path;

fn exercise_1(input_text: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = pattern.captures_iter(&input_text).map(|cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        a * b
    }).sum();

    sum
}

fn exercise_2(input_text: &str) -> i32 {
    let pattern = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let splits: Vec<&str> = pattern.split(&input_text).collect();

    let operations =

    todo!()

}


fn main() {
    let path = Path::new("input.txt");
    let input_text = fs::read_to_string(path).expect("Failed to read file");

    let sum_1 = exercise_1(&input_text);
    let sum_2 = exercise_2(&input_text);

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);

}
