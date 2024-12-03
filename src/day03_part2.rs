use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 2 --");
    let now = Instant::now();

    let input = include_str!("../assets/day03_input_demo2.txt");
    // let input = include_str!("../assets/day03_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let bigs = line.split("do").collect::<Vec<_>>();
                let mut result = 0;
                for big in bigs {
                    if big.starts_with("n't()") {
                        result += 0;
                    } else if big.starts_with("()") {
                        let muls = big.split("mul").collect::<Vec<_>>();
                        for a_mul in muls {
                            if a_mul.starts_with('(') {
                                let a_mul2 = a_mul
                                    .strip_prefix('(')
                                    .unwrap()
                                    .split(')')
                                    .map(|a_mul3| {
                                        let (x, y) = a_mul3.split_once(",").unwrap_or_default();
                                        let x2 = x.parse::<u32>().unwrap_or(0);
                                        let y2 = y.parse::<u32>().unwrap_or(0);
                                        // println!("{},{} => {},{}", x, y, x2, y2);
                                        x2 * y2
                                    })
                                    .collect::<Vec<_>>();
                                result += a_mul2[0]
                            } else {
                                result += 0
                            }
                        }
                    } else {
                        result += 0;
                    }
                }
                result
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day03_input_demo2.txt")),
            Some(48)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day03_input.txt")),
            Some(74361272)
        );
    }
}
