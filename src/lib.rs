#[path = "libs/literal.rs"]
mod lit;
use std::collections::{LinkedList, BinaryHeap};

use lit::*;

#[path = "libs/mapping.rs"]
mod map;
use map::*;

#[path = "libs/chunks.rs"]
mod chk;
use chk::*;

use std::collections::HashMap;

//
// ────────────────────────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N   A N D   U T I L S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────────────────────────
//

const DEFAULT_MEMORY: usize = 2;
const DEFAULT_THRESHOLD: f32 = 0.1;
const DEFAULT_MULTIPLIER: u16 = 7;

fn translate(iter: Vec<String>, multiplier: u16) -> Vec<Vec<u16>> {
	
	let mut sum: u16 = 0;
	let mut phrasevec: Vec<u16> = Vec::new();
	let mut _final: Vec<Vec<u16>> = Vec::new();

	for phrase in iter {
		for word in phrase.split_whitespace() {
			for c in word.chars() {
				sum += (c as u16) * multiplier
			}
			phrasevec.push(sum);
			sum = 0;
		}
		_final.push(phrasevec.clone());
		_final.clear();
	}
	
	return _final;
}

fn merge_hashmaps<T: std::hash::Hash + std::cmp::Eq>(map1: HashMap<T, T>, map2: HashMap<T, T>) -> HashMap<T, T> {
    map1.into_iter().chain(map2).collect()
}

//
// ────────────────────────────────────────────────── I ──────────
//   :::::: L E A R N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

type Learnt = Vec<u16>;

// __learn__(...) wrapper
pub fn learn<T: Literal<Vec<String>> + Clone + ToString>(data: std::collections::HashMap<T, T>, memory: Option<usize>, multiplier: Option<u16>) -> Learnt {

	let x: Map<T> = data.to_map();
	let new_map: Map<String> = Map::<String> {
		keys: x.keys.literal(),
		values: x.values.literal()
	};

	match (memory, multiplier) {
		(None, None) => __learn__(new_map, DEFAULT_MEMORY, DEFAULT_MULTIPLIER),
		(None, Some(x)) => __learn__(new_map, DEFAULT_MEMORY, x),
		(Some(x), None) => __learn__(new_map, x, DEFAULT_MULTIPLIER),
		(Some(x1), Some(x2)) => __learn__(new_map, x1, x2)
	}
}

// The main algorithm
fn __learn__(rawdata: Map<String>, memory: usize, multiplier: u16) -> Learnt {
	// First, we translate `data`
	let data: Map<Vec<u16>> = Map::<Vec<u16>> {
		keys: translate(rawdata.keys, multiplier),
		values: translate(rawdata.values, multiplier)
	};

	let mut mega: Vec<u16> = Vec::new();

	let mut krealmem: usize = 0;
	let mut vrealmem: usize = 0;

	let mut key_length: usize = 0;
	let mut value_length: usize = 0;

	for key in &data.keys {
		// First, we check if the memory is too big.
		krealmem = if memory >= key.len() {
			key.len()
		} else {
			memory
		};

		for key_chunk in key.into_chunks(krealmem).iterate() {
			for value in &data.values {
				value_length = value.len();
				vrealmem = if memory >= value.len() {
					value.len()
				} else {
					memory
				};

				for value_chunk in value.into_chunks(vrealmem).iterate() {
					mega.push(value_chunk.iter().sum::<u16>() / key_chunk.iter().sum::<u16>());
				};
			};
		};
	};
	return mega;
}

//
// ────────────────────────────────────────────────────── I ──────────
//   :::::: R E L E A R N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

/*

The user can obtain new learning data, well, we can add that data to one of two things, we can add that to the HashMap / Map or we can that to the `mega` Vec<u16>. These two options are the functions `relearn_indirect(...)` & `relearn_direct(...)`

~ `relearn_indirect(...)`:

* Returns the original HashMap
* All the original data (being literals) are still legible
* Doesn't really 'relearn' the stuff, you have to feed it to the `learn(...)` function.

* It is meant for the need of writing it in another file, reset the RAM and still have all the data.

~ `relearn_direct(...)`:

* Returns a Vec<u16> for feeding directly into the `run(...)` function.
* Not suited for external writing (just a list of u16 values)
* Doesn't recompute all the values

*/


// __relearn_direct__(...) wrapper
pub fn relearn_direct<'a,
T: Literal<Vec<String>>
+ Clone
+ ToString
+ std::hash::Hash
+ std::cmp::Eq
>(data: HashMap<T, T>, new_data: HashMap<T, T>, memory: Option<usize>, multiplier: Option<u16>) -> &'a Vec<f32> {
	match (memory, multiplier) {
		(None, None) => __relearn_direct__(data, new_data, DEFAULT_MEMORY, DEFAULT_MULTIPLIER),
		(None, Some(x)) => __relearn_direct__(data, new_data, DEFAULT_MEMORY, x),
		(Some(x), None) => __relearn_direct__(data, new_data, x, DEFAULT_MULTIPLIER),
		(Some(x1), Some(x2)) => __relearn_direct__(data, new_data, x1, x2)
	}
}

fn __relearn_direct__<'a,

	T: Literal<Vec<String>>
	+ Clone
	+ ToString
	+ std::hash::Hash
	+ std::cmp::Eq

>(data: HashMap<T, T>, new_data: HashMap<T, T>, memory: usize, multiplier: u16) -> &'a Vec<f32> {
	// First, we merge maps
	let old_length = data.len();
	let x = merge_hashmaps::<T>(data, new_data).to_map();

	// Now, we translate it.

	let map: Map<Vec<u16>> = Map::<Vec<u16>> {
		keys: translate(x.keys.literal(), multiplier),
		values: translate(x.values.literal(), multiplier)
	};

	let mut mega: Vec<f32> = Vec::new();

	// Now, we relearn the releations JUST in the unknown section

	let mut krealmem: usize;
	let mut vrealmem: usize;

	let mut key_length: usize;
	let mut value_length: usize;

	for key in map.keys {
		key_length = key.len();
		krealmem = if memory >= key_length {
			key_length
		} else {
			memory
		};

		for key_chunk in key.into_chunks(krealmem).iterate() {
			for value in &map.values[old_length ..] {
				value_length = value.len();
				vrealmem = if memory >= value_length {
					value_length
				} else {
					memory
				};

				for value_chunk in value.into_chunks(vrealmem).iterate() {
					mega.push(value_chunk.iter().sum::<u16>() as f32 / key_chunk.iter().sum::<u16>() as f32);
				};
			};
		};
	};
	return &mega;
}

//
// ────────────────────────────────────────────── I ──────────
//   :::::: R U N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────
//

