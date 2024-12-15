use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day15_input_demo1.txt");
    // let input = include_str!("../assets/day15_input_demo2.txt");
    let input = include_str!("../assets/day15_input.txt");

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
    // parsing
    let mut part1 = true;
    let mut gr = String::new();
    let mut moves = Vec::new();
    input.lines().for_each(|line| {
        if part1 {
            if line.is_empty() {
                part1 = false;
            } else {
                gr.push_str(line);
                gr.push('\n');
            }
        } else {
            moves.extend(line.chars());
        }
    });
    let mut grid = Grid2D::new(gr.as_str());
    // grid.print();
    // println!("moves = {:?}", moves);

    // moves
    let mut current = grid.get_vec_of_char_positions('@')[0];
    // println!("current is at {:?}", current);
    for mv in moves {
        // println!("move is {}", mv);
        let right = grid.max_c - current.1 - 1;
        let bottom = grid.max_l - current.0 - 1;
        match mv {
            '>' => {
                if move_right(&mut grid, current) {
                    current = (current.0, current.1 + 1);
                }
            }
            'v' => {
                grid = grid.rotate();
                if move_right(&mut grid, (right, current.0)) {
                    current = (current.0 + 1, current.1);
                }
                grid = grid.rotate();
                grid = grid.rotate();
                grid = grid.rotate();
            }
            '<' => {
                grid = grid.rotate();
                grid = grid.rotate();
                if move_right(&mut grid, (bottom, right)) {
                    current = (current.0, current.1 - 1);
                }
                grid = grid.rotate();
                grid = grid.rotate();
            }
            '^' => {
                grid = grid.rotate();
                grid = grid.rotate();
                grid = grid.rotate();
                if move_right(&mut grid, (current.1, bottom)) {
                    current = (current.0 - 1, current.1);
                }
                grid = grid.rotate();
            }
            _ => panic!("unexpected move"),
        }
        // grid.print();
    }


    // count
    let fish = grid.get_vec_of_char_positions('O');
    Some(fish.iter().map(|(l,c)| l*100+c).sum())
}

fn move_right(grid: &mut Grid2D, current: (usize, usize)) -> bool {
    let mut row = grid.grid[current.0].clone();
    // println!("current is {:?} - row is {:?}", current, row);
    let start = current.1;
    if let Some(space) = row.iter().skip(start).position(|&x| x == '.') {
        if space == grid.max_l {
            return false;
        }
        let wall = row.iter().skip(start).position(|&x| x == '#').unwrap();
        if wall < space {
            return false;
        }
        for x in (start+1..=start + space).rev() {
            row[x] = row[x - 1];
            grid.set_at((current.0, x), row[x - 1]);
        }
        grid.set_at((current.0, start), '.');

        // println!("row after is {:?}", row);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo1.txt")),
            Some(2028)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo2.txt")),
            Some(10092)
        );
        assert_eq!(get_answer(include_str!("../assets/day15_input.txt")), Some(1294459));
    }
}
