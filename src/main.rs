use std::io;
use triangle::Triangle;
fn main() {
    let a: f64 = get_input("Please enter side 1: ");
    let b: f64 = get_input("Please input side 2 : ");
    let c: f64 = get_input("Please enter side 3 : ");
    match Triangle::new(a, b, c) {
        Ok(t) => {
            println!("Triangle: {:?}", t);
            println!("Perimeter: {}", t.perimeter());
            println!("Area: {}", t.area());
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("Please press Enter key to exit ...");
    let mut exit = String::new();
    io::stdin()
        .read_line(&mut exit)
        .expect("Unable to parse string");
}

fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to get string");
    let input: f64 = input.trim().parse().expect("Please enter valid float");
    input
}
