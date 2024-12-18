use std::collections::BTreeMap;
use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
use crate::utils::pause;
//use crate::utils::math;

const DEBUG: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day18_input_demo1.txt");
    let input = include_str!("../assets/day18_input.txt");

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
    let data = input
        .lines()
        .map(|line| {
            let v = line
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            (v[1], v[0])
        })
        .take(1024)
        .collect::<Vec<_>>();

    let mut grid = Grid2D::new_empty(71, 71, '.');
    for d in data {
        grid.set_at(d, '#');
    }
    // grid.print();

    //grid.print_with_vec(&data, '#');

    Some(dijkstra(&grid, (0, 0), (grid.max_l-1, grid.max_c-1)))
}

fn dijkstra(
    graph: &Grid2D,
    start_node: (usize, usize),
    end_node: (usize, usize), //dejavu: &mut Vec<bool>,
) -> usize {
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
            if DEBUG {
                println!(
                    "on ajoute le noeud {},{}, voisin de {:?} à la distance {dist}",
                    l2, c2, start_node
                );
            }
        }
    }

    // node -(d)-> new_node
    while !prec.is_empty() {
        let (node, path_dist) = prec.pop_first().unwrap();
        if DEBUG {
            println!(
                "noeud voisin de {:?} (distance jusque là = {path_dist}) :",
                node
            );
        }
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
                if DEBUG {
                    println!(" {},{} à +{dist}", l2, c2);
                }
                let new_node = (l2 as usize, c2 as usize);
                let new_dist = path_dist + dist; // le chemin pour aller du départ à new_node en passant par node
                if ans.get(&new_node).is_none() || new_dist < ans.get(&new_node).unwrap().1 {
                    // si c'est plus court que pour aller à new_node par "l'ancien" chemin
                    ans.insert(new_node, (node, new_dist));
                    prec.insert(new_node, new_dist);
                    if DEBUG {
                        println!(
                            "on ajoute la distance totale de start à {:?} = {new_dist}",
                            new_node
                        );
                    }
                } else if DEBUG {
                    println!("on ne l'ajoute pas");
                }
            }
        }
    }
    if DEBUG {
        for a in &ans {
            println!("{:?}", a);
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

    let (_, dist) = ans.get(&end_node).unwrap();
    *dist
}

