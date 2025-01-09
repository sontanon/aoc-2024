use anyhow::{anyhow, ensure, Result};
use std::{collections::HashSet, fs};

fn exercise_1(input_str: &str) -> Result<usize> {
    let (upper_block, lower_block) = input_str
        .split_once("\n\n")
        .ok_or(anyhow!("Expected a block with two newlines"))?;

    let towels: HashSet<String> = upper_block.split(", ").map(String::from).collect();
    ensure!(!towels.is_empty(), "Require more than one towel");

    let patterns: Vec<String> = lower_block
        .lines()
        .filter_map(|line| {
            if line.trim().len() > 0 {
                return Some(String::from(line))
            }
            None
        })
        .collect();
    ensure!(!patterns.is_empty(), "Require more than one tile");

    Ok(patterns
        .iter()
        .filter(|&p| pattern_possible(p, &towels))
        .count())
}

fn pattern_possible(pattern: &str, towels: &HashSet<String>) -> bool {
    if towels.contains(pattern) || pattern.len() == 0 {
        return true;
    }

    (1..=(pattern.len() - 1)).any(|k| {
        let (s_left, s_right) = pattern.split_at(k);
        towels.contains(s_left) && pattern_possible(s_right, towels)
    })
}

fn number_per_pattern(pattern: &str, towels: HashSet<String>) -> usize {
    todo!()
}

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_possible() {
        let towels: HashSet<String> = HashSet::from([
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ]);

        assert!(pattern_possible("brwrr", &towels));
        assert!(pattern_possible("bggr", &towels));
        assert!(pattern_possible("gbbr", &towels));
        assert!(pattern_possible("rrbgbr", &towels));
        assert!(!pattern_possible("ubwu", &towels));
        assert!(pattern_possible("bwurrg", &towels));
        assert!(pattern_possible("brgr", &towels));
        assert!(!pattern_possible("bbrgwb", &towels));
    }

    #[test]
    fn test_exercise_1() {
        let input_str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(exercise_1(input_str).unwrap(), 6);
    }
}
