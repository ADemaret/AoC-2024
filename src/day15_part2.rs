use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day15_input_demo1.txt");
    // let input = include_str!("../assets/day15_input_demo2.txt");
    // let input = include_str!("../assets/day15_input_demo3.txt");
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
                for c in line.chars() {
                    match c {
                        '#' => gr.push_str("##"),
                        'O' => gr.push_str("[]"),
                        '.' => gr.push_str(".."),
                        '@' => gr.push_str("@."),
                        _ => panic!(),
                    }
                    //gr.push_str(line);
                }
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
                // grid.print();
                //    pause::pause();

                if move_bottom(&mut grid, current) {
                    current = (current.0 + 1, current.1);
                }
                // grid.print();
                // pause::pause();
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
                if move_top(&mut grid, current) {
                    current = (current.0 - 1, current.1);
                }
            }
            _ => panic!("unexpected move"),
        }
        // grid.print();
        // pause::pause();
    }
    // grid.print();

    // count
    // println!("count :");
    // let fish = grid.get_vec_of_char_positions('[');
    // Some(
    //     fish.iter()
    //         .map(|(l, c)| {
    //             let lmin = *l.min(&(grid.max_l - l));
    //             let cmin = *c.min(&(grid.max_c - c));
    //             let res = 100 * lmin + c;
    //             // println!("{},{} => {}*100 + {} = {}", l, c, lmin, cmin, res);
    //             res
    //         })
    //         .sum(),
    // )
    let fish = grid.get_vec_of_char_positions('[');
    Some(fish.iter().map(|(l, c)| l * 100 + c).sum())
}

fn move_right(grid: &mut Grid2D, current: (usize, usize)) -> bool {
    let mut row = grid.grid[current.0].clone();
    let org_symbol = grid.get_at(current);
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
        for x in (start + 1..=start + space).rev() {
            row[x] = row[x - 1];
            grid.set_at((current.0, x), row[x - 1]);
        }
        grid.set_at((current.0, start), '.');
        grid.set_at((current.0, start + 1), org_symbol);

        // println!("row after is {:?}", row);
        return true;
    }
    false
}

fn move_bottom(grid: &mut Grid2D, current: (usize, usize)) -> bool {
    let moved;

    // create a list of all nodes to move
    let mut to_move_down = Vec::new();
    match grid.get_at((current.0 + 1, current.1)) {
        '[' => {
            moved = can_move_down(grid, (current.0 + 1, current.1), &mut to_move_down);
        }
        ']' => {
            moved = can_move_down(grid, (current.0 + 1, current.1 - 1), &mut to_move_down);
        }
        '.' => {
            moved = true;
        }
        '#' => {
            return false;
        }
        _ => {
            panic!();
        }
    }
    if moved {
        // move all bricks to be moved
        // println!("brick to move down : {:?}", to_move_down);
        for brick in &to_move_down {
            grid.set_at(*brick, '.');
            grid.set_at((brick.0, brick.1 + 1), '.');
        }
        for brick in &to_move_down {
            grid.set_at((brick.0 + 1, brick.1), '[');
            grid.set_at((brick.0 + 1, brick.1 + 1), ']');
        }
        grid.set_at((current.0 + 1, current.1), '@');
        grid.set_at(current, '.');
        return true;
    }
    false
}

fn move_top(grid: &mut Grid2D, current: (usize, usize)) -> bool {
    let moved;

    // create a list of all nodes to move
    let mut to_move = Vec::new();
    match grid.get_at((current.0 - 1, current.1)) {
        '[' => {
            moved = can_move_up(grid, (current.0 - 1, current.1), &mut to_move);
        }
        ']' => {
            moved = can_move_up(grid, (current.0 - 1, current.1 - 1), &mut to_move);
        }
        '.' => {
            moved = true;
        }
        '#' => {
            return false;
        }
        _ => {
            panic!();
        }
    }
    if moved {
        // move all bricks to be moved
        // println!("brick to move up : {:?}", to_move);
        for brick in &to_move {
            grid.set_at(*brick, '.');
            grid.set_at((brick.0, brick.1 + 1), '.');
        }
        for brick in &to_move {
            grid.set_at((brick.0 - 1, brick.1), '[');
            grid.set_at((brick.0 - 1, brick.1 + 1), ']');
        }
        grid.set_at((current.0 - 1, current.1), '@');
        grid.set_at(current, '.');
        return true;
    }
    false
}

