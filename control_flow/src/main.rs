fn main() {
    let number: i32 = 3;

    let comparison = if number < 5 {
        println!("{} < 5", number);
        "smaller"
    } else if number == 5 {
        println!("{} == 5", number);
        "equal"
    } else {
        println!("{} > 5", number);
        "larger"
    };
    println!("Comparison result is {}", comparison);

    let mut number1: i32 = 0;
    let number2 = loop {
        number1 += 1;
        if number1 == 10 {
            break number1 * 2;
        }
    };
    println!("number2 = {}", number2);

    while number1 > 0 {
        number1 -= 1;
    }
    println!("number1 = {}", number1);

    let strings = ["a", "b", "cd", "foo"];
    for s in strings {
        println!("s is {}", s);
    }

    for number3 in (0..10).rev() {
        println!("number3 = {}", number3);
    }
}
