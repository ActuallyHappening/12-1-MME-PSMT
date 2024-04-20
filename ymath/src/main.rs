use std::f64::consts::{E, TAU};

use scientific::Precision;
use tracing::info;
use ymath::trapezoidal_rule;

fn test_quadratic() {
	let number = 10;
	let precision = Precision::Digits(100);

	let f = |x: f64| 6.0 * x.powi(2) + 4.0 * x - 7.0;
	let sum = trapezoidal_rule(f, 0.0, 1.0, number, precision).unwrap();
	info!(test_sum = %sum);
}

#[allow(non_snake_case)]
fn main() {
	tracing_subscriber::fmt::init();

	info!("Starting");

	// test_quadratic();

	let d = 0.8109f64;
	let e_r = |x: f64| E.powf(d * (x - 0.5f64)) - 1.0;
	let e_l = |x: f64| E.powf(-d * (x - 0.5f64)) - 1.0;
	let t_t = |x: f64| 0.5 * ((TAU / 2.0) * x).sin().powf(0.5) + 0.5;
	let t_b = |x: f64| -0.5 * ((TAU / 2.0) * x).sin().powf(0.5) + 0.5;
	let g = 2.8025625;
	let l = |x: f64| ((1.0 / 10.0) * x + 0.1).ln() + g;

	let I = 0.6197;
	let number = 20;
	let precision = Precision::Digits(100);
	let w = 27.0;
	let h = 22.0;
	info!(%number, ?precision, "Computing ...");

	let h_tl = 1.4;
	let A_tl = trapezoidal_rule(|x| t_t(x) - l(x), 0.0, I, number, precision).unwrap();
	let V_tl = A_tl * w * h * h_tl;
	info!(%A_tl, %V_tl);

	let h_m = 1.9;
	let A_m = trapezoidal_rule(|x| l(x) - e_l(x), 0.0, 0.5, number, precision).unwrap()
		+ trapezoidal_rule(|x| l(x) - e_r(x), 0.5, I, number, precision).unwrap()
		+ trapezoidal_rule(|x| t_t(x) - e_r(x), I, 1.0, number, precision).unwrap();
	let V_m = A_m * w * h * h_m;
	info!(%A_m, %V_m);

	let h_b = 0.5;
	let A_b = trapezoidal_rule(|x| e_l(x) - t_b(x), 0.0, 0.5, number, precision).unwrap()
		+ trapezoidal_rule(|x| e_r(x) - t_b(x), 0.5, 1.0, number, precision).unwrap();
	let V_b = A_b * w * h * h_b;
	info!(%A_b, %V_b);

	let V_t = V_tl + V_m + V_b;
	info!(%V_t, "Final computed volume");

	info!("Finished");
}
