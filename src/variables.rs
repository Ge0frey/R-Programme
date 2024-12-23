fn main() {
    let mut y = 15; //declared y -> Have to make y mutable using the mut key word in order to re-assign
    println!("y is: {}", y);
    y = 10; //re-assigned y
    println!("y is: {}", y);

    let x = 6;  //declared x in outer scope
    println!("x is: {}", x);

    {
        let x = x - 2; // declared x in inner scope -> change in scope allows to shadow values from the upper scope 
        println!("x is: {}", x);
    }

    let x = 5; //re-declared x -> do not have to make x mutable to re-declare it using the let key word
    println!("x is: {}", x);

    const SECONDS_IN_MINUTES: u32 = 60; //const keyword makes values immutable
    println!("{}", SECONDS_IN_MINUTES);
}
