#[derive(Debug)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}
impl Direction { 
    fn print (&self) {
        println!("{:#?}",&self );
    }
}

fn direction_update (dir: Direction) {
    match dir {
        Direction :: Forward    => { println!("\n Forward");},
        Direction :: Backward   => { println!("Backward");},
        Direction :: Left       => { println!("Left");},
        Direction :: Right      => { println!("Right");},
       
    }
}



fn main() {
    
    let mut counter : f32 = 0.0;
    let direction = Direction :: Forward;
    
    
    
    let result = loop {
        
        direction.print();
        direction_update(Direction::Backward);
        direction_update(Direction::Left);
        direction_update(Direction::Right);    
        
        
        counter += 0.25;

        if counter == 0.75 {
            break }
    };

}
