//Arithmetics and type casting;
use std::io;

fn main() {
    let x: u8 = 6; //unsigned range -> 0 <-> 255;
    let y: i8 = 10; //signed range -> -128 <-> 127

    let z = (x as i8) + y; //explicit type conversion
    println!("{}", z);

    let v = 200_i64; //type casting using _
    let p = 20i32; //type casting by adding type directly

    let u = v/(p as i64); //type casting using the as keyword
    println!("{}", u);
 
}
