#[cfg(feature = "debug")]
use colored::Colorize;
use rand::seq::index;

use crate::Literal;

#[cfg(feature = "debug")]
macro_rules! debug_mode {
	($command: expr, $($args: expr), *) => {
		println!("{}", format!($command, $($args), *).bright_yellow());
	};
	($command: expr, String) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! debug_mode {
	($command: expr, $($args: expr), *) => {};
	($command: expr, String) => {};
}

#[cfg(feature = "easy_panic")]
macro_rules! easy_panic {
	() => {
		panic!("(Easy panic) Error message should be included in 'debug' mode, else, you can activate RUST_BACKTRACE=1 to see the backtrace.");
	};
}

#[cfg(not(feature = "easy_panic"))]
macro_rules! easy_panic {
	() => {};
	($u: expr) => {
		return;
	};
}

/// # Map
/// A map is a collection of key-value pairs, you can compare it to a hashmap or
/// a dictionary, you can form a Map with the `new()` function, or from a
/// vector of tuples with the `from(...)` function. ## Example
/// ```rust
/// use mapping::Map;
/// let mut map = Map::new();
///
/// // Let's add some values.
/// map.push("key", "value");
/// map.push("key2", "value2");
/// map.push("key3", "value3");
///
/// Now, let's clear the map.
/// ```
///
/// You can also make a Map from a vector of tuples.
/// ```rust
/// use mapping::Map;
/// let mut map = Map::from(vec![("key", "value"), ("key2", "value2"), ("key3", "value3")]);
/// ```
pub struct Map {
	pub keys: Vec<DynType>,
	pub values: Vec<DynType>,
}

enum DynType {
	Literal,
	LiteralRef,
	Index
}

fn move_index<T>(vec: &mut Vec<T>, idx: usize, to: usize) {
	let tmp = vec.remove(idx);
	vec.insert(to, tmp);
}

impl<'a, T> Map
where
	T: Literal<String>,
{
	/// Creates a new map.
	/// # Examples
	/// ```rust
	/// use mapping::Map;
	/// let mut map = Map::new();
	/// ```
	fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	/// Adds a new key-value pair to the map, its arguments can be a Literal, an Index or a &Literal.
	/// # Examples
	/// ```rust
	/// use mapping::Map;
	/// let mut map = Map::new();
	/// map.insert("key", "value");
	/// ```
	fn insert(&mut self, key: DynType, value: DynType, index: usize) {
		self.keys.insert(index, key);
		match value {
			DynType::Literal => self.values.insert(index, key),
			DynType::LiteralRef => self.values.insert(index, key),
			DynType::Index => self.values.insert(index, &self.values[value]),
		}
	}

	/// Pushes a new key-value pair to the end of the map, you can compare the
	/// `push` method with a `push` method in a `Vec`. # Examples
	/// ```rust
	/// use mapping::Map;
	/// let mut map = Map::new();
	/// map.push("key", "value");
	/// ```
	fn push(&mut self, key: DynType, value: DynType) {
		self.keys.push(key);
		self.values.push(value);
	}

	/// Clears the map completely.
	/// # Examples
	/// ```rust
	/// use mapping::Map;
	/// let mut map = Map::new();
	///
	/// // We add some key-value pairs.
	/// map.push("key", "value");
	/// map.push("key2", "value2");
	///
	/// // Now we clear the map.
	/// map.clear();
	/// ```
	fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	/// Encourages some keys more than others, so you can value better *expected
	/// output* and disencourage bad *expected output*. It takes an index of the
	/// key you want to encourage and how much you want to encourage it, ***take
	/// into account that the "how much?" number must not be greater than the
	/// index of the key you want to encourage!*** # Examples
	/// ```rust
	/// use mapping::Map;
	/// let mut map = Map::new();
	/// map.push("key", "value");
	/// map.push("key2", "value2");
	/// map.push("key3", "value3");
	/// map.push("key4", "value4");
	/// map.encourage(2, 1);
	/// ```
	fn encourage(&mut self, idx: usize, how_much: usize) {
		let klen = self.keys.len();
		if idx >= klen {
			debug_mode!("Index of key to encourage or index + how_much to encourage is out of bounds: {} + {}, length is {}", idx, how_much, klen);
			easy_panic!(());
		};
		// If it underflows, it will be 0
		if idx - how_much >= klen {
			debug_mode!("(Key will be moved to the first rank, the program will continue) Index to search - how_much is out of bounds: {} - {}, length is {}", idx, how_much, klen);
			move_index(self.keys.as_mut(), idx, 0);
		};
		move_index(&mut self.keys, idx, idx - how_much);
		move_index(&mut self.values, idx, idx - how_much);
	}

	/// Encourages a string, instead of an index, if the string doesn't exist in
	/// the map, it will panic!(), it takes a string and how much you want to
	/// encourage it, ***take into account that the "how much?" number must not
	/// be greater than the string you want to encourage!*** # Examples
	/// ```rust
	/// use mapping::Map;
	/// let mut map = Map::new();
	/// map.push("key", "value");
	/// map.push("key2", "value2");
	/// map.push("key3", "value3");
	/// map.encourage_by_str("key2", 1);
	fn encourage_by_str(&mut self, str_: T, how_much: usize)
	where
		T: PartialEq<T>,
	{
		let idx = self.keys.iter().position(|x| *x == str_).unwrap();
		self.encourage(idx, how_much);
	}

	/// Disencourages some keys more than others, so you can value better
	/// *expected output* and disencourage bad *expected output*. It takes an
	/// index of the key you want to disencourage and how much you want to
	/// disencourage it, ***take into account that the "how much?" number must
	/// not be greater than the index of the key you want to disencourage!***
	fn disencourage(&mut self, idx: usize, how_much: usize) {
		let klen = self.keys.len();
		if idx >= self.keys.len() {
			debug_mode!("Index of key to encourage or index + how_much to encourage is out of bounds: {} + {}, length is {}", idx, how_much, self.keys.len());
			easy_panic!(());
		};

		if idx + how_much >= klen {
			debug_mode!("(Key will be moved to the last rank, the program will continue) Index to search + how_much is out of bounds: {} + {}, length is {}", idx, how_much, klen);
			move_index(self.keys.as_mut(), idx, klen - 1);
		};
	}

	/// Disencourages a string, instead of an index, if the string doesn't exist
	/// in the map, it will panic!(), it takes a string and how much you want to
	/// disencourage it, ***take into account that the "how much?" number must
	/// not be greater than the string you want to disencourage!***
	fn disencourage_by_str(&mut self, str_: T, how_much: usize)
	where
		T: PartialEq<T>,
	{
		let idx = self.keys.iter().position(|x| *x == str_).unwrap();
		self.disencourage(idx, how_much);
	}
}

impl<T> From<Vec<(T, T)>> for Map
where
	T: Literal<String>,
{
	/// Creates a new map from a vector of `(T, T)`
	/// # Examples
	/// ```rust
	/// use mapping::Map;
	/// let vec = vec![("key", "value")];
	/// let map = Map::from(vec);
	/// ```
	fn from(what: Vec<(T, T)>) -> Self {
		let mut map = Self::new();
		for (key, value) in what {
			map.keys.push(key);
			map.values.push(value);
		};
		map
	}
}

// Implement Iterator
impl<T> Iterator for Map
where
	T: Literal<String>,
{
	type Item = (T, T);
	fn next(&mut self) -> Option<Self::Item> {
		match self.keys.pop() {
			Some(key) => match self.values.pop() {
				Some(value) => Some((key, value)),
				None => None,
			},
			None => None,
		}
	}
}
