use crate::Literal;

pub struct Map<T> {
	pub keys: Vec<T>,
	pub values: Vec<T>,
}

impl<T> Map<T>
where
	T: Literal<String>,
{
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

	// These methods are used for more complex operations, like encouraging the map to analyze a certain key more often.

	fn encourage(&self, index: usize) {
		todo!()
	}

	fn discourage(&self, index: usize) {
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

impl<T> Iterator for Map<T> {
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

pub struct DynMap<'item, T>
where
	T: Dyn,
{
	pub keys: Vec<(&'item T, u8)>,
	pub values: Vec<T>,
}

pub trait Dyn: DynS + DynN {}

// Dynamic for String
pub trait DynS {
	fn _type() -> u8;
}

// Dynamic for Number
pub trait DynN {
	fn _type() -> u8;
}

impl<S: IsString> DynS for S {
	#[inline]
	fn _type() -> u8 {
		0b00000000
	}
}

impl<Usize: IsUsize> DynN for Usize {
	#[inline]
	fn _type() -> u8 {
		0b11111111
	}
}

trait IsString {}

impl IsString for String {}
impl IsString for &String {}

impl IsString for str {}
impl IsString for &str {}

trait IsUsize {}
impl IsUsize for usize {}

/*

+---------------------+      +-------------------+
|H|E|L|L|O| |W|O|R|L|D+----->+H|O|L|A| |M|U|N|D|O| <----+ Explicit String
+---------------------+      ++------------------+
							  ^
+-----------------+          ++-----------------+
|F|O|O| |&| |B|A|R+--------->+0x4a595bdc55bf2627|  <----+ Address pointing to String
+-----------------+          +------------------+

+---------------------+      +-+
|L|O|R|E|M| |I|P|S|U|M+----->+0|                   <----+ Index pointing to the first element
+---------------------+      +-+

*/

// This is how a dynamic map is used. While the normal map just accepts `String`(s), the dynamic map accepts all types that implement the `Dyn` trait. This means that the map can be used to store various types, even in the same map. And example of this is the previous map, that stores a string, an address pointing to that string and an index pointing to the first element, being, again, that string.
