use crate::utils::grid2d::Grid2D;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum GridDir {
    Up,
    Down,
    Left,
    Right,
}

impl GridDir {
    pub fn new(dir:&str) -> GridDir {
        match dir {
            "Up" => {GridDir::Up},
            "Down" => {GridDir::Down},
            "Left" => {GridDir::Left},
            "Right" => {GridDir::Right},
            _ => panic!("Invalid GridDir")
        }
    }
    pub fn invert_dir(&mut self) {
        match &self {
            GridDir::Up => *self = GridDir::Down,
            GridDir::Down => *self = GridDir::Up,
            GridDir::Left => *self = GridDir::Right,
            GridDir::Right => *self = GridDir::Left,
        }
    }

    pub fn move_right(&mut self) {
        match &self {
            GridDir::Up => *self = GridDir::Right,
            GridDir::Right => *self = GridDir::Down,
            GridDir::Down => *self = GridDir::Left,
            GridDir::Left => *self = GridDir::Up,
        }
    }

    pub fn move_point(&self, grid: &Grid2D, pt: (usize, usize)) -> Option<(usize, usize)> {
        // test if move is out of bound
        match &self {
            GridDir::Up => {
                if pt.0 == 0 {
                    None
                } else {Some((pt.0-1,pt.1))}
            }
            GridDir::Down => {
                if pt.0 == grid.max_l - 1 {
                    None
                } else {Some((pt.0+1,pt.1))}
            }
            GridDir::Left => {
                if pt.1 == 0 {
                    None
                } else {Some((pt.0,pt.1-1))}
            }
            
            GridDir::Right => {
                if pt.1 == grid.max_c - 1 {
                    None
                } else {Some((pt.0,pt.1+1))}
            }
        }
    }
}
