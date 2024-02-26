// our compiler now uses the Debug trait
#[derive(Debug)]

// a struct, which stores named fields for values
struct Rectangle {
    width: u32,
    height: u32,
}

// an implementation block for our Rectangle struct
// implementation blocks hold the methods for a struct
// the first argument in a method is always &self
// which takes a reference to the Rectangle instance
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// we can also define associated functions
// unlike methods, these are not tied to an instance of our struct
// we don't need to pass in the &self argument
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect3 contains a square, it's size is: {}", &rect3.width);

    // structs do not have the display trait, so cannot be printed normally
    // adding :? specifies we want our struct to use the Debug trait
    // the Debug trait allows us to print our struct clearly
    // adding # inbetween prints each field on a new line
    println!("rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    )
}
