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
    // Deconstructing map into his two arrays

    let dec = map.deconstruct();
    let keys = dec.keys;
    let ckeys = keys.clone();
    let values = dec.values;

    // Now, let's create a mega array.

    let mut frequency: mapping::Deconstructed<String, usize> = mapping::Deconstructed {
        keys: Vec::new(),
        values: Vec::new(),
        size: 0
    };

    for key in keys {
        for word in key.split_whitespace() {
            let container = contains(&ckeys, word.to_string());
            
            if container.0 && container.1 < frequency.keys.len() {
                frequency.values[container.1] += 1;
            }

            else {
                frequency.keys.push(word.to_string());
                frequency.values.push(1);
            };

        };
    };

    println!("{:?}", frequency.keys);
    println!("{:?}", frequency.values);
}