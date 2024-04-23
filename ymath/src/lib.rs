use std::ops::Div;

pub mod exact_value;

pub struct PartialSurd<V, E> {
	pub value: V,
	pub exponent: E,
}

pub type SurdBase = PartialSurd<num::rational::Ratio<num::BigInt>, num::BigInt>;

/// Simplifies sqrt(4) into 2,
/// sqrt(6) into sqrt(6),
pub fn simplify_square_root(value: num::BigInt) -> (num::BigInt, Vec<(num::BigInt, num::BigInt)>) {
	let mut prime_factors = prime_decompose(value);
	let mut outside_factor = num::BigInt::from(0);
	for (ref base, exponent) in prime_factors.iter_mut() {
		let num_brought_outside = exponent.clone().div(num::BigInt::from(2));
		outside_factor += base * num_brought_outside.clone();
		*exponent -= num_brought_outside * num::BigInt::from(2);
	}

	(outside_factor, prime_factors)
}

// pub fn prime_recompose(prime_factors: Vec<(num::BigInt, num::BigInt)>) -> num::BigInt {
// 	let mut ret = num::BigInt::from(1);

// 	for (base, exponent) in prime_factors.iter() {
// 		let mut factor = base.clone();

// 		for _ in 0..exponent.clone() {
// 			ret *= factor.clone();
// 		}
// 	}

// 	return ret
// }

pub fn prime_decompose(value: num::BigInt) -> Vec<(num::BigInt, num::BigInt)> {
	let mut value = value.clone();
	let mut result = Vec::new();
	let mut i = num::BigInt::from(2);
	while i.clone() * i.clone() <= value {
		let mut count = num::BigInt::from(0);
		while value.clone() % i.clone() == num::BigInt::from(0) {
			value /= i.clone();
			count += num::BigInt::from(1);
		}
		if count > num::BigInt::from(0) {
			result.push((i.clone(), count.clone()));
		}
		i += num::BigInt::from(1);
	}
	if value > num::BigInt::from(1) {
		result.push((value.clone(), num::BigInt::from(1)));
	}
	result
}

#[test]
fn prime_decompose_works() {
	let test_cases = [
		(2, vec![(2, 1)]),
		(6, vec![(2, 1), (3, 1)]),
	];

	for (value, expected) in test_cases.iter() {
		let value = num::BigInt::from(*value);
		let expected = expected.iter().map(|(p, e)| (num::BigInt::from(*p), num::BigInt::from(*e))).collect::<Vec<_>>();
		assert_eq!(prime_decompose(value), expected);
	}
}