trait shape {
    fn area(&self) -> u32;
}

struct Rectangle{
    x: u32,
    y: u32
}

struct Circle {
    radius: f64
}

impl shape for Rectangle{
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl shape for Circle{
    fn area(&self) -> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

fn main() {
    println!("Hello, world!");

    // instantiating the struct and get area
    let c = Circle{radius:3.3};
    let r = Rectangle{x:4,y:3};

    println!("circle area is {}", c.area());
    print!("rectangle area is {}",r.area());
}