/// receive a brick
/// true if brick can go down
fn can_move_down(
    grid: &mut Grid2D,
    current: (usize, usize),
    to_move_down: &mut Vec<(usize, usize)>,
) -> bool {
    // space after brick
    if grid.get_at((current.0 + 1, current.1)) == '.'
        && grid.get_at((current.0 + 1, current.1 + 1)) == '.'
    {
        to_move_down.push(current);
        return true;
    }
    let mut left_ok = false;
    let mut right_ok = false;
    match grid.get_at((current.0 + 1, current.1)) {
        '[' => {
            left_ok = can_move_down(grid, (current.0 + 1, current.1), to_move_down);
            if left_ok {
                to_move_down.push((current.0 + 1, current.1));
            }
        }
        ']' => {
            left_ok = can_move_down(grid, (current.0 + 1, current.1 - 1), to_move_down);
            if left_ok {
                to_move_down.push((current.0 + 1, current.1 - 1));
            }
        }
        '.' => {
            left_ok = true;
        }
        _ => {}
    }
    match grid.get_at((current.0 + 1, current.1 + 1)) {
        '[' => {
            right_ok = can_move_down(grid, (current.0 + 1, current.1 + 1), to_move_down);
            if right_ok {
                to_move_down.push((current.0 + 1, current.1 + 1));
            }
        }
        ']' => {
            // right_ok = can_move_down(grid, (current.0 + 1, current.1 + 2), to_move_down);
            // if right_ok {
            //     to_move_down.push((current.0 + 1, current.1 + 2));
            // }
            right_ok = true;
        }
        '.' => {
            right_ok = true;
        }
        _ => {}
    }
    if left_ok && right_ok {
        to_move_down.push(current);
        return true;
    }
    false
}

/// receive a brick
/// true if brick can go up
fn can_move_up(
    grid: &mut Grid2D,
    current: (usize, usize),
    to_move: &mut Vec<(usize, usize)>,
) -> bool {
    // space after brick
    if grid.get_at((current.0 - 1, current.1)) == '.'
        && grid.get_at((current.0 - 1, current.1 + 1)) == '.'
    {
        to_move.push(current);
        return true;
    }
    let mut left_ok = false;
    let mut right_ok = false;
    match grid.get_at((current.0 - 1, current.1)) {
        '[' => {
            left_ok = can_move_up(grid, (current.0 - 1, current.1), to_move);
            if left_ok {
                to_move.push((current.0 - 1, current.1));
            }
        }
        ']' => {
            left_ok = can_move_up(grid, (current.0 - 1, current.1 - 1), to_move);
            if left_ok {
                to_move.push((current.0 - 1, current.1 - 1));
            }
        }
        '.' => {
            left_ok = true;
        }
        _ => {}
    }
    match grid.get_at((current.0 - 1, current.1 + 1)) {
        '[' => {
            right_ok = can_move_up(grid, (current.0 - 1, current.1 + 1), to_move);
            if right_ok {
                to_move.push((current.0 - 1, current.1 + 1));
            }
        }
        ']' => {
            right_ok = true;
            // right_ok = can_move_up(grid, (current.0 - 1, current.1 + 2), to_move);
            // if right_ok {
            //     to_move.push((current.0 - 1, current.1 + 2));
            // }
        }
        '.' => {
            right_ok = true;
        }
        _ => {}
    }
    if left_ok && right_ok {
        to_move.push(current);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day15_input_demo1.txt")),
        //     Some(2028)
        // );
        // assert_eq!(
        //     get_answer(include_str!("../assets/day15_input_demo2.txt")),
        //     Some(9021)
        // );
        // assert_eq!(
        //     get_answer(include_str!("../assets/day15_input.txt")),
        //     Some(1294459)
        // );
    }
}
