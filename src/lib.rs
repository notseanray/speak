use std::collections::HashMap;

// ─── INTERNAL STUFF ─────────────────────────────────────────────────────────────

pub trait Literal {
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

fn translate<'a>(vec: Vec<String>, multiplier: u32) -> &'a Vec<Vec<u32>> {

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
	return &result;
}

fn chunks<'a, T>(vec: Vec<T>, size: usize) -> Vec<&'a [T]> {
	if size >= vec.len() {
		size = vec.len();
	}

	let res: Vec<&[T]> = Vec::new();

	for i in (size..vec.len()).step_by(size) {
		res.push(&vec[i - size .. size]);
	}; if size % vec.len() != 0 {
		res.push(&vec[vec.len() - size .. size]);
	};

	return res;
}

fn learn<T: Literal>(map: HashMap<T, T>) -> &[f32] {

}
