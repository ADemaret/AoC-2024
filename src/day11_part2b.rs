use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 2b --");
    let now = Instant::now();

    // let input = include_str!("../assets/day11_input_demo1.txt");
    let input = include_str!("../assets/day11_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<u64> {
    let mut stones = HashMap::new();
    
    input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .for_each( |stone| { 
            stones.entry(stone).and_modify(|counter| *counter += 1).or_insert(1);
        });

    for _ in 0..75 {
        // println!("{:?}", stones);
        stones = blink(stones);
    }
    Some(stones.values().sum())
}

fn blink(stones: HashMap<u64,u64>) -> HashMap<u64,u64> {
    let mut new_stones = HashMap::new();
    for (stone_val,nbr) in stones {
        if stone_val == 0 {
            // println!("  - 0 -> 1");
            new_stones.entry(1).and_modify(|counter| *counter += nbr).or_insert(nbr);
        } else if let Some((x1, x2)) = has_even_digit(stone_val) {
            // println!("  - {} has even digits -> {} {}", stone_val, x1, x2);
            new_stones.entry(x1).and_modify(|counter| *counter += nbr).or_insert(nbr);
            new_stones.entry(x2).and_modify(|counter| *counter += nbr).or_insert(nbr);
        } else {
            // println!("  - {} -> {}", stones[i], stones[i] * 2024);
            new_stones.entry(stone_val*2024).and_modify(|counter| *counter += nbr).or_insert(nbr);
        }
    }
    new_stones
}


fn has_even_digit(x: u64) -> Option<(u64, u64)> {
    let digits = x.ilog10() + 1;
    if (digits) % 2 == 0 {
        let div = 10u64.pow(digits / 2);
        let x1 = x / div;
        let x2 = x % div;
        return Some((x1, x2));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day11_input_demo1.txt")),
            Some(65601038650482)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(259112729857522)
        );
    }
}
