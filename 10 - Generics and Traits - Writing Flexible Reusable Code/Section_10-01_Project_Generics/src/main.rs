use num_traits::{Float, ToPrimitive};

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

// First version: we can pass in BOTH f32 or f64
// Second version: we can pass in any type of numbers
fn solve_generic<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    //let a_f64 = a as f64;
    // Note that a.to_f64() returns an Option<f64>
    let a_f64 = a.to_f64().unwrap();

    println!("Solve: {}", solve(a_f64, b));

    // ---------------

    let a: f32 = 3.0;
    let b: f32 = 4.0;

    println!("Solve Generic: {}", solve_generic::<f32>(a, b));

}