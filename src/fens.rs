//functions, expressions and statements;
fn main () { //this is the main function;
    let number = {  //this is an exprection as it returns values of an arithmetic operation
        let x = 50; //this is a statement and does not return any value;
        //statements mostly include declarations of variables and functions that are not expected to have a return value;
        x + 50
    };

    println!("{}", number); 

    let added = add_numbers(8,20);

    println!("{}", added);
}

fn add_numbers(x:i32, y:i32) -> i32 {  //this is a function returning an expression
    let result = x + y;  //do not add a semi colon to a returnable expression unless using the return key word;
    if result > 10 {
        return result - 10;
    }
    result
}