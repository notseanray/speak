use std::collections::HashMap;

#[path = "libs/chunks.rs"]

mod chunksmod;
use chunksmod::*;

#[path = "libs/mapping.rs"]

mod mapsmod;
use mapsmod::*;

// ─── INTERNAL STUFF ─────────────────────────────────────────────────────────────

// The default multiplier is 7 because it's really rare that a normal phrase (Without using rare unicode characters) to have more than 85 characters.
// 85 because, if the avg char haves 110 as value, ⌊ 2^16 / (90 * 7) ⌋ = 85. And there's no word in the English language with more than 45 words (Without being a highly technical word, like the full version of titin or something like that.)

pub(crate) trait Literal<T> {
	fn literal(self) -> T;
}

impl Literal<String> for String {
	fn literal(self) -> String {
		self
	}
}

impl Literal<String> for &str {
	fn literal(self) -> String {
		self.to_string()
	}
}

// i love rust i love rust so much am gonna set my pc on fire to symbolize how much i love rust

// Generic literal implementation
impl<T> Literal<Vec<String>> for Vec<T> where T: Literal<String> {
	fn literal(self) -> Vec<String> {
		self.iter().map(|s| s.literal()).collect::<Vec<String>>()
	}
}

fn translate(vec: Vec<String>, multiplier: u32) -> Vec<Vec<u32>> {

	let mut ram: Vec<u32> = Vec::new();
	let mut result: Vec<Vec<u32>> = Vec::new();
	let mut sum: u32;

	for phrase in vec {
		for word in phrase.split_whitespace() {
			sum = 0;
			for c in word.chars() {
				sum += c as u32 * multiplier;
			};
			ram.push(sum);
		};
		result.push(ram.clone());
		ram.clear();
	};

	return result;
}

// ─── MAIN ALGORITHM THING ───────────────────────────────────────────────────────

fn __learn__<'a, T: Literal<String> + Clone>(dict: HashMap<T, T>, multiplier: usize) where T: Literal<String> + Clone {
}
