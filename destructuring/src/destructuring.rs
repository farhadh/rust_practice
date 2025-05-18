struct Point {
    x: i32,
    y: i32,
}

struct Coordinate {
    x: f64,
    y: f64,
}

fn main() {
    let p: Point  = Point { x: 10, y: 20 };
    let c: Coordinate = Coordinate { x: 3.5, y: 7.2 };

    // Rust can infer the types of p and c based on their usage
    let Point { x: a, y: b } = p;
    println!("Point: a = {}, y = {}", a, b);
    
    let Coordinate { x, y } = c;
    println!("Coordinate: x = {}, y = {}", x, y);
}
