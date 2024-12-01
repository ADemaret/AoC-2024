use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day01_input_demo1.txt");
    let input = include_str!("../assets/day01_input.txt");

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
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().map(|p| p.parse::<u32>().unwrap()).collect::<Vec<_>>();
        vec1.push(parts[0]);
        vec2.push(parts[1]);
    });
    vec1.sort();
    vec2.sort();
    let mut total = 0;
    for i in vec1 {
        let nbr = vec2.iter().filter(|&x| *x==i).count() as u32;
        total += i * nbr;
    }
    // println!("{:?}",vec1);
    // println!("{:?}",vec2);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day01_input_demo1.txt")), Some(31));
        assert_eq!(get_answer(include_str!("../assets/day01_input.txt")), Some(21607792));
    }
}
