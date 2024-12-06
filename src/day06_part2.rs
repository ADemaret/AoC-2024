use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
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
    let mut grid = Grid2D::new(input);
    // grid.print();

    // remember grid
    let grid_org = grid.clone();

    // first moves to get the positions
    // move
    let start_pos = grid.get_vec_of_char_positions('^')[0];
    // start up
    let mut dir = (-1, 0);
    let mut pos = Some(start_pos);
    let mut positions = Vec::new();
    positions.push((pos.unwrap().0, pos.unwrap().1));
    loop {
        pos = one_move(&mut grid, (pos.unwrap().0, pos.unwrap().1), &mut dir);
        if pos.is_none() {
            // end
            break;
        } else if !positions.contains(&(pos.unwrap().0, pos.unwrap().1)) {
            if let Some(upos) = pos {
            positions.push(upos);
            } else {
                panic!();
            }

            // println!("{:?}",positions);
        }
    }

    // then test obstructions on the positions
    grid = grid_org;

    let mut result = 0;
    for l in 0..grid.max_l {
        println!("line {}/{}", l, grid.max_l);
        for c in 0..grid.max_c {
            if positions.contains(&(l, c)) && test_obstruction(&grid, (l, c)) {
                result += 1;
            }
        }
    }
    Some(result)
}

fn test_obstruction(grid: &Grid2D, obs: (usize, usize)) -> bool {
    // move
    let start_pos = grid.get_vec_of_char_positions('^')[0];
    // println!("start-pos : {:?}", start_pos);
    // start up
    let mut dir = (-1, 0);
    let mut pos = Some(start_pos);
    let mut positions = Vec::new();
    let mut prev_positions = Vec::new();
    positions.push((pos.unwrap().0, pos.unwrap().1));
    let mut grid2 = grid.clone();
    grid2.grid[obs.0][obs.1] = 'O';
    loop {
        pos = one_move(&mut grid2, (pos.unwrap().0, pos.unwrap().1), &mut dir);
        if pos.is_none() {
            // end
            return false;
        } else if !prev_positions.contains(&(pos, dir)) {
            prev_positions.push((pos, dir));
            // println!("{:?}",positions);
        } else {
            return true;
        }
    }
}

fn one_move(
    grid: &mut Grid2D,
    start_pos: (usize, usize),
    dir: &mut (isize, isize),
) -> Option<(usize, usize)> {
    let pos = start_pos;
    grid.grid[pos.0][pos.1] = '.';

    // grid.print_with_vec(&[pos], 'X');
    // pause::pause();

    // out of boundaries
    if (pos.0 == 0 && dir.0 == -1)
        || (pos.1 == 0 && dir.1 == -1)
        || (pos.0 == grid.max_l - 1 && dir.0 > 0)
        || (pos.1 == grid.max_c - 1 && dir.1 > 0)
    {
        return None;
    }
    // move
    let next_pos = (
        (pos.0 as isize + dir.0) as usize,
        (pos.1 as isize + dir.1) as usize,
    );
    if grid.grid[next_pos.0][next_pos.1] != '.' {
        match dir {
            // up -> right
            (-1, 0) => *dir = (0, 1),
            // right -> down
            (0, 1) => *dir = (1, 0),
            // down -> left
            (1, 0) => *dir = (0, -1),
            // left -> up
            (0, -1) => *dir = (-1, 0),
            _ => {
                panic!("wrong direction")
            }
        }
        return Some(pos);
    }
    Some(next_pos)
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
