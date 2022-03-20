use std::collections::HashMap;

// ─── INTERNAL STUFF ─────────────────────────────────────────────────────────────

// The default multiplier is 7 because it's really rare that a normal phrase (Without using rare unicode characters) to have more than 85 characters.
// 85 because, if the avg char haves 110 as value, ⌊ 2^16 / (90 * 7) ⌋ = 85. And there's no word in the English language with more than 45 words (Without being a highly technical word, like the full version of titin or something like that.)

static DEFAULT_MULTIPLIER: usize = 7;

trait Literal {
	fn literal(self) -> String;
}

impl Literal for String {
	fn literal(self) -> String {
		self
	}
}

impl Literal for &str {
	fn literal(self) -> String {
		self.to_string()
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

fn chunks<'a, T>(vec: Vec<T>, size: usize) -> Vec<Vec<T>> {
	let fsize: usize;

	if size >= vec.len() {
		fsize = vec.len();
	} else {
		fsize = size;
	};

	let mut res: Vec<Vec<T>> = Vec::new();

	for i in (fsize..vec.len()).step_by(fsize) {
		res.push(vec[i - fsize .. fsize].to_vec());
	}; if fsize % vec.len() != 0 {
		res.push(vec[vec.len() - fsize .. fsize].to_vec());
	};

	return res;
}

struct Map<T> {
	keys: Vec<T>,
	values: Vec<T>
}

// In this case this function is public, because maybe an user would like to translate the HashMap, for some reason idk
pub trait ToMap<T> {
	fn to_map(self) -> Map<T>;
}

impl ToMap<String> for HashMap<String, String> {
	fn to_map(self) -> Map<String> {
		return Map::<String> {
			keys: self.clone().into_keys().collect::<Vec<String>>(),
			values: self.into_values().collect::<Vec<String>>()
		};
	}
}

// ─── MAIN ALGORITHM THING ───────────────────────────────────────────────────────

fn __learn__<'a, T>(map: HashMap<String, String>, multiplier: usize) {
}
