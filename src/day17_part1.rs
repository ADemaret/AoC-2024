use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day17_input_demo1.txt");
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

fn get_answer(input: &str) -> Option<String> {
    // parse
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
            prgs = line
                .split([' ', ','])
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
        }
    });

    let mut regs_org = regs.clone();
    let prgs_org = prgs.clone();
    println!("regs : {:?}", regs);
    println!("prgs : {:?}", prgs);
    let a = 0;
    let b = 1;
    let c = 2;

    // // test
    // // If register C contains 9, the program 2,6 would set register B to 1.
    // regs[c] = 9;
    // prgs = vec![2,6];
    // part1(&mut regs, &prgs);
    // assert_eq!(regs[b],1);
    // // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    // regs[a] = 10;
    // prgs = vec![5,0,5,1,5,4];
    // assert_eq!(part1(&mut regs, &prgs),Some("0,1,2".to_string()));    
    // // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    // regs[a] = 2024;
    // prgs = vec![0,1,5,4,3,0];
    // assert_eq!(part1(&mut regs, &prgs),Some("4,2,5,6,7,7,7,7,3,1,0".to_string()));
    // assert_eq!(regs[a],0);
    // // If register B contains 29, the program 1,7 would set register B to 26.
    // regs[b] = 29;
    // prgs = vec![1,7];
    // part1(&mut regs, &prgs);
    // assert_eq!(regs[b],26);
    // // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    // regs[b] = 2024;
    // regs[c] = 43690;
    // prgs = vec![4,0];
    // part1(&mut regs, &prgs);
    // assert_eq!(regs[b],44354);
    
    part1(&mut regs_org, &prgs_org)

}

fn part1(regs: &mut Vec<usize>, prgs: &Vec<usize>) -> Option<String> {
    let a = 0;
    let b = 1;
    let c = 2;

    let mut pter: usize = 0;
    //let mut todo = VecDeque::new();
    let mut opcode;
    let mut lit_operand;
    let mut comb_operand = 0;
    let mut do_not_jump = false;
    let mut out = String::new();
    loop {
        println!("regs : {:?}", regs);

        opcode = prgs[pter];
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
        println!("doing opcode {} to operand {}", opcode, lit_operand);

        match prgs[pter] {
            0 => {
                println!("-- adv");
                // The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                let num = regs[0];
                let den = 2_usize.pow(comb_operand as u32);
                regs[a] = num / den;
                println!("{}/{} => regs[a] = {}", num, den, regs[a]);
            }
            1 => {
                println!("-- bxl");
                // the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                print!("{}^{} => ", regs[b], lit_operand);
                regs[b] ^= lit_operand;
                println!("regs[b] = {}", regs[b]);
            }
            2 => {
                println!("-- bst");
                // calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                print!("{}%8 => ", comb_operand);
                regs[b] = comb_operand % 8;
                println!("regs[b] = {}", regs[b]);
            }
            3 => {
                println!("-- jnz");
                //does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if regs[a] != 0 {
                    println!("pter = {}", lit_operand);
                    pter = lit_operand;
                    do_not_jump = true;
                } else {
                    println!("regs[a] = 0 => skip")
                }
            }
            4 => {
                println!("-- bxc");
                // calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                print!("{}^{} => ", regs[b], regs[c]);
                regs[b] ^= regs[c];
                println!("regs[b] = {}", regs[b]);
            }
            5 => {
                println!("-- out");
                //calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                print!("{}%8 => out ", comb_operand);
                let z1 = format!("{}", comb_operand % 8)
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                print!(" => {} ", z1);
                out += z1.as_str();
                out += ",";
                println!("=> out = {}", out);
            }
            6 => {
                println!("-- bdv");
                // works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                let num = regs[a];
                let den = 2_usize.pow(comb_operand as u32);
                regs[b] = num / den;
                println!("{}/{} => regs[b] = {}", num, den, regs[b]);
            }
            7 => {
                println!("-- cdv");
                // works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                let num = regs[a];
                let den = 2_usize.pow(comb_operand as u32);
                regs[c] = num / den;
                println!("{}/{} => regs[c] = {}", num, den, regs[c]);
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

    out.pop(); // remove last ','
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day17_input_demo1.txt")),
            Some("4,6,3,5,6,3,5,2,1,0".to_string())
        );
        assert_eq!(get_answer(include_str!("../assets/day17_input.txt")), Some("7,1,5,2,4,0,7,6,1".to_string()));
    }
}
