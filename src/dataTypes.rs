fn main () {
    //this file will handle data types in rust
    //All types are immutable unless you use the key word mut
    //scalar data types in rust
    let x: i32 = 8; //explicit declaration of the data type this is a signed integer of size 32 bits

    let floating_point: f32 = 22.6; //floating point values can either be f32 or f64;

    let true_or_false: bool = true; //booleon values; can also be represented using 0(false) or 1(true);

    let letter: char = 'c';

    println!("{}\n{}\n{}\n{}\n", x,floating_point,true_or_false, letter);

    //compound data types in rust -> tuples and arrays;
    let tuple: (i32, bool, char) = (1, true, 's'); //carries different data types 
    println!("{}\n", tuple.2); 

    let arr: [i32; 5] = [1,2,3,4,5]; //arrays must have the same data types 
    //when declaring the array type you must also highlight the size of the array;
    println!("{}", arr[4]);

}