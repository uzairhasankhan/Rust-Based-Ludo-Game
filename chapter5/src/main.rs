#[derive(Debug)]

struct Child {

    name   : String,
    age    : u8,
    dob    : String,
    bgroup : String,

}

fn main() {
    let child = child1(String :: from("Abeeha Khan"), 3, String :: from ("19Dec2916"),String :: from ("AB"));
    println!("{:#?}",child);
}

fn child1 (name:String,age:u8,dob:String,bgroup:String) -> Child {
    Child {
        name,
        age,
        dob,
        bgroup,
    }
}
