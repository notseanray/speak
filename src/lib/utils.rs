// * /////////////////////////////
// ^ CONFIG //////////////////////
// * /////////////////////////////

pub struct Config {
    pub multiplier: u32,
    pub threshold: f32,
    pub memory: i32,
}

pub(crate) static CONFIG: Config = Config {
    multiplier: 13,
    threshold: 0.1,
    memory: 1
};

// * /////////////////////////////
// ^ MAPS ////////////////////////
// * /////////////////////////////

pub mod mapping {
    #[allow(non_camel_case_types)]
    pub struct map<K, V> {
        // This is just a wrapper for the Deconstructed map.
        pub entries: Vec<(K, V)>,
    }

    pub(crate) struct Deconstructed<K, V> {
        pub keys: Vec<K>,
        pub values: Vec<V>
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

            for (key, value) in self.entries.iter() {
                keys.push(key);
                values.push(value);
            }

            Deconstructed { keys, values }
        }
    }

    pub(crate) fn translate(vec: &Vec<&String>) -> Vec<Vec<u32>> {
        // Keys:
        let mut result: Vec<Vec<u32>> = Vec::new();
        let mut ram: Vec<u32> = Vec::new();
        for pkey in vec.iter() {
            let mut sum: u32 = 0;
            for word in pkey.split_whitespace() {
                for c in word.chars() {
                    sum += super::CONFIG.multiplier * c as u32;
                };
                ram.push(sum);
                sum = 0;
            }
            result.push(ram.clone());
            ram.clear();
        }
        return result;
    }
}

// * /////////////////////////////
// ^ MISC. ///////////////////////
// * ////////////////////////////

pub(crate) fn sum(vec: Vec<u32>) -> u32{
    let mut sum: u32 = 0;
    for each in vec.iter() {
        sum += each;
    };
    return sum as u32;
}

pub(crate) fn sort(vec: Vec<f32>) -> Vec<f32>{
    let mut v = vec;
    for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i] > v[j] {
                v.swap(i, j);
            };
        };
    };
    return v;
}