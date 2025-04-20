#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
struct Rgb(u8, u8, u8);

fn main() {
    let mut rect1 = Rectangle::square(30.0);
    rect1.width = 40.0;
    println!("Area of {:?} is {}", rect1, rect1.area());

    let width: f32 = 50.0;
    let rect2 = Rectangle { width, ..rect1 };
    println!("Area of {:?} is {}", rect2, rect2.area());

    let rgb = Rgb(0x63, 0x66, 0xf1);
    println!("Hex for {:?} is {}", rgb, rgb_to_hex(&rgb));
}

fn rgb_to_hex(rgb: &Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
}
