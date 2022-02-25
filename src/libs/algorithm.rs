//! Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use crate::*;

pub struct Learnt { // It'sn't meant to be used by the user, just returned by the learn function and fed into the run function.
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
    
	let mut mem: usize;
    
    let inputvec_length: usize = inputvec.len() - 1;
    mem = if memory >= inputvec_length {
        inputvec.len()
    } else {
        memory
    };

/*
Now we compare:
Currently we have an input, and we're going to try and relate our learning experiences to this input.
This works because if A/B = A/C then B = C, so we're seeing if B ≈ C, if that's the case, that means that
B is probably the same word or derivated from C, and that means that we can use B, because we learned that
C is a good election when A is in our input.

So, we're finding the relation between the expected input, the expected output and the real input. But we're
also spliting all this three things in chunks, previously to this change in the algorithm we observed word by word
but splitting into chunks means that we can observe "I'm fine", that gives more information than "I'm" & "fine".

So, if the current input chunk (we're going to call it ichunk) divided by the value chunk (vchunk) is at a close distance from
a mega array (kchunk / vchunk) that means that ichunk is close to kchunk, that means that we're going to use vchunk.

I don't know if that's a good explanation, but that's our process, and that's the thing I'm trying to make from an explanation
from Rust code that's efficient, because that's the point of making this in Rust instead of Python or talking to a human. (Apart from
creating a good crate).
*/

	for X in (memory..inputvec_length).step_by(mem) {
		for vvec in learnt_data
						.translated_deconstructed
						.values.iter() {

			vvec_length = vvec.len();
			if memory >= vvec_length {
				vvec_memory = vvec_length;
			} else {
				vvec_memory = memory;
			};

			for Y in (0..vvec_length).step_by(vvec_memory) {
				for Z in (0..learnt_data.learn_vec) {
					//if ((inputvec[X - mem .. X].iter().sum() as f32 / vvec[Y - vvec_memory .. Y]) - 1).abs() <= threshold {
				}
				}
			};
		};
	};
	return result
}
