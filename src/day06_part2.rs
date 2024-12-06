use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
use crate::utils::grid_dir::GridDir;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

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
    // get grid and start position
    let mut grid = Grid2D::new(input);
    let start_pos = grid.get_vec_of_char_positions('^')[0];
    grid.set_at(start_pos, '.');

    // grid.print();

    // usual path of the guard
    let mut dir = GridDir::new("Up");
    let mut pos = Some(start_pos);
    let mut path = Vec::new();
    path.push(pos.unwrap());
    loop {
        pos = one_move(&grid, pos.unwrap(), &mut dir);
        if pos.is_none() {
            // end
            break;
        } else if !path.contains(&(pos.unwrap().0, pos.unwrap().1)) {
            if let Some(upos) = pos {
                path.push(upos);
            } else {
                panic!();
            }
        }
    }

    // then test obstructions on the positions
    let mut result = 0;
    for l in 0..grid.max_l {
        // println!("line {}/{}", l, grid.max_l);
        for c in 0..grid.max_c {
            if path.contains(&(l, c)) && test_obstruction(&grid, start_pos, (l, c)) {
                result += 1;
            }
        }
    }
    Some(result)
}

fn test_obstruction(grid: &Grid2D, start_pos: (usize, usize), obs: (usize, usize)) -> bool {
    // move
    let mut dir = GridDir::new("Up");
    let mut pos = Some(start_pos);
    let mut positions = Vec::new();
    let mut prev_positions = Vec::new();
    positions.push(pos.unwrap());

    let mut grid2 = grid.clone();
    grid2.set_at(obs, 'O');
    loop {
        pos = one_move(&grid2, (pos.unwrap().0, pos.unwrap().1), &mut dir);
        if pos.is_none() {
            // end
            return false;
        } else if !prev_positions.contains(&(pos, dir.current)) {
            prev_positions.push((pos, dir.current));
        } else {
            // in a loop
            return true;
        }
    }
}

fn one_move(grid: &Grid2D, start_pos: (usize, usize), dir: &mut GridDir) -> Option<(usize, usize)> {
    let pos = start_pos;

    // grid.print_with_vec(&[pos], 'X');
    // pause::pause();

    if let Some(next_pos) = dir.move_point(grid, pos) {
        // move
        if grid.get_at(next_pos) != '.' {
            dir.move_right();
            Some(pos)
        } else {
            Some(next_pos)
        }
    } else {
        // out of boundaries
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input_demo1.txt")),
            Some(6)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt")),
            Some(1562)
        );
    }
}
