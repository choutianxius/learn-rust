fn main() {
    let x: f64 = 12.0;
    let y: f64 = 2.4;
    let x1: i32 = 120_000;
    let x2 = 1_024i32;

    let mut quotient: f64 = x / y;
    println!("{x} / {y} = {quotient}");
    let truncated = x1 / x2;
    println!("{x1} / {x} = {truncated}");
    quotient = f64::from(x1) / f64::from(x2);
    println!("quotient = {quotient}");
    let product = x1 * x2;
    println!("{} * {} = {}", x1, x2, product);
    let sum = x + y;
    println!("{x} + {y} = {sum}");
    let difference = x2 - x1;
    println!("{x2} - {x1} = {difference}");

    let c = 'c';
    let z: char = 'Z';
    let cat = '😻';
    println!("{c}, {z}, {cat}");

    let tup: (f64, i32, char) = (x, x2, cat);
    let (t1, t2, t3) = tup;
    let cat1 = tup.2;
    println!("{t1}, {t2}, {t3}, {cat1}");

    let days = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
    let integers: [i32; 5] = [0, 1, 2, 3, 4];
    let addresses = [0usize; 100];
    println!(
        "{}, {}, {}",
        days[0],
        integers[1],
        addresses[addresses.len() - 1]
    );
}
