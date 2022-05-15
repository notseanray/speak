//! Speak crate made by Alex G. C. See LICENSE for more
//! information about the copyright.
// If you want to see the utils scroll to the last line of the file.

#![doc = document_features::document_features!()]
#![allow(non_snake_case)]

//
// ────────────────────────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N   A N D   U T I L S : :  :   :    :     :
// :          :
// ────────────────────────────────────────────────────────────────────────────────────────────────
//

#[cfg(feature = "fancy_docs")]
#[cfg_attr(doc, aquamarine::aquamarine)]
///## Memory
///
///Every phrase is made up from words. We make a phrase from adding sequences
/// of words together. Well, the `MEMORY` parameter is used to define how many
/// words we take into account into analyzing a phrase.
///
///The functions that takes this parameter take into account that maybe the
/// length of the phrase divided by the number of words in the phrase is not an
/// integer. So this functions will take into account until the last words, and
/// then scan the words between the length of the phrase minus the memory and
/// the length of the word.
///
///```mermaid
/// graph TD
/// 	A("Hi,")
/// 	B("my")
/// 	C{{"name"}}
/// 	D("is")
/// 	E("Alex")
///
/// 	F["Not found!"]
///
/// 	style F stroke-dasharray: 5 5
///
/// 	X["Iteration 1"]
/// 	Y["Iteration 2"]
/// 	Z["Bugged iteration 2"]
///
/// 	X-->A;
/// 	X-->B;
/// 	X-->C;
///
/// 	Y-->C;
/// 	Y-->D;
/// 	Y-->E;
///
/// 	Z-->D;
/// 	Z-->E;
/// 	Z-->F;
/// ```
///
///###### Honestly, I just wanted to show you how it works, and this graph.
pub const DEFAULT_MEMORY: usize = 2;

#[cfg(not(feature = "fancy_docs"))]
///## Memory
///
///Every phrase is made up from words. We make a phrase from adding sequences
/// of words together. Well, the `MEMORY` parameter is used to define how many
/// words we take into account into analyzing a phrase.
///
///The functions that takes this parameter take into account that maybe the
/// length of the phrase divided by the number of words in the phrase is not an
/// integer. So this functions will take into account until the last words, and
/// then scan the words between the length of the phrase minus the memory and
/// the length of the word.
pub const DEFAULT_MEMORY: usize = 2;

///## Threshold
///As you know, we divide two values to find their relations. Well, that
/// relation is then checked against the threshold, if it doesn't passes the
/// threshold, the word is not elected. This is the operation to determine if a
/// word is elected. As you can see, if the threshold is too low (less than 0.1
/// is not recommended), the word "spaghetti" and the word "spagetti" will not
/// be relationated. But if the threshold is too high (more than 0.3 is not
/// recommended), a lot of words, even if they are very different, will be
/// relationated and the final result will not have sense.
pub const DEFAULT_THRESHOLD: f32 = 0.1;
pub const DEFAULT_MAX_OUTPUT_LENGTH: usize = 2;

#[cfg(feature = "fancy_docs")]
#[cfg_attr(doc, aquamarine::aquamarine)]
/// <h1>Randomness</h1>
///
/// ### What does this mean?
/// There's two ways the algorithm works, the first way is **analyzing every
/// single entry**, this method is slow, and doesn't have the ability to
/// *encourage* or *disencourage* some entry.
///
/// The second method is **analyzing every single entry until a break point,
/// then aplying a distribution**, this method is more fast, when the break
/// point is reached, the algorithm will start to ignore some cases. The
/// distribution used is very simple just: <h3 align="center"><img src="https://render.githubusercontent.com/render/math?math=\bbox[%230d1117]{\color{%23fff}{%5Cbigg%5C%7B%5Cbegin%7Barray%7D%7Bll%7D%09i%20%5Cleq%20%5Ctext%7Brange%7D%20%26%20%5Cdotsc%09%5C%5C%09i%20%3E%20%5Ctext%7Brange%7D%20%26%20R%5Cin%5C%7B0%2C...%2C%5C%23V%5C%7D%5C%20%5Cbigg%5C%7B%5Cbegin%7Barray%7D%7Bll%7D%09%09R%20%3C%20i%20%26%20%5Cdotsc%09%09%5C%5C%09%09R%20%5Cgeq%20i%20%26%20%5Ctext%7Bpass%7D%09%5Cend%7Barray%7D%5Cend%7Barray%7D}}" /></h3>
///
/// The distribution is very simple, and just random enough to serve our
/// purpose. ### Why use a distribution?
/// Activating the randomness will change the way that the `run` algorithm
/// works, adding a new system, the *ranking system*. The ranking system will
/// take into account just the first `RANGE` entries, and then will use the
/// distribution, so the last entry is very unlikely to be analyzed, but the
/// first one after the range is almost guaranteed to be analyzed. We use this
/// because now we can *rank* the entries, encouraging or disencouraging them by
/// changing the index.
pub const DEFAULT_RANGE: usize = 2;

