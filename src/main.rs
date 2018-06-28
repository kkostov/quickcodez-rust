fn main() {
    let result = divide(8.0, 4.0);
    match result {
        None => println!("Cannot divide by 0"),
        Some(j) => println!("The result was {}", j)
    }
}

fn divide(numerator: f32, denominator: f32) -> Option<f32> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
