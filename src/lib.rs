//! Speak crate made by Alex G. C. See LICENSE for more
//! information about the copyright.

#![doc = document_features::document_features!()]

// #![warn(missing_docs)]
#![allow(non_snake_case)]

//
// ────────────────────────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N   A N D   U T I L S : :  :   :    :     :
// :          :
// ────────────────────────────────────────────────────────────────────────────────────────────────
//

pub const DEFAULT_MEMORY: usize = 2;
// pub const DEFAULT_MEMORY: usize = 2;
pub const DEFAULT_THRESHOLD: f32 = 0.1;
pub const DEFAULT_MAX_OUTPUT_LENGTH: usize = 2;
pub const DEFAULT_RANGE: usize = 2;

#[cfg(not(feature = "fancy_docs"))]
pub const DEFAULT_RANGE: usize = 2;

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

#[inline]
fn translate(vec: &Vec<&str>) -> Vec<Vec<usize>> {
	let mut result: Vec<Vec<usize>> = Vec::new();
	let mut wordvec: Vec<usize> = Vec::new();

	let mut sum: usize = 0;

	for &phrase in vec {
		for word in phrase.split_whitespace() {
			for c in word.chars() {
				sum += c as usize;
			}
			wordvec.push(((sum << 1) + 1) << 1 + 1);
		}
		result.push(wordvec.clone());
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
			- ($IChunk.iter().sum::<usize>() as f32 / $VChunk.iter().sum::<usize>() as f32))
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

macro_rules! check_for_random {
	($i: expr, $range: expr) => {
		// if rand::thread_rng().gen_range(
		// 	0..({
		// 		if $i >= $range {
		// 			$range
		// 		} else {
		// 			$i + 1
		// 		}
		// 	} + 1),
		// ) < $range
		// {
		// 	println!("Passed {}", $i);
		// } 

		if $i < $range {
			println!("Passed {}", $i);
			true
		} else {
			
					if rand::thread_rng().gen_range(
						0..$i
					) < $range {
						true
					} else {
						false
					}
		}
	};
}

//
// ────────────────────────────────────────────────────────────────── I
// ──────────   :::::: M A I N   F U N C T I O N S : :  :   :    :     :
// :          :
// ────────────────────────────────────────────────────────────────────────────
//

//
// ────────────────────────────────────────────────────── I ──────────
//   :::::: D Y N A M I C : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

#[inline]
/// <h1>Learn</h1>
///
/// The `learn` function takes a `DynMap` and an `usize`.
/// The `DynMap` is the data that will be learnt, being composed of key-value
/// pairs (Expected inputs and outputs). The memory, explained at full in the
/// [memory documentation][mem] is used to take a number of works into an
/// imaginary *buffer* (separating the phrase into chunks.). This is useful
/// because we don't really talk word by word, we talk *syntactic units* (for
/// example, the phrase "What is your name?" is composed by *"What is"* & *"your
/// name"*. Asking something, and that thing being your name.). This function
/// is very simple, and it returns a `Vec<Vec<f32>>` meant to be fed into the
/// [run function](run) as the *`learnt`* parameters.
///
/// ## How does it work?
/// The `learn` function (being an inline wrapper of a private function) takes
/// your input (a [*DynMap*][dynmap]) and for each key and value translates the
/// words into numbers. Being those numbers stored in a Vec<Vec<usize>>.
///
/// ```text
/// Vec<Vec<usize>>
/// ^   ^   ^
/// |   |   A word
/// |   +
/// |   Vector of words
/// +
/// Vector of phrases
/// ```
///
/// Now, it breaks those vectors into chunks of `MEMORY` words, if the phrase is
/// shorter than the memory, it will be just one chunk being composed of the
/// whole phrase.
///
/// Now, for each chunk collection of keys, it calculates the relation between
/// that chunk and all the chunks in the value.
///
/// ```text
/// +--------------------------+
/// |                       (Using words because they're
/// v                       easier to understand)
/// +----+----+-----------+    |
/// | What is | Your name +<---+ Here we can see the calculation
/// +----+-+--+--+---+----+    | between these two phrase. While
/// ^ ^     ^   ^         | the value is longer than the
/// | |     |   |         | key, this isn't a problem
/// | +-----+   |         |    |
/// |       |   |         | It calculates the relation
/// +-----------+         | between the chunks by summing
/// |       |             | the whole chunk and dividing it
/// |       |             | by the  sum of the value.
/// |       |             |    |
/// +----+---+---+-----+-------+-+
/// | Hi, my | name is | is Alex |
/// +--------+---------+-+-------+
///                       ^
///                       |
///                       +
///                       See the Memory documentation
///                       to see why this is repited!
/// ```
///
/// After calculating the relations between these chunks, it pushes that
/// calculation to a `Vec<f32>`, that, after all the chunks have been pushed, is
/// cloned into a bigger vector, ready to be used in the main `Vec<Vec<f32>>`.
///
/// [dynmap]: ./struct.DynMap.html
/// [mem]: ./constant.DEFAULT_MEMORY.html
/// [run]: ./fn.run.html
pub fn learn(map: &DynMap, memory: Option<usize>) -> Vec<Vec<f32>> {
	match memory {
		Some(mem) => _train(map, mem),
		None => _train(map, DEFAULT_MEMORY),
	}
}

fn _train(map: &DynMap, MEMORY: usize) -> Vec<Vec<f32>> {
	// First ,we translate the map into two vectors:

	// We need to translate the map to learn it, not only because of simplycity, but
	// also because it's faster.

	let TKeys = translate(&map.keys);
	let TValues = translate(&map.values);

	let mut mega: Vec<Vec<f32>> = Vec::new();
	let mut ram: Vec<f32> = Vec::new();

	for (key, value) in TKeys.iter().zip(TValues) {
		for keyChunk in key.into_chunks(MEMORY).base {
			for valueChunk in value.into_chunks(MEMORY).base {
				ram.push(
					keyChunk.iter().sum::<usize>() as f32 / valueChunk.iter().sum::<usize>() as f32,
				);
			}
		}
		mega.push(ram.clone());
		ram.clear();
	}

	return mega;
}

#[inline]
#[must_use = "The run function is very expensive!"]
/// <h1>Run</h1>
///
/// The `run` function is the most expensive (and the most complex) function out
/// of the main two functions (`learn` & `run`). It takes a lot of parameters
/// (Optional a lot of them), and it returns a String. The parameters are:
///
/// - Input: `&str` (Obligatory)
/// - Map: `DynMap` (Obligatory)
/// - Learnt: `Vec<Vec<f32>>` (Obligatory)
///
/// - Memory: `usize` (Optional)
/// - Threshold: `f32` (Optional)
/// - Max Output Length: `usize` (Optional)
/// - Range: `usize`
///
/// As you can see, this function takes a lot of arguments, so let's explain
/// them:
///
/// ### Input
/// The input is a `&str`, it doesn't need anything more. Just a phrase, it can
/// be whatever you want.
///
/// ### Map
/// The map is a `DynMap`, it must be the same data that the algorithm used to
/// learn.
///
/// ### Learnt
/// The learnt is a `Vec<Vec<f32>>`, you can get it from the `learn` function,
/// **don't modify this vector**.
///
/// ### Memory
/// As the [memory documentation][mem] explains, the memory is the number of
/// words per chunk, and it's optional. Please, if you really want to understand
/// the memory, read the documentation.
///
/// ### Threshold
/// The threshold is a `f32` optional, and it's used to filter the output by
/// differences. If your input is different from the analyzing-in-moment key,
/// and the difference is higher than the threshold, the output will be
/// filtered.
///
/// ### Max Output Length
/// The output is always composed of various short phrases (Depending on the
/// memory), so you can set the maximum length (in short phrases) that can
/// contain the output.
///
/// If your memory is short, I recommend setting the maximum length to a high
/// number, like 4, the other way around if your memory is long.
///
/// ### Range
/// The range is a `usize` optional, and it's used to filter the key-value based
/// on the rank in the map. If you use a high number (Take into account the
/// length of the map), more pairs will be analyzed. I recommend setting the the
/// range to the length of the map divided by the percentage of the map you
/// *think* will be bad pairs (For example nonsensical input coming from users.)
/// 
/// **You can always learn more about these variables going to their default-value documentation.**
/// 
/// [mem]: constant.DEFAULT_MEMORY.html
pub fn run(
	input: &str,
	map: &DynMap,
	learnt: &Vec<Vec<f32>>,
	MEMORY: Option<usize>,
	THRESHOLD: Option<f32>,
	MAX_OUTPUT_LENGTH: Option<usize>,
	RANGE: Option<usize>,
) -> String {
	match (MEMORY, THRESHOLD, MAX_OUTPUT_LENGTH, RANGE) {
		(Some(mem), Some(threshold), Some(output_length), Some(range)) => {
			_run(input, learnt, map, mem, threshold, output_length, range)
		}
		(Some(mem), Some(threshold), Some(output_length), None) => _run(
			input,
			learnt,
			map,
			mem,
			threshold,
			output_length,
			DEFAULT_RANGE,
		),
		(Some(mem), Some(threshold), None, Some(range)) => _run(
			input,
			learnt,
			map,
			mem,
			threshold,
			DEFAULT_MAX_OUTPUT_LENGTH,
			range,
		),
		(Some(mem), Some(threshold), None, None) => _run(
			input,
			learnt,
			map,
			mem,
			threshold,
			DEFAULT_MAX_OUTPUT_LENGTH,
			DEFAULT_RANGE,
		),
		(Some(mem), None, Some(output_length), Some(range)) => _run(
			input,
			learnt,
			map,
			mem,
			DEFAULT_THRESHOLD,
			output_length,
			range,
		),
		(Some(mem), None, Some(output_length), None) => _run(
			input,
			learnt,
			map,
			mem,
			DEFAULT_THRESHOLD,
			output_length,
			DEFAULT_RANGE,
		),
		(Some(mem), None, None, Some(range)) => _run(
			input,
			learnt,
			map,
			mem,
			DEFAULT_THRESHOLD,
			DEFAULT_MAX_OUTPUT_LENGTH,
			range,
		),
		(Some(mem), None, None, None) => _run(
			input,
			learnt,
			map,
			mem,
			DEFAULT_THRESHOLD,
			DEFAULT_MAX_OUTPUT_LENGTH,
			DEFAULT_RANGE,
		),
		(None, Some(threshold), Some(output_length), Some(range)) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			threshold,
			output_length,
			range,
		),
		(None, Some(threshold), Some(output_length), None) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			threshold,
			output_length,
			DEFAULT_RANGE,
		),
		(None, Some(threshold), None, Some(range)) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			threshold,
			DEFAULT_MAX_OUTPUT_LENGTH,
			range,
		),
		(None, Some(threshold), None, None) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			threshold,
			DEFAULT_MAX_OUTPUT_LENGTH,
			DEFAULT_RANGE,
		),
		(None, None, Some(output_length), Some(range)) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			DEFAULT_THRESHOLD,
			output_length,
			range,
		),
		(None, None, Some(output_length), None) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			DEFAULT_THRESHOLD,
			output_length,
			DEFAULT_RANGE,
		),
		(None, None, None, Some(range)) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			DEFAULT_THRESHOLD,
			DEFAULT_MAX_OUTPUT_LENGTH,
			range,
		),
		(None, None, None, None) => _run(
			input,
			learnt,
			map,
			DEFAULT_MEMORY,
			DEFAULT_THRESHOLD,
			DEFAULT_MAX_OUTPUT_LENGTH,
			DEFAULT_RANGE,
		),
	}
}

