use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 1 --");
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

    let mut result = 0;
    for (i, p) in patterns.iter().enumerate() {
        // print!("-- pattern {}", i);
        if breadth_first_search(p, &towels) {
            // println!(" -> ok");
            result += 1;
        } else {
            // println!(" -> nope");
        }
    }
    Some(result)
}

fn breadth_first_search(p: &str, towels: &Vec<&str>) -> bool {
    let end = p.len();
    let mut queue = Vec::new();

    // first towels
    for (i, t) in towels.iter().enumerate() {
        if p.starts_with(*t) {
            // println!("for {} at pos 0, take {}", p, t);
            queue.push(t.len());
        }
    }
    queue.sort();
    queue.reverse();

    // println!("{:?}",queue);

    while !queue.is_empty() {
        let pos = queue[0]; // higher
        queue.remove(0);
        // trouvé
        if pos == end {
            // println!("found");
            return true;
        }
        // Check the neighboring nodes for any that we've not visited yet.
        for (i, t) in towels.iter().enumerate() {
            if p[pos..].starts_with(*t) {
                // println!("for {} at pos {}, take {}", p, pos, t);
                queue.push(pos + t.len());
            }
        }
        queue.sort();
        queue.reverse();
        // println!("{:?}",queue);
    }
    // not found
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input_demo1.txt")),
            Some(6)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day19_input.txt")),
            Some(209)
        );
    }
}
