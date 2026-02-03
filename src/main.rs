use optimization_engine::optimizer::Optimizer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let value: f64 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(100.0);
    let iterations: u32 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(100);
    let lr: f64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(0.01);
    let momentum: f64 = args.get(4).and_then(|v| v.parse().ok()).unwrap_or(0.9);

    let optimizer = Optimizer::new(iterations, lr, momentum);

    let gd = optimizer.gradient_descent(value);
    let mom = optimizer.momentum_optimize(value);

    println!("Input value: {}", value);
    println!("Gradient Descent result: {}", gd);
    println!("Momentum Optimizer result: {}", mom);
}
