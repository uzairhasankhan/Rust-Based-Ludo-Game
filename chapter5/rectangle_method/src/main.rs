#[derive(Debug)]

struct Rectangle {
    height : u32,
    width  : u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.height * self.width
    } 
}

fn main() {
    let rect_1 = Rectangle {
        height : 25,
        width  : 10,
    };
    println!("{:#?}",rect_1.area());
}
