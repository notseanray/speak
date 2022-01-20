//region

/*
Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak-rust for more information.

This crate allows you to talk with your machine. 0 dependencies neededt. (That's the goal).

TODO(s):
- Check if this works in Linux, MacOS and old Windows.
- Program the algorithm.
- Create the documentation.
- Configuration available (With default values).

- Complete the Rust version.
- Create the Python version.
- Create the WebAssembly version.
- Create the C++ version.

I think 4 libs will be enough.

- Create some standarized tests.
- Learn how to use Docker.
- Fail the tests.
- Cry. ✅

*/


#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

//endregion

// ^ CONFIG
pub struct CONFIG {
    MULTIPLIER: u32,
    THRESHOLD: f32}
static config: CONFIG = CONFIG {
    MULTIPLIER: 17,
    THRESHOLD: 0.1};


// & TYPES (map and Deconstructed)
// region
pub mod mapping {
    pub struct map<K, V> {
        // This is just a wrapper for the Deconstructed map.
        pub entries: Vec<(K, V)>,
    }

    pub(crate) struct Deconstructed<K, V> {
        pub keys: Vec<K>,
        pub values: Vec<V>,
        pub size: usize, // I know, this isn't the best way to do it, but I'm fighting with the borrow checker and the len function.
    }

    impl<K, V> map<K, V> {
        // ^ UX:

        pub fn new() -> Self {
            map {
                entries: Vec::new(),
            }
        }

        pub fn from(entries: Vec<(K, V)>) -> map<K, V> {
            return map { entries };
        }

        // ^ Auxiliar

        pub(crate) fn deconstruct(&self) -> Deconstructed<&K, &V> {
            let mut keys = Vec::new();
            let mut values = Vec::new();

            let mut size: usize = 0;
            for (key, value) in self.entries.iter() {
                keys.push(key);
                values.push(value);
                size += 1;
            }

            Deconstructed { keys, values, size }
        }
    }

    impl<K, V> Deconstructed<K, V> {
        pub(crate) fn reconstruct(&self) -> map<&K, &V> {
            let mut entries = Vec::new();
            for i in 0..self.size {
                entries.push((&self.keys[i], &self.values[i]));
            }

            return map { entries };
        }
    }
    pub(crate) fn translate(vec: &Vec<&String>) -> Vec<Vec<u32>> {
        // Keys:
        let mut result: Vec<Vec<u32>> = Vec::new();
        let mut ram: Vec<u32> = Vec::new();
        for pkey in vec.iter() {
            let mut sum: u32 = 0;
            for word in pkey.split_whitespace() {
                for c in pkey.chars() {
                    sum += super::config.MULTIPLIER * c as u32;
                }
                ram.push(sum);
                sum = 0;
            }
            result.push(ram.clone());
            ram.clear();
        }
        return result;
    }
}

//endregion

// ^ AUXILIAR FUNCTIONS
//region

fn contains(vec: &Vec<&String>, s: String) -> (bool, usize) {
    for (i, item) in vec.iter().enumerate() {
        if item == &&s {
            return (true, i);
        };
    }
    return (false, 0);
}

fn sum(vec: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for each in vec.iter() {
        sum += each;
    }
    return sum;
}

//endregion

pub fn train(map: mapping::map<String, String>, memory: usize) { // -> Vec<Vec<f32>>
    let dec = map.deconstruct();
    let keys = mapping::translate(&dec.keys);
    let values = mapping::translate(&dec.values);

    let mut TrainedData: Vec<Vec<f32>> = Vec::new();
    let mut ram: Vec<f32> = Vec::new();

    let mut guess: u32 = 0;
    for (i, aphrase) in keys.iter().enumerate() {
        if i - memory >= 0 {
            // Then we guess the next word
            // if sum(aphrase[i - memory..i].to_vec())
        };
    };
}

//region

pub fn run(RawInput: String, map: mapping::map<String, String>, TrainedData: Vec<f32>) {
    let mut input: Vec<f32> = Vec::new();
    let mut sum: u32 = 0;
    // &**********************************
    // ^ Translating the input to numbers.

    for (i, word) in RawInput.split_whitespace().enumerate() {
        for c in word.chars() {
            sum += config.MULTIPLIER * c as u32;
        }
        input.push(sum as f32);
        sum = 0;
    }

    // ^ Calculating the result

    let mut result: String = String::new();
    for (i, input_word) in input.iter().enumerate() {}
}

//endregion