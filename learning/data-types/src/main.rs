// a 'scalar' type represents a single value
// rust has four primary scalar types
// integers, floating-point numbers, Booleans and characters

// integer: a number without a fractional component (unsigned: can only be positive, signed: can be positive or negative)
// floating-point number: a number with a fractional component (all floating-point numbers are signed)
// Boolean: represents true or false values, they are 1 byte in size
// character: represents a single unicode character, must be written in single quotes ' '

// a compound type can group multiple values into one type
// rust has two primitive compound types
// tuples and arrays

// tuple: a group of values, can have a variety of types (tuples have a fixed length that cannot change after declaration)
// array: a group of values, but unlike a tuple, every value must be of the same type. arrays also have a fixed length.

// vector: very similar to an array, but can grow or shrink in size

// variables annotated for clarity
fn main() {
    // scalar types
    let x: u32 = 5; // unsigned 32 bit integer
    let y: f32 = 2.4; // floating-point 32 bit integer
    let t: bool = true; // boolean 
    let c: char = 'z'; // character

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (x, y, z) = tup; // pattern matching, 'y' would equal 6.4
    let six_point_four = tup.1; // accessing with dot notation, this would equal 6.4 

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array, type annotated as [type; quantity]
    let b = [3; 5]; // we can initialise an array this way, giving us [3, 3, 3, 3, 3]

    let first = a[0]; // accessing array values by their index
    let second = a[1];
}
