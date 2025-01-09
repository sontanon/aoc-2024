use anyhow::{anyhow, ensure, Result};
use std::fs;

fn preprocessing(input_str: &str) -> Result<(Vec<String>, Vec<String>)> {
    let (upper_block, lower_block) = input_str.split_once("\n\n").ok_or(anyhow!("Expected a block with two newlines"))?;

    let patterns: Vec<String> = upper_block.split(", ").map(|c| String::from(c)).collect();
    ensure!(patterns.len() > 0, "Require more than one pattern");

    let tiles: Vec<String> = lower_block
        .lines()
        .map(|c| String::from(c))
        .collect();
    ensure!(tiles.len() > 0, "Require more than one tile");

    Ok((patterns, tiles))
}

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let (patterns, tiles) = preprocessing(&input_str)?;

    Ok(())
}
