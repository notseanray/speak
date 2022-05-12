use std::ops::{Index, Range};
use crate::Literal;

pub struct Map<T> {
	pub keys: Vec<T>,
	pub values: Vec<T>,
}

impl<T> Map<T> where T: Literal<String> {
	#[inline]
	fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	#[inline]
	fn push(&self, to_insert: (T, T)) {
		self.keys.push(to_insert.0);
		self.values.push(to_insert.1);
	}

	#[inline]
	fn insert(&self, to_insert: (T, T), index: usize) {
		self.keys.insert(index, to_insert.0);
		self.values.insert(index, to_insert.1);
	}

	#[inline]
	fn clear(&self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	fn pop(&self) -> (Option<T>, Option<T>) {
		(self.keys.pop(), self.values.pop())
	}

	#[inline]
	fn remove(&self, index: usize) -> (T, T) {
		(self.keys.remove(index), self.values.remove(index))
	}

	#[inline]
	fn get(&self, index: usize) -> (T, T) {
		(self.keys[index], self.values[index])
	}

	#[inline]
	fn get_key(&self, index: usize) -> T {
		self.keys[index]
	}

	#[inline]
	fn get_value(&self, index: usize) -> T {
		self.values[index]
	}

	//
	// ────────────────────────────────────────────────────────────────────── I ──────────
	//   :::::: C O M P L E X   M E T H O D S : :  :   :    :     :        :          :
	// ────────────────────────────────────────────────────────────────────────────────
	//

	// These methods are used for more complex operations.

	fn encourage(&self, index: usize) {
		unimplemented!()
	}

	fn discourage(&self, index: usize) {
		unimplemented!()
	}

	fn promote(&self, index: T) {
		unimplemented!()
	}
}

// TODO i don't know how to constrain this `'a` lifetime
// impl<'a, T> Index<Range<usize>> for Map<T> {
// 	type Output = (&'a [T], &'a [T]);
// 	fn index(&self, index: Range<usize>) -> (&[T], &[T]) {
// 		return (&self.keys[index], &self.values[index]);
// 	}
// }

impl<T> Index<usize> for Map<T> {
	type Output = (T, T);
	fn index(&self, index: usize) -> &(T, T) {
		&(self.keys[index], self.values[index])
	}
}

impl<T> Index<String> for Map<T> where T: PartialEq + Literal<String> {
	type Output = (T, T);
	fn index(&self, index: String) -> &(T, T) {
		// Search the index of the key
		let index = self.keys.iter().position(|&x| x.literal() == index);
		match index {
			Some(index) => &(self.keys[index], self.values[index]),
			None => panic!("Key not found"),
		}
	}
}
