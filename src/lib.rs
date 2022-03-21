use std::collections::HashMap;

#[path = "libs/chunks.rs"]

mod chunks;
use chunks::*;

#[path = "libs/mapping.rs"]

mod maps;
use maps::*;

// ─── INTERNAL STUFF ─────────────────────────────────────────────────────────────

// The default multiplier is 7 because it's really rare that a normal phrase (Without using rare unicode characters) to have more than 85 characters.
// 85 because, if the avg char haves 110 as value, ⌊ 2^16 / (90 * 7) ⌋ = 85. And there's no word in the English language with more than 45 words (Without being a highly technical word, like the full version of titin or something like that.)

static DEFAULT_MULTIPLIER: usize = 7;


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

impl Literal<Vec<String>> for Vec<&str> {
	fn literal(self) -> Vec<String> {
		let mut vec: Vec<String> = Vec::new();
		for string in self {
			vec.push(string.to_string());
		};
		return vec;
	}
}

impl Literal<Vec<String>> for Vec<String> {
	fn literal(self) -> Vec<String> {
		self
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

fn __learn__<'a, T>(map: HashMap<String, String>, multiplier: usize) {
}
