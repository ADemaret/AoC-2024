use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 2 --");
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

// not 392, not 399

fn is_report_ok(report: Vec<u32>) -> bool {
    let mut prev = report[0];
    let increase = report[1] > prev;
    for i in 1..report.len() {
        if !((increase
            && report[i] > prev
            && prev.abs_diff(report[i]) >= 1
            && prev.abs_diff(report[i]) <= 3)
            || (!increase
                && report[i] < prev
                && prev.abs_diff(report[i]) >= 1
                && prev.abs_diff(report[i]) <= 3))
        {
            return false;
        }

        prev = report[i];
    }
    true
}

fn get_answer(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut problem = 0;
                let report = line
                    .split_whitespace()
                    .map(|level| level.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                // print!("{:?} ", report);

                for i in 0..report.len() {
                    let mut report_copy = report.clone();
                    report_copy.remove(i);
                    if is_report_ok(report_copy) {
                        return 1;
                    } else {
                        problem += 1;
                        if problem > 1 {
                            return 0;
                        }
                    }
                }
                0
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
            Some(4)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            Some(413)
        );
    }
}
