use std::time::Instant;

// personal functions
use crate::utils::grid2d::{self, Grid2D};
use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

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

    let ants = input
        .lines()
        .enumerate()
        .map(|(l, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch != '.')
                .map(|(c, ch)| (l as isize, c as isize, ch))
                .collect::<Vec<_>>()
        })
        .filter(|vec| !vec.is_empty())
        .flatten()
        .collect::<Vec<_>>();

    // println!("{:?}", ants);

    let mut antinodes = Vec::new();
    for a1 in &ants {
        for a2 in &ants {
            if (a1.0, a1.1) != (a2.0, a2.1) && a1.2 == a2.2 {
                add_antinodes(grid.clone(), &mut antinodes, *a1, *a2);
            }
        }
    }
    let anti2 = antinodes
        .iter()
        .map(|&(u1, u2)| (u1 as usize, u2 as usize))
        .collect::<Vec<_>>();
    // grid.print_with_vec(&anti2, '#');
    Some(antinodes.len())
}

fn add_antinodes(
    grid: Grid2D,
    antinodes: &mut Vec<(isize, isize)>,
    a1: (isize, isize, char),
    a2: (isize, isize, char),
) {
    let dx = a1.0 - a2.0;
    let dy = a1.1 - a2.1;
    let b1x;
    let b1y;
    let b2x;
    let b2y;
    if dx < 0 {
        b1x = a1.0.min(a2.0) + dx;
        b2x = a1.0.max(a2.0) - dx;
    } else {
        b1x = a1.0.max(a2.0) + dx;
        b2x = a1.0.min(a2.0) - dx;
    }
    if dy < 0 {
        b1y = a1.1.min(a2.1) + dy;
        b2y = a1.1.max(a2.1) - dy;
    } else {
        b1y = a1.1.max(a2.1) + dy;
        b2y = a1.1.min(a2.1) - dy;
    }
    // println!(
    //     "{} in {},{} and {} in {},{} create antinodes at {},{} and {},{}",
    //     a1.2, a1.0, a1.1, a2.2, a2.0, a2.1, b1x, b1y, b2x, b2y
    // );

    if !antinodes.contains(&(b1x, b1y))
        && b1x >= 0
        && b1x < grid.max_l as isize
        && b1y >= 0
        && b1y < grid.max_c as isize
    {
        antinodes.push((b1x, b1y));
    }
    if !antinodes.contains(&(b2x, b2y))
        && b2x >= 0
        && b2x < grid.max_l as isize
        && b2y >= 0
        && b2y < grid.max_c as isize
    {
        antinodes.push((b2x, b2y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day08_input_demo1.txt")),
            Some(14)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day08_input.txt")),
            Some(351)
        );
    }
}
