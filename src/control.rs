//conditions and control flow;
use std::io;
fn main () {
    let condition = 2f32 == 3.2;
    println!("{}", condition);

    let condition2 = false || !condition;
    println!("{}", condition2);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("There was an error reading the line");

    let int_input:i32 = input.trim().parse().unwrap();
    println!("{}", int_input);

    if int_input == 1 {
        println!("true");
    } else {
        println!("false");
    }
}
