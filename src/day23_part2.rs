use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

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
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (l1, l2) = line.split_once("-").unwrap();
        if !tree.contains_key(l1) {
            tree.insert(l1, vec![l2]);
        } else {
            let mut links = tree.get(&l1).unwrap().clone();
            links.push(l2);
            tree.insert(l1, links);
        }
        if !tree.contains_key(l2) {
            tree.insert(l2, vec![l1]);
        } else {
            let mut links = tree.get(&l2).unwrap().clone();
            links.push(l1);
            tree.insert(l2, links);
        }
    });

    // for t in tree {
    //     println!("{:?}",t);
    // }

    let mut sol = Vec::new();
    let mut found = false;
    'out: for t in &tree {
        let size = t.1.len();
        let mut disc_set = HashMap::new();
        let candidates = t.1;
        // check connections between a node and the following in the list
        for i in 0..size {
            let c2 = tree.get(candidates[i]).unwrap();
            for j in i + 1..size {
                if !c2.contains(&candidates[j]) {
                    // println!("{} and {} are not linked", candidates[i], candidates[j]);
                    *disc_set.entry(candidates[i]).or_insert(0) += 1;
                    *disc_set.entry(candidates[j]).or_insert(0) += 1;
                    // if more than one node is not connected, it is not the searched clique
                    if disc_set.iter().filter(|&(_, v)| *v > 1).count() > 1 {
                        continue 'out;
                    } else {
                        found = true;
                    }
                }
            }
        }
        // println!("{:?}", t);
        // println!("{:?}", disc_set);
        // println!("Solution found");

        if found {
            sol = t.1.clone();
            sol.push(t.0);
            let refused = disc_set.iter().filter(|&(_, v)| *v > 1).collect::<Vec<_>>()[0];
            let zz = sol.iter().position(|&x| x == *refused.0).unwrap();
            sol.remove(zz);
            sol.sort();
            // println!("{:?}", sol.join(","));
            break;
        }
    }

    if found {
        let str = sol.join(",").to_string();
        return Some(str);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt")),
            Some("co,de,ka,ta".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt")),
            Some("bs,ey,fq,fy,he,ii,lh,ol,tc,uu,wl,xq,xv".to_string())
        );
    }
}
