use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

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
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let mut vec;
        let (l1, l2) = line.split_once("-").unwrap();

        if tree.contains_key(l1) {
            vec = tree.get(&l1).unwrap().clone();
            vec.push(l2);
            tree.insert(l1, vec);
        } else {
            tree.insert(l1, vec![l2]);
        }

        if tree.contains_key(l2) {
            vec = tree.get(&l2).unwrap().clone();
            vec.push(l1);
            tree.insert(l2, vec);
        } else {
            tree.insert(l2, vec![l1]);
        }
    });

    // t.0  - t1   -
    // "ta" - "co" - "de"
    let mut trios = HashSet::new();
    for t in &tree {
        if t.0.starts_with('t') {
            // println!("{}", t.0);
            for t1 in t.1 {
                // println!("  {}", t1);
                if let Some(t2) = tree.get(t1) {
                    // println!("    {:?}", t2);
                    let common = t2
                        .iter()
                        .filter(|&x| t.1.contains(x))
                        .cloned()
                        .collect::<Vec<_>>();
                    // println!("    => common : {:?}", common);
                    for last in common {
                        let mut v = vec![t.0, t1, last];
                        v.sort();
                        // println!("      insert {:?}",v);
                        trios.insert(v.clone());
                    }
                }
            }
        }
    }

    // for t in &trios {
    //     println!("{:?}", t);
    // }
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
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt")),
            Some(1378)
        );
    }
}
