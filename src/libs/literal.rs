pub trait Literal<T> {
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

// This trait only accepts &str, but I have to use generics because of the main functions that uses generics.

impl<T> Literal<Vec<String>> for Vec<T> where T: ToString {
	fn literal(self) -> Vec<String> {
		self.iter().map(|s| s.to_string()).collect()
	}
}
