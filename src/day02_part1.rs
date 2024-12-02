use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

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
    Some(
        input
            .lines()
            .map(|line| {
                let mut this_report = 0;
                let report = line
                    .split_whitespace()
                    .map(|level| level.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                // println!("{:?}", report);
                let mut prev = report[0];
                let mut increase = report[1] > prev;
                for i in 1..report.len() {
                    if increase
                        && report[i] > prev
                        && prev.abs_diff(report[i]) >= 1
                        && prev.abs_diff(report[i]) <= 3
                    {
                        this_report = 1
                    } else if !increase
                        && report[i] < prev
                        && prev.abs_diff(report[i]) >= 1
                        && prev.abs_diff(report[i]) <= 3
                    {
                        this_report = 1
                    } else {
                        this_report = 0;
                        break;
                    }

                    prev = report[i];
                }
                // println!("{}", this_report);
                this_report
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
            get_answer(include_str!("../assets/day02_input_demo1.txt")),
            Some(2)
        );
        assert_eq!(get_answer(include_str!("../assets/day02_input.txt")), Some(356));
    }
}
