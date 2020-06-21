fn main() {
    let mut v = vec![10,20,30,40];
    println!("{:?}",v);
    
    v.push(50);
    println!("{:?}",v);

    v.pop();
    println!("{:?}",v);

    let mut v1 : Vec <i32>  = Vec :: new();
    println!("{:?}",v1);
    v1.push(100);
    println!("{:?}",v1)


}
