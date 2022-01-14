#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_macros)]


pub mod mapping {
    pub struct map<K, V> {
        entries: Vec<(K, V)>,
    }

    pub(crate) struct Deconstructed<K, V> {
        pub keys: Vec<K>,
        pub values: Vec<V>
    }

    impl<K, V> map<K, V> {
// Implements for maps of all types
// ^ UX:
        
            pub fn new() -> map<K, V> {
                map {
                    entries: Vec::new(),
                }
            }

            pub fn from(entries: Vec<(K, V)>) -> map<K, V> {
                map {
                    entries
                }
            }

// ^ Auxiliar

        pub(crate) fn deconstruct(&self) -> Deconstructed<&K, &V> {
            let mut keys = Vec::new();
            let mut values = Vec::new();

            for (key, value) in self.entries.iter() {
                keys.push(key);
                values.push(value);
            }

            Deconstructed { keys, values }
        }

        pub(crate) fn reconstruct(keys: Vec<K>, values: Vec<V>) -> map<K, V> {
            let mut entries: Vec<(K, V)> = Vec::new();
            for i in 0..keys.len() {
                entries.push((keys[i], values[i]));
            }
            return map {entries};
        }
    }
}

pub fn train(map: mapping::map<K, V>) {

    // Deconstructing map into his two arrays
    let dec: mapping::Deconstructed<K, V> = map.deconstruct();
    let keys = dec.keys;
    let values = dec.values;
    
    // Now, let's create a mega array.
    let mut mega: Deconstructed<K, u32>
}
