fn main() {

    let v = vec![SpreadSheet :: Integer(10),SpreadSheet :: Font(2.3),SpreadSheet :: Text(String :: from ("Allah u Akbar"))];

    println!("{:?}",v);
    
}
#[derive(Debug)]
enum SpreadSheet {
    Integer(i32),
    Font(f64),
    Text(String),
}
