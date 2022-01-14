#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_macros)]


pub mod mapping {
    pub struct map<K, V> {
        pub entries: Vec<(K, V)>,
    }

    pub(crate) struct Deconstructed<K, V> {
        pub keys: Vec<K>,
        pub values: Vec<V>
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

            for (key, value) in self.entries.iter() {
                keys.push(key);
                values.push(value);
            };

            Deconstructed { keys, values }
        }
    }
    pub(crate) fn contains(map: Vec<String>, target: String) -> bool {
        for entry in map {
            if entry == target {
                return true;
            }
        }
        return false;
    }
}

pub fn train(map: mapping::map<String, String>) {
    // Deconstructing map into his two arrays

    let dec = map.deconstruct();
    let keys = dec.keys;
    let values = dec.values;

    // Now, let's create a mega array.

    let mut frequency: mapping::Deconstructed<String, u32> = mapping::Deconstructed {
        keys: Vec::new(),
        values: Vec::new()
    };

    for key in keys {
        for word in key.split_whitespace() {
            let mut count = 0;
            if frequency.keys.contains(&word.to_string()) {
                
            };
        };
    };
}