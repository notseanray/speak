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
#[doc(hidden)]
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
	/// The list of keys (Because `DynMap`(s) are formed of key-value pairs)
	pub keys: Vec<&'a str>,
	/// The list of values (Because `DynMap`(s) are formed of key-value pairs)
	pub values: Vec<&'a str>,
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub enum DE<'s> {
	String(&'s str),
	Number(usize),
}

#[doc(hidden)]
pub trait Dyn {
	fn to_enum(&self) -> DE;
	fn to_str(&self) -> &str;
	fn to_usize(&self) -> usize;
}

impl<'a> Dyn for DE<'a> {
	#[inline]
	fn to_enum(&self) -> DE {
		*self
	}

	#[inline]
	fn to_str(&self) -> &str {
		match *self {
			DE::String(_) => self.to_str(),
			DE::Number(_) => panic!("This is not a String, this is a number."),
		}
	}

	#[inline]
	fn to_usize(&self) -> usize {
		match *self {
			DE::String(_) => panic!("This is a String, not a Number."),
			DE::Number(_) => self.to_usize(),
		}
	}
}

impl Dyn for &str {
	#[inline]
	fn to_enum(&self) -> DE {
		DE::String(self)
	}

	#[inline]
	fn to_str(&self) -> &str {
		self
	}

	#[inline]
	fn to_usize(&self) -> usize {
		panic!("This is a String, not a Number.");
		// 0
	}
}

impl Dyn for usize {
	#[inline]
	fn to_enum(&self) -> DE {
		DE::Number(*self)
	}

	#[inline]
	fn to_str(&self) -> &str {
		panic!("This is a Number, not a String.");
		// "x"
	}

