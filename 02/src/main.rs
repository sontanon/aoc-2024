use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn valid_level(level: &[i32]) -> bool {
    // If the slice has less than 2 elements, it is an automatic success.
    if level.len() < 2 {
        return true;
    }

    let ascending = level[0] < level[1];

    level.windows(2).all(|pair| {
        let i = pair[0];
        let j = pair[1];
        valid_condition(i, j, ascending)
    })
}

// fn valid_level_by_removing_one(level: &[i32]) -> bool {
//     // If the slice has less than 2 elements, it is an automatic success.
//     if level.len() < 2 {
//         return true;
//     }
//
//     // Index to store the index of the first violation if any exists.
//     let mut invalid_pair: Option<usize> = None;
//
//     // Determine if the slice is ascending or descending.
//     let ascending = level[0] < level[1];
//
//     // Iterate over the pairs in the level.
//     // The enumeration allows us to keep track of the index in the slice.
//     for (k, pair) in level.windows(2).enumerate() {
//         let i = pair[0];
//         let j = pair[1];
//         if !valid_condition(i, j, ascending) {
//             invalid_pair = Some(k);
//             break;
//         }
//     }
//
//     // If the loop completes with no violation, we can return true.
//     if invalid_pair.is_none() {
//         return true;
//     }
//
//     // Otherwise, the slice may be valid if we remove one of the elements.
//     let k = invalid_pair.unwrap();
//
//     // Edge cases:
//     // The violating pair is the last one. This is an automatic success by removing the last element
//     if k == level.len() - 2 {
//         return true;
//     }
//     // Otherwise, k must be less than the length of the slice - 2 (so we can safely access k + 2).
//
//     // The violating pair is the first one.
//     if k == 0 {
//         return boundary_condition(level[0], &level[2..], level[0] < level[2])
//             || valid_level(&level[1..]);
//     }
//
//     // The violating pair is the second one.
//     // And the ascending direction changed.
//     if k == 1 && (level[0] < level[1]) != (level[1] < level[2]) {
//         if valid_level(&level[1..]) {
//             return true;
//         }
//     }
//
//     // There are now two choices:
//     // 1. Remove the right element, i.e., k + 1.
//     let remove_right = {
//         let i = level[k];
//         boundary_condition(i, &level[k + 2..], ascending)
//     };
//
//     // 2. Remove the left element, i.e., k.
//     let remove_left = {
//         let i = level[k - 1];
//         boundary_condition(i, &level[k + 1..], ascending)
//     };
//
//     remove_left || remove_right
// }

fn valid_condition(i: i32, j: i32, ascending: bool) -> bool {
    if ascending {
        (i < j) && (j - i) <= 3
    } else {
        (i > j) && (i - j) <= 3
    }
}

// fn boundary_condition(i: i32, slice: &[i32], ascending: bool) -> bool {
//     match slice.len() {
//         0 => panic!("Slice must have at least one element"),
//         1 => valid_condition(i, slice[0], ascending),
//         2.. => {
//             valid_condition(i, slice[0], ascending)
//                 && ((i < slice[0]) == (slice[0] < slice[1]))
//                 && valid_level(slice)
//         }
//     }
// }

fn exercise_1(path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let valid_levels: Result<Vec<bool>, _> = buffered
        .lines()
        .map(|line| -> Result<bool, Box<dyn std::error::Error>> {
            let line = line?;
            let level: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<_, _>>()?;
            Ok(valid_level(&level))
        })
        .collect();

    Ok(valid_levels?.iter().filter(|&&valid| valid).count() as i32)
}

fn exercise_2(path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let levels: Result<Vec<Vec<i32>>, _> = buffered
        .lines()
        .map(|line| -> Result<Vec<i32>, Box<dyn std::error::Error>> {
            let line = line?;
            let level: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<_, _>>()?;
            Ok(level)
        })
        .collect();

    let exploded_levels: Vec<Vec<Vec<i32>>> = levels?
        .iter()
        .map(
            |level|  -> Vec<Vec<i32>> {
                (0..level.len())
                    .map(
                        |skip_idx| -> Vec<i32> {
                            level
                                .iter()
                                .enumerate()
                                .filter(|(i, slice)| *i != skip_idx)
                                .map(|(_, x)| *x)
                                .collect()
                        }
                    )
                    .collect()
            }
        )
        .collect();

    let valid_levels: Vec<bool> = exploded_levels
        .iter()
        .map(
            |exploded_level| -> bool {
                exploded_level
                    .iter()
                    .any(
                        |level| -> bool {
                            valid_level(level)
                        }
                    )
            }
        )
        .collect();

    Ok(valid_levels.iter().filter(|&&valid| valid).count() as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("input.txt");

    let sum_1 = exercise_1(path)?;
    let sum_2 = exercise_2(path)?;

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);

    Ok(())
}
