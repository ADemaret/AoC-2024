use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day09_input_demo1.txt");
    let input = include_str!("../assets/day09_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<u64> {
    let disk = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    // extend
    let mut extend: Vec<i64> = Vec::new();
    for (i, d) in disk.iter().enumerate() {
        if i % 2 == 0 {
            for _j in 0..*d {
                extend.push(i as i64 / 2);
            }
        } else {
            for _j in 0..*d {
                extend.push(-1);
            }
        }
    }
    // println!("{:?}", extend);

    // move
    let mut a = 0;
    let mut z = extend.len() - 1;
    loop {
        //find last valid
        while extend[z] < 0 {
            z -= 1;
        }
        if extend[a] == -1 {
            extend[a] = extend[z];
            extend[z] = -2;
        }
        a += 1;
        if extend[a] == -2 {
            extend.truncate(z+1);
            break;
        }
    }
    // println!("{:?}", extend);

    // checksum
    Some(
        extend
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                // println!("+ {} x {}", i, &val);
                i as u64 * val as u64
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day09_input_demo1.txt")),
            Some(1928)
        );
        assert_eq!(get_answer(include_str!("../assets/day09_input.txt")), Some(6378826667552));
    }
}
