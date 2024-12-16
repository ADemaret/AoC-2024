use std::collections::BTreeMap;
use std::fmt::{self};
use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
// use crate::utils::pause;
//use crate::utils::math;

const DEBUG: bool = true;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day16_input_demo1.txt");
    let input = include_str!("../assets/day16_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq)]
struct PtAndDir {
    pt: (usize, usize),
    dir: (isize, isize),
}
impl fmt::Display for PtAndDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self.dir {
            (0, 1) => ">",
            (0, -1) => "<",
            (-1, 0) => "^",
            (1, 0) => "v",
            _ => panic!(),
        };
        write!(f, "({},{}) {}", self.pt.0, self.pt.1, dir)
    }
}
impl fmt::Debug for PtAndDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self.dir {
            (0, 1) => ">",
            (0, -1) => "<",
            (-1, 0) => "^",
            (1, 0) => "v",
            _ => panic!(),
        };
        write!(f, "({},{}) {}", self.pt.0, self.pt.1, dir)
    }
}

fn get_answer(input: &str) -> Option<usize> {
    let mut grid = Grid2D::new(input);
    // grid.print();

    let mut start_node = PtAndDir {
        pt: grid.get_vec_of_char_positions('S')[0],
        dir: (0, 0),
    };
    let end_node = grid.get_vec_of_char_positions('E')[0];
    grid.set_at(end_node, '.');
    Some(dijkstra(&grid, &mut start_node, end_node))
}

fn dijkstra(grid: &Grid2D, start_node: &mut PtAndDir, end_node: (usize, usize)) -> usize {
    // on va obtenir une liste de noeuds et leur distance par rapport au point de départ
    let mut ans: BTreeMap<PtAndDir, (PtAndDir, usize)> = BTreeMap::new();
    // prec est la liste des points précédents
    let mut prec: BTreeMap<PtAndDir, usize> = BTreeMap::new();

    // les points liés au point de départ
    start_node.dir = (0,1);
    ans.insert(*start_node, (*start_node, 0));
    prec.insert(*start_node,0);
    start_node.dir = (-1,0);
    ans.insert(*start_node, (*start_node, 1000));
    prec.insert(*start_node,1000);

    // println!("ans for start nodes =");
    // for zz in ans.clone() {
    //     println!("{:?}", zz);
    // }
    // println!("--");

    // node -(d)-> new_node
    while !prec.is_empty() {
        let (node, path_dist) = prec.pop_first().unwrap();
        //if node.pt != end_node {
            // des 4 points qu'on peut atteindre depuis node...
            for dir in [(0, 1), (1, 0), (0, -1_isize), (-1_isize, 0)] {
                let l2 = node.pt.0 as isize + dir.0;
                let c2 = node.pt.1 as isize + dir.1;
                // si on n'est pas dehors
                if l2 >= 0
                    && c2 >= 0
                    && l2 < grid.max_l as isize
                    && c2 < grid.max_c as isize
                    && grid.get_at((l2 as usize, c2 as usize)) == '.'
                {
                    // nd = nouvelle direction
                    // orientation = comment on était en arrivant ici
                    let nd = match dir {
                        (0, 1) => 0,
                        (1, 0) => 1,
                        (0, -1_isize) => 2,
                        (-1_isize, 0) => 3,
                        _ => panic!(),
                    };
                    // original direction
                    let or = match node.dir {
                        (0, 1) => 0,
                        (1, 0) => 1,
                        (0, -1_isize) => 2,
                        (-1_isize, 0) => 3,
                        _ => panic!(),
                    };
                    let dist;
                    if nd == or { dist = 1;}                    
                    else if [(0,2),(2,0),(1,3),(3,1)].contains(&(nd,or)) { dist = 2001;}
                    else {dist = 1001;}
                    
                    // new-node = pt d'arrivée
                    let new_node = PtAndDir {
                        pt: (l2 as usize, c2 as usize),
                        dir,
                    };
                    let new_dist = path_dist + dist; // le chemin pour aller du départ à new_node en passant par node

                    // est-ce qu'on s'est déjà retrouvé à ce point là dans cette direction là ?
                    if let Some(previous) = ans.get(&new_node) {
                                                
                            let old_dist = previous.1;
                            if new_dist < old_dist {
                                // meilleur moyen d'arriver à ans
                                ans.insert(new_node, (node, new_dist));
                                prec.insert(new_node, new_dist);
                                if DEBUG {
                                    // println!(
                                    //     "meilleure dist pour arriver à {} = {}",
                                    //     new_node,
                                    //     new_dist
                                    // );
                                }
                            }                        
                    }
                    else {
                        // nouveau moyen d'arriver à ans
                        ans.insert(new_node, (node, new_dist));
                        prec.insert(new_node, new_dist);
                        // println!(
                        //     "on peut arriver à {:?} en venant de {:?} à un coût de {}",
                        //     new_node, node, new_dist
                        // );
                    } 
                }    
            }
        //}
    }
    if DEBUG {
        // for a in &ans {
        //     println!("{:?}", a);
        // }
    }
    // if let Some(test) = ans.get(&start_node) {
    //     println!("test {:?}",test);
    //     pause::pause();
    // }

    // for zz in ans.clone() {
    //     println!("zz {:?}", zz);
    // }

    let mut dists = Vec::new();
    for dir in [(0, 1), (1, 0), (0, -1_isize), (-1_isize, 0)] {
        let node = PtAndDir { pt: end_node, dir };
        if let Some(result) = ans.get(&node) {
            dists.push(result.1);
        }
    }
    println!("dists = {:?}", dists);
    *dists.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day16_input_demo1.txt")),
            Some(7036)
        );
        assert_eq!(get_answer(include_str!("../assets/day16_input.txt")), Some(83432));
    }
}
