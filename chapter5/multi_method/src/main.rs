use std :: io;
#[derive(Debug)]
struct Rectangle {
    height : u32,
    width  : u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.height * self.width

    }


    fn can_hold (&self, other : &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn new_entry (width : u32, height : u32) -> Rectangle {
        Rectangle {width : width , height : height }        
    }    
}


fn main() {
    //let rect_1 = Rectangle {height : 100, width  : 50,};
    //let rect_2 = Rectangle {height : 90,  width  : 30,};
    //let rect_3 = Rectangle {height : 70,  width  : 20,};

    println!("PLease enter height & width of rectangle:");
    let (input1,input2) = user_input();
    

    // println!("\nPLease enter width of rectangle:");
    // let input2 = user_input2();
    

    let rect_4 = Rectangle :: new_entry (input1,input2);
    println!("\nThe area of rectangle is {:#?}",rect_4.area());

    //println!("{:#?}",rect_4);
    
    //println!("Can rect_1 hold rect_2 {:#?}",rect_1.can_hold(&rect_2));
    //println!("\nCan rect_1 hold rect_3 {:#?}",rect_1.can_hold(&rect_3));

    //println!("\nArea of rect_1 is : {:#?}",rect_1.area());
    //println!("\nArea of rect_2 is : {:#?}",rect_2.area());
    //println!("\nArea of rect_3 is : {:#?}",rect_3.area());

}

fn user_input () -> (u32,u32) {
    let mut height = String :: new();
    io :: stdin().read_line(&mut height)
        .expect("Fail to read line");
        
    let mut height_integer : u32 = height.trim().parse()
        .expect("PLease enter the value");    
println!("You enter {:#?} as height",height_integer);

    let mut width = String :: new();
        io :: stdin().read_line(&mut width)
            .expect("Fail to read line");
            
    let mut width_integer : u32 = width.trim().parse()
            .expect("PLease enter the value");

println!("You enter {:#?} as width",width_integer);
            (height_integer,width_integer)
        

}
// fn user_input2 () -> u32 {
//     let mut width = String :: new();
//     io :: stdin().read_line(&mut width)
//         .expect("Fail to read line");
        
//     let mut width_integer : u32 = width.trim().parse()
//         .expect("PLease enter the value");    
//         width_integer

// }