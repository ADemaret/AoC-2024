use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 21 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day21_input_demo1.txt");
    // let input = include_str!("../assets/day21_input.txt");

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
    let mut memo: HashMap<Vec<char>, usize> = HashMap::new();

    let result = input
        .lines()
        .map(|line| {
            let code = line.chars().collect::<Vec<_>>();
            let len = get_moves(&mut memo, &code);
            let num = line
                .chars()
                .take(3)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            // println!("line {}, code {:?}, len {}", line, code, len);
            // println!("memo size is {}",memo.len());
            num * len
        })
        .sum::<usize>();

    Some(result)
}

fn get_moves(memo: &mut HashMap<Vec<char>, usize>, code: &Vec<char>) -> usize {
    let numpad = set_numpad();
    let dirpad = set_dirpad();

    let mut length = Vec::new();
    let mut all_paths = get_num_paths(&numpad, code);
    // println!("{:?}", all_paths);
    for ap in &all_paths {
        let mut sublen = 0;
        let mut ll = 0;
        // cut string
        let string: String = ap.into_iter().collect();
        let buff = string.split('A').collect::<Vec<&str>>();
        // println!("buf = {:?}", buff);
        for buff2 in buff {
            let mut x = buff2.chars().collect::<Vec<_>>();
            x.push('A');
            let mut dummy = vec![x.clone()];
            // println!("before {:?}", dummy);
            if let Some(x2) = memo.get(&x.clone()) {
                ll = *x2;
            } else {
                for _ in 0..2 {
                    get_dir_paths(&dirpad, &mut dummy);
                }
                ll = dummy.iter().map(|v| v.len()).min().unwrap();
                memo.insert(x, ll);
            }
            // println!("after {:?}", ll);
            sublen += ll;
        }
        // println!("** sum is {}",sublen-1);
        length.push(sublen - 1);
        sublen = 0;
    }
    let l = length.iter().min().unwrap();
    // println!("min = {:?}", l);
    *l
}

fn get_dir_paths(numpad: &Grid2D, code: &mut Vec<Vec<char>>) {
    let mut result = Vec::new();
    //result.push(Vec::new());

    for a_code in code.clone() {
        let mut all_paths = Vec::new();
        all_paths.push(Vec::new());
        let mut prev = 'A';

        for dest in a_code {
            let prev_all_path = all_paths.clone();
            let pos = get_paths(numpad, prev, dest);
            for ap in all_paths.iter_mut() {
                ap.push(pos[0].clone());
            }
            if pos.len() > 1 {
                for i in 1..pos.len() {
                    let mut new_paths = prev_all_path.clone();
                    for ap in new_paths.iter_mut() {
                        ap.push(pos[i].clone());
                        all_paths.push(ap.clone());
                    }
                }
            }
            for ap in all_paths.iter_mut() {
                ap.push(vec!['A']);
                prev = dest;
            }
        }
        // flatten
        for ap in all_paths {
            let zz = ap.into_iter().flatten().collect::<Vec<char>>();
            result.push(zz);
        }
        // numpadseq
    }
    *code = result.clone();
}

fn get_num_paths(numpad: &Grid2D, code: &Vec<char>) -> Vec<Vec<char>> {
    let mut all_paths = Vec::new();
    all_paths.push(Vec::new());
    let mut prev = 'A';

    for dest in code {
        let prev_all_path = all_paths.clone();
        let pos = get_paths(&numpad, prev, *dest);
        for ap in all_paths.iter_mut() {
            ap.push(pos[0].clone());
        }
        if pos.len() > 1 {
            for i in 1..pos.len() {
                let mut new_paths = prev_all_path.clone();
                for ap in new_paths.iter_mut() {
                    ap.push(pos[i].clone());
                    all_paths.push(ap.clone());
                }
            }
        }
        for ap in all_paths.iter_mut() {
            ap.push(vec!['A']);
            prev = *dest;
        }
    }
    // flatten
    let mut numpadseq: Vec<Vec<char>> = Vec::new();
    for ap in all_paths {
        let zz = ap.into_iter().flatten().collect::<Vec<char>>();
        numpadseq.push(zz);
    }
    numpadseq
}

fn get_paths(numpad: &Grid2D, from: char, to: char) -> Vec<Vec<char>> {
    // let mut all_results = Vec::new();

    // get one possibility
    let mut result = Vec::new();
    if let Some((l1, c1)) = numpad.get_char_position(from) {
        if let Some((l2, c2)) = numpad.get_char_position(to) {
            // if l1 < l2 {
            for _ in l1..l2 {
                result.push('v');
            }
            // } else if l2 < l1 {
            for _ in l2..l1 {
                result.push('^');
            }
            // }
            // if c1 < c2 {
            for _ in c1..c2 {
                result.push('>');
            }
            // } else if c2 < c1 {
            for _ in c2..c1 {
                result.push('<');
            }
            // }
        }
    }

    // get permutations
    let len = result.len();
    let mut perm = result
        .into_iter()
        .permutations(len)
        .unique()
        .collect::<Vec<_>>();

    // remove perm that goes on '#'
    for i in (0..perm.len()).rev() {
        if !path_is_allowed(numpad, from, perm[i].clone()) {
            perm.remove(i);
        }
    }
    perm
}

fn path_is_allowed(pad: &Grid2D, from: char, path: Vec<char>) -> bool {
    if let Some((l1, c1)) = pad.get_char_position(from) {
        let mut current = (l1, c1);
        for step in &path {
            match step {
                '>' => {
                    current = (current.0, current.1 + 1);
                }
                'v' => {
                    current = (current.0 + 1, current.1);
                }
                '<' => {
                    current = (current.0, current.1.saturating_sub(1));
                }
                '^' => {
                    current = (current.0.saturating_sub(1), current.1);
                }
                _ => {}
            }
            if pad.get_at(current) == '#' {
                // println!("** removing path {:?} from {from}",path);
                return false;
            }
        }
    }
    true
}

fn set_numpad() -> Grid2D {
    let mut numpad = Grid2D::new_empty(4, 3, '#');
    numpad.set_at((0, 0), '7');
    numpad.set_at((0, 1), '8');
    numpad.set_at((0, 2), '9');
    numpad.set_at((1, 0), '4');
    numpad.set_at((1, 1), '5');
    numpad.set_at((1, 2), '6');
    numpad.set_at((2, 0), '1');
    numpad.set_at((2, 1), '2');
    numpad.set_at((2, 2), '3');
    // numpad.set_at((3,0), '#');
    numpad.set_at((3, 1), '0');
    numpad.set_at((3, 2), 'A');
    // numpad.print();
    numpad
}

fn set_dirpad() -> Grid2D {
    let mut dirpad = Grid2D::new_empty(2, 3, '#');
    //dirpad.set_at((0,0), '#');
    dirpad.set_at((0, 1), '^');
    dirpad.set_at((0, 2), 'A');
    dirpad.set_at((1, 0), '<');
    dirpad.set_at((1, 1), 'v');
    dirpad.set_at((1, 2), '>');
    dirpad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day21_input_demo1.txt")),
            Some(126384)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day21_input.txt")),
            Some(224326)
        );
    }
}
