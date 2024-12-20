use std::{collections::BTreeMap, time::Instant};

// personal functions
use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day20_input_demo1.txt");
    let input = include_str!("../assets/day20_input.txt");

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
    let grid = Grid2D::new(input);
    // grid.print();
    let start_node = grid.get_vec_of_char_positions('S')[0];
    let end_node = grid.get_vec_of_char_positions('E')[0];

    let path = dijkstra(&grid, start_node, end_node);
    // grid.print_with_vec(&path, 'O');

    // path
    let mut result = 0;
    for (i1, p1) in path.iter().enumerate() {
        // needs 3 steps separation else result is path
        if i1 <= path.len() - 3 {
            let offset = i1 + 3;
            for (i2, p2) in path[offset..].iter().enumerate() {
                if (p1.0.abs_diff(p2.0) == 2 && p1.1 == p2.1)
                    || (p1.1.abs_diff(p2.1) == 2 && p1.0 == p2.0)
                {
                    let win = (i2 + offset).abs_diff(i1) - 2;
                    // println!(
                    //     "en faisant sauter le mur entre {:?},{i1} et {:?},{i2}, on gagne {}",
                    //     p1, p2, win
                    // );
                    if win >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }

    Some(result)
}

fn dijkstra(
    graph: &Grid2D,
    start_node: (usize, usize),
    end_node: (usize, usize), //dejavu: &mut Vec<bool>,
) -> Vec<(usize, usize)> {
    // on va obtenir une liste de noeuds et leur distance par rapport au point de départ
    let mut ans: BTreeMap<(usize, usize), ((usize, usize), usize)> = BTreeMap::new();
    // prec est la liste des points précédents
    let mut prec: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    // le point de départ
    ans.insert(start_node, (start_node, 0));

    // les points liés au point de départ
    for dir in [(0, 1), (1, 0), (0, -1_isize), (-1_isize, 0)] {
        let l2 = start_node.0 as isize + dir.0;
        let c2 = start_node.1 as isize + dir.1;
        if l2 >= 0
            && c2 >= 0
            && l2 < graph.max_l as isize
            && c2 < graph.max_c as isize
            && graph.get_at((l2 as usize, c2 as usize)) != '#'
        {
            let dist = 1;
            ans.insert((l2 as usize, c2 as usize), (start_node, 0));
            prec.insert((l2 as usize, c2 as usize), dist);
        }
    }

    // node -(d)-> new_node
    while !prec.is_empty() {
        let (node, path_dist) = prec.pop_first().unwrap();

        for dir in [(0, 1), (1, 0), (0, -1_isize), (-1_isize, 0)] {
            let l2 = node.0 as isize + dir.0;
            let c2 = node.1 as isize + dir.1;
            if l2 >= 0
                && c2 >= 0
                && l2 < graph.max_l as isize
                && c2 < graph.max_c as isize
                && graph.get_at((l2 as usize, c2 as usize)) != '#'
            {
                let dist = 1;

                let new_node = (l2 as usize, c2 as usize);
                let new_dist = path_dist + dist; // le chemin pour aller du départ à new_node en passant par node
                if ans.get(&new_node).is_none() || new_dist < ans.get(&new_node).unwrap().1 {
                    // si c'est plus court que pour aller à new_node par "l'ancien" chemin
                    ans.insert(new_node, (node, new_dist));
                    prec.insert(new_node, new_dist);
                }
            }
        }
    }

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut node = end_node;
    path.push(node);
    while node != start_node {
        (node, _) = *ans.get(&node).unwrap();
        path.insert(0, node);
    }
    // println!("chemin le plus court : {:?}", path);
    // graph.print_with_vec(&path, '*');

    //let (_, dist) = ans.get(&end_node).unwrap();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day20_input_demo1.txt")),
        //     None
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input.txt")),
            Some(1502)
        );
    }
}
