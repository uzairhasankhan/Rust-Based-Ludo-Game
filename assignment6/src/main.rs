use :: std :: io;
use std::collections::HashMap;
use rand::Rng;
fn main() {
    
let player_no = player_no();
let total_player : HashMap< String , u32 > = player_name(player_no);
let dice = dice(total_player, player_no);






//println!("{:?}",secret_number);

    
}

fn player_no () -> u32 {
    
    let mut players = String :: new();
    println!("How many players will play the game?");

    io::stdin()
    .read_line(&mut players)
    .expect("Failed to read your input");

    let players: u32 = players.trim().parse().expect("Please type a number!");
    println!("{} players will play the game.",players);
    players
}

fn player_name (no_players: u32, ) -> HashMap<String, u32> {

    let mut number = 0;
    let mut name: HashMap<String, u32> = HashMap :: new ();
    println!("Please enter the name of players");

    while number < no_players {

      
    let dice_roll = rand::thread_rng().gen_range(0, 7);
    let mut player = String :: new();
    io::stdin()
    .read_line(&mut player)
    .expect("Failed to read your input");
    let player : String = player.trim().into();
    
    name.insert(player, dice_roll);
    number+=1;
    }
    name

    
    
}

fn dice (scores : HashMap<String , u32>, player_n0 : u32)  {
    let mut num = 0;
    let mut io = String :: new();
    while num < 10 {
        for (key, value) in &scores {
        
        println!("Dice Roll for {} is {}", key, value);
    }
    io::stdin()
    .read_line(&mut io)
    .expect("Failed to read your input");
    //crate :: player_name(player_n0);
    num+=1;
}

}
