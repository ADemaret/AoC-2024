use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day14_input_demo1.txt");
    let input = include_str!("../assets/day14_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

fn get_answer(input: &str) -> Option<i64> {
    let mut max_density = 0;
    let mut result = None;
    for i in 0..10000 {
        if let Some(answer) = get_answer2(input, i, false) {
            if max_density < answer {
                max_density = answer;
                result = Some(i);
            }
        }
    }
    if let Some(r) = result {
        get_answer2(input, r, true);
    }

    result
}

//

fn get_answer2(input: &str, secondes: i64, print: bool) -> Option<usize> {
    // let secondes = 100;
    // let grid_x = 11;
    // let grid_y = 7;
    let grid_x = 101;
    let grid_y = 103;

    // input
    let bots = input
        .lines()
        .map(|line| {
            line.split(['=', ',', ' '])
                .filter_map(|a| a.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // final position
    let mut finals = Vec::new();
    for bot in bots {
        let x =
            (((bot[0] + grid_x) % grid_x + ((bot[2] + grid_x) % grid_x) * secondes + 3 * grid_x)
                % grid_x) as usize;
        let y =
            (((bot[1] + grid_y) % grid_y + ((bot[3] + grid_y) % grid_y) * secondes + 3 * grid_y)
                % grid_y) as usize;
        finals.push((y, x));
    }

    // grid for debug
    if print {
        let grid = Grid2D::new_empty(grid_x as usize, grid_y as usize, '.');
        grid.print_with_vec(&finals, 'X');
    }
    // density in central zone
    let zx = grid_x / 3;
    let zy = grid_y / 3;
    let result = quadrant(
        &finals,
        zx as usize,
        (2 * zx) as usize,
        zy as usize,
        (2 * zy) as usize,
    );
    // println!(
    //     "at {}, result = {}",
    //     secondes,
    //     result
    // );
    Some(result)
}

fn quadrant(
    finals: &Vec<(usize, usize)>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
) -> usize {
    // println!("{}<=x<={}, {}<=y<={}",x_min,x_max,y_min,y_max);
    let mut result = 0;
    for f in finals {
        if f.1 >= x_min && f.1 <= x_max && f.0 >= y_min && f.0 <= y_max {
            // println!("{:?}",f);
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day14_input_demo1.txt")),
        //     None
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day14_input.txt")),
            Some(6398)
        );
    }
}
