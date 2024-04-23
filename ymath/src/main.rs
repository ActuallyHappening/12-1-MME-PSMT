use num::{pow::{self, Pow}, BigInt, BigUint};
use tracing::info;
use ymath::{prime_decompose, simplify_square_root};

fn main() {
	tracing_subscriber::fmt::init();


	info!(val = ?prime_decompose(208.into()), simplified = ?simplify_square_root(272.into()));

	let num1 = BigInt::from(2);
	let num2 = BigUint::from(3u32);

	let res = Pow::pow(num1, num2);
}