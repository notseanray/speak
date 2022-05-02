use crate::Literal;

#[cfg(feature = "debug")]
macro_rules! debug_mode {
	($command: expr) => {
		$command
	};
}

#[cfg(not(feature = "debug"))]
macro_rules! debug_mode {
	($command: expr) => {};
}

pub struct Map<T> {
	pub(crate) keys: Vec<T>,
	pub(crate) values: Vec<T>
}

impl<T> Map<T> {
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new()
		}
	}

	pub fn from(from: Vec<(T, T)>) -> Self {
		let mut map = Self::new();
		for (key, value) in from {
			map.keys.push(key);
			map.values.push(value);
		};

		return map;
	}

	pub fn insert(&mut self, key: T, value: T, index: usize) {
		self.keys.insert(index, key);
		self.values.insert(index, value);
	}

	pub fn disencourage_by_key(&mut self, to_search: String, how_much: usize) where T: Literal<String> {
		let idx = self.keys.iter().position(|key| key.literal() == to_search);
		// match idx {
		// }
	}

	pub fn push(&mut self, key: T, value: T) {
		self.keys.push(key);
		self.values.push(value);
	}

	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	pub fn iter(&self) -> impl Iterator<Item = (&T, &T)> {
		self.keys.iter().zip(self.values.iter())
	}
}
