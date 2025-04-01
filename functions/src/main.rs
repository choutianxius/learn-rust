fn main() {
    println!("Hello, world!");
    another_function();
    greet("Majulian");
    println!("8 + 1 = {}", plus_one(8));
    println!("12 + 2 = {}", plus_two(12));
}

fn another_function() {
    println!("Another function");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// don't
fn plus_two(x: i32) -> i32 {
    return x + 2;
}
