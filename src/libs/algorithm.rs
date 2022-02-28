//! Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use paste::paste;
use crate::*;

pub struct Learnt { // It'sn't meant to be used by the user, just returned by the learn function and fed into the run function.
	learn_vec: Vec<Vec<f32>>,
	translated_deconstructed: Deconstructed<Vec<u32>>,
	raw_deconstructed: Deconstructed<String>
}

macro_rules! mainloop {
	($parent_vec: expr,
	$namevec: ident,
	$iterator: ident,
	$iteratorx: ident,
	$next: block) => {
		paste! {
			for $iterator in $parent_vec.iter() {

				[< length_ $namevec >] = $namevec.len();
				[< memory_ $namevec >] = if memory >= [< length_ $namevec >] {
					memory
				} else {
					[<length_ $namevec>]
				};

				for $iteratorx in ([< memory_ $namevec >]..$parent_vec.iter()).step_by([< memory_ $namevec >]) {
					[<chunk_ $namevec>] = &$namevec[$iterator[0] - $namevec_memory .. $iterator[0]];
					$next
				}
			}
		};
	};

	($namevec: ident,
	$iterator: expr,
	$next: block) => {
		paste! {
			let [< length_ $namevec >] = $namevec.len();
			let [< memory_ $namevec >]: usize =
			if memory >= [< length_ $namevec >] {
				memory
			} else {
				[< length_$namevec >]
			};
			
			for $iterator in([< memory_ $namevec >].. [< length_ $namevec >]).step_by([< memory_ $namevec >]) {
				[< chunk_ $namevec >] = &$namevec[$iterator - [< memory_ $namevec >] .. $iterator];
				
				$next
			}
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

	let mut length_vvec: usize;
	let mut memory_vvec: usize;
	let mut chunk_vvec: &[u32];

	let mut length_mvec: usize;
	let mut memory_mvec: usize;
	let mut chunk_mvec: &[f32];

	let mut chunk_inputvec: &[u32];

	mainloop!(
		inputvec,
		X,
		{
			mainloop!(
				vvec,
				"vvec",
				vvec,
				Y,
				{
					mainloop!(
						mvec,
						"mvec",
						Z,
						{
							println!("hi")
						};
					);
				}
			);
		}
	);
	return result;
}
