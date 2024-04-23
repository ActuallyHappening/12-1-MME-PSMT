use tracing::info;
use ymath::{prime_decompose, simplify_square_root};

fn main() {
	tracing_subscriber::fmt::init();


	info!(val = ?prime_decompose(208.into()), simplified = ?simplify_square_root(208.into()));
}