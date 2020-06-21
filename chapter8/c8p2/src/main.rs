fn main() {
    let mut v: Vec<i32> =Vec :: new();
    v.push(10);

    let element  = v[0];

    println!("{:?}",v);
    v.push(20);
    println!("{:?}",v);

    
    let element1 = v[1];

    println!("{}",element);
    println!("{}",element1);

    println!("{:?}",v);

    let element2 = v.get(1);
    println!("{:?}",element2);

    match element2 {
        Some(value) => println!("{}",value),
        
        None        => println!("Nothing"),
        }
}



