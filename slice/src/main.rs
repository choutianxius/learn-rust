fn main() {
    let mut s = String::from("Hello, world!");

    let word = first_word(&s);
    println!("First word from is {}", word);

    let word = first_word(&s[1..]);
    println!("First word from substr is {}", word);
    s.clear();

    let s_literal = "hello world";
    let word = first_word(&s_literal[..]);
    println!("First word from str literal is {}", word);

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);
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
