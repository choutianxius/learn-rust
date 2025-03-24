fn main() {
    // let x = 12;
    let mut x = 1;
    println!("x = {x}");
    x = 3;
    println!("x = {x}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("inner shadow x = {x}");
    }

    println!("outer shadow x = {x}");

    const HOUR_SECONDS: u32 = 60 * 60;
    println!("HOURS_SECONDS = {HOUR_SECONDS}");
}
