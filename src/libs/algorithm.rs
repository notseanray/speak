//! Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use crate::*;

/// # WARNING
/// Do not use this struct, just use it in with the `run(...)` function.

pub struct Learnt {
	learn_vec: Vec<Vec<f32>>,
	translated_deconstructed: Deconstructed<Vec<u32>>,
	raw_deconstructed: Deconstructed<String>
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
	
    let mut data: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    for x in 0..decdata.values.len() {
		data.push((decdata.keys[x].clone(), decdata.values[x].clone()));
    }
	
    let mut mega: Vec<Vec<f32>> = Vec::new();
    let mut ram: Vec<f32> = Vec::new();
	
    // Now, we can start learning the data relations between the keys and values.
	
    let mut key_length: usize;
    let mut value_length: usize;

    let mut mem: usize;

    for (key, value) in data {
		key_length = key.len();
        value_length = value.len();
		
        mem = if memory >= key_length {
			key_length
        } else {
			memory
        };
		
        for x in (mem..key_length).step_by(mem) {
			for y in (mem..value_length).step_by(mem) {
                // We can now learn the relation between the key and value.
                ram.push(
					key[x - mem..x].iter().sum::<u32>() as f32 /
					value[y - mem..y].iter().sum::<u32>() as f32,
                );
            }
        }
		
        mega.push(ram.clone());
        ram.clear();
    }
	
    return Learnt {
		learn_vec: mega,
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
	let mut result: String = String::new();
    // First, we translate the input into a vector
    let mut inputvec: Vec<u32> = Vec::new();
    {
		let mut sum: u32 = 0;
        for word in input.split_whitespace() {
            for char in word.chars() {
                sum += char as u32;
            }
            inputvec.push(sum);
            sum = 0;
        }
    };

    // Then, we calculate the distance between the input and the learning data.

    let mut int_chunk: &[u32];
	
	let mut vvec_length: usize;
	let mut vvec_memory: usize;
	let mut vvec_chunk: &[u32];

	let mut key_length: usize;
	let mut key_memory: usize;
	let mut key_chunk: &[f32];

	let mut key_chunk_raw: Vec<f32>; // As you can probably notice, I don't like to use Vecs, but I need to do it...

	let input_memory: usize;
    let inputvec_length: usize = inputvec.len() - 1;
    
    input_memory = if memory >= inputvec_length {
        inputvec.len()
    } else {
        memory
    };

	for X in (input_memory..inputvec_length).step_by(input_memory) {
		int_chunk = &inputvec[X - input_memory .. X];
		for (IVVEC, vvec) in learnt_data
						.translated_deconstructed
						.values.iter().enumerate() {

			vvec_length = vvec.len();
			vvec_memory = if memory >= vvec_length { vvec_length } else { memory };
			
			for Y in (vvec_memory..vvec_length).step_by(vvec_memory) {
				vvec_chunk = &vvec[Y - vvec_memory .. Y];
				//[Y * keys_length



key_length = learnt_data.translated_deconstructed.keys.len();

/*
thinking this part right know, because a key has length X and a value length Y, then how can we know that (iterator) * key length is the way to go?
key_chunk_raw = learnt_data.learn_vec[IVVEC]
	.iter()
	.enumerate()
	.filter(|(i, _)| i % key_length == 0)
	.map(|(_, v)| *v)
	.collect::<Vec<f32>>();

key_chunk = &key_chunk_raw[];*/

if ((
	(int_chunk.iter().sum::<u32>() as f32) /
	(vvec_chunk.iter().sum::<u32>() as f32)) /
	key_chunk.iter().sum::<f32>()
	- 1.0).abs() <= threshold {
		//result.push_str()
};

			};
		};
	};
	return result;
}
