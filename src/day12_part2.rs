use std::{collections::HashMap, time::Instant};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
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

    // remove prolonging perimeters
    // println!("cells :");
    // for cell in &cells {
    //     println!("{:?}", cell);
    // }
    for l in 0..grid.max_l {
        for c in 0..grid.max_c {
            let cell1 = *cells.get(&(l, c)).unwrap();
            let mut new_perim = cell1.2;
            if c < grid.max_c - 1 {
                let cell_east = *cells.get(&(l, c + 1)).unwrap();
                // same zone and both have north perim
                if cell1.0 == cell_east.0 && cell1.2[0] && cell_east.2[0] {
                    new_perim[0] = false;
                }
                // same zone and both have south perim
                if cell1.0 == cell_east.0 && cell1.2[1] && cell_east.2[1] {
                    new_perim[1] = false;
                }
            }
            if l < grid.max_l - 1 {
                let cell_south = *cells.get(&(l + 1, c)).unwrap();
                // same zone and both have west perim
                if cell1.0 == cell_south.0 && cell1.2[2] && cell_south.2[2] {
                    new_perim[2] = false;
                }
                // same zone and both have east perim
                if cell1.0 == cell_south.0 && cell1.2[3] && cell_south.2[3] {
                    new_perim[3] = false;
                }
            }
            cells.insert((l, c), (cell1.0, cell1.1, new_perim));
        }
    }
    // println!("cells after purge :");
    // for cell in &cells {
    //     println!("{:?}", cell);
    // }

    // get area and perimeter
    let mut areas: HashMap<u32, usize> = HashMap::new();
    let mut perim: HashMap<u32, usize> = HashMap::new();
    for cell in &cells {
        let v = areas.entry(cell.1 .1).or_insert(0);
        *v += 1;
        let v = perim.entry(cell.1 .1).or_insert(0);
        *v += cell.1 .2.iter().filter(|&&x| x).count();
    }

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
    cells: &mut HashMap<(usize, usize), (char, u32, [bool; 4])>,
) {
    let adjacents = grid.get_adjacents_ortho(l, c);
    let symbol = grid.get_at((l, c));
    let mut perim: [bool; 4] = [false, false, false, false];
    // north
    perim[0] = l == 0 || !(l > 0 && adjacents.contains(&(l - 1, c, symbol)));
    // south
    perim[1] = !adjacents.contains(&(l + 1, c, symbol));
    // west
    perim[2] = c == 0 || !(c > 0 && adjacents.contains(&(l, c - 1, symbol)));
    // east
    perim[3] = !adjacents.contains(&(l, c + 1, symbol));

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
            Some(80)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo2.txt")),
            Some(436)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo4.txt")),
            Some(236)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo5.txt")),
            Some(368)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo3.txt")),
            Some(1206)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt")),
            Some(818286)
        );
    }
}
