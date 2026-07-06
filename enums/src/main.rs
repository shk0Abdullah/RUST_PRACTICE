// Enums: lets you enumerate over the defined Variants
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn calculate_area(shape: Shape) -> f64 {
    // pattern matching
    return match shape {
        Shape::Rectangle(w, h) => w * h,
        Shape::Circle(x) => 3.14 * x * x,
    };
}

fn main() {
    let shape = Shape::Rectangle(30.0, 50.0);
    print!("{}", calculate_area(shape));
}
