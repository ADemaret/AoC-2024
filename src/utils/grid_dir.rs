use crate::utils::grid2d::Grid2D;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct GridDir {
    pub current: Direction,
}

impl GridDir {
    pub fn new(dir:&str) -> GridDir {
        match dir {
            "Up" => GridDir { current:Direction::Up},
            "Down" => GridDir { current:Direction::Down},
            "Left" => GridDir { current:Direction::Left},
            "Right" => GridDir { current:Direction::Right},
            _ => panic!("Invalid direction")
        }
    }
    pub fn invert_dir(&mut self) {
        match &self.current {
            Direction::Up => self.current = Direction::Down,
            Direction::Down => self.current = Direction::Up,
            Direction::Left => self.current = Direction::Right,
            Direction::Right => self.current = Direction::Left,
        }
    }

    pub fn move_right(&mut self) {
        match &self.current {
            Direction::Up => self.current = Direction::Right,
            Direction::Right => self.current = Direction::Down,
            Direction::Down => self.current = Direction::Left,
            Direction::Left => self.current = Direction::Up,
        }
    }

    pub fn move_point(&self, grid: &Grid2D, pt: (usize, usize)) -> Option<(usize, usize)> {
        // test if move is out of bound
        match &self.current {
            Direction::Up => {
                if pt.0 == 0 {
                    None
                } else {Some((pt.0-1,pt.1))}
            }
            Direction::Down => {
                if pt.0 == grid.max_l - 1 {
                    None
                } else {Some((pt.0+1,pt.1))}
            }
            Direction::Left => {
                if pt.1 == 0 {
                    None
                } else {Some((pt.0,pt.1-1))}
            }
            
            Direction::Right => {
                if pt.1 == grid.max_c - 1 {
                    None
                } else {Some((pt.0,pt.1+1))}
            }
        }
    }
}
