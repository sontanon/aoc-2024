use anyhow::{anyhow, ensure, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn exercise(input_str: &str) -> Result<(usize, usize)> {
    let (upper_block, lower_block) = input_str
        .split_once("\n\n")
        .ok_or(anyhow!("Expected a block with two newlines"))?;

    let towels: HashSet<String> = upper_block.split(", ").map(String::from).collect();
    ensure!(!towels.is_empty(), "Require more than one towel");

    let patterns: Vec<String> = lower_block
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                return Some(String::from(line));
            }
            None
        })
        .collect();
    ensure!(!patterns.is_empty(), "Require more than one tile");

    let mut memo_possible = HashMap::new();
    let mut memo_counts = HashMap::new();

    let possible = patterns
        .iter()
        .filter(|&p| pattern_possible(p, &towels, &mut memo_possible))
        .count();

    let counts = patterns
        .iter()
        .filter_map(|p| {
            if pattern_possible(p, &towels, &mut memo_possible) {
                return Some(ways_to_build_pattern(
                    p,
                    &towels,
                    &mut memo_possible,
                    &mut memo_counts,
                ));
            }
            None
        })
        .sum();

    Ok((possible, counts))
}

fn pattern_possible(
    pattern: &str,
    towels: &HashSet<String>,
    memo_possible: &mut HashMap<String, bool>,
) -> bool {
    if let Some(&result) = memo_possible.get(pattern) {
        return result;
    }

    if towels.contains(pattern) || pattern.is_empty() {
        return true;
    }

    let result = (1..=(pattern.len() - 1)).any(|k| {
        let (s_left, s_right) = pattern.split_at(k);
        towels.contains(s_left) && pattern_possible(s_right, towels, memo_possible)
    });

    memo_possible
        .entry(pattern.to_string())
        .insert_entry(result);
    result
}

fn ways_to_build_pattern(
    pattern: &str,
    towels: &HashSet<String>,
    memo_possible: &mut HashMap<String, bool>,
    memo_counts: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&result) = memo_counts.get(pattern) {
        return result;
    }

    if let Some(&result) = memo_possible.get(pattern) {
        if !result {
            return 0;
        }
    }

    if pattern.is_empty() {
        return 1;
    }

    let count = (1..=(pattern.len()))
        .filter_map(|k| {
            let (s_left, s_right) = pattern.split_at(k);
            if towels.contains(s_left) && pattern_possible(s_right, towels, memo_possible) {
                return Some(ways_to_build_pattern(
                    s_right,
                    towels,
                    memo_possible,
                    memo_counts,
                ));
            }
            None
        })
        .sum();

    memo_counts.entry(pattern.to_string()).insert_entry(count);
    count
}

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let (result_1, result_2) = exercise(&input_str)?;
    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);

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

        let mut memo = HashMap::new();

        assert!(pattern_possible("brwrr", &towels, &mut memo));
        assert!(pattern_possible("bggr", &towels, &mut memo));
        assert!(pattern_possible("gbbr", &towels, &mut memo));
        assert!(pattern_possible("rrbgbr", &towels, &mut memo));
        assert!(!pattern_possible("ubwu", &towels, &mut memo));
        assert!(pattern_possible("bwurrg", &towels, &mut memo));
        assert!(pattern_possible("brgr", &towels, &mut memo));
        assert!(!pattern_possible("bbrgwb", &towels, &mut memo));
    }

    #[test]
    fn test_ways_to_build_pattern() {
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

        let mut memo_possible = HashMap::new();
        let mut memo_counts = HashMap::new();

        assert_eq!(
            ways_to_build_pattern("brwrr", &towels, &mut memo_possible, &mut memo_counts),
            2
        );
        assert_eq!(
            ways_to_build_pattern("bggr", &towels, &mut memo_possible, &mut memo_counts),
            1
        );
        assert_eq!(
            ways_to_build_pattern("gbbr", &towels, &mut memo_possible, &mut memo_counts),
            4
        );
        assert_eq!(
            ways_to_build_pattern("rrbgbr", &towels, &mut memo_possible, &mut memo_counts),
            6
        );
        assert_eq!(
            ways_to_build_pattern("bwurrg", &towels, &mut memo_possible, &mut memo_counts),
            1
        );
        assert_eq!(
            ways_to_build_pattern("brgr", &towels, &mut memo_possible, &mut memo_counts),
            2
        );
        assert_eq!(
            ways_to_build_pattern("ubwu", &towels, &mut memo_possible, &mut memo_counts),
            0
        );
        assert_eq!(
            ways_to_build_pattern("bbrgwb", &towels, &mut memo_possible, &mut memo_counts),
            0
        );
    }
}
