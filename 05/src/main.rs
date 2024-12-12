use anyhow::{anyhow, Result};
use std::{any, collections::{btree_map::Keys, HashMap}, fs, path::Path};

fn main() -> Result<()> {
    let input_string = fs::read_to_string("input.txt")?;
    let (rules, print_queue) = input_string.split_once("\n\n").ok_or_else(|| anyhow!("Input must contain two sections separated by double newline."))?;

    let rules: Result<Vec<(usize, usize)>> = rules
        .lines()
        .map(
            |line| -> Result<(usize, usize)> {
                let (x, y) = line.split_once("|").ok_or_else(|| anyhow!("Rule must contain two numbers separated by a pipe."))?;
                Ok((x.parse()?, y.parse()?))
            }
        )
        .collect();

    let print_queue: Result<Vec<Vec<usize>>> = print_queue
        .lines()
        .map(
            |line| -> Result<Vec<usize>> {
                line
                    .split(",")
                    .map(|x| x.parse().map_err(anyhow::Error::from)) 
                    .collect()
            }
        )
        .collect();

    let mut after_rules = rules?
        .iter()
        .fold(HashMap::new(), |mut map, (x, y)| {
            map.entry(*x).or_insert(Vec::new()).push(*y);
            map
        });
    after_rules.values_mut().for_each(|v| v.sort_unstable());


    let sum_of_middle_value_of_valid_lines: usize = print_queue?
        .iter()
        .filter(|v| {
            !v.windows(2).any(|w| {
                let x = w[0];
                let y = w[1];
                after_rules.get(&y).map_or(false, |v| v.contains(&x))
            })
        })
        .map(|v| v[v.len() / 2])
        .sum();

    println!("{}", sum_of_middle_value_of_valid_lines);


    Ok(())
}
