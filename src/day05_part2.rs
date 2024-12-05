use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day05_input_demo1.txt");
    let input = include_str!("../assets/day05_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<u32> {
    let mut section = 0;
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    input.lines().for_each(|line| {
        if !line.is_empty() {
            if section == 0 {
                let (x, y) = line.split_once('|').unwrap();
                rules.push((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()));
            } else {
                let update = line
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                updates.push(update);
            }
        } else {
            section = 2;
        }
    });
    // println!("{:?}", rules);
    // println!("{:?}", updates);

    Some(
        updates
            .iter_mut()
            .map(|u| {
                let mut is_wrong_once = false;
                let mut is_wrong = test_is_wrong(u.to_vec(), rules.clone());
                while is_wrong.0 {
                    is_wrong_once = true;
                    u.swap(is_wrong.1 .0, is_wrong.1 .1);
                    // println!("swapping => {:?}", u);
                    is_wrong = test_is_wrong(u.to_vec(), rules.clone());
                }

                if !is_wrong.0 && is_wrong_once{
                    // println!("ok => add {}", u[((u.len() + 1) / 2) - 1]);
                    u[((u.len() + 1) / 2) - 1]
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn test_is_wrong(u: Vec<u32>, rules: Vec<(u32, u32)>) -> (bool, (usize, usize)) {
    // println!("testing {:?}", u);
    let mut is_wrong = false;
    let mut r1 = 0;
    let mut r2 = 0;
    for i in 0..u.len() - 1 {
        for j in 1..u.len() {
            if i < j {
                let to_test = (u[j], u[i]);
                if rules.contains(&to_test) {
                    // println!("against rule {:?}", to_test);
                    is_wrong = true;
                    r1 = j;
                    r2 = i;
                    break;
                }
            }
        }
    }
    (is_wrong, (r1, r2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day05_input_demo1.txt")),
            Some(123)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day05_input.txt")),
            Some(6204)
        );
    }
}
