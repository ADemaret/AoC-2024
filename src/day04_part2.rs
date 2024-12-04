use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day04_input_demo1.txt");
    let input = include_str!("../assets/day04_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 1992 too low

fn get_answer(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (l, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for char in line.chars() {
            grid[l].push(char);
        }
    }
    // println!("{:?}", grid);

    let mut count = 0;

    // find 1
    // M.M
    // .A.
    // S.S
    for (l, line) in grid.iter().enumerate() {
        if l <= grid.len() - 3 {
            for c in 0..=line.len() - 3 {
                if line[c] == 'M'
                    && grid[l + 1][c + 1] == 'A'
                    && grid[l + 2][c + 2] == 'S'
                    && grid[l][c + 2] == 'M'
                    && grid[l + 2][c] == 'S'
                {
                    // println!("found in {},{} - type1", l, c);
                    count += 1;
                }
            }
        }
    }

    // find 2
    // M.S
    // .A.
    // M.S
    for (l, line) in grid.iter().enumerate() {
        if l <= grid.len() - 3 {
            for c in 0..=line.len() - 3 {
                if line[c] == 'M'
                    && grid[l + 1][c + 1] == 'A'
                    && grid[l + 2][c + 2] == 'S'
                    && grid[l + 2][c] == 'M'
                    && grid[l][c + 2] == 'S'
                {
                    // println!("found in {},{} - type2", l, c);
                    count += 1;
                }
            }
        }
    }

    // find 3
    // S.S
    // .A.
    // M.M
    for (l, line) in grid.iter().enumerate() {
        if l <= grid.len() - 3 {
            for c in 0..=line.len() - 3 {
                if line[c] == 'S'
                    && grid[l + 1][c + 1] == 'A'
                    && grid[l + 2][c + 2] == 'M'
                    && grid[l + 2][c] == 'M'
                    && grid[l][c + 2] == 'S'
                {
                    // println!("found in {},{} - type3", l, c);
                    count += 1;
                }
            }
        }
    }

    // find 4
    // S.M
    // .A.
    // S.M
    for (l, line) in grid.iter().enumerate() {
        if l <= grid.len() - 3 {
            for c in 0..=line.len() - 3 {
                if line[c] == 'S'
                    && grid[l + 1][c + 1] == 'A'
                    && grid[l + 2][c + 2] == 'M'
                    && grid[l + 2][c] == 'S'
                    && grid[l][c + 2] == 'M'
                {
                    // println!("found in {},{} - type4", l, c);
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day04_input_demo1.txt")),
            Some(9)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day04_input.txt")),
            Some(2048)
        );
    }
}
