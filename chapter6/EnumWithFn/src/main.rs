fn main() {
    let result = Coins :: Quarter (USstates :: Abalama);
    let result12 = Coins :: Quarter (USstates :: Alaska);
    let result1 = value_in_cents(result12);
    
    println!("{:#?}",result1);
}

#[derive(Debug)]
 enum Coins {
     Penny,
     Nickel,
     Dime,
     Quarter(USstates),
}
#[derive(Debug)]
enum USstates {
    Abalama,
    Alaska,
}

fn value_in_cents (coin:Coins) -> u32 {
    match coin {
        Coins :: Penny => 1,
        Coins :: Nickel => 5,
        Coins :: Dime  => 10,
        Coins :: Quarter (state) => {
            println!("{:#?}",state);
            25
        },
    }
}