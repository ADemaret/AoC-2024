use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 2 --");
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
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    // println!("{:?}", stones);
    let mut result = 0;
    let mut prev_values = HashMap::new();
    for stone in stones {
        result += blink(stone, 0,&mut prev_values);
    }
    Some(result)
}

fn blink(stone_val: u64, level: u32, prev_values : &mut HashMap<(u64,u32),u64>) -> u64 {

    if let Some(result) = prev_values.get(&(stone_val,level)) {
        return *result;
    }
    let mut result;

    if level == 74 {
        if stone_val == 0 {
            return 1;
        } else if let Some((_, _)) = has_even_digit(stone_val) {
            return 2;
        } else {
            return 1;
        }
    }

    if stone_val == 0 {
        // println!("  - 0 -> 1");
        result = blink(1, level + 1, prev_values);
    } else if let Some((x1, x2)) = has_even_digit(stone_val) {
        // println!("  - {} has even digits -> {} {}", stone_val, x1, x2);
        result = blink(x1, level + 1, prev_values);
        result += blink(x2, level + 1, prev_values);
    } else {
        result = blink(stone_val * 2024, level + 1,prev_values);
    }
    prev_values.insert((stone_val,level), result);

    result
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
        // assert_eq!(
        //     get_answer(include_str!("../assets/day11_input_demo1.txt")),
        //     Some(55312)
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(259112729857522)
        );
    }
}
