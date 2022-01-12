#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

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
    
macro_rules! deconstruct {
    ( $($v: expr, $nickname: expr), *) => {
        {
            $(
                let mut $nickname = Vec::new()
                for each in v.iter() {
                    $nickname.push(each)
                };
            );
        };
    };
};

pub fn train(map: map<K, V>) {
    // Deconstructing map into his two arrays
    map::deconstruct!(K, keys, V, values)
    // Now, let's create a mega array.

    for key in keys.iter() {
        
    }
}