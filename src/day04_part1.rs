use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 1 --");
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

// 2648 too low

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

    // find horizontal
    for (l, line) in grid.iter().enumerate() {
        for c in 0..line.len() - 3 {
            if line[c] == 'X'
                && grid[l][c + 1] == 'M'
                && grid[l][c + 2] == 'A'
                && grid[l][c + 3] == 'S'
            {
                // println!("found in {},{} - horiz",l,c);
                count += 1;
            }
        }
    }
    // find horizontal rev
    for (l, line) in grid.iter().enumerate() {
        for c in 0..line.len() - 3 {
            if line[c] == 'S'
                && grid[l][c + 1] == 'A'
                && grid[l][c + 2] == 'M'
                && grid[l][c + 3] == 'X'
            {
                // println!("found in {},{} - horiz rev",l,c+3);
                count += 1;
            }
        }
    }

    // find vertical
    for (l, line) in grid.iter().enumerate() {
        if l < grid.len() - 3 {
            for c in 0..line.len() {
                if line[c] == 'X'
                    && grid[l + 1][c] == 'M'
                    && grid[l + 2][c] == 'A'
                    && grid[l + 3][c] == 'S'
                {
                    // println!("found in {},{} - vert",l,c);
                    count += 1;
                }
            }
        }
    }
    // find vertical rev
    for (l, line) in grid.iter().enumerate() {
        if l < grid.len() - 3 {
            for c in 0..line.len() {
                if line[c] == 'S'
                    && grid[l+1][c] == 'A'
                    && grid[l+2][c] == 'M'
                    && grid[l+3][c] == 'X'
                {
                    // println!("found in {},{} - vert rev",l+3,c);
                    count += 1;
                }
            }
        }
    }

    // find diag1
    for (l, line) in grid.iter().enumerate() {
        if l < grid.len() - 3 {
            for c in 0..line.len()-3 {
                if line[c] == 'X'
                    && grid[l + 1][c+1] == 'M'
                    && grid[l + 2][c+2] == 'A'
                    && grid[l + 3][c+3] == 'S'
                {
                    // println!("found in {},{} - diag1",l,c);
                    count += 1;
                }
            }
        }
    }
    // find diag1 rev
    for (l, line) in grid.iter().enumerate() {
        if l < grid.len() - 3 {
            for c in 0..line.len()-3 {
                if line[c] == 'S'
                    && grid[l+1][c+1] == 'A'
                    && grid[l+2][c+2] == 'M'
                    && grid[l+3][c+3] == 'X'
                {
                    // println!("found in {},{} - diag1 rev",l+3,c+3);
                    count += 1;
                }
            }
        }
    }

       // find diag2
       for (l, line) in grid.iter().enumerate() {
        if l >= 3 {
            for c in 0..line.len()-3 {
                if line[c] == 'X'
                    && grid[l - 1][c+1] == 'M'
                    && grid[l - 2][c+2] == 'A'
                    && grid[l - 3][c+3] == 'S'
                {
                    // println!("found in {},{} - diag2",l,c);
                    count += 1;
                }
            }
        }
    }
    // find diag2 rev
    for (l, line) in grid.iter().enumerate() {
        if l >= 3 {
            for c in 0..line.len()-3 {
                if line[c] == 'S'
                    && grid[l-1][c+1] == 'A'
                    && grid[l-2][c+2] == 'M'
                    && grid[l-3][c+3] == 'X'
                {
                    // println!("found in {},{} - diag2 rev",l-3,c+3);
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
            Some(18)
        );
        assert_eq!(get_answer(include_str!("../assets/day04_input.txt")), Some(2685));
    }
}
