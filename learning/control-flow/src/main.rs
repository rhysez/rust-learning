// in rust, 'if' expressions must return a boolean value
// for example, we can't use 'if (x)', but we can use 'if (x < y)'
// we can also omit parentheses around the expression
// this is a little different from languages like JavaScript

fn main() {
    let x = 3;
    
    let condition = true;

    // here we can use if statements inside of let statements
    // we can also use 'if condition', because 'condition' is a boolean
    // if 'condition' were not a boolean, this would panic 
    let foo = if condition { 5 } else { 6 };

    if x < 5 {
        println!("condition was true!")
    } else {
        println!("condition was false :(")
    }

    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("{element}")
    }

    // 1..4 is a range
    // this will print all numbers between and including 1 and 4
    // if we remove "=" from 4, it will not include 4
    for number in 1..=4 {
        println!("{}!", number);
    }
}