#[must_use = "The run function is expensive, you can't waste it like that!"]
fn _run<'a>(
	rawinput: &str,
	learnt: &Vec<Vec<f32>>,
	map: &DynMap,
	MEMORY: usize,
	THRESHOLD: f32,
	MAX_OUTPUT_LENGTH: usize,
	RANGE: usize,
) -> String {
	let mut input: Vec<usize> = Vec::new();
	let mut sum: usize;

	for word in rawinput.split_whitespace() {
		sum = 0;
		for c in word.chars() {
			sum += c as usize;
		}
		input.push(((sum << 1) + 1) << 1 + 1);
	}

	let mut result: String = String::new();
	let Mega = learnt;
	let TMap = translate(&map.values);
	let RMap = &map.values;

	let mut subphrases: usize = 0;
	let mut calculation: f32;
	let mut BestMatch: Option<(f32, usize, usize)> = None;
	let mut BestMatch_unwrap: (f32, usize, usize);
	// For each word
	for IChunk in input.into_chunks(MEMORY).base {
		debug_mode!("\n##################\n\nIC -> {:?}", IChunk);
		for (i, value) in TMap.iter().enumerate() {
			// Let's see if we are going to analyze this phrase
			if check_for_random!(i, RANGE) {
				break
			};

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
									&RMap[BestMatch.unwrap().1]
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

use rand::Rng;

#[path = "libs/chunks.rs"]
mod chunks;
pub use chunks::*;

#[path = "libs/mapping.rs"]
mod mapping;
pub use mapping::*;

#[path = "libs/serialization.rs"]
mod seri;
pub use seri::*;
