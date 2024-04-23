use scientific::{Precision, Scientific};

pub enum ExactRealExpression {
	// primitive
	Zero,
	Integer(num::BigInt),
	Transcendental(Box<dyn Transcendental>),
	// composite
	Addition(Box<Self>, Box<Self>),
	// Multiplication(Box<Self>, Box<Self>),
	Rational(num::rational::Ratio<Box<Self>>),
	Exponent(PartialSurd<Box<Self>, Box<Self>>),
}

pub trait Transcendental {
	fn evaluate(&self, precision: Precision) -> Scientific;
}

pub struct PartialSurd<V, E> {
	pub value: V,
	pub exponent: E,
}

pub type ExactValue = ExactRealExpression;