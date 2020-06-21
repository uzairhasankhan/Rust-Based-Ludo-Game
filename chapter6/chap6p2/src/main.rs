fn main() {
    let ip_1 = IPAddressType :: V4 (String :: from("127.0.0.1"));
    let ip_2 = IPAddressType :: V6 (String :: from("::1"));

    println!("{:#?}",ip_1);
    println!("{:#?}",ip_2);
}
#[derive(Debug)]
enum IPAddressType {
    V4 (String),
    V6 (String)
}
