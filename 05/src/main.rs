use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, VecDeque};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

fn preprocessing(input_string: &str) -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    let (rules, print_queue) = input_string
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Input must contain two sections separated by double newline."))?;
    let rules: Result<Vec<(usize, usize)>> = rules
        .lines()
        .map(|line| -> Result<(usize, usize)> {
            let (x, y) = line
                .split_once("|")
                .ok_or_else(|| anyhow!("Rule must contain two numbers separated by a pipe."))?;
            Ok((x.parse()?, y.parse()?))
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

fn main() -> Result<()> {
    let input_string = fs::read_to_string("input.txt")?;

    let sum_1 = exercise_1(&input_string)?;
    println!("Sum 1: {sum_1}");

    let sum_2 = exercise_2(&input_string)?;
    println!("Sum 2: {sum_2}");

    Ok(())
}

#[derive(Debug)]
struct Rules {
    page_number: usize,
    pages_before: BTreeSet<usize>,
    pages_after: BTreeSet<usize>,
}

fn exercise_2(input_string: &str) -> Result<usize> {
    let (raw_rules, print_queue) = preprocessing(input_string)?;

    let mut rules: BTreeMap<usize, Rules> = raw_rules.iter().fold(BTreeMap::new(), |mut map, (x, y)| {
            // Given `x|y`, x goes before y, so store x in y's "pages_before" set.
            map.entry(*y)
                .or_insert(Rules {
                    page_number: *y,
                    pages_before: BTreeSet::new(),
                    pages_after: BTreeSet::new(),
                })
                .pages_before
                .insert(*x);
            // Given `x|y`, y goes after x, so store y in x's "pages_after" set.
            map.entry(*x)
                .or_insert(Rules {
                    page_number: *x,
                    pages_before: BTreeSet::new(),
                    pages_after: BTreeSet::new(),
                })
                .pages_after
                .insert(*y);
            map
        });

    let num_unique_pages= rules.len();

    let mut pages_ring: VecDeque<usize> = VecDeque::new();
    let mut pages_set: HashSet<usize> = HashSet::new();
    let seed = *rules.keys().next().unwrap();

    // Insert the first element of the rules as the start of the ring.
    pages_ring.push_front(seed);
    pages_set.insert(seed);

    // Set the focus as the first element of the rules.
    let mut top_page = *pages_ring.front().unwrap();
    let mut bot_page = *pages_ring.back().unwrap();
    while pages_set.len() != num_unique_pages {
        println!("`pages_ring`: {pages_ring:?}; `top_page`: {top_page} -> `bot_page`: {bot_page}");

        // Given the top page, find a page that goes before/ontop of it.
        let new_top_page = {
            let pages_before_top = &mut rules.get_mut(&top_page).unwrap().pages_before;
            println!("\t`pages_before_top`: {pages_before_top:?}");
            if pages_before_top.is_empty() {
                None
            } else {
                let new_top_page = *pages_before_top.iter().next().unwrap();
                pages_before_top.remove(&new_top_page);
                Some(new_top_page)
            }
        };
        // If we have a new top page, insert it at the top of the ring.
        if let Some(new_top_page) = new_top_page {
            println!("\t\tFound a new potential top page: {new_top_page}");

            // If the element is already in the ring, ignore it.
            if pages_set.contains(&new_top_page) {
                println!("\t\t`new_top_page` is already in the ring. Continuing...");
            } else {
                // Insert the element into the ring and set.
                pages_ring.push_front(new_top_page);
                pages_set.insert(new_top_page);

                top_page = new_top_page;
                println!("\t\t`pages_ring`: {pages_ring:?}; `top_page`: {top_page} -> `bot_page`: {bot_page}");
            }
        }

        // Given the bottom page, find a page that goes after/under it.
        let new_bot_page = {
            let pages_after_bot = &mut rules.get_mut(&bot_page).unwrap().pages_after;
            println!("\t`pages_after_bot`: {pages_after_bot:?}");
            if pages_after_bot.is_empty() {
                None
            } else {
                let new_bot_page = *pages_after_bot.iter().next().unwrap();
                pages_after_bot.remove(&new_bot_page);
                Some(new_bot_page)
            }
        };
        // If we have a new bottom page, insert it at the bottom of the ring.
        if let Some(new_bot_page) = new_bot_page {
            println!("\t\tFound a new potential bot page: {new_bot_page}");

            // If the element is already in the ring, ignore it.
            if pages_set.contains(&new_bot_page) {
                println!("\t\t`new_bot_page` is already in the ring. Continuing...");
            } else {
                // Insert the element into the ring and set.
                pages_ring.push_back(new_bot_page);
                pages_set.insert(new_bot_page);

                bot_page = new_bot_page;
                println!("\t\t`pages_ring`: {pages_ring:?}; `top_page`: {top_page} -> `bot_page`: {bot_page}");
            }
        }
    }
    println!("Broke out of the loop successfully\n{pages_ring:?}");

    todo!()
}

fn exercise_1(input_string: &str) -> Result<usize> {
    let (rules, print_queue) = preprocessing(input_string)?;

    let mut after_rules: HashMap<usize, Vec<usize>> =
        rules.iter().fold(HashMap::new(), |mut map, (x, y)| {
            map.entry(*x).or_default().push(*y);
            map
        });
    after_rules.values_mut().for_each(|v| v.sort_unstable());
    let sum_of_middle_value_of_valid_lines: usize = print_queue
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
    Ok(sum_of_middle_value_of_valid_lines)
}
