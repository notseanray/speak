//! Speak crate made by Alex G. C. Copyright (c) 2022. See LICENSE for more information about the copyright

#[path = "libs/literal.rs"]
mod lit;
use lit::*;

#[path = "libs/mapping.rs"]
mod map;
use map::*;

#[path = "libs/chunks.rs"]
mod chk;
use chk::*;

//
// ────────────────────────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N   A N D   U T I L S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────────────────────────
//

// Explicit docs because it uses fancy graphics

#[cfg_attr(doc, aquamarine::aquamarine)]
///## Memory
///
///Every phrase is made up from words. We make a phrase from adding sequences of words together. Well, ///the `memory` parameter is used to define how many words we take into account into analyzing a phrase.
///
///The functions that takes this parameter take into account that maybe the length of the phrase divided ///by the number of words in the phrase is not an integer. So this functions will take into account ///until the last words, and then scan the words between the length of the phrase minus the memory and ///the length of the word.
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

fn translate<T: Literal<String>>(vec: &Vec<T>) -> Vec<Vec<u16>> {
	let mut result: Vec<Vec<u16>> = Vec::new();
	let mut new_phrase: Vec<u16> = Vec::new();
	let mut sum: u16 = 0;
	for phrase in vec {
		for word in phrase.literal().split_whitespace() {
			for c in word.chars() {
				sum += c as u16;
			}
			new_phrase.push(sum.pow(10 / 9));
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

macro_rules! checkmem {
    ($mem: expr, $($key: expr, $keyname: ident),*) => {
        $(
            $keyname = if $mem > $key.len() {
                $key.len()
            } else {
                $mem
            };
        )*
    };
}

//
// ────────────────────────────────────────────────────────────────── I ──────────
//   :::::: M A I N   F U N C T I O N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────
//

fn _train<'a, T: Literal<String> + Chunkable<'a, String>>(
	map: &'a Map<T>,
	memory: usize,
) -> (Vec<Vec<f32>>, Map<Vec<u16>>) {
	// Create a translated map

	let translated_map: Map<Vec<u16>> = Map::<Vec<u16>> {
		keys: translate(&map.keys),
		values: translate(&map.values),
	};

	let mut mega: Vec<Vec<f32>> = Vec::new();
	let mut ram: Vec<f32> = Vec::new();
	for (key, value) in translated_map.iter() {
		for keyChunk in key.into_chunks(memory).base {
			for valueChunk in value.into_chunks(memory).base {
				ram.push(
					keyChunk.iter().sum::<u16>() as f32 / valueChunk.iter().sum::<u16>() as f32,
				);
			}
		}
		mega.push(ram.clone());
		ram.clear();
	}
	return (mega, translated_map);
}

fn _run<'a, T: Literal<String>>(
	rawinput: T,
	learnt: (Vec<Vec<f32>>, Map<Vec<u16>>),
	memory: usize,
) -> String {
	// First, we translate the input.

	let mut input: Vec<f32> = Vec::new();
	let mut sum: u16;

	for word in rawinput.literal().split_whitespace() {
		sum = 0;
		for c in word.chars() {
			sum += c as u16;
		}
		input.push((sum as f32).powf(1.00793650794))
	}

	let TMap: Map<Vec<u16>> = learnt.1;
	let Mega: Vec<Vec<f32>> = learnt.0;

	// Real Memory Section: (All ...RM are real memory.)

	// input real mem
	let mut IRM: usize;

	// key real mem
	let mut KRM: usize;

	// value real mem
	let mut VRM: usize;

	checkmem!(memory, input, IRM, KRM, VRM);

	return String::new();
}
