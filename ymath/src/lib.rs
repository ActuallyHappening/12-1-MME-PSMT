use scientific::{Precision, Scientific};
use tracing::debug;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Scientific error: {0}")]
	Scientific(#[from] scientific::Error),
	#[error("Conversion error: {0}")]
	ConvertError(#[from] scientific::ConversionError),
}

pub fn trapezoidal_rule(
	func: impl Fn(f64) -> f64,
	bottom_a: f64,
	top_b: f64,
	number: u16,
	precision: Precision,
) -> Result<f64, Error> {
	let func = |x: &Scientific| {
		let fin: f64 = x.into();
		let func_output = func(fin);
		Scientific::try_from(func_output)
	};
	let top_b = Scientific::try_from(top_b)?;
	let bottom_a = Scientific::try_from(bottom_a)?;

	let strip_delta = (&top_b - &bottom_a).div_rpsp(&Scientific::from(number), precision)?;

	let ends = &func(&bottom_a)? + &func(&top_b)?;
	let middle: Scientific = {
		// let middle_num = number - 2;
		let multiples = 1..number;
		// assert_eq!(multiples.len(), middle_num as usize, "Multiples length is not equal to middle_num");

		let mut sum = Scientific!(0);
		for m in multiples {
			let m = Scientific::from(m);
			let x = &bottom_a + &(&m * &strip_delta);
			let strip_sum = func(&x)?;
			debug!(%m, ?strip_sum, "Computing");
			sum = &sum + &strip_sum;
		}

		debug!(%sum, "Finished sum");
		sum
	};

	let ret = &(strip_delta.div_rpsp(&Scientific!(2.0), precision))?
		* &(&ends + &(&Scientific!(2.0) * &middle));
	Ok((&ret).into())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_zero() {
		let func = |_| 0.0;
		let approx = trapezoidal_rule(func, 0.0, 10.0, 5, Precision::Digits(10)).unwrap();
		assert_eq!(approx, 0.0);
	}

	#[test]
	fn test_constant() {
		let c = 6.9;
		let func = |_| c;
		let approx = trapezoidal_rule(func, 0.0, 10.0, 10, Precision::Digits(10)).unwrap();
		assert_eq!(approx, c * 10.0);
	}
}
