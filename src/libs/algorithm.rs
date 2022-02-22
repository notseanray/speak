//! Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use crate::*;

pub struct Learnt { // It'sn't meant to be used by the user, just returned by the train function and fed into the run function. I was going to use Learn-2F-ed but the name was confusing, my dreams crushed and my day was ruined.

    pub learn_vec: Vec<Vec<f32>>,
    pub translated_deconstructed: Deconstructed<Vec<u32>>,
    pub raw_deconstructed: Deconstructed<String>

}

pub(crate) fn __train__<T: Literal>(rawdata: Map<T>, memory: usize) -> Learnt {
    let dec: Deconstructed<String> = deconstruct(rawdata);
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

    let mut key_chunk: &[u32]; // ⇐ Slice of the key
    let mut value_chunk: &[u32]; // ⇐ Slice of the value

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
            key_chunk = &key[x - mem..x];

            for y in (mem..value_length).step_by(mem) {
                value_chunk = &value[y - mem..y];
                // We can now learn the relation between the key and value.
                ram.push(
                    key_chunk.iter().sum::<u32>() as f32 / value_chunk.iter().sum::<u32>() as f32,
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

    // Then, we calculate the distance between the input and the training data.

    let mut int_chunk: &[u32];
    let mut mem: usize;
    
    let inputvec_length: usize = inputvec.len();
    mem = if memory >= inputvec_length {
        inputvec.len()
    } else {
        memory
    };
    
    for x in (mem..inputvec_length).step_by(mem) {
        int_chunk = &inputvec[x - mem..x];
        
    }

    return result;
}