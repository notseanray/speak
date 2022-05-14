use crate::Literal;
pub struct Map<T> {
	pub keys: Vec<T>,
	pub values: Vec<T>,
}

impl<T> Map<T> where T: Literal<String> {
	#[inline]
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	#[inline]
	pub fn push(&mut self, to_insert: (T, T)) {
		self.keys.push(to_insert.0);
		self.values.push(to_insert.1);
	}

	#[inline]
	pub fn insert(&mut self, to_insert: (T, T), index: usize) {
		self.keys.insert(index, to_insert.0);
		self.values.insert(index, to_insert.1);
	}

	#[inline]
	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	pub fn pop(&mut self) -> (Option<T>, Option<T>) {
		(self.keys.pop(), self.values.pop())
	}

	#[inline]
	pub fn remove(&mut self, index: usize) -> (T, T) {
		(self.keys.remove(index), self.values.remove(index))
	}

	//
	// ────────────────────────────────────────────────────────────────────── I
	// ──────────   :::::: C O M P L E X   M E T H O D S : :  :   :    :     :
	// :          :
	// ────────────────────────────────────────────────────────────────────────────────
	//

	// These methods are used for more complex operations, like encouraging the map
	// to analyze a certain key more often.

	pub fn encourage(&self, index: usize) {
		todo!()
	}

	pub fn discourage(&self, index: usize) {
		todo!()
	}
}

// TODO i don't know how to constrain this `'a` lifetime
// impl<T> Index<Range<usize>> for Map<T> {
// 	type Output = (&'a [T], &'a [T]);
// 	fn index(&self, index: Range<usize>) -> (&[T], &[T]) {
// 		return (&self.keys[index], &self.values[index]);
// 	}
// }

impl<T> Iterator for Map<T> where T: Literal<String> {
	type Item = (T, T);

	#[inline]
	fn next(&mut self) -> Option<(T, T)> {
		match self.keys.pop() {
			Some(k) => match self.values.pop() {
				Some(v) => Some((k, v)),
				None => None,
			},
			None => None,
		}
	}
}

//
// ──────────────────────────────────────────────────────────────── I ──────────
//   :::::: D Y N A M I C   M A P S : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────
//

// Dynamic maps are more complex than static maps

pub struct DynMap<T>
where
	T: Dyn,
{
	pub keys: Vec<T>,
	pub values: Vec<T>,
}

pub trait Dyn {
	// Being that integer 255 (0b11111110) for a Literal, 240 (0b11110000) is the
	// code for a &Literal and 0 (0b0) is the code for an usize.
	fn _type() -> u8;
}

impl Dyn for &str {
	fn _type() -> u8 {
		255 // 0b11111110
	}
}

impl Dyn for String {
	fn _type() -> u8 {
		240 // 0b11110000
	}
}

impl Dyn for usize {
	fn _type() -> u8 {
		0 // 0b0
	}
}

/*

+---------------------+      +-------------------+
|H|E|L|L|O| |W|O|R|L|D+----->+H|O|L|A| |M|U|N|D|O| <----+ Explicit String
+---------------------+      ++------------------+
							  ^ Points to:
+-----------------+          ++-----------------+
|F|O|O| |&| |B|A|R+--------->+0x0a595bdc55bf2627|  <----+ Address pointing to String
+-----------------+          +------------------+

+---------------------+      +-+
|L|O|R|E|M| |I|P|S|U|M+----->+0|                   <----+ Index pointing to the first element
+---------------------+      +-+

*/

// This is how a dynamic map is used. While the normal map just accepts
// Strings the dynamic map accepts all types that implement the `Dyn`
// trait. This means that the map can be used to store various types, even in
// the same map, the map can store:
//
// - Strings
// - References to Strings.
// - Indexes to strings (numbers, being Usize)

// Adding references to an usize is not necessary, because a reference is just a
// number, so it would be the double of the size of a number.

// * Ok, now we can start with the main implementations

impl<T> DynMap<T> where T: Dyn {
	#[inline]
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	#[inline]
	pub fn push(&mut self, to_insert: (T, T)) {
		self.keys.push(to_insert.0);
		self.values.push(to_insert.1);
	}

	#[inline]
	pub fn insert(&mut self, to_insert: (T, T), index: usize) {
		self.keys.insert(index, to_insert.0);
		self.values.insert(index, to_insert.1);
	}

	#[inline]
	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	pub fn pop(&mut self) -> (Option<T>, Option<T>) {
		(self.keys.pop(), self.values.pop())
	}
	
	#[inline]
	pub fn remove(&mut self, index: usize) -> (T, T) {
		(self.keys.remove(index), self.values.remove(index))
	}

	//
	// ────────────────────────────────────────────────────────────────────── I ──────────
	//   :::::: C O M P L E X   M E T H O D S : :  :   :    :     :        :          :
	// ────────────────────────────────────────────────────────────────────────────────
	//

	pub fn encourage(&self, index: usize) {
		todo!()
	}

	pub fn discourage(&self, index: usize) {
		todo!()
	}
}

// * Let's implement some other traits.

impl<T> From<Vec<(T, T)>> for DynMap<T> where T: Dyn {
	fn from(to_insert: Vec::<(T, T)>) -> Self {
		let mut new: Self = Self::new();
		for (k, v) in to_insert {
			new.values.push(v);
			new.keys.push(k);
		}

		new
	}
}
