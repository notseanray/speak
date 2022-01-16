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
#![allow(unused_macros)]

pub mod mapping {
    use std::fmt::Display;
    pub struct map<K, V> { // This is just a wrapper for the Deconstructed map.
        pub entries: Vec<(K, V)>,
    }

    pub(crate) struct Deconstructed<K, V> {
        pub keys: Vec<K>,
        pub values: Vec<V>,
        pub size: usize // I know, this isn't the best way to do it, but I'm fighting with the borrow checker and the len function.
    }

    impl<K, V> map<K, V> {
// Implements for maps of all types
// ^ UX:

            pub fn new() -> Self {
                map {
                    entries: Vec::new()
                }
            }

            pub fn from(entries: Vec<(K, V)>) -> map<K, V> {
                return map {
                    entries
                };
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
            };

            Deconstructed { keys, values, size }
        }
    }

    impl<K, V> Deconstructed<K, V> {
        pub(crate) fn reconstruct(&self) -> map<&K, &V> {
            let mut entries = Vec::new();

            for i in 0..self.size {
                entries.push((&self.keys[i], &self.values[i]));
            }

            return map {
                entries
            };
        }
    }
    
    pub(crate) fn translate(vec: &Vec<&String>) -> Vec<Vec<u32>> {
        // Keys:
        let mut V: Vec<Vec<u32>> = Vec::new();
        let mut ram: Vec<u32> = Vec::new();
        for pkey in vec.iter() {
            let mut sum: u32 = 0;
            for word in pkey.split_whitespace() {
                for c in pkey.chars() {
                    sum += 5 * c as u32;
                };
                ram.push(sum);
                sum = 0;
            };
            V.push(ram.clone());
            ram.clear();
        };
        return V;
    }
}

fn contains(vec: &Vec<&String>, s: String) -> (bool, usize) {
    for (i, item) in vec.iter().enumerate() {
        if item == &&s {
            return (true, i);
        };
    };
    return (false, 0);

}

pub fn train(map: mapping::map<String, String>) {
    let dec = map.deconstruct();
    let keys = mapping::translate(&dec.keys);
    let values = mapping::translate(&dec.values);

    let mut mega: Vec<f32> = Vec::new();
    for aphrase in keys.iter() {
        for (x, aword) in aphrase.iter().enumerate() { 
            for bphrase in values.iter() {
                for (z, bword) in bphrase.iter().enumerate() {
                    mega.push(*aword as f32 / *bword as f32 + ((1 + x) / (1 + z)) as f32);
                };
            }
        }
    };

    println!("{:?}", mega);
}