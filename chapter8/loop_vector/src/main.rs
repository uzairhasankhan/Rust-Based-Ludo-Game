fn main() {
    let mut  v = vec![1,2,3,4,5];

    for values in &mut v {
        *values+= 50
    }

    println!("{:?}",v);
}
