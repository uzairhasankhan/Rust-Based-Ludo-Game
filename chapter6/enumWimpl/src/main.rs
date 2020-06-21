fn main() {
    let m = Message :: Write(String :: from ("Assalam u Alaikum"));
    m.msg();
}

#[derive(Debug)]
enum Message {
    Quit(u32),
    Write(String),
    Move{x : i32, y : i32},
    ChangColour(i32,i32,i32)
}

impl Message {
    fn msg (&self) {
        println!("{:#?}",&self);
    }
}