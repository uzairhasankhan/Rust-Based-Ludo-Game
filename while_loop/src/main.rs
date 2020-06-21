fn main() {
    let mut child = 0;

    while child < 5 {
        println!("Mubarak");
        child = child + 1;
    }

    println!("\n");
    let mut sibling =0;
    let sibling_name = ["uzair","umair", "sumaieya","tuba","taha", "muhiba"];

    while sibling < sibling_name.len() {
        println!("{}", sibling_name[sibling]);
        sibling =sibling+1;
    }

}
