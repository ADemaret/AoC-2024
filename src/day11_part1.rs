use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 1 --");
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

fn get_answer(input: &str) -> Option<usize> {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();
    
    // println!("{:?}", stones);
        for _ in 0..25 {
            blink(&mut stones);
            // println!("blink {} - {} items", i,stones.len());
        }
    
    Some(stones.len())
}

fn blink(stones: &mut Vec<u64>) {
    let mut stones_to_add = Vec::new();
    for stone_val in &mut *stones {
        //let stone_val = stones[i];
        if *stone_val == 0 {
            // println!("  - 0 -> 1");
            *stone_val = 1;
        } else if let Some((x1, x2)) = has_even_digit(*stone_val) {
            // println!("  - {} has even digits -> {} {}", stone_val, x1, x2);
            *stone_val = x1;
            stones_to_add.push(x2);
        } else {
            // println!("  - {} -> {}", stones[i], stones[i] * 2024);
            *stone_val *= 2024;
        }
    }
    stones.extend(stones_to_add);
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
            Some(55312)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(217812)
        );
    }
}