#[cfg(not(feature = "fancy_docs"))]
/// <h1>Randomness</h1>
/// Randomness is an optional (but highly recommended) feature that will pass
/// some randomness to the algorithm.
///
/// ### What does this mean?
/// There's two ways the algorithm works, the first way is **analyzing every
/// single entry**, this method is slow, and doesn't have the ability to
/// *encourage* or *disencourage* some entry.
///
/// The second method is **analyzing every single entry until a break point,
/// then aplying a distribution**, this method is more fast, when the break
/// point is reached, the algorithm will start to ignore some cases. The
/// distribution used is very simple.
///
/// The distribution is very simple, and just random enough to serve our
/// purpose. ### Why use a distribution?
/// Activating the randomness will change the way that the `run` algorithm
/// works, adding a new system, the *ranking system*. The ranking system will
/// take into account just the first `RANGE` entries, and then will use the
/// distribution, so the last entry is very unlikely to be analyzed, but the
/// first one after the range is almost guaranteed to be analyzed. We use this
/// because now we can *rank* the entries, encouraging or disencouraging them by
/// changing the index.
pub const DEFAULT_RANGE: usize = 3;

// ↑
// $$
// \bigg\{\begin{array}{ll}
// 	i \leq X & \dotsi
// 	\\
// 	i \geq X & R\big\{\begin{array}{ll}

// 		1 & \dotsi
// 		\\
// 		0 & \text{continue}
// 	\end{array}
// \end{array}
// $$

fn translate<T: Literal<String>>(vec: &Vec<T>) -> Vec<Vec<u32>> {
	let mut result: Vec<Vec<u32>> = Vec::new();
	let mut new_phrase: Vec<u32> = Vec::new();
	let mut sum: u32 = 0;
	for phrase in vec {
		for word in phrase.literal().split_whitespace() {
			for c in word.chars() {
				sum += c as u32;
			}
			// I just did this, this implementation is 0.3 ms faster
			new_phrase.push(((sum << 1) + 1) << 1 + 1);
			sum = 0;
		}
		result.push(new_phrase.clone());
		new_phrase.clear()
	}

	return result;
}

// fn merge_hashmaps<T: std::hash::Hash + std::cmp::Eq>(map1: HashMap<T, T>,
// map2: HashMap<T, T>) -> HashMap<T, T> { 	map1.into_iter().chain(map2).
// collect() }

// Long calculation I don't want to explain.
macro_rules! calculation {
	($MChunk: expr, $IChunk: expr, $VChunk: expr) => {
		($MChunk.iter().sum::<f32>()
			- ($IChunk.iter().sum::<u32>() as f32 / $VChunk.iter().sum::<u32>() as f32))
			.abs()
	};
}

// If the debug mode is enabled, print those statements, else, do nothing.

#[cfg(feature = "debug")]
use colored::Colorize;

#[cfg(feature = "debug")]
macro_rules! debug_mode {
	($command: expr, $($args: expr), *) => {
		println!("{} {}", "debug".bold().red(), format!($command, $($args), *).bright_yellow());
	};
	($command: expr) => {
		println!("{}", format!($command).bright_yellow());
	};
}

#[cfg(not(feature = "debug"))]
macro_rules! debug_mode {
	($command: expr, $($args: expr), *) => {};
	($command: expr, String) => {};
}

#[cfg(feature = "randomness")]
macro_rules! check_for_random {
	($i: expr, $range: expr) => {
		if rand::thread_rng().gen_range(
			0..({
				if $i >= $range {
					$range
				} else {
					$i + 1
				}
			} + 1),
		) < $range
		{
			println!("Passed {}", $i);
		}
	};
}

#[cfg(not(feature = "randomness"))]
macro_rules! check_for_random {
	($i: expr, $range: expr) => {};
}

//
// ────────────────────────────────────────────────────────────────── I
// ──────────   :::::: M A I N   F U N C T I O N S : :  :   :    :     :
// :          :
// ────────────────────────────────────────────────────────────────────────────
//

//
// ────────────────────────────────────────────────── I ──────────
//   :::::: L E A R N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

