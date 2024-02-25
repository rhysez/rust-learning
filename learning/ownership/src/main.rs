fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s is moved into takes_ownership, no longer accessible

    let x = 5;
    makes_copy(x); // x is copied into makes_copy, x is still accessible
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string is dropped, scope ends

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}