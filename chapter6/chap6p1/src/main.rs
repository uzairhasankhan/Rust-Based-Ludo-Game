fn main() {
    let ip_1 = IPAdressEntry {
        kind    : IPAdressKind :: V4,
        address : String :: from ("127.0.0.1") 
    };
    let ip_2 = IPAdressEntry{
        kind    : IPAdressKind :: V6,
        address : String :: from("::1")
    };

    println!("{:#?}",ip_1);
    println!("{:#?}",ip_2);
}
#[derive(Debug)]
enum IPAdressKind {
    V4,
    V6
}
#[derive(Debug)]
struct IPAdressEntry {
    kind    : IPAdressKind,
    address : String,
}

impl IPAdressEntry {
    fn entry (kind : IPAdressKind,address : String ) -> IPAdressEntry {
        IPAdressEntry {
        kind    : kind,
        address : address,
    }
}
}
