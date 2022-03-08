// Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use crate::*;

/// # WARNING
/// Do not use this struct, just use it in with the `run(...)` function.

pub struct Learnt {
	pub learn_vec: Vec<Vec<f32>>,
	pub translated_deconstructed: Deconstructed<Vec<u32>>,
	pub raw_deconstructed: Deconstructed<String>
}

//
// ──────────────────────────────────────────────────────────────────── I ──────────
//   :::::: T R A I N   F U N C T I O N : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────────
//

pub(crate) fn __learn__<T: Literal>(rawdata: Map<T>, memory: usize, multiplier: u32) -> Learnt {
    let dec: Deconstructed<String> = deconstruct::<T>(rawdata);
    let decdata: Deconstructed<Vec<u32>> = Deconstructed {
        keys: translate(&dec.keys, multiplier),
        values: translate(&dec.values, multiplier),
    };

	let mut kvec_length: usize;
	let mut kmem: usize;

	let mut vvec_length: usize;
	let mut vmem: usize;

	let mut ram: Vec<f32> = Vec::new();
	let mut learn_vec: Vec<Vec<f32>> = Vec::new();

	for kvec in &decdata.keys {
		kvec_length = kvec.len();
		kmem = if memory >= kvec_length {
			kvec_length
		} else {
			memory
		};

		for kchunk in kvec.chunks(kmem) {
			for vvec in &decdata.values {
				vvec_length = vvec.len();
				vmem = if memory >= vvec_length {
					vvec_length
				} else {
					memory
				};

				for vchunk in vvec.chunks(vmem) {
					ram.push(
						kchunk.iter().sum::<u32>() as f32 /
						vchunk.iter().sum::<u32>() as f32
					);
				};
			};
			learn_vec.push(ram.clone());
			ram.clear();
		};
	};

    return Learnt {
		learn_vec,
        translated_deconstructed: decdata,
        raw_deconstructed: dec,
	}
}

//
// ──────────────────────────────────────────────────────────────── I ──────────
//   :::::: R U N   F U N C T I O N : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────
//

pub(crate) fn __run__(
	input: String,               // The input string
    learnt_data: Learnt,        // The learnt data
    threshold: f32,              // The threshold (default: 0.4)
    memory: usize,
	multiplier: u32
) -> String {
	let mut result: String = String::new();
    // First, we translate the input into a vector
    let mut inputvec: Vec<u32> = Vec::new();
		let mut sum: u32 = 0;
        for word in input.split_whitespace() {
            for char in word.chars() {
                sum += (char as u32) * multiplier;
            }
            inputvec.push(sum);
            sum = 0;
    };

	// Input
	let input_memory: usize;
	let input_length: usize = inputvec.len() + 1;
	let mut input_chunk: &[u32];

	input_memory = if memory > inputvec.len() {
		inputvec.len()
	} else {
		memory
	};

	// Translated Values
	let mut values_memory: usize;
	let mut values_length: usize;
	let mut values_chunk: &[u32];

	// Mega values
	let mut mega_memory: usize;
	let mut mega_length: usize;
	let mut mega_chunk: &[f32];

	for x in (input_memory .. input_length).step_by(input_memory) {
		input_chunk = &inputvec[x - input_memory .. x];
		println!("IC # {} | {:#?}", x, input_chunk);
		for vvec in &learnt_data.translated_deconstructed.values {
			println!("vvec");
			values_length = vvec.len();
			values_memory = if memory > values_length {
				values_length
			} else {
				memory
			};
			for y in (values_memory .. values_length + 1).step_by(values_memory) {
				values_chunk = &vvec[y - values_memory .. y];
				println!("VC # {} | {:#?}", y, values_chunk);

				for mega_vec in &learnt_data.learn_vec {
					mega_length = mega_vec.len();
					mega_memory = if memory > mega_length {
						mega_length
					} else {
						memory
					};

				println!("MV # {:?}", mega_vec);

					for float_index in (mega_memory .. mega_length + 1).step_by(mega_memory) {
						mega_chunk = &mega_vec[float_index - mega_memory .. float_index];
// Now, let's ask the question

println!("@@@@@ {} / {} = {}", input_chunk.iter().sum::<u32>(), values_chunk.iter().sum::<u32>(), input_chunk.iter().sum::<u32>() as f32 / values_chunk.iter().sum::<u32>() as f32);

println!("!!!!!! {} / {} = {} - {} = {}", input_chunk.iter().sum::<u32>(), values_chunk.iter().sum::<u32>(), input_chunk.iter().sum::<u32>() as f32 / values_chunk.iter().sum::<u32>() as f32, mega_chunk.iter().sum::<f32>(), ((input_chunk.iter().sum::<u32>() as f32 /
values_chunk.iter().sum::<u32>() as f32) -
mega_chunk.iter().sum::<f32>()) + 1.0);

if ((input_chunk.iter().sum::<u32>() as f32 /
values_chunk.iter().sum::<u32>() as f32) /
mega_chunk.iter().sum::<f32>()).abs() - 1.0 <= threshold {
	result.push_str("Elected ");
}

					};
				};
			};
		};
	};
	return result;
}
