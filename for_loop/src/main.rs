// // 

// //If the enum is C-like (as in your example), then you can create a static array of each of the variants and return an iterator of references to them:

// use self::Direction::*;
// use std::slice::Iter;

// #[derive(Debug)]
// pub enum Direction {
//     Forward,
//     Left,
//     Backward,
//     Right,
// }

// impl Direction {
//     fn iterator() -> Iter<'static, Direction> {
//         static DIRECTIONS: [Direction; 4] = [Forward, Left, Backward, Right];
//         DIRECTIONS.iter()
//     }
// }


// fn main() {
//     for dir in Direction::iterator() {
//         println!("{:?}", dir);
//     }
// }

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self) -> Self {
        use Direction::*;
        match *self {
            North => South,
            South => East,
            East => West,
            West => North,
        }
    }
}

fn main() {
    use Direction::*;

    for i in &[North, South, East, West] {
        println!("{:?}",i.turn());
    }
}