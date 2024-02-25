// statements are instructions that perform some action and do not return a value
// expressions evaluate to a resultant value. in short, they return something

fn main() {
    let sum = my_function(11, 12); // 'let' a statement, 'my_function()' is an expression because it returns 23
    
    let five = five();
    
    println!("Sum is {sum}"); // an expression, returns sum
    
    println!("Five is {five}")
}

// we must specify the return type of this function if we use 'return' 
// here, our return type is a 32 bit integer
// for the last expression in a function, we omit the semi-colon and can omit the 'return' keyword
fn my_function(x: i32, y: i32) -> i32 {
    println!("You have passed {x} and {y} into my_function");
    x + y // an expression, x + y evaluates to 23
}

// no return keyword, no semicolon, just 5
// this is a valid function in rust
// 5 is an expression, we're returning the number 5
fn five() -> i8 {
    5
}