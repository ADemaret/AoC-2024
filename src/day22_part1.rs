use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo1.txt");
    let input = include_str!("../assets/day22_input.txt");

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
    let mut secrets = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..2000 {
        for s in secrets.iter_mut() {
            *s = next_secret(*s);
        }
    }
    // println!("{:?}",secrets);
    Some(secrets.iter().sum::<usize>())
}

fn next_secret(secret: usize) -> usize {
    let mut curr = mix(secret, secret * 64);
    curr = prune(curr);
    curr = mix(curr, curr / 32);
    curr = prune(curr);
    curr = mix(curr, curr * 2048);
    curr = prune(curr);
    curr
}

fn mix(secret: usize, val: usize) -> usize {
    secret ^ val
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        assert_eq!(mix(42, 15), 37);
        assert_eq!(prune(100000000), 16113920);
        let x = next_secret(123);
        assert_eq!(x, 15887950);
        let x = next_secret(x);
        assert_eq!(x, 16495136);
        let x = next_secret(x);
        assert_eq!(x, 527345);
        let x = next_secret(x);
        assert_eq!(x, 704524);
        let x = next_secret(x);
        assert_eq!(x, 1553684);
        let x = next_secret(x);
        assert_eq!(x, 12683156);
        let x = next_secret(x);
        assert_eq!(x, 11100544);
        let x = next_secret(x);
        assert_eq!(x, 12249484);
        let x = next_secret(x);
        assert_eq!(x, 7753432);
        let x = next_secret(x);
        assert_eq!(x, 5908254);
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt")),
            Some(37327623)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input.txt")),
            Some(13234715490)
        );
    }
}
