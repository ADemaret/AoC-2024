use std::{
    collections::HashMap,
    time::Instant,
};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
    // let input = include_str!("../assets/day12_input_demo2.txt");
    // let input = include_str!("../assets/day12_input_demo3.txt");
    let input = include_str!("../assets/day12_input.txt");

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

    // HashMap<(l,c),(char,zone_id,perim)>
    let mut cells = HashMap::new();

    // tag the adjacents zones
    let mut zone_id = 0;
    for l in 0..grid.max_l {
        for c in 0..grid.max_c {
            // new region
            if !cells.contains_key(&(l, c)) {
                // println!(
                //     "new zone in {},{} : {} {}",
                //     l,
                //     c,
                //     grid.get_at((l, c)),
                //     zone_id
                // );
                tag_adjacent_cells(&grid, l, c, zone_id, &mut cells);
                zone_id += 1;
            }
        }
    }

    // get area and perimeter
    let mut areas: HashMap<u32, usize> = HashMap::new();
    let mut perim: HashMap<u32, usize> = HashMap::new();
    for cell in &cells {
        let v = areas.entry(cell.1 .1).or_insert(0);
        *v += 1;
        let v = perim.entry(cell.1 .1).or_insert(0);
        *v += cell.1 .2;
    }
    // println!("{:?}", areas);
    // println!("{:?}", perim);

    let mut result = 0;
    for i in 0..areas.len() {
        // println!(
        //     "Zone {} has area {} and perim {}",
        //     i,
        //     areas.get(&(i as u32)).unwrap(),
        //     perim.get(&(i as u32)).unwrap()
        // );
        result += areas.get(&(i as u32)).unwrap() * perim.get(&(i as u32)).unwrap();
    }
    Some(result)
}

fn tag_adjacent_cells(
    grid: &Grid2D,
    l: usize,
    c: usize,
    zone_id: u32,
    cells: &mut HashMap<(usize, usize), (char, u32, usize)>,
) {
    let adjacents = grid.get_adjacents_ortho(l, c);
    let perim = 4 - adjacents
        .iter()
        .filter(|&x| x.2 == grid.get_at((l, c)))
        .count();
    cells.insert((l, c), (grid.get_at((l, c)), zone_id, perim));

    for adj in adjacents {
        if !cells.contains_key(&(adj.0, adj.1)) && adj.2 == grid.get_at((l, c)) {
            tag_adjacent_cells(grid, adj.0, adj.1, zone_id, cells);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            Some(140)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo2.txt")),
            Some(772)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo3.txt")),
            Some(1930)
        );
        assert_eq!(get_answer(include_str!("../assets/day12_input.txt")), Some(1370100));
    }
}
