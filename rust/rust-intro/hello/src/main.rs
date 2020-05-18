fn main() {
    println!("Hello, {}", "Takashi");
    println!(
        "radius {:.1}, pi {:.3}, area {:.3}",
        3.2,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI,
    );
}
