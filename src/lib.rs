#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

pub mod mapping {
    pub struct map<K, V> {
        entries: Vec<(K, V)>
    }

    impl<K, V> map<K, V> { // Implements for maps of all types
        pub fn new() -> map<K, V> {
            map {
                entries: Vec::new()
            }
        }

        pub fn keys(&self) -> Vec<&K>{
            let mut x: Vec<&K> = Vec::new();
            for (key, _) in &self.entries {
                x.push(&key);
            }
            x
        }

        pub fn values(&self) -> Vec<&V>{
            let mut x: Vec<&V> = Vec::new();
            for (_, value) in &self.entries {
                x.push(&value)
            }
            x
        }
    }
}