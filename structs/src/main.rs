struct Point {
    x: f64,
    y: int,
}

fn main() {
    let mut origin = Point { x: (0 as int) as f64, y: 0i };
    origin.x = 12.5  as f64;
    println!("The origin is at ({}, {})", origin.x, origin.y);
}