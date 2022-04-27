//! Speak crate made by Alex G. C. Copyright (c) 2022. See LICENSE for more information about the copyright

#![allow(non_snake_case)]
#![must_use]

#[path = "libs/chunks.rs"]
mod chunks;
pub use chunks::*;

#[path = "libs/literal.rs"]
mod literal;
pub use literal::*;

#[path = "libs/mapping.rs"]
mod mapping;
pub use mapping::*;

//
// ────────────────────────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N   A N D   U T I L S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────────────────────────
//

// Explicit docs because it uses fancy graphics

#[cfg_attr(doc, aquamarine::aquamarine)]
///## Memory
///
///Every phrase is made up from words. We make a phrase from adding sequences of words together. Well,
///the `MEMORY` parameter is used to define how many words we take into account into analyzing a phrase.
///
///The functions that takes this parameter take into account that maybe the length of the phrase divided
///by the number of words in the phrase is not an integer. So this functions will take into account
/// until the last words, and then scan the words between the length of the phrase minus the memory and
/// the length of the word.
///
///```mermaid
///graph TD
///	A("Hi,")
///	B("my")
///	C{{"name"}}
///	D("is")
///	E("Alex")
///
///	F["Not found!"]
///
///	style F stroke-dasharray: 5 5
///
///	X["Iteration 1"]
///	Y["Iteration 2"]
///	Z["Bugged iteration 2"]
///
///	X-->A;
///	X-->B;
///	X-->C;
///
///	Y-->C;
///	Y-->D;
///	Y-->E;
///
///	Z-->D;
///	Z-->E;
///	Z-->F;
///```
///
///###### Honestly, I just wanted to show you how it works, and this graph.
pub const DEFAULT_MEMORY: usize = 2;

///## Threshold
///As you know, we divide two values to find their relations. Well, that relation is then checked against the threshold, if it doesn't passes the threshold, the word is not elected.
///This is the operation to determine if a word is elected. As you can see, if the threshold is too low (less than 0.1 is not recommended), the word "spaghetti" and the word "spagetti" will not be relationated. But if the threshold is too high (more than 0.3 is not recommended), a lot of words, even if they are very different, will be relationated and the final result will not have sense.
pub const DEFAULT_THRESHOLD: f32 = 0.1;
pub const DEFAULT_OUTPUT_LENGTH: usize = 2;

fn translate<T: Literal<String>>(vec: &Vec<T>) -> Vec<Vec<u32>> {
	let mut result: Vec<Vec<u32>> = Vec::new();
	let mut new_phrase: Vec<u32> = Vec::new();
	let mut sum: u32 = 0;
	for phrase in vec {
		for word in phrase.literal().split_whitespace() {
			for c in word.chars() {
				sum += c as u32;
			}
			new_phrase.push(((sum << 1) + 1) << 1 + 1);
			sum = 0;
		}
		result.push(new_phrase.clone());
		new_phrase.clear()
	}

	return result;
}

// fn merge_hashmaps<T: std::hash::Hash + std::cmp::Eq>(map1: HashMap<T, T>, map2: HashMap<T, T>) -> HashMap<T, T> {
// 	map1.into_iter().chain(map2).collect()
// }

// Long calculation I don't want to explain.
macro_rules! calculation {
	($MChunk: expr, $IChunk: expr, $VChunk: expr) => {
		($MChunk.iter().sum::<f32>()
			- ($IChunk.iter().sum::<u32>() as f32 / $VChunk.iter().sum::<u32>() as f32)).abs()
	};
}

//
// ────────────────────────────────────────────────────────────────── I ──────────
//   :::::: M A I N   F U N C T I O N S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────
//

//
// ────────────────────────────────────────────────── I ──────────
//   :::::: L E A R N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

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
	for (key, value) in translated_map.iter() {
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
	return (mega, translated_map.values, map.keys.literal());
}

pub fn run<'a, T: Literal<String>>(
	rawinput: T,
	learnt: &(Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>),
	MEMORY: Option<usize>,
	THRESHOLD: Option<f32>,
) -> String {
	match (MEMORY, THRESHOLD) {
		(Some(mem), Some(thr)) => _run(rawinput.literal(), learnt, mem, thr),
		(Some(mem), None) => _run(rawinput.literal(), learnt, mem, DEFAULT_THRESHOLD),
		(None, Some(thr)) => _run(rawinput.literal(), learnt, DEFAULT_MEMORY, thr),
		(None, None) => _run(rawinput.literal(), learnt, DEFAULT_MEMORY, DEFAULT_THRESHOLD),
	}
}

fn _run<'a>(
	rawinput: String,
	learnt: &(Vec<Vec<f32>>, Vec<Vec<u32>>, Vec<String>),
	MEMORY: usize,
	THRESHOLD: f32,
) -> String {
	// First, we translate the input.

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

	let mut calculation: f32;
	let mut BestMatch: Option<(f32, usize, usize)> = None;
	let mut BestMatch_unwrap: (f32, usize, usize);

	// For each word
	for IChunk in input.into_chunks(MEMORY).base {
		println!("\n##################\n\nIC -> {:?}", IChunk);
		for (i, value) in TMap.iter().enumerate() {
			println!("I = {}: V = {:?}", i, value);
			for (j, VChunk) in value.into_chunks(MEMORY).base.iter().enumerate() {
				println!("{}: VC -> {:?}", j, VChunk);
				for MVec in Mega {
					println!("MV -> {:?}", MVec);
					for MChunk in MVec.into_chunks(MEMORY).base {
						calculation = calculation!(MChunk, IChunk, VChunk);
						if calculation < THRESHOLD {
							if (BestMatch == None) || (calculation < BestMatch.unwrap().0) {
								BestMatch = Some((calculation, i, j));
								println!("BestMatch Elected!: {:?}", BestMatch.unwrap());
								println!("@@@@@@@@@@@@@");
								println!("{} :: {:?}", BestMatch.unwrap().0, RMap[BestMatch.unwrap().1]);
							};
						};
					}
				}
			}
		}

		if BestMatch != None {
			// Ok, i is the vector of the value and j is the vector of the chunk. So we have to recover the value from just two numbers.

			BestMatch_unwrap = BestMatch.unwrap();
			result.push_str(
				&RMap[BestMatch_unwrap.1]
					.split_whitespace()
					.collect::<Vec<&str>>()
					.into_chunks(MEMORY)
					.base[BestMatch_unwrap.2]
					.join(" "),
			);
		};
	}

	return result;
}