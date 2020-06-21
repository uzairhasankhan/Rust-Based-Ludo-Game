fn main() {
    let (x,y,z) = square(10,5,3.9);
    println!("{},\n{},\n{}",x,y,z);
}

fn square(x : i32, y : i32, z : f64) -> (i32,i32,f64) {
    let resultx = x * x;
    let resulty = y * y;
    let resultz = z * z;
    (resultx,resulty,resultz)
    
}

