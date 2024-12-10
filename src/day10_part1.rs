use std::time::Instant;

// personal functions
// use crate::utils::grid2d;
use crate::utils::grid2d::Grid2D;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day10_input_demo1.txt");
    // let input = include_str!("../assets/day10_input_demo2.txt");
    // let input = include_str!("../assets/day10_input_demo3.txt");
    let input = include_str!("../assets/day10_input.txt");

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
    let grid = Grid2D::new(input);
    // grid.print();

    let mut result = 0;
    for l in 0..grid.max_l {
        for c in 0..grid.max_c {
            if grid.get_at((l, c)) == '0' {
                // println!("start at {},{}", l, c);
                let mut nines = Vec::new();
                count_paths(&grid, (l, c, '0'), &mut nines);
                // println!("nines = {:?} => score is {}", nines,nines.len());
                result += nines.len();
            }
        }
    }
    Some(result)
}

fn count_paths(
    grid: &Grid2D,
    current : (usize,usize,char),
    nines: &mut Vec<(usize, usize,char)>,
) -> usize {
    let mut result = 0;

    if current.2 == '9' {
        // println!("9 reached at {:?}", current);
        if !nines.contains(&current) {
            nines.push(current);
        }
        return 1;
    }
    for adj in grid.get_adjacents_ortho(current.0,current.1) {
        if adj.2 as usize == current.2 as usize + 1 {
            // println!("{:?} -> {:?}", current, adj);
            result += count_paths(grid, adj, nines);
        }
    }
    // println!("result : {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo1.txt")),
            Some(2)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo2.txt")),
            Some(4)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo3.txt")),
            Some(36)
        );
        assert_eq!(get_answer(include_str!("../assets/day10_input.txt")), Some(820));
    }
}
