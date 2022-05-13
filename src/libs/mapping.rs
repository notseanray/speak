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
	fn push(&mut self, to_insert: (T, T)) {
		self.keys.push(to_insert.0);
		self.values.push(to_insert.1);
	}

	#[inline]
	fn insert(&mut self, to_insert: (T, T), index: usize) {
		self.keys.insert(index, to_insert.0);
		self.values.insert(index, to_insert.1);
	}

	#[inline]
	fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	fn pop(&mut self) -> (Option<T>, Option<T>) {
		(self.keys.pop(), self.values.pop())
	}

	#[inline]
	fn remove(&mut self, index: usize) -> (T, T) {
		(self.keys.remove(index), self.values.remove(index))
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
}

// TODO i don't know how to constrain this `'a` lifetime
// impl<T> Index<Range<usize>> for Map<T> {
// 	type Output = (&'a [T], &'a [T]);
// 	fn index(&self, index: Range<usize>) -> (&[T], &[T]) {
// 		return (&self.keys[index], &self.values[index]);
// 	}
// }

impl<T> Iterator for Map<T> {
	type Item = (T, T);
	
	#[inline]
	fn next(&mut self) -> Option<(T, T)> {
		match self.keys.pop() {
			Some(k) => match self.values.pop() {
				Some(v) => Some((k, v)),
				None => None
			},
			None => None
		}
	}
}