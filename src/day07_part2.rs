use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

    println!("The answer is : {}", get_answer(input));
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            let sol = parts.0.parse::<u64>().unwrap();
            let params = parts
                .1
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if sol_exists(sol, params) {
                sol
            } else {
                0
            }
        })
        .sum()
}

fn sol_exists(sol: u64, params: Vec<u64>) -> bool {
    let mut possibilities: Vec<u64> = Vec::new();

    possibilities.push(next_operation(sol, params.clone(), 0, 0, "add"));
    possibilities.push(next_operation(sol, params.clone(), 0, 0, "mult"));
    possibilities.push(next_operation(sol, params.clone(), 0, 0, "con"));
    // println!("equ {:?} => first pair gives {:?}",equ,possibilities);

    for index in 2..params.len() {
        let mut to_add = Vec::new();
        for poss in possibilities.iter_mut() {
            if *poss != 0 {
                to_add.push(next_operation(sol, params.clone(), poss.clone(), index, "add"));
                to_add.push(next_operation(sol, params.clone(), poss.clone(), index, "con"));
                *poss = next_operation(sol, params.clone(), poss.clone(), index, "mult");
            }
        }
        for ta in to_add {
            possibilities.push(ta.clone());
        }
        // println!("equ {:?} => index {} gives {:?}",equ,index, possibilities);
    }
    // println!("equ {:?} => done {:?}",equ,possibilities);
    if possibilities.contains(&sol) {
        return true;
    }
    false
}

fn next_operation(sol: u64, param: Vec<u64>, temp_sol: u64, index: usize, sign: &str) -> u64 {
    let poss: u64;
    match sign {
        "add" => {
            if index == 0 {
                // print!(" {}+{}",param[0],param[1]);
                poss = param[0] + param[1];
            } else {
                // print!(" {}+{}",temp_sol,param[index]);
                poss = temp_sol + param[index];
            }
        }
        "mult" => {
            if index == 0 {
                // print!(" {}x{}",param[0],param[1]);
                poss = param[0] * param[1];
            } else {
                if index == param.len() - 1 {
                    if sol % param[index] != 0 {
                        //println!("{}%{}={} refused",sol,param[index],sol%param[index]);
                        return 0;
                    }
                }
                // print!(" {}x{}",temp_sol,param[index]);
                poss = temp_sol * param[index];
            }
        }
        "con" => {
            if index == 0 {
                // print!(" {}x{}",param[0],param[1]);
                poss = my_concat(param[0], param[1]);
            } else {
                // print!(" {}x{}",temp_sol,param[index]);
                poss = my_concat(temp_sol, param[index]);
            }
        }
        _ => panic!(),
    }
    if poss > sol {
        // println!(" = {} too high =>0",poss);
        return 0;
    }
    // println!(" = {}",poss);
    poss
}

fn my_concat(a: u64, b: u64) -> u64 {
    // length of b
    let width = b.ilog10()+1;
    // println!("concat of {},{} is {}",a,b,a * 10u64.pow(width as u32)+b);
    a * 10u64.pow(width) + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            11387
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            472290821152397
        );
    }
}
