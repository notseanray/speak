#[path = "mapping.rs"]
mod mapping;
use mapping::*;

pub(crate) trait Literal<T> {
	fn literal(self) -> T;
}

impl Literal<String> for String {
	fn literal(self) -> String {
		self
	}
}

impl Literal<String> for &str {
	fn literal(self) -> String {
		self.to_string()
	}
}

impl Literal<Vec<String>> for Vec<String> {
	fn literal(self) -> Vec<String> {
		self
	}
}

impl<T> Literal<Vec<String>> for Vec<T> where T: ToString + Literal<Vec<String>> {
	fn literal(self) -> Vec<String> {
		self.iter().map(|s| s.to_string()).collect()
	}
}

impl<T> Literal<Map<String>> for Map<T> where T: Literal<Vec<String>> + ToString {
	fn literal(self) -> Map<String> {
		Map::<String> {
			keys: self.keys.literal(),
			values: self.values.literal()
		}
	}
}
