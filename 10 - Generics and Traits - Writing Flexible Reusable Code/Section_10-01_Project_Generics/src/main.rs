use num_traits::{Float, ToPrimitive};

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

// First version: we can pass in BOTH f32 or f64
fn solve_generic_1<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

// Second version: we can pass in any combination of types of numbers implementing Float
fn solve_generic_2<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

// Third version: Any combination of numbers
fn solve_generic_3<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
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

    println!("Solve Generic <T>: {}", solve_generic_1::<f32>(a, b));


    // ---------------

    let a: f32 = 3.0;
    let b: f64 = 4.0;

    println!("Solve Generic <T, U>: {}", solve_generic_2::<f32, f64>(a, b));


    // ---------------

    let a: i32 = 3;
    let b: u8 = 4;

    println!("Solve Generic <T, U> (ToPrimitive): {}", solve_generic_3::<i32, u8>(a, b));
}