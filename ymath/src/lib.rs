pub struct PartialSurd<V, E> {
	pub value: V,
	pub exponent: E,
}

pub type SurdBase = PartialSurd<num::rational::Ratio<num::BigInt>, num::BigInt>;
