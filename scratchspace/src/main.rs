use std::{f64::consts::{E, TAU}, ops::Add};

use scientific::Precision;
use tracing::{debug, info};
use ymath::trapezoidal_rule_strip_num;

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
	let l = |x: f64| ((1.0 / 10.0) * x.add(1.0)).ln() + g;

	let l_test = l(0.5);
	debug!(l = %l_test, "L at x = 0.5");

	let I = 0.6197;
	let number = 20;
	let precision = Precision::Digits(100);
	let w = 27.0;
	let h = 22.0;
	info!(%number, ?precision, "Computing ...");

	let h_tl = 1.4;
	let d_tl = |x| t_t(x) - l(x);
	let d_tl_test = d_tl(0.5);
	debug!(d_tl = %d_tl_test, "D at x = 0.5");
	let A_tl = trapezoidal_rule_strip_num(d_tl, 0.0, 0.6, 12, precision).unwrap();
	let V_tl = A_tl * w * h * h_tl;
	info!(%A_tl, %V_tl, "proper way");

	// let h_tl = 1.4;
	// let A_tl = trapezoidal_rule_strip_num(|x| t_t(x) - l(x), 0.0, I, number, precision).unwrap();
	// let V_tl = A_tl * w * h * h_tl;
	// info!(%A_tl, %V_tl);

	// // T_{m}\left(x\right)=\left\{0\le x<I:l\left(x\right),I\le x\le1:t_{t}\left(x\right)\right\}
	// let h_m = 1.9;
	// let A_m = trapezoidal_rule_strip_num(|x| l(x) - e_l(x), 0.0, 0.5, number, precision).unwrap()
	// 	+ trapezoidal_rule_strip_num(|x| l(x) - e_r(x), 0.5, I, number, precision).unwrap()
	// 	+ trapezoidal_rule_strip_num(|x| t_t(x) - e_r(x), I, 1.0, number, precision).unwrap();
	// let V_m = A_m * w * h * h_m;
	// info!(%A_m, %V_m);

	// let h_b = 0.5;
	// let A_b = trapezoidal_rule_strip_num(|x| e_l(x) - t_b(x), 0.0, 0.5, number, precision).unwrap()
	// 	+ trapezoidal_rule_strip_num(|x| e_r(x) - t_b(x), 0.5, 1.0, number, precision).unwrap();
	// let V_b = A_b * w * h * h_b;
	// info!(%A_b, %V_b);

	// let V_t = V_tl + V_m + V_b;
	// info!(%V_t, "Final computed volume");

	info!("Finished");
}
