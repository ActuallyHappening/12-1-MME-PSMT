use scientific::{Precision, Scientific};

/// Positive only primitive number
pub enum RealPrimitiveLiteral {
	Zero,
	Integer(num::BigUint),
	Transcendental(Box<dyn Transcendental>),
}
type Primitive = RealPrimitiveLiteral;

pub enum ExactRealExpression {
	Primitive(Primitive),
	// composite
	Addition(Box<Self>, Box<Self>),
	// Multiplication(Box<Self>, Box<Self>),
	Rational(num::rational::Ratio<Box<Self>>),
	Exponent(PartialSurd<Box<Self>, Box<Self>>),
}

pub trait Transcendental {
	fn evaluate_unchecked(&self, precision: Precision) -> Scientific;

	fn evaluate_checked(&self, precision: Precision) -> Option<Scientific> {
		let ret = self.evaluate_unchecked(precision);
		if ret < Scientific!(0) {
			None
		} else {
			Some(ret)
		}
	}

	fn evaluate(&self, precision: Precision) -> Scientific {
		let ret = self.evaluate_unchecked(precision);
		if ret < Scientific!(0) {
			panic!("Transcendental function returned a negative value: {}", ret);
		}
		ret
	}
}

pub struct PartialSurd<V, E> {
	pub value: V,
	pub exponent: E,
}

pub type ExactValue = ExactRealExpression;

pub fn simplify_basic_fractions(value: ExactValue) -> ExactValue {
	match value {
		ExactValue::Addition(lhs, rhs) => {
			let (lhs, rhs) = (&*lhs, &*rhs);
			match (lhs, rhs) {
				(ExactValue::Rational(lhs), ExactValue::Rational(rhs)) => {
					let (lhs_denom, lhs_num) = (lhs.denom(), lhs.numer());

					todo!()
				}
				v => todo!()
			}
		}
		v => v,
	}
}
