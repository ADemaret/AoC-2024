use std::time::Instant;


// personal functions
// use crate::utils::grid2d::Grid2D;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day14_input_demo1.txt");
    let input = include_str!("../assets/day14_input.txt");

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
    let secondes = 100;
    // let grid_x = 11;
    // let grid_y = 7;
    let grid_x = 101;
    let grid_y = 103;

    // input
    let bots = input
        .lines()
        .map(|line| {
            line.split(['=', ',', ' '])
                .filter_map(|a| a.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // final position
    let mut finals = Vec::new();
    for bot in bots {
        let x =
            (((bot[0] + grid_x) % grid_x + ((bot[2] + grid_x) % grid_x) * secondes + 3 * grid_x)
                % grid_x) as usize;
        let y =
            (((bot[1] + grid_y) % grid_y + ((bot[3] + grid_y) % grid_y) * secondes + 3 * grid_y)
                % grid_y) as usize;
        finals.push((y, x));
    }

    // grid for debug
//         let grid = Grid2D::new("...........
// ...........
// ...........
// ...........
// ...........
// ...........
// ...........");
//             grid.print_with_vec(&finals,'X');

    // count in quadrant
    let x_mid = ((grid_x - 1) / 2) as usize;
    let y_mid = ((grid_y - 1) / 2) as usize;
    let mut result = [0, 0, 0, 0];

    // // println!("qu1 :");
    result[0] = quadrant(&finals, 0, x_mid - 1, 0, y_mid - 1);
    // // println!("qu2 : ");
    result[1] = quadrant(&finals, x_mid + 1, grid_x as usize, 0, y_mid - 1);
    // // println!("qu2 : ");
    result[2] = quadrant(
        &finals,
        x_mid + 1,
        grid_x as usize,
        y_mid + 1,
        grid_y as usize,
    );
    // // println!("qu4 : ");
    result[3] = quadrant(&finals, 0, x_mid - 1, y_mid + 1, grid_y as usize);

    // // println!("finals : {:?}", finals);
    // // println!("result : {:?}", result);

    Some(result.iter().product())
}

fn quadrant(
    finals: &Vec<(usize, usize)>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
) -> usize {
    // println!("{}<=x<={}, {}<=y<={}",x_min,x_max,y_min,y_max);
    let mut result = 0;
    for f in finals {
        if f.1 >= x_min && f.1 <= x_max && f.0 >= y_min && f.0 <= y_max {
            // println!("{:?}",f);
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day14_input_demo1.txt")),
        //     Some(12)
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day14_input.txt")),
            Some(219512160)
        );
    }
}
