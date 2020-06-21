use std :: io;
#[derive(Debug)]

struct Rectangle {
    height: u32,
    width : u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.height * self.width
    }
    
    fn can_hold (&self, others : &Rectangle) -> bool {
        self.height > others.height && self.width > others.width    
    }

    fn new_entry (height : u32, width : u32) -> Rectangle {
        Rectangle {
            height : height,
            width  : width,
        }
}
}

fn main() {
    let rect_1 = Rectangle { height : 5,   width : 5};

    println!("Please enter height");

    let (input_1,input_2) = input(); 
    let rect_2 = Rectangle :: new_entry(input_1,input_2);
    let area = rect_2.area();

    println!("\nThe area of rectangle is {}",area);

    let check = rect_1.can_hold(&rect_2);
    println!("\nrect_1 can hold rect_2: {}",check);


}

fn input () -> (u32,u32) {
    let mut input_1 = String :: new();
    
    io :: stdin().read_line(&mut input_1)
        .expect("Fail to read line");

    let input_1_integer : u32 = input_1.trim().parse()
        .expect("Please enter number");
        println!("\nYou enter height as :{}",input_1_integer);

    
        println!("\nPlease enter width");
        let mut input_2 = String :: new();
    
    io :: stdin().read_line(&mut input_2)
            .expect("Fail to read line");
    
    let input_2_integer :u32 = input_2.trim().parse()
            .expect("Please enter number");
            println!("\nYou enter width as :{}",input_2_integer);

    (input_1_integer,input_2_integer)



}
