use regex::Regex;
use std::fs::{self};
use std::path::Path;

fn exercise_1(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    pattern.captures_iter(input_text).map(|cap| {
        let a = cap.get(1).ok_or("Missing first argument")?.as_str().parse::<i32>()?;
        let b = cap.get(2).ok_or("Missing second argument")?.as_str().parse::<i32>()?;
        Ok(a * b)
    }).sum()
}

#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    DoNot
}

fn exercise_2(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let pattern = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))")?;

    let operations: Result<Vec<Operation>, _> = pattern.captures_iter(input_text).map(|cap| -> Result<Operation, Box<dyn std::error::Error>> {
        match cap.get(1).ok_or("Missing operation")?.as_str() {
            "do()" => Ok(Operation::Do),
            "don't()" => Ok(Operation::DoNot),
            _ => {
                let a = cap.get(2).ok_or("Missing first argument")?.as_str().parse::<i32>()?;
                let b = cap.get(3).ok_or("Missing second argument")?.as_str().parse::<i32>()?;
                Ok(Operation::Mul(a, b))
            }
        }
    }).collect();


    let mut sum = 0;
    let mut do_operation = true;

    for op in operations? {
        match op {
            Operation::Mul(a, b) => {
                if do_operation {
                    sum += a * b;
                }
            },
            Operation::Do => {
                do_operation = true;
            },
            Operation::DoNot => {
                do_operation = false;
            }
        }
    }

    Ok(sum)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("input.txt");
    let input_text = fs::read_to_string(path).expect("Failed to read file");

    let sum_1 = exercise_1(&input_text)?;
    let sum_2 = exercise_2(&input_text)?;

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);

    Ok(())

}
