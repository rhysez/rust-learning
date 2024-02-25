// example of shadowing in rust
// variables are immutable by default, however we can re-produce them with the let keyword
// this is called shadowing
// what is the benefit? it means that we can perform transformations on variables
// but have the variable be immutable after those transformations have been completed

fn main() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
