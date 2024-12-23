fn main() {
    let mut y = 15;
    println!("y is: {}", y);
    y = 10;
    println!("y is: {}", y);

    let x = 6; 
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    let x = 5;
    println!("x is: {}", x);

    const SECONDS_IN_MINUTES: u32 = 60;
    println!("{}", SECONDS_IN_MINUTES);
}
