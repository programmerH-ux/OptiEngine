use optimization_engine::optimizer::Optimizer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let value: f64 = args.get(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(100.0);

    let iterations: u32 = args.get(2)
        .and_then(|v| v.parse().ok())
        .unwrap_or(50);

    let optimizer = Optimizer::new(iterations);
    let result = optimizer.optimize(value);

    println!("Input value: {}", value);
    println!("Iterations: {}", iterations);
    println!("Result: {}", result);
}
