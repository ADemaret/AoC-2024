use std::{collections::VecDeque, time::Instant};

// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    // println!("-- Advent of Code - Day 17 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day17_input_demo2.txt");
    let input = include_str!("../assets/day17_input.txt");

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
    // parse
    let mut prog_str = "";
    let mut regs = Vec::new();
    let mut prgs = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        if i < 3 {
            regs.push(
                line.split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>()[0],
            );
        }
        if i == 4 {
            (_, prog_str) = line.split_at(9);
            prgs = line
                .split([' ', ','])
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
        }
    });

    // println!("regs : {:b},{:b},{:b}", regs[0], regs[1], regs[2]);
    // println!("prgs : {:?}", prgs);

    Some(bfs(&mut regs,&prgs))
}

fn bfs(
    regs : &mut Vec<usize>,
    prgs: &Vec<usize>) -> usize {

    let mut regs_org = regs.clone();
    let prgs_org = prgs.clone();
    let mut queue = VecDeque::new();
    let offset = 3;

    let pos = prgs.len();
    queue.push_back((pos, 0));

    while let Some((pos, a_result)) = queue.pop_front() {
        // trouvé
        if pos == 0 {
            return a_result;
        }

        // MAIN LOOP
        let mut test;
        // println!("searching for pos {} = {} ", pos, prgs[pos]);
        //println!("test with A = {:b}",(1<<(offset*(15-pos)))+base);
        for i in 0..(1 << offset) {
            regs_org[0] = (a_result << offset) + i;
            regs_org[1] = 0;
            regs_org[2] = 0;
            test = regs_org[0];
            let prog2 = part1(&mut regs_org, &prgs_org);
            // println!("{:b} => {:?}", test, prog2);
            if compare_n_last(prgs, &prog2, 17 - pos) {
                // println!(" ** found with {:b} => {:?}", test, prog2);
                //let new_result = test;
                // pause::pause();
                queue.push_back((pos - 1, test));
            }
        }
    }
    0
}
// not found

fn compare_n_last(a: &[usize], b: &[usize], n: usize) -> bool {
    // println!(" Comparing first {n} elements of {:?} and {:?}", a, b);
    a.get(a.len() - n..) == b.get(b.len() - n..)
}

fn part1(regs: &mut [usize], prgs: &[usize]) -> Vec<usize> {
    let a = 0;
    let b = 1;
    let c = 2;

    let mut pter: usize = 0;
    //let mut todo = VecDeque::new();
    // let mut opcode;
    let mut lit_operand;
    let mut comb_operand = 0;
    let mut do_not_jump = false;
    let mut out = Vec::new();
    loop {
        // println!("regs : {:b},{:b},{:b}", regs[0],regs[1],regs[2]);

        // opcode = prgs[pter];
        match prgs[pter + 1] {
            0 => {
                lit_operand = 0;
                comb_operand = 0;
            }
            1 => {
                lit_operand = 1;
                comb_operand = 1;
            }
            2 => {
                lit_operand = 2;
                comb_operand = 2;
            }
            3 => {
                lit_operand = 3;
                comb_operand = 3
            }
            4 => {
                lit_operand = 4;
                comb_operand = regs[a];
            }
            5 => {
                lit_operand = 5;
                comb_operand = regs[b];
            }
            6 => {
                lit_operand = 6;
                comb_operand = regs[c];
            }
            7 => {
                lit_operand = 7; /*comb_operand = 0;*/
            }
            _ => panic!(),
        }
        // println!("doing opcode {} to operand {}", opcode, lit_operand);

        match prgs[pter] {
            0 => {
                // println!("-- adv");
                // The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                let num = regs[0];
                let den = 2_usize.pow(comb_operand as u32);
                regs[a] = num / den;
                // println!("A = A >> x => {}/{} => regs[a] = {}", num, den, regs[a]);
            }
            1 => {
                // println!("-- bxl");
                // the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                // print!("{}^{} => ", regs[b], lit_operand);
                regs[b] ^= lit_operand;
                // println!("regs[b] = {}", regs[b]);
            }
            2 => {
                // println!("-- bst");
                // calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                // print!("{}%8 => ", comb_operand);
                regs[b] = comb_operand % 8;
                // println!("regs[b] = {}", regs[b]);
            }
            3 => {
                // println!("-- jnz");
                //does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if regs[a] != 0 {
                    // println!("pter = {}", lit_operand);
                    pter = lit_operand;
                    do_not_jump = true;
                } else {
                    // println!("regs[a] = 0 => skip")
                }
            }
            4 => {
                // println!("-- bxc");
                // calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                // print!("{}^{} => ", regs[b], regs[c]);
                regs[b] ^= regs[c];
                // println!("regs[b] = {:b}", regs[b]);
            }
            5 => {
                // println!("-- out");
                //calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                // print!("{}%8 => out ", comb_operand);
                let z1 = comb_operand % 8;
                out.push(z1);
                // println!("=> out = {}", out);
                // pause::pause();
            }
            6 => {
                // println!("-- bdv");
                // works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                let num = regs[a];
                let den = 2_usize.pow(comb_operand as u32);
                regs[b] = num / den;
                // println!("{}/{} => regs[b] = {}", num, den, regs[b]);
            }
            7 => {
                // println!("-- cdv");
                // works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                let num = regs[a];
                let den = 2_usize.pow(comb_operand as u32);
                regs[c] = num / den;
                // println!("{}/{} => regs[c] = {}", num, den, regs[c]);
            }
            _ => panic!(),
        }

        if !do_not_jump {
            pter += 2;
        } else {
            do_not_jump = false;
        }

        if pter >= prgs.len() {
            break;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day17_input.txt")),
            Some(37222273957364)
        );
    }
}