#[cfg(feature = "notvisible")]
pub mod traditional {
	use super::*;
	pub fn learn<'a, T: Literal<String> + ToString>(
		map: &'a Map<T>,
		memory: Option<usize>,
	) -> (Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>) {
		match memory {
			Some(mem) => _train(map, mem),
			None => _train(map, DEFAULT_MEMORY),
		}
	}

	fn _train<'a, T: Literal<String> + ToString>(
		map: &'a Map<T>,
		MEMORY: usize,
	) -> (Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>) {
		// Create a translated map

		let translated_map: Map<Vec<u32>> = Map::<Vec<u32>> {
			keys: translate(&map.keys),
			values: translate(&map.values),
		};

		let mut mega: Vec<Vec<f32>> = Vec::new();
		let mut ram: Vec<f32> = Vec::new();
		for (key, value) in translated_map.keys.iter().zip(translated_map.values.iter()) {
			for keyChunk in key.into_chunks(MEMORY).base {
				for valueChunk in value.into_chunks(MEMORY).base {
					ram.push(
						keyChunk.iter().sum::<u32>() as f32 / valueChunk.iter().sum::<u32>() as f32,
					);
				}
			}
			mega.push(ram.clone());
			ram.clear();
		}
		// println!("{:#?}", map[2]);
		debug_mode!("learn::mega -> {:#?}\n---------------------------\n", mega);
		return (mega, translated_map.values, map.values.literal());
	}

	pub fn run<'a, T: Literal<String>>(
		rawinput: T,
		learnt: &(Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>),
		MEMORY: Option<usize>,
		THRESHOLD: Option<f32>,
		MAX_OUTPUT_LENGTH: Option<usize>,
		RANGE: Option<usize>,
	) -> String {
		// * I know, this function is a mess, I cannot do nothing to help that, sorry,
		// * this is what happens when you use 4 different variables, that you get 4^2
		// * different things to check.

		match (MEMORY, THRESHOLD, MAX_OUTPUT_LENGTH, RANGE) {
			(Some(mem), Some(threshold), Some(output_length), Some(range)) => _run(
				rawinput.literal(),
				learnt,
				mem,
				threshold,
				output_length,
				range,
			),
			(Some(mem), Some(threshold), Some(output_length), None) => _run(
				rawinput.literal(),
				learnt,
				mem,
				threshold,
				output_length,
				DEFAULT_RANGE,
			),
			(Some(mem), Some(threshold), None, Some(range)) => _run(
				rawinput.literal(),
				learnt,
				mem,
				threshold,
				DEFAULT_MAX_OUTPUT_LENGTH,
				range,
			),
			(Some(mem), Some(threshold), None, None) => _run(
				rawinput.literal(),
				learnt,
				mem,
				threshold,
				DEFAULT_MAX_OUTPUT_LENGTH,
				DEFAULT_RANGE,
			),
			(Some(mem), None, Some(output_length), Some(range)) => _run(
				rawinput.literal(),
				learnt,
				mem,
				DEFAULT_THRESHOLD,
				output_length,
				range,
			),
			(Some(mem), None, Some(output_length), None) => _run(
				rawinput.literal(),
				learnt,
				mem,
				DEFAULT_THRESHOLD,
				output_length,
				DEFAULT_RANGE,
			),
			(Some(mem), None, None, Some(range)) => _run(
				rawinput.literal(),
				learnt,
				mem,
				DEFAULT_THRESHOLD,
				DEFAULT_MAX_OUTPUT_LENGTH,
				range,
			),
			(Some(mem), None, None, None) => _run(
				rawinput.literal(),
				learnt,
				mem,
				DEFAULT_THRESHOLD,
				DEFAULT_MAX_OUTPUT_LENGTH,
				DEFAULT_RANGE,
			),
			(None, Some(threshold), Some(output_length), Some(range)) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				threshold,
				output_length,
				range,
			),
			(None, Some(threshold), Some(output_length), None) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				threshold,
				output_length,
				DEFAULT_RANGE,
			),
			(None, Some(threshold), None, Some(range)) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				threshold,
				DEFAULT_MAX_OUTPUT_LENGTH,
				range,
			),
			(None, Some(threshold), None, None) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				threshold,
				DEFAULT_MAX_OUTPUT_LENGTH,
				DEFAULT_RANGE,
			),
			(None, None, Some(output_length), Some(range)) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				DEFAULT_THRESHOLD,
				output_length,
				range,
			),
			(None, None, Some(output_length), None) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				DEFAULT_THRESHOLD,
				output_length,
				DEFAULT_RANGE,
			),
			(None, None, None, Some(range)) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				DEFAULT_THRESHOLD,
				DEFAULT_MAX_OUTPUT_LENGTH,
				range,
			),
			(None, None, None, None) => _run(
				rawinput.literal(),
				learnt,
				DEFAULT_MEMORY,
				DEFAULT_THRESHOLD,
				DEFAULT_MAX_OUTPUT_LENGTH,
				DEFAULT_RANGE,
			),
		}
	}

	// Please don't try to understand this, it's just pain, I know

	fn _run<'a>(
		rawinput: String,
		learnt: &(Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>),
		MEMORY: usize,
		THRESHOLD: f32,
		MAX_OUTPUT_LENGTH: usize,
		RANGE: usize,
	) -> String {
		let mut input: Vec<u32> = Vec::new();
		let mut sum: u32;

		for word in rawinput.split_whitespace() {
			sum = 0;
			for c in word.chars() {
				sum += c as u32;
			}
			input.push(((sum << 1) + 1) << 1 + 1);
		}

		let mut result: String = String::new();

		// Raw Map
		let RMap: &Vec<String> = &learnt.2;

		// Translated Map
		let TMap: &Vec<Vec<u32>> = &learnt.1;

		// Mega Vec
		let Mega: &Vec<Vec<f32>> = &learnt.0;

		let mut subphrases: usize = 0;
		let mut calculation: f32;
		let mut BestMatch: Option<(f32, usize, usize)> = None;
		let mut BestMatch_unwrap: (f32, usize, usize);
		// For each word
		for IChunk in input.into_chunks(MEMORY).base {
			debug_mode!("\n##################\n\nIC -> {:?}", IChunk);
			for (i, value) in TMap.iter().enumerate() {
				// Let's see if we are going to use this phrase
				check_for_random!(i, RANGE);
				debug_mode!("I = {}: V = {:?}", i, value);
				for (j, VChunk) in value.into_chunks(MEMORY).base.iter().enumerate() {
					debug_mode!("{}: VC -> {:?}", j, VChunk);
					for MVec in Mega {
						debug_mode!("MV -> {:?}", MVec);
						for MChunk in MVec.into_chunks(MEMORY).base {
							calculation = calculation!(MChunk, IChunk, VChunk);
							if calculation < THRESHOLD {
								if (BestMatch == None) || (calculation < BestMatch.unwrap().0) {
									BestMatch = Some((calculation, i, j));
									debug_mode!("BestMatch Elected!: {:?}", BestMatch.unwrap());
									debug_mode!("@@@@@@@@@@@@@",);
									debug_mode!(
										"{} :: {:?}",
										BestMatch.unwrap().0,
										RMap[BestMatch.unwrap().1]
									);
								};
							};
						}
					}
				}
			}

			if BestMatch != None {
				// Ok, i is the vector of the value and j is the vector of the chunk. So we have
				// to recover the value from just two numbers.

				BestMatch_unwrap = BestMatch.unwrap();
				result.push_str(
					&RMap[BestMatch_unwrap.1]
						.split_whitespace()
						.collect::<Vec<&str>>()
						.into_chunks(MEMORY)
						.base[BestMatch_unwrap.2]
						.join(" "),
				);

				subphrases += 1;

				/*
				I cannot convert &RMap[BestMatch_unwrap.1]
				.split_whitespace()
				.collect::<Vec<&str>>()
				.into_chunks(MEMORY)
				.base

				into a variable, I tried with a lots of things.
				*/
				if BestMatch_unwrap.2
					== &RMap[BestMatch_unwrap.1]
						.split_whitespace()
						.collect::<Vec<&str>>()
						.into_chunks(MEMORY)
						.base
						.len() - 1
				{
					if subphrases > MAX_OUTPUT_LENGTH {}
					result.push('.');
				}
			};
		}
		return result;
	}
}

// 🦀

// Before declaring the Dynamic module, let's create a little function, so, if
// the user disables the `dynamic` feature, but not explicitly enables the
// `traditional` feature, the only function it was use is the `help` function.

//
// ────────────────────────────────────────────────────── I ──────────
//   :::::: D Y N A M I C : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

pub fn learn<T>(
	map: &DynMap<T>,
	memory: Option<usize>,
) -> (Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>)
where
	T: Dyn,
{
	match memory {
		Some(mem) => _train(map, mem),
		None => _train(map, DEFAULT_MEMORY),
	}
}

fn _train<T>(map: &DynMap<T>, MEMORY: usize) -> (Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>)
where
	T: Dyn,
{

	return (Vec::new(), Vec::new(), Vec::new());
}

use rand::Rng;

#[path = "libs/chunks.rs"]
mod chunks;
pub use chunks::*;

#[path = "libs/literal.rs"]
mod literal;
pub use literal::*;

#[path = "libs/mapping.rs"]
mod mapping;
pub use mapping::*;
