use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 25 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day25_input_demo1.txt");
    let input = include_str!("../assets/day25_input.txt");

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
    let mut grids: Vec<Grid2D> = Vec::new();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|gr| {
        grids.push(Grid2D::new(gr));
    });
    for gr in grids {
        if gr.get_at((0, 0)) == '#' {
            // lock
            let a_lock = grid_to_lock(gr);
            locks.push(a_lock);
        } else {
            // key
            let a_key = grid_to_key(gr);
            keys.push(a_key);
        }
    }

    // println!("locks");
    // for l in &locks {
    //     println!("{:?}", l);
    // }
    // println!("keys");
    // for k in &keys {
    //     println!("{:?}", k);
    // }

    let mut result = 0;
    for l in &locks {
        for k in &keys {
            let mut fits = true;
            for i in 0..5 {
                if l[i] + k[i] > 5 {
                    fits = false;
                }
            }
            if fits == true {
                result += 1;
            }
        }
    }

    Some(result)
}

fn grid_to_lock(grid: Grid2D) -> Vec<usize> {
    let mut result = Vec::new();
    for c in 0..grid.max_c {
        for l in 0..grid.max_l {
            if grid.get_at((l, c)) == '.' {
                result.push(l - 1);
                break;
            }
        }
    }
    result
}

fn grid_to_key(grid: Grid2D) -> Vec<usize> {
    let mut result = Vec::new();
    for c in 0..grid.max_c {
        for l in (0..grid.max_l).rev() {
            if grid.get_at((l, c)) == '.' {
                result.push(grid.max_l - l - 2);
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day25_input_demo1.txt")),
            Some(3)
        );
        assert_eq!(get_answer(include_str!("../assets/day25_input.txt")), Some(2840));
    }
}
