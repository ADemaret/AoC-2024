use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 24 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day24_input_demo1.txt");
    // let input = include_str!("../assets/day24_input_demo2.txt");
    let input = include_str!("../assets/day24_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

enum Instr {
    And,
    Or,
    Xor,
}

struct Instruction {
    v1: String,
    v2: String,
    instr: Instr,
    v3: String,
    done: bool,
}

fn get_answer(input: &str) -> Option<usize> {
    let mut blocks = Vec::new();
    input.split("\n\n").for_each(|block| {
        blocks.push(block);
    });

    let mut values: HashMap<String, usize> = HashMap::new();
    blocks[0].lines().for_each(|line| {
        let (n, v) = line.split_once(": ").unwrap();
        values.insert(n.to_string(), v.parse::<usize>().unwrap());
    });
    // println!("values {:?}", values);

    let mut instr: Vec<Instruction> = Vec::new();
    blocks[1].lines().for_each(|line| {
        let chunks = line.split(' ').collect::<Vec<&str>>();
        let v1 = chunks[0].to_string();
        let v2 = chunks[2].to_string();
        let v3 = chunks[4].to_string();
        let op = match chunks[1] {
            "AND" => Instr::And,
            "OR" => Instr::Or,
            "XOR" => Instr::Xor,
            _ => panic!(),
        };
        instr.push(Instruction {
            v1,
            v2,
            v3,
            instr: op,
            done: false,
        });
    });

    let mut nbr_update = 0;
    let mut nbr_notdone = instr.len();
    loop {
        for (index,a_i) in instr.iter_mut().enumerate() {
            if !a_i.done {
                if let Some(v1) = values.get(&a_i.v1) {
                    if let Some(v2) = values.get(&a_i.v2) {
                        let v3: String = a_i.v3.clone();
                        match a_i.instr {
                            Instr::And => {
                                // println!("line {index} sets {} to {}",v3.clone(),v1 & v2);
                                values.insert(v3.clone(), v1 & v2);
                            }
                            Instr::Or => {
                                // println!("line {index} sets {} to {}",v3.clone(),v1 | v2);
                                values.insert(v3, v1 | v2);
                            }
                            Instr::Xor => {
                                // println!("line {index} sets {} to {}",v3.clone(),v1 ^ v2);
                                values.insert(v3, v1 ^ v2);
                            }
                        };
                        a_i.done = true;
                        nbr_update += 1;
                        nbr_notdone -= 1;
                        // println!("values {:?}", values);
                    }
                }
            }
        }
        if nbr_notdone == 0 {
            // println!("all done !");
            break;
        }
        else if nbr_update == 0 {
            panic!("no more moves");
        } else {
            // println!("not done : {}",nbr_notdone);
        }
    }

    let mut answer = Vec::new();
    for i in 0..99 {
        let key = format!("z{:02}",i).as_str().to_string();
        if let Some(zx) = values.get_key_value(&key) {
            // print!("{},",zx.1);
            answer.push(zx.1);
        } else {
            break;
        }
    }
    // println!();

    //
    answer.reverse();
    Some(answer.iter().fold(0, |acc, &bit| (acc << 1) | bit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day24_input_demo1.txt")),
            Some(4)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day24_input_demo2.txt")),
            Some(2024)
        );
        assert_eq!(get_answer(include_str!("../assets/day24_input.txt")), Some(58639252480880));
    }
}