	#[inline]
	fn to_usize(&self) -> usize {
		*self
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

#[doc(hidden)]
macro_rules! define {
	($e: expr, $vec: expr) => {
		match $e {
			DE::String(_) => $e.to_str(),
			DE::Number(_) => $vec[$e.to_usize()]
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! is_string_m {
	($e: expr) => {
		match $e {
			&str => true,
			_ => false,
		}
	};
}

#[doc(hidden)]
pub trait IsString {
	fn is_string(&self) -> bool;
}

impl IsString for &str {
	fn is_string(&self) -> bool {
		true
	}
}

impl IsString for usize {
	fn is_string(&self) -> bool {
		false
	}
}

// If you're wondering why use can use map![...] instead of map!(), all macros
// can do it, but we only do it with some because it's idiomatic.
#[macro_export]
/// <h1>map![...]</h1>
///
/// This macro is used to create a dynamic map. The recommended usage is by
/// using it with square brackets, it takes an undefined number of tuples of the
/// form `(key, value)`. It has a **special syntax!** You can also input a
/// number! For example `(key, 3)` To get the third index of the map. (You can
/// also use it with keys, or even with both.)
///
/// ## Example
/// ```rust
/// use speak::DynMap;
/// let map = map![
///     ("Hi, how are you?", "I'm good, thanks!"),
///     ("So... How is your pet?", "Elizabeth the III is fine"),
///     ("What's your favourite animal?", "I love cats!")
///     ("Do you like cats?", 2) // Here, while the key is different,
///                                 the value is pointing to the second index,
///                                 the "I love cats!" phrase.
/// //    ...
/// ];
/// ```
/// You can see the definition of the macro in the <docs.rs> page or building it
/// with `rustdoc`.
macro_rules! map {
	($(($e1: expr, $e2: expr)),*) => {
		{
			let mut temp = speak::DynMap::new();
			$(
				temp.keys.push(
					match &$e1.is_string() {
						true => $e1.to_str(),
						false => temp.keys[$e1.to_usize()]
					});

					temp.values.push(match &$e2.is_string() {
						true => $e2.to_str(),
						false => temp.values[$e2.to_usize()]
					});
			)*
			temp
		}
	};
}

impl<'a> DynMap<'a> {
	#[inline]
	/// <h1>new()</h1>
	///
	/// <p>
	///
	/// This inline function creates a new empty dynamic map, If you want to
	/// create a map with values, you can use the `map!` macro.
	///
	/// </p>
	///
	/// ## Example
	/// ```rust
	/// use speak::DynMap;
	/// let map = DynMap::new();
	/// ```
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	#[inline]
	/// <h1>push(...)</h1>
	///
	/// The `push` function is used to push a tuple to the end of the map. It
	/// takes a `&str` type. You can use `push_dyn` to push a dynamic type
	/// (Admiting numbers and references to strings).
	///
	/// You can use the `map![...]` macro to create a map from a list of tuples.
	///
	/// ## Example
	/// ```rust
	/// use speak::DynMap;
	/// let mut map = DynMap::new();
	/// map.push(("Hello World", "Hola mundo!"));
	/// ```
	pub fn push(&mut self, to_insert: (&'a str, &'a str)) {
		self.keys.push(to_insert.0);
		self.values.push(to_insert.1);
	}

	#[inline]
	/// <h1>push_dyn(...)</h1>
	///
	/// The `push_dyn` function is used to push a tuple to the end of the map.
	/// It takes a dynamic type. You can use `push` to push a `&str` type.
	///
	/// ## Example
	/// ```rust
	/// use speak::DynMap;
	/// let mut map = DynMap::new();
	/// map.push_dyn((&DE::String("Hello World"), &DE::String("Hola mundo!"))); // You can also use a number, instead of a string (referencing an index of the same map.)
	/// ```
	pub fn push_dyn(&mut self, to_insert: (&'a DE, &'a DE)) {
		self.keys.push(define!(to_insert.0, self.keys));
		self.values.push(define!(to_insert.1, self.values));
	}

	#[inline]
	/// <h1>insert_dyn(...)</h1>
	///
	/// The `insert_dyn` function is used to insert a tuple to the map at a
	/// given index. It takes a dynamic type. You can use `insert` to insert a
	/// `&str` type. Take into account that this function is as effient as the
	/// built-in `insert` function.
	///
	/// ## Example
	/// ```rust
	/// use speak::DynMap;
	///
	/// fn main() {
	///
	/// 	let mut map = DynMap::new();
	/// 	map.insert_dyn((&DE::String("Hello World"), &DE::String("Hola mundo!")), 0); // You can also use a number, instead of a string (referencing an index of the same map.)
	/// }
	pub fn insert_dyn(&mut self, to_insert: (&'a DE, &'a DE), index: usize) {
		self.keys.insert(index, define!(to_insert.0, self.keys));
		self.values.insert(index, define!(to_insert.1, self.values));
	}

	#[inline]
	/// <h1>insert(...)</h1>
	///
	/// The `insert` function is used to insert a tuple to the map at a
	/// given index. It takes a `&str` type. You can use `insert_dyn` to
	/// insert a dynamic type (Admiting numbers and references to strings). Take
	/// into account that this function is as effient as the built-in `insert`
	/// function.
	///
	/// ## Example
	/// ```rust
	/// use speak::DynMap;
	///
	/// fn main() {
	/// 	let mut map = DynMap::new();
	/// 	map.insert(("Hello World", "Hola mundo!"), 0);
	/// }
	///```
	pub fn insert(&mut self, to_insert: (&'a str, &'a str), index: usize) {
		self.keys.insert(index, to_insert.0);
		self.values.insert(index, to_insert.1);
	}

	#[inline]
	/// <h1>clear()</h1>
	/// 
	/// This function is used to clear the map. It clears both the keys and the values.
	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	/// <h1>pop()</h1>
	/// It removes and returns the last element of the map.
	/// It returns a tuple of the removed element. (If it exists)
	pub fn pop(&mut self) -> (Option<&'a str>, Option<&'a str>) {
		(self.keys.pop(), self.values.pop())
	}

	#[inline]
	/// <h1>remove()</h1>
	/// It removes and returns the element at the given index.
	/// It returns a tuple of the removed element. (If it exists)
	pub fn remove(&mut self, index: usize) -> (&'a str, &'a str) {
		(self.keys.remove(index), self.values.remove(index))
	}

	#[inline]
	/// <h1>move_tuple(...)</h1>
	/// It moves the element at the given index to the index of the second parameter.
	pub fn move_tuple(&mut self, index: usize, to: usize) {
		move_index(&mut self.keys, index, to);
		move_index(&mut self.values, index, to);
	}

	#[inline]
	/// <h1>search_key(...)</h1>
	/// Searchs for the given key in the map. It has a O(n) complexity at max.
	pub fn search_key(&self, key: &str) -> Option<usize> {
		self.keys.iter().position(|&k| k == key)
	}

	#[inline]
	/// <h1>search_value(...)</h1>
	/// Searchs for the given value in the map. It has a O(n) complexity at max.
	pub fn search_value(&self, value: &str) -> Option<usize> {
		self.values.iter().position(|&v| v == value)
	}

	// Searches for a tuple, being formed by a key-value pair.
	#[inline]
	/// <h1>search_tuple(...)</h1>
	/// Searchs for the given tuple in the map, it takes a tuple of the form (key, value).
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
	/// <h1>len()</h1>
	/// Returns the number of elements in the map.
	pub fn len(&self) -> usize {
		self.keys.len()
	}

	#[inline]
	/// <h1>is_empty()</h1>
	/// Returns true if the map is empty.
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

	/// <h1>encourage(...)</h1>
	/// It encourages the given key, by ranking it higher. (See also [ranking system])
	// TODO: Add the ranking system
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
	type Item = (&'a str, &'a str);
	fn next(&mut self) -> Option<Self::Item> {
		if self.keys.len() == 0 {
			return None;
		}
		Some((self.keys.remove(0), self.values.remove(0)))
	}
}
