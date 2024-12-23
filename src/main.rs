fn main() {
    println!("Hello world");

    let x = 6;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    let x = 5;
    println!("x is: {}", x);
}
