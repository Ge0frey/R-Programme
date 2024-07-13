fn main () {
    println!("Hello world");
    print_x();
}

fn print_x () {
    let  x = 50;
    println!("The value of X is: {}", x);


    {
        let x = x - 100;
        println!("The value of x is: {}", x);
    }

    let x = x + 1;
    println!("The value of x is: {}", x);
}