use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::iter::IntoIterator;

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&input_str)?;
    println!("Result 2: {}", result_2);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let pebbles = Pebbles::from_str(input_str)?;

    Ok(pebbles.blink_count_efficient(25))
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let pebbles = Pebbles::from_str(input_str)?;

    Ok(pebbles.blink_count_efficient(75))
}

#[derive(Debug, Clone)]
enum BlinkResult {
    One(Pebble),
    Two(Pebble, Pebble),
}

impl IntoIterator for BlinkResult {
    type Item = Pebble;
    type IntoIter = std::vec::IntoIter<Pebble>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            BlinkResult::One(p) => vec![p].into_iter(),
            BlinkResult::Two(p_1, p_2) => vec![p_1, p_2].into_iter(),
        }
    }
}

#[derive(PartialEq, Debug, Eq, std::hash::Hash, Clone)]
struct Pebble(usize);

impl Pebble {
    fn num_digits(&self) -> usize {
        match usize::checked_ilog10(self.0) {
            Some(n) => (n as usize) + 1,
            None => 1,
        }
    }

    fn blink(&self) -> BlinkResult {
        if self.0 == 0 {
            return BlinkResult::One(Pebble(1));
        }

        let c = self.num_digits();
        if c % 2 != 0 {
            return BlinkResult::One(Pebble(self.0 * 2024));
        }

        let mid_c = c / 2;

        let mid_digits = usize::pow(10, mid_c as u32);

        let left = self.0 / mid_digits;
        let right = self.0 % mid_digits;

        BlinkResult::Two(Pebble(left), Pebble(right))
    }
}

#[derive(Debug, PartialEq)]
struct Pebbles {
    pebbles: Vec<Pebble>,
}

fn blink_k_times_increment(
    pebble: Pebble,
    k: usize,
    blink_results_memo: &mut HashMap<Pebble, BlinkResult>,
    blink_increments_memo: &mut HashMap<(usize, Pebble), usize>,
) -> usize {
    // Base case, no blinks, no increments.
    if k == 0 {
        return 0;
    }

    // Check if we already have the result for blinking this pebble k times.
    if let Some(&increment) = blink_increments_memo.get(&(k, pebble.clone())) {
        return increment;
    }

    // Get the result of blinking the pebble once.
    let result = if let Some(b_r) = blink_results_memo.get(&pebble) {
        b_r.to_owned()
    } else {
        let result = pebble.blink();
        blink_results_memo.insert(pebble.clone(), result.clone());
        result
    };

    // If k = 1, we can return the increment depending on the result.
    if k == 1 {
        match result {
            BlinkResult::One(_) => {
                blink_increments_memo.insert((k, pebble.clone()), 0);
                return 0;
            }
            BlinkResult::Two(_, _) => {
                blink_increments_memo.insert((k, pebble.clone()), 1);
                return 1;
            }
        }
    }

    // Otherwise, we need to recurse.
    match result {
        BlinkResult::One(p) => {
            // Calculate the increment for blinking k - 1 times.
            let increment = blink_k_times_increment(p, k - 1, blink_results_memo, blink_increments_memo);
            // After calculating the increment, we can store it in the memoization hashmap for future use.
            blink_increments_memo.insert((k, pebble.clone()), increment);
            increment
        }
        BlinkResult::Two(p_1, p_2) => {
            let increment_1 = blink_k_times_increment(p_1, k - 1, blink_results_memo, blink_increments_memo);
            let increment_2 = blink_k_times_increment(p_2, k - 1, blink_results_memo, blink_increments_memo);
            let increment = increment_1 + increment_2 + 1; // Notice the +1 here.
            blink_increments_memo.insert((k, pebble.clone()), increment);
            increment
        }
    }
}

impl Pebbles {
    fn from_str(input_str: &str) -> Result<Self> {
        let pebbles: Result<Vec<Pebble>> = input_str
            .split_whitespace()
            .map(|c| -> Result<Pebble> {
                let i = c.parse::<usize>()?;
                Ok(Pebble(i))
            })
            .collect();
        let pebbles = pebbles?;
        Ok(Self { pebbles })
    }

    fn blink_count_efficient(self, n: usize) -> usize {
        // The number of final pebbles. Must start with the initial pebbles.
        let initial_count = self.pebbles.len();

        // Use these hashmaps to memoize the results.
        let mut blink_results: HashMap<Pebble, BlinkResult> = HashMap::new();
        let mut blink_increments: HashMap<(usize, Pebble), usize> = HashMap::new();

        self.pebbles.into_iter()
            .fold(initial_count, |acc, pebble| {
                acc + blink_k_times_increment(pebble, n, &mut blink_results, &mut blink_increments)
            })
    }

    // fn blink(self, n: usize) -> usize {
    //     // Use this hashmap to memoize the results.
    //     let mut blink_results: HashMap<Pebble, BlinkResult> = HashMap::new();
    //     // Insert base case.
    //     blink_results.insert(Pebble(0), BlinkResult::One(Pebble(1)));

    //     self.pebbles
    //         .into_iter()
    //         .map(|pebble| {
    //             let pebbles = (0..n).fold(vec![pebble], |pebbles, _| {
    //                 pebbles
    //                     .into_iter()
    //                     .flat_map(|p| {
    //                         let result = if let Some(&ref b_r) = blink_results.get(&p) {
    //                             b_r.to_owned()
    //                         } else {
    //                             let result = p.blink();
    //                             blink_results.insert(p.clone(), result.clone());
    //                             result
    //                         };
    //                         result
    //                     })
    //                     .collect()
    //             });
    //             pebbles.len()
    //         })
    //         .sum()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_input() -> &'static str {
        "125 17"
    }

    #[fixture]
    fn sample_pebbles() -> Pebbles {
        Pebbles {
            pebbles: vec![Pebble(125), Pebble(17)],
        }
    }

    #[fixture]
    fn exptected_output() -> usize {
        55312
    }

    #[rstest]
    fn test_from_str(#[from(sample_input)] input: &str, #[from(sample_pebbles)] expected: Pebbles) {
        let pebbles = Pebbles::from_str(input).unwrap();
        assert_eq!(pebbles, expected);
    }
    #[rstest]
    fn test_exercise_1(
        #[from(sample_input)] input: &str,
        #[from(exptected_output)] expected: usize,
    ) {
        let result = exercise_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    fn test_exercise_2(
        #[from(sample_input)] input: &str,
        #[from(exptected_output)] expected: usize,
    ) {
        let result = exercise_2(input).unwrap();
        assert_eq!(result, expected);
    }
}
