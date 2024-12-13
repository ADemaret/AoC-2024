use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day13_input_demo1.txt");
    let input = include_str!("../assets/day13_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<i64> {
    let mut l0 = vec![0, 0];
    let mut l1 = vec![0, 0];
    let mut l2 = vec![0, 0];
    Some(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i % 4 == 0 {
                    l0 = line
                        .split(['+', ',', ' '])
                        .filter_map(|a| a.parse::<i64>().ok())
                        .collect::<Vec<_>>();
                    // println!("ligne 0 : {:?}",l0);
                }
                if i % 4 == 1 {
                    l1 = line
                        .split(['+', ',', ' '])
                        .filter_map(|a| a.parse::<i64>().ok())
                        .collect::<Vec<_>>();
                    // println!("ligne 1 : {:?}",l1);
                }
                if i % 4 == 2 {
                    l2 = line
                        .split(['=', ',', ' '])
                        .filter_map(|a| a.parse::<i64>().ok())
                        .collect::<Vec<_>>();
                    // println!("ligne 2 : {:?}",l2);
                    // l2[0] = a*l0[0] + b*l1[0]
                    // l2[1] = a*l0[1] + b*l1[1]
                    // a + 3b is min
                    // a < 100 && b < 100
                    let div = l0[0] * l1[1] - l0[1] * l1[0];
                    // println!("div : {}x{}-{}*{} = {}",l0[0],l1[1],l0[1],l1[0],div);
                    if div != 0 {
                        let a = (l2[0] * l1[1] - l2[1] * l1[0]) / div;
                        let b = (l2[1] * l0[0] - l2[0] * l0[1]) / div;
                        // print!("{},{}",a,b);

                        if l0[0] * a + l1[0] * b != l2[0] || l0[1] * a + l1[1] * b != l2[1] {
                            // println!(" => wrong");
                            return 0;
                        }
                        // println!("{}, {}",a,b);
                        if a < 100 && b < 100 && a > 0 && b > 0 {
                            // println!(" => sol is {}",a*3+b);
                            // println!("---");
                            return a * 3 + b;
                        } else {
                            // println!("no sol");
                            // println!("---");
                            return 0;
                        }
                    }
                }
                0
            })
            //.inspect(|x| println!("{}",x))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day13_input_demo1.txt")),
            Some(480)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day13_input.txt")),
            Some(31589)
        );
    }
}
