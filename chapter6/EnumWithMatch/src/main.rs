fn main() {
    
    let mut counter =0;
    let mut dir_type = Direction :: Forward;

    
    
    //let result2 = update_dir(result1);
    //result2.print();    
    
    //let result3 = update_dir(result2);
    //result3.print();

    //let result4 = update_dir(result3);
    //result4.print();

    
    
    let result = loop {
        
    //dir_type.print();
    //println!("{:#?}",result4);

    let dir_type = update_dir (&dir_type); 
    dir_type.print(); 
    
    counter += 1;

        if counter == 10 {
            break }
    };

}

#[derive(Debug)]
enum Direction {
    Forward,
    Left,
    Backward,
    Right,

}

impl Direction { 
    fn print (&self) {
        println!("{:#?}",&self );
    }
}


fn update_dir (dir: &Direction) -> Direction {
    match dir {
        Direction :: Forward    => Direction::Left,
        Direction :: Left       => Direction::Backward,
        Direction :: Backward   => Direction::Right,
        Direction :: Right      => Direction::Forward,
    }
}
