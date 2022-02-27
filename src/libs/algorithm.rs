//! Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use crate::*;

pub struct Learnt { // It'sn't meant to be used by the user, just returned by the learn function and fed into the run function.
	learn_vec: Vec<Vec<f32>>,
	translated_deconstructed: Deconstructed<Vec<u32>>,
	raw_deconstructed: Deconstructed<String>
}

#[feature(concat_idents)]
macro_rules! mainloop {
	($parent_vec: ident,
	$namevecstring: expr,
	$iterator: ident,
	$next: block) => {

		for $iterator in $parent_vec.iter() {

			length_$namevecstring = $namevec.len();
			$namevec_memory = if memory >= $namevec_length {
				memory
			} else {
				$namevec_length
			};

			for $iterator[0] in ($namevec_memory..$parent_vec.iter()).step_by($namevec_memory) {
				$namevec_chunk = &$namevec[$iterator[0] - $namevec_memory .. $iterator[0]];
				$next
			}
		};
	};

	($namevec: ident,
	$namevecstring: expr,
	$iterator: expr,
	$next: block) => {
		let length_$namevecstring = $namevec.len();
		let $namevec_memory: usize = if memory >= $namevec_length {
			memory
		} else {
			$namevec_length
		};

		for $iterator[0] in ($namevec_memory .. $namevec_length).step_by($namevec_memory) {
			$namevec_chunk = &$namevec[$iterator[0] - $namevec_memory .. $iterator[0]];

			$next
		}
	}
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

#[allow(non_snake_case)]
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

	
	let mut vvec_length: usize;
	let mut vvec_memory: usize;
	let mut vvec_chunk: &[u32];

	let mut mvec_length: usize;
	let mut mvec_memory: usize;
	let mut mvec_chunk: &[f32];
	
	let keys_length: usize = learnt_data.translated_deconstructed.keys.len();
	
	let inputvec_memory: usize; 
    let mut inputvec_chunk: &[u32];
	
	mainloop!(
		inputvec,
		inputvec, //namevecstring
		"X",
		{
			mainloop!(
				vvec,
				"vvec",
				learnt_data.translated_deconstructed.values.iter().enumerate(),
				Y,
				{
					mainloop!(
						"mvec",
						mvec,
						Z,
						{
							println!("hi")
						};
					);
				};
			);
		}
	);
	return result;
}
