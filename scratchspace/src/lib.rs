use scientific::{Precision, Scientific};
use tracing::{debug, trace};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Scientific error: {0}")]
	Scientific(#[from] scientific::Error),
	#[error("Conversion error: {0}")]
	ConvertError(#[from] scientific::ConversionError),
}

pub fn trapezoidal_rule_strip_num(
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

	let total_width = &top_b - &bottom_a;
	let strip_width = total_width.div_rpsp(&Scientific::from(number), precision)?;
	debug!(%strip_width, %bottom_a, %top_b, "Beginning trapezoidal rule calculation");

	let bottom = func(&bottom_a)?;
	let top = func(&top_b)?;
	let ends = &bottom + &top;
	debug!(%bottom, %top, %ends, "Ends E = {:.4} + {:.4} = {:.4}", f64::from(&bottom), f64::from(&top), f64::from(&ends));
	let middle: Scientific = {
		let multiples = 1..number;

		let mut sum = Scientific!(0);

		#[cfg(feature = "debug")]
		let mut m_values = Vec::new();

		for m in multiples {
			let m = Scientific::from(m);
			let x = &bottom_a + &(&m * &strip_width);
			let strip_sum = func(&x)?;
			trace!(%m, %x, %strip_sum, "Computing func at x, func({}) = {}", x, func(&x)?);

			#[cfg(feature = "debug")]
			m_values.push(strip_sum.clone());

			sum = &sum + &strip_sum;
		}

		trace!(%sum, "Finished sum");

		#[cfg(feature = "debug")]
		{
			let m_values: Vec<f64> = m_values.iter().map(|x| x.into()).collect();
			let mut m_message = String::from("M = ");
			m_message.push_str(&format!("{:.4}", m_values.first().unwrap_or(&0.0)));
			for v in m_values.iter().skip(1) {
				m_message.push_str(&format!(" + {:.4}", v));
			}
			let sum: f64 = m_values.iter().sum();
			m_message.push_str(&format!(" = {:.4}", sum));
			debug!(?m_values, %m_message, "Middle values");
		}

		sum
	};

	// ret = (w / 2) * (E + 2M)
	let ret = &(strip_width.div_rpsp(&Scientific!(2.0), precision))?
		* &(&ends + &(&Scientific!(2.0) * &middle));
	
	debug!(%ret, "Finished trapezoidal rule calculation: {:.4} / 2 * ({:.4} + 2 * {:.4})", f64::from(&strip_width), f64::from(&ends), f64::from(&middle));

	Ok((&ret).into())
}

#[cfg(test)]
mod tests {
	use tracing::info;

use super::*;

	#[test]
	fn test_zero() {
		let func = |_| 0.0;
		let approx = trapezoidal_rule_strip_num(func, 0.0, 10.0, 5, Precision::Digits(10)).unwrap();
		assert_eq!(approx, 0.0);
	}

	#[test]
	fn test_constant() {
		let c = 6.9;
		let func = |_| c;
		let approx = trapezoidal_rule_strip_num(func, 0.0, 10.0, 10, Precision::Digits(10)).unwrap();
		assert_eq!(approx, c * 10.0);
	}

	#[allow(dead_code)]
	fn test_quadratic() {
		let number = 10;
		let precision = Precision::Digits(100);

		let f = |x: f64| 6.0 * x.powi(2) + 4.0 * x - 7.0;
		let sum = trapezoidal_rule_strip_num(f, 0.0, 1.0, number, precision).unwrap();
		info!(test_sum = %sum);
	}
}
