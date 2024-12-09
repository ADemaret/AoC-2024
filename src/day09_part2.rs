use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 2 --");
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

    // extend : tuples of (index,length)
    let mut extend: Vec<(i64, i32)> = Vec::new();
    for (i, d) in disk.iter().enumerate() {
        if i % 2 == 0 {
            for _j in 0..*d {
                extend.push((i as i64 / 2, *d as i32));
            }
        } else {
            for _j in 0..*d {
                extend.push((-1, *d as i32));
            }
        }
    }
    // println!("{:?}", extend);

    // move
    let mut z = extend.len() - 1;
    'outer: loop {
        let mut a = 0;
        // println!("{} : {:?}, {} : {:?}", a, extend[a], z, extend[z]);

        // find last chunk
        while extend[z].0 < 0 || extend[z].1 < 0 {
            // while not moved yet
            // println!("skip {:?} at {}", extend[z], z);
            z -= 1;
        }
        // println!("maybe move {:?} ?", extend[z]);

        // find first empty space that is large enough
        while !(extend[a].0 == -1 && extend[a].1 >= extend[z].1) {
            if a == z {
                // println!("no space found");
                if z <= 1 || z <= extend[z].1 as usize {
                    break 'outer;
                }
                z -= extend[z].1 as usize;
                continue 'outer;
            }
            a += 1;
        }

        // println!("yes at {}", a);
        // move it
        let size_space = extend[a].1;
        let size_moved = extend[z].1;
        // println!("moving {:?} at {:?}", extend[z], extend[a]);
        for l in 0..size_moved {
            extend[a + l as usize].0 = extend[z - l as usize].0;
            extend[a + l as usize].1 = -3; // moved
            extend[z - l as usize].0 = -2;
        }
        for l in size_moved + 1..size_space + 1 {
            extend[a + l as usize - 1].1 = size_space - size_moved;
        }
        z -= size_moved as usize;
        // println!("{:?}", extend);
        // pause::pause();
    }

    // checksum
    Some(
        extend
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                if val.0 > 0 {
                    // println!("+ {} x {}", i, val.0);
                    i as u64 * val.0 as u64
                } else {
                    0
                }
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
            Some(2858)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt")),
            Some(6413328569890)
        );
    }
}
