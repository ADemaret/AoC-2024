use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day19_input_demo1.txt");
    let input = include_str!("../assets/day19_input.txt");

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
    let mut towels: Vec<&str> = Vec::new();
    let mut patterns: Vec<&str> = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            towels = line.split(", ").collect();
        } else if i > 1 {
            patterns.push(line);
        }
    });

    // println!("{:?}", towels);
    // println!("{:?}", patterns);

    let mut memo = HashMap::new();
    let mut result = 0;
    for (i, p) in patterns.iter().enumerate() {
        // print!("-- pattern {}", i);
        let res = depth_first_search(p, &towels, &mut memo);
        // println!(" -> {}", res);
        result += res;
    }
    // for m in memo {
    //     println!("{:?}",m);
    // }
    Some(result)
}

fn depth_first_search(p: &str, towels: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    // println!("pattern {}",p);
    if let Some(result) = memo.get(&p.to_string()) {
        // println!(" found in memo => {}",result);
        return *result;
    }
    if p.is_empty() {
        // println!(" empty => 1");
        1
    } else {
        let mut result = 0;
        for t in towels {
            if p.starts_with(t) {
                let sub = p.strip_prefix(t).unwrap();
                let r = depth_first_search(sub, towels, memo);
                memo.insert(sub.to_string(), r);
                result += r;
            }
        }
        // println!(" => {}",result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input_demo1.txt")),
            Some(16)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day19_input.txt")),
            Some(777669668613191)
        );
    }
}
