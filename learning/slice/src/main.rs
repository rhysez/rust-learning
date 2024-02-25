fn main() {
    let s2 = "hello world";

    let word = first_word(s2);

    println!("The word is: {}", word)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
