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

pub(crate) fn __learn__<T: Literal>(rawdata: Map<T>, memory: usize) -> Learnt {
    let dec: Deconstructed<String> = deconstruct::<T>(rawdata);
    let decdata: Deconstructed<Vec<u32>> = Deconstructed {
        keys: translate(&dec.keys),
        values: translate(&dec.values),
    };

	println!("{:#?}", decdata.keys);

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

println!("{:#?}", learn_vec);

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
) -> String {
	unimplemented!();
	let mut result: String = String::new();
    // First, we translate the input into a vector
    /*let mut inputvec: Vec<u32> = Vec::new();
    {
		let mut sum: u32 = 0;
        for word in input.split_whitespace() {
            for char in word.chars() {
                sum += char as u32;
            }
            inputvec.push(sum);
            sum = 0;
        }
    };*/
}