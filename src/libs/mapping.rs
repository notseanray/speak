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
// ──────────────────────────────────────────────────────────────── I ──────────
//   :::::: D Y N A M I C   M A P S : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────
//

// Dynamic maps are more complex than static maps

/// # DynMap
/// A Dynamic Map is a list of key-value pairs (Like a `HashMap` or a
/// Dictionary), but it is not limited to that, it can be any type that has the
/// `Dyn` trait implemented, even in the same map.
///
/// These types are:
/// - `String`
/// - `&str`
/// - `usize`
///
/// So, it can contain a String, a reference to a String or an usize, being that
/// usize the index of the value you want to get. Take this example about this
/// dynamism:
///
/// ```text
/// +---------------------+      +-------------------+
/// |H|E|L|L|O| |W|O|R|L|D+----->+H|O|L|A| |M|U|N|D|O| <----+ Explicit String
/// +--------+------------+      +-------------------+
/// 							  ^ Points to:
/// +--------+--------+          +------------------+
/// |F|O|O| |&| |B|A|R+--------->+0x0a595bdc55bf2627|  <----+ Address pointing to String
/// +-----------------+          +------------------+
///
/// +---------------------+      +-+
/// |L|O|R|E|M| |I|P|S|U|M+----->+0|                   <----+ Index pointing to
/// +---------------------+      +-+                          the first element
/// ```
///
/// ## Ranking system
/// The ranking system is the way that maps use to encourage or discourage
/// certain strings (or indexes). The way it works is by only analyzing some
/// indexes, indeed, it only analyzes the index if a random number is higher
/// than the ranking of the index. (So it's very hard to analyze the index 1000
/// but easy to analyze the index 10).
///
/// Also, you can modify the `RANGE` variable to guarantee a certain number of
/// strings read, when using the main functions.
///
/// +---------+
/// |         |
/// | Entry 1 |
/// |         | <----+ These ones are in range
/// | Entry 2 |
/// |         |
/// +---------+
///
/// +---------+
/// |         |
/// | Entry 3 |
/// |         |
/// | Entry 4 |        All these are selected "randomly"
/// |         | <----+ taking into account their index
/// | Entry 5 |        number after n (Example: Index 3
/// |         |        - n is Index 1).
/// | Entry 6 |
/// |         |
/// +---------+
///
/// ## Example:
/// ```rust
/// use speak::DynMap;
/// let mut map = DynMap::new();
/// ```

#[derive(Debug)]
pub struct DynMap<'a> {
	pub keys: Vec<&'a str>,
	pub values: Vec<&'a str>,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum DE {
	String,
	Number,
}

pub trait Dyn {
	fn isstr(&self) -> bool;
	fn str(&self) -> &str;
	fn usize(&self) -> usize;
	fn usize_plus<T>(&self, vec: &Vec<T>) -> &str;
}

impl Dyn for DE {
	#[inline]
	fn isstr(&self) -> bool {
		match self {
			DE::String => true,
			DE::Number => false,
		}
	}

	#[inline]
	fn str(&self) -> &str {
		self.confirm_string()
	}

	#[inline]
	fn usize(&self) -> usize {
		self.confirm_usize()
	}

	#[inline]
	fn usize_plus<T>(&self, vec: &Vec<T>) -> &str {
		self.confirm_usize_plus(vec)
	}
}

pub(crate) trait ToUsize {
	fn confirm_usize(&self) -> usize;
	fn confirm_usize_plus<T>(&self, vec: &Vec<T>) -> &str;
}
impl ToUsize for DE {
	#[inline]
	fn confirm_usize(&self) -> usize {
		match self {
			DE::String => panic!("This is not a number!"),
			DE::Number => self.usize(),
		}
	}

	#[inline]
	fn confirm_usize_plus<T>(&self, vec: &Vec<T>) -> &str {
		match self {
			DE::String => panic!("This is not a number!"),
			DE::Number => self.str(),
		}
	}
}

pub trait ToString {
	fn confirm_string(&self) -> &str;
}

impl ToString for DE {
	#[inline]
	fn confirm_string(&self) -> &str {
		match self {
			DE::String => self.str(),
			DE::Number => panic!("This is not a string!"),
		}
	}
}

// This function sees if the index takes to an usize, in that case, it calls
// itself again
pub(crate) fn take_to_root<'a>(vec: &'a Vec<&'a DE>, index: usize) -> &'a str {
	match vec[index].isstr() {
		false => take_to_root(vec, vec[index].usize()),
		true => vec[index].str(),
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

macro_rules! is_str {
	($($e: expr),*) => {
		$(
			match $e {
				DE::String => true,
				DE::Number => false
			}
		)*
	};
}

macro_rules! define {
	($e: expr, $vec: expr) => {
		if $e.isstr() {
			$e.str()
		} else {
			$e.usize_plus(&$vec)
		}
	};
}

/// This function is used to move an index to a new index.
#[macro_export]
macro_rules! map {
	($(($e1: expr, $e2: expr)),*) => {
		{
			let mut temp = speak::DynMap::new();
			$(
				temp.push((speak::define!($e1), speak::define!($e2)));
			)*
			temp
		}
	};
}

impl<'a> DynMap<'a> {
	#[inline]
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	#[inline]
	pub fn push(&mut self, to_insert: (&'a DE, &'a DE)) {
		self.keys.push(define!(to_insert.0, self));
		self.values.push(define!(to_insert.1, self));
	}

	#[inline]
	pub fn insert(&mut self, to_insert: (&'a DE, &'a DE), index: usize) {
		self.keys.insert(index, define!(to_insert.0, self));
		self.values.insert(index, define!(to_insert.1, self));
	}

	#[inline]
	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	pub fn pop(&mut self) -> (Option<&'a DE>, Option<&'a DE>) {
		(self.keys.pop(), self.values.pop())
	}

	#[inline]
	pub fn remove(&mut self, index: usize) -> (&'a DE, &'a DE) {
		(
			define!(self.keys).remove(index),
			define!(self.values).remove(index),
		)
	}

	#[inline]
	pub fn move_tuple(&mut self, index: usize, to: usize) {
		move_index(&mut self.keys, index, to);
		move_index(&mut self.values, index, to);
	}

	#[inline]
	pub fn search_key(&self, key: &str) -> Option<usize> {
		self.keys.iter().position(|&k| k == key)
	}

	#[inline]
	pub fn search_value(&self, value: &str) -> Option<usize> {
		self.values
			.iter()
			.position(|&v| v.confirm_string() == value)
	}

	// Searches for a tuple, being formed by a key-value pair.
	#[inline]
	pub fn search_tuple(&self, tuple: (&str, &str)) -> Option<usize> {
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
	pub fn encourage_by_str(&mut self, string: &str, how_much: usize) {
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
	pub fn discourage_by_str(&mut self, string: &str, how_much: usize) {
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

// Iterator
impl<'a> Iterator for DynMap<'a> {
	type Item = (&'a DE, &'a DE);
	fn next(&mut self) -> Option<Self::Item> {
		if self.keys.len() == 0 {
			return None;
		}
		Some((self.keys.remove(0), self.values.remove(0)))
	}
}
