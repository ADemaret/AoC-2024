use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

    println!("The answer is : {}", get_answer(input));
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            // println!("Equ {}", line);
            let parts = line.split_once(": ").unwrap();
            let sol = parts.0.parse::<u64>().unwrap();
            let params = parts
                .1
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if sol_exists(sol, params) {
                // println!("adding {}", sol);
                sol
            } else {
                0
            }
        })
        .sum()
}

fn sol_exists(sol: u64, params: Vec<u64>) -> bool {
    // println!("- {}, {:?}", sol, params);
    if params.len() == 1 {
        // if params[0] == sol {
        //     println!("ok : {}", params[0]);
        // } else {
        //     println!("no : {}", params[0]);
        // }
        return params[0] == sol;
    }

    // we start by last param to filter at each step
    let (last, params) = params.split_last().unwrap();

    // add
    if *last <= sol && sol_exists(sol - last, params.to_vec()) {
        return true;
    }
    // mult
    if sol % *last == 0 && sol_exists(sol / last, params.to_vec()) {
        return true;
    }
    // con
    // ex : 486:6*8||6 => modulo = 10 => 486%10 = 6 && sol_exists(48,[6,8]))
    let modulo = 10u64.pow((*last).ilog10() + 1);
    if sol % modulo == *last && sol_exists(/*(*/ sol/modulo /*) as u64*/, params.to_vec()) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            11387
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            472290821152397
        );
    }
}
