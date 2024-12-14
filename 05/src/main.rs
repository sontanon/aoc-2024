use anyhow::{anyhow, Result};
use std::{
    collections::HashSet,
    fs,
};

fn main() -> Result<()> {
    let input_string = fs::read_to_string("input.txt")?;
    let (raw_rules, print_queue) = preprocessing(&input_string)?;
    let rules = build_rules_set(&raw_rules);

    let sum_1 = exercise_1(&print_queue, &rules);
    println!("Sum 1: {sum_1}");

    let sum_2 = exercise_2(&print_queue, &rules);
    println!("Sum 2: {sum_2}");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    before: usize,
    after: usize,
}

/// Parse the input string into a vector of rules and a vector of print queues.
/// 
/// The rules are pairs of numbers, separated by a pipe. Example:
/// ```
/// 15|78
/// 65|46
/// 65|23
/// ```
/// 
/// Each print queue is a list of numbers separated by commas. Example:
/// ```
/// 38,68,88,11,13,64,29,37,92,72,26,83,89
/// ```
fn preprocessing(input_string: &str) -> Result<(Vec<Rule>, Vec<Vec<usize>>)> {
    let (rules, print_queue) = input_string
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Input must contain two sections separated by double newline."))?;
    let rules: Result<Vec<Rule>> = rules
        .lines()
        .map(|line| -> Result<Rule> {
            let (before, after) = line
                .split_once("|")
                .ok_or_else(|| anyhow!("Rule must contain two numbers separated by a pipe."))?;
            Ok(Rule { before: before.parse()?, after: after.parse()?})
        })
        .collect();
    let print_queue: Result<Vec<Vec<usize>>> = print_queue
        .lines()
        .map(|line| -> Result<Vec<usize>> {
            line.split(",")
                .map(|x| x.parse().map_err(anyhow::Error::from))
                .collect()
        })
        .collect();
    Ok((rules?, print_queue?))
}

fn build_rules_set(raw_rules: &[Rule]) -> HashSet<Rule> {
    raw_rules.iter().copied().collect()
}

/// The goal of this exercise is to find the sum of the middle value of each valid line.
fn exercise_1(print_queue: &[Vec<usize>], rules: &HashSet<Rule>) -> usize {
    let valid_lines: Vec<&Vec<usize>> = print_queue
        .iter()
        .filter(|v| {
            v.windows(2).all(|w| {
                let before = w[0];
                let after = w[1];
                rules.contains(&Rule { before, after })
            })
        })
        .collect();
    
    valid_lines
        .iter()
        .map(|v| v[v.len() / 2])
        .sum()
}


/// This function takes an invalid line and a set of rules and returns a corrected line.
fn transform_invalid_line(invalid_line: &[usize], rules: &HashSet<Rule>) -> Vec<usize>{

    let mut buffer = invalid_line.to_vec();

    let mut k: usize = 0;
    while k < buffer.len() - 1 {
        let before = buffer[k];
        let after = buffer[k + 1];

        // If the pair is in the set, this is not the violating element and we can continue the search.
        if rules.contains(&Rule {before, after}) {
            k += 1;
            continue;
        }

        // At this point we have a violation.
        // Swap the elements.
        buffer[k] = after;
        buffer[k + 1] = before;

        // We have to go back and check the previous pair (unless we are at the start).
        k = k.saturating_sub(1);
    }
    buffer
}

/// The goal of this exercise is to find the sum of the middle value of each corrected line.
fn exercise_2(print_queue: &[Vec<usize>], rules: &HashSet<Rule>) -> usize {
    let invalid_lines: Vec<&Vec<usize>> = print_queue
        .iter()
        .filter(|v| {
            v.windows(2).any(|w| {
                let before = w[0];
                let after = w[1];
                !rules.contains(&Rule {before, after})
            })
        })
        .collect();

    let corrected_lines: Vec<Vec<usize>> = invalid_lines
        .iter()
        .map(|line| {
            transform_invalid_line(line, rules)
        })
        .collect();

    corrected_lines
        .iter()
        .map(|v| v[v.len() / 2])
        .sum()
}