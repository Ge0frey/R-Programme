use std::io;

//std is the name of the crate(packages) while io is the name  of the module being imported from the std library;

fn main () {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{}", input);
}
