use std::{collections::HashSet, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

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
    let pcs = input
        .lines()
        .map(|line| line.split("-").collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    let mut trios = HashSet::new();
    for (i,l1) in pcs.iter().enumerate() {
        println!("{i}");
        for l2 in &pcs {
            for l3 in &pcs {
                if l1 != l2 && l1 != l3 && l2 != l3 {
                    if (l1[1] == l2[0] && l2[1] == l3[0] && l3[1] == l1[0])
                        || (l1[1] == l2[0] && l2[1] == l3[1] && l3[0] == l1[0])
                        || (l1[0] == l2[0] && l2[1] == l3[0] && l3[1] == l1[1])
                        || (l1[0] == l2[0] && l2[1] == l3[1] && l3[0] == l1[1])
                    {
                        let mut a_trio = vec![l1[0], l1[1], l2[1]];
                        if l1[0].starts_with('t')
                            || l1[1].starts_with('t')
                            || l2[1].starts_with('t')
                        {
                            a_trio.sort();
                            trios.insert(a_trio);
                        }
                    } else if l1[1] == l2[1] && l2[0] == l3[0] && l3[1] == l1[0]
                        || (l1[1] == l2[1] && l2[0] == l3[1] && l3[0] == l1[0])
                        || (l1[0] == l2[1] && l2[0] == l3[0] && l3[1] == l1[1])
                        || (l1[0] == l2[1] && l2[0] == l3[1] && l3[0] == l1[1])
                    {
                        let mut a_trio = vec![l1[0], l1[1], l2[0]];
                        if l1[0].starts_with('t')
                            || l1[1].starts_with('t')
                            || l2[0].starts_with('t')
                        {
                            a_trio.sort();
                            trios.insert(a_trio);
                        }
                    }
                }
            }
        }
    }
    for t in &trios {
        println!("{:?}", t);
    }
    Some(trios.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt")),
            Some(7)
        );
        assert_eq!(get_answer(include_str!("../assets/day23_input.txt")), Some(1378));
    }
}
