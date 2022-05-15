use crate::Literal;
use std::fmt;

//
// ──────────────────────────────────────────────────────── I ──────────
//   :::::: F E A T U R E S : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────
//

#[cfg(feature = "easy_panic")]
use colored::Colorize;

#[cfg(feature = "easy_panic")]
#[macro_use]
macro_rules! easy_panic {
	() => {
		panic!("Easy Panic: The panic message should be in the debug information.");
	};

	($($arg:tt)*) => {
		panic!("Easy Panic: ", $($arg)*);
	};
}

#[cfg(not(feature = "easy_panic"))]
#[macro_export]
macro_rules! easy_panic {
	() => {};
	($($arg:tt)*) => {};
}

//
// ────────────────────────────────────────────────────────── I ──────────
//   :::::: M A I N   M A P S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────
//

#[derive(Debug)]
pub struct Map<T> {
	pub keys: Vec<T>,
	pub values: Vec<T>,
}

impl<T> Map<T>
where
	T: Literal<String>,
{
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

impl<T> Iterator for Map<T>
where
	T: Literal<String>,
{
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

// I cannot implement this.
#[inline]
fn move_index<T>(vec: &mut Vec<T>, idx: usize, to: usize) {
	let tmp = vec.remove(idx);
	vec.insert(to, tmp);
}

impl<T> DynMap<T>
where
	T: Dyn,
	Vec<T>: Copy,
{
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

	#[inline]
	pub fn move_tuple(&mut self, index: usize, to: usize) {
		move_index(&mut self.keys, index, to);
		move_index(&mut self.values, index, to);
	}

	#[inline]
	pub fn search_key(&self, key: &T) -> Option<usize>
	where
		T: PartialEq,
	{
		self.keys.iter().position(|k| k == key)
	}

	#[inline]
	pub fn search_value(&self, value: &T) -> Option<usize>
	where
		T: PartialEq,
	{
		self.values.iter().position(|v| v == value)
	}

	// Searches for a tuple, being formed by a key-value pair.
	#[inline]
	pub fn search_tuple(&self, tuple: (&T, &T)) -> Option<usize>
	where
		T: PartialEq,
	{
		match self.search_key(tuple.0) {
			Some(_) => match self.search_value(tuple.1) {
				Some(idx) => Some(idx),
				None => None,
			},
			None => None,
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.keys.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.keys.len() == 0
	}

	//
	// ────────────────────────────────────────────────────────────────────── I
	// ──────────   :::::: C O M P L E X   M E T H O D S : :  :   :    :     :
	// :          :
	// ────────────────────────────────────────────────────────────────────────────────
	//

	// Maybe you're wondering "¿How these work?", well, I'm going to explain them:
	//
	// Imagine you have a list of 200 strings, and you have to read until the end,
	// You could read every single one, but that would take a lot of time. So,
	// thinking about it you come to the conclusion that you can read the most
	// important ones first, and then you can toss a coin to see if you should read
	// the next one. That takes the (Time to read * Amount guaranteed to read) +
	// (Total amount of strings - Amount read) / 2 (If the coin toss is completely
	// random).

	// Ok, that's very good, now you can read less stings than you thought, but,
	// what if you can continue? Let's, instead, roll a dice, a magic dice, and for
	// each time you roll the dice, you add a new side to the dice. So, if you roll
	// the dice 2 times, the roll has 2 sides, if you roll the dice 3 times, the
	// roll has 3 sides, and so on.

	// The dice starts with the amount of guaranteed reads, what happens when you
	// roll the dice? If the roll is less than the amount of guaranteed reads, you
	// read the next string, otherwise, pass to the next roll. So that the roll no.
	// 34 is more probable than the roll no. 35, and so on.

	// This is a distribution, that gives us the power to `rank` all the entries,
	// encouraging some 'good' entries with the probability of being analyzed, and
	// disencouraging the 'bad' ones with the probability of not being analyzed.

	/*

	+---------+
	|         |
	| Entry 1 |
	|         | <----+ This ones are in range
	| Entry 2 |
	|         |
	+---------+

	+---------+
	|         |
	| Entry 3 |
	|         |
	| Entry 4 |        All these are selected "randomly"
	|         | <----+ taking into account their index
	| Entry 5 |        number after n (Example: Index 3
	|         |        - n is Index 1).
	| Entry 6 |
	|         |
	+---------+

	*/

	// In this case, encouraging is just the ranking system taking into account, so,
	// we can encourage a key by ranking it higher.
	pub fn encourage(&mut self, index: usize, how_much: usize) {
		if index < how_much || index >= self.keys.len() {
			easy_panic!("Index out of bounds, make sure that 'how much' is less than the index from which you want to encourage: {} - {} is less than 0 (It underflows) AND make sure that the index is less than the length of the map.", index, how_much);
		} else {
			self.move_tuple(index, index - how_much);
		}
	}

	#[inline]
	pub fn encourage_unchecked(&mut self, index: usize, how_much: usize) {
		self.move_tuple(index, index - how_much);
	}

	#[inline]
	pub fn encourage_by_str(&mut self, string: T, how_much: usize)
	where
		T: PartialEq,
	{
		let idx = self
			.search_key(&string)
			.unwrap_or_else(|| panic!("String not found"));

		self.encourage(idx, idx - how_much);
	}

	pub fn discourage(&mut self, index: usize, how_much: usize) {
		if index >= self.keys.len() || index + how_much >= self.keys.len() {
			easy_panic!("Index out of bounds, make sure that index ({}) + how much you want to disencourage ({}) is less than the total length of the map ({})", index, how_much, self.keys.len());
		};
		self.move_tuple(index, index + how_much);
	}

	#[inline]
	pub fn discourage_unchecked(&mut self, index: usize, how_much: usize) {
		self.move_tuple(index, index + how_much);
	}

	#[inline]
	pub fn discourage_by_str(&mut self, string: T, how_much: usize)
	where
		T: PartialEq,
	{
		let idx = self
			.search_key(&string)
			.unwrap_or_else(|| panic!("String not found"));

		self.discourage(idx, how_much);
	}

	// fn into_tuples(self) -> Vec<(T, T)> {
	// 	self.keys
	// 		.into_iter()
	// 		.zip(self.values.into_iter())
	// 		.collect::<Vec<(T, T)>>()
	// }
}

// ─── OTHER IMPLEMENTATIONS ──────────────────────────────────────────────────

// From:
impl<T> From<Vec<(T, T)>> for DynMap<T>
where
	T: Dyn, Vec<T>: Copy
{
	fn from(to_insert: Vec<(T, T)>) -> Self {
		let mut new: Self = Self::new();
		for (k, v) in to_insert {
			new.values.push(v);
			new.keys.push(k);
		}
		return new;
	}
}
