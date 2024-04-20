use scientific::Precision;
use tracing::info;
use ymath::trapezoidal_rule;

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting");

    let c = 6.9;
    let func = |_| c;
    let approx = trapezoidal_rule(func, 0.0, 10.0, 10, Precision::Digits(10)).unwrap();
    assert_eq!(approx, c * 10.0);
}
