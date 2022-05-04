#[cfg(feature = "debug")]
use colored::Colorize;

use crate::Literal;

#[cfg(feature = "debug")]
macro_rules! debug_mode {
	($command: expr, $($args: expr), *) => {
		println!("{}", format!($command, $($args), *).bright_yellow());
	};
}

#[cfg(not(feature = "debug"))]
macro_rules! debug_mode {
	($command: expr, String) => {};
}

#[cfg(feature = "easy_panic")]
#[macro_export]
macro_rules! easy_panic {
	() => {
		panic!("(Easy panic) Error message should be included in 'debug' mode, else, you can activate RUST_BACKTRACE=1 to see the backtrace.");
	};
}

#[cfg(not(feature = "easy_panic"))]
#[macro_export]
macro_rules! easy_panic {
	() => {};
	($u: expr) => {
		return;
	};
}

pub struct Map<T> {
	pub(crate) keys: Vec<T>,
	pub(crate) values: Vec<T>,
}

fn move_index<T>(vec: &mut Vec<T>, idx: usize, to: usize) {
	let tmp = vec.remove(idx);
	vec.insert(to, tmp);
}

impl<'a, T> Map<T> where T: Literal<String> {
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	pub fn insert(&mut self, key: T, value: T, index: usize) {
		self.keys.insert(index, key);
		self.values.insert(index, value);
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

	pub fn encourage(&mut self, idx: usize, how_much: usize) {
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

	pub fn encourage_by_str(&mut self, str_: T, how_much: usize)
	where
		T: PartialEq<T>,
	{
		let idx = self.keys.iter().position(|x| *x == str_).unwrap();
		self.encourage(idx, how_much);
	}

	pub fn disencourage(&mut self, idx: usize, how_much: usize) {
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

	pub fn disencourage_by_str(&mut self, str_: T, how_much: usize)
	where
		T: PartialEq<T>,
	{
		let idx = self.keys.iter().position(|x| *x == str_).unwrap();
		self.disencourage(idx, how_much);
	}
}

impl<T> From<Vec<(T, T)>> for Map<T> where T: Literal<String> {
	fn from(what: Vec<(T, T)>) -> Self {
		let mut map = Self::new();
		for (key, value) in what {
			map.keys.push(key);
			map.values.push(value);
		}
		map
	}
}
