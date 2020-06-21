#[derive(Debug)]

struct Rectangle {
    height : u32,
    width  : u32,
}

fn main() {
    let result = Rectangle { 
        height : 30, 
        width : 30
    };
    
    println!("The are of Rectangle is {:#?}",area(&result));
    //println!("{:#?}",result);
} 

fn area (rectangle : &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
