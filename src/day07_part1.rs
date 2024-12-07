use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 205 too low
// 3749 too low

fn get_answer(input: &str) -> Option<u64> {
    let mut equs = Vec::new();
    input.lines().for_each(|line| {
        let parts = line.split_once(": ").unwrap();
        let sol = parts.0.parse::<u64>().unwrap();
        let params = parts
            .1
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        equs.push((sol, params));
    });

    let mut result = 0;
    let mut possibilities: Vec<u64> = Vec::new();
    for equ in equs.iter() {

        possibilities.push(next_operation(equ.clone(), 0, 0, "add"));
        possibilities.push(next_operation(equ.clone(), 0, 0, "mult"));
        // println!("equ {:?} => first pair gives {:?}",equ,possibilities);
        

        for index in 2..equ.1.len() {
            
            let mut to_add = Vec::new();
            for poss in possibilities.iter_mut() {
                if *poss != 0 {
                    to_add.push(next_operation(equ.clone(), poss.clone(), index, "add"));
                    *poss = next_operation(equ.clone(), poss.clone(), index, "mult");                    
                }
            }
            for ta in to_add {
                possibilities.push(ta.clone());
            }
            // println!("equ {:?} => index {} gives {:?}",equ,index, possibilities);
        }        
        // println!("equ {:?} => done {:?}",equ,possibilities);
        if possibilities.contains(&equ.0){
            result += equ.0;
        }
        possibilities.clear();

    }
    Some(result)
}

fn next_operation(equ: (u64, Vec<u64>), temp_sol: u64, index: usize, sign: &str) -> u64 {
    let poss : u64;
    match sign {
        "add" => {
            if index == 0 {
                // print!(" {}+{}",equ.1[0],equ.1[1]);
                poss = equ.1[0] + equ.1[1];
            } else {
                // print!(" {}+{}",temp_sol,equ.1[index]);
                poss = temp_sol + equ.1[index];
            }
        }
        "mult" => {
            if index == 0 {
                // print!(" {}x{}",equ.1[0],equ.1[1]);
                poss = equ.1[0] * equ.1[1];
            } else {
                // print!(" {}x{}",temp_sol,equ.1[index]);
                poss = temp_sol * equ.1[index];
            }
        }
        _ => panic!(),
    }
    if poss > equ.0 {
        // println!(" = {} too high =>0",poss);
        return 0;
    }
    // println!(" = {}",poss);
    poss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            Some(3749)
        );
        assert_eq!(get_answer(include_str!("../assets/day07_input.txt")), Some(5540634308362));
    }
}
