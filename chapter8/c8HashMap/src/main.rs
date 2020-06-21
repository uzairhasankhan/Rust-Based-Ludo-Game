use std :: collections :: HashMap;
fn main() {
    // let mut map = HashMap :: new();
    // map.insert(String :: from ("Blue"),10);
    // map.insert(String :: from("Yellow"),50);
    let teams = vec! [String :: from ("Blue"),String :: from ("Yellow")];
    let initial_scores = vec! [10,50];

    let map: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}",map);
}
