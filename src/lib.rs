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

//endregion

// ^ Config
pub struct Config {
    multiplier: u32,
    threshold: f32,
    memory: i32,
}

static CONFIG: Config = Config {
    multiplier: 13,
    threshold: 0.1,
    memory: 1
};

// & TYPES (map and Deconstructed)
// region
pub mod mapping {
    #[allow(non_camel_case_types)]
    pub struct map<K, V> {
        // This is just a wrapper for the Deconstructed map.
        pub entries: Vec<(K, V)>,
    }

    pub(crate) struct Deconstructed<K, V> {
        pub keys: Vec<K>,
        pub values: Vec<V>,
        pub size: usize,
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
            };

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
//endregion

// ^ AUXILIAR FUNCTIONS
//region

pub(crate) mod math {
    fn contains(vec: &Vec<&String>, s: String) -> (bool, usize) {
        for (i, item) in vec.iter().enumerate() {
            if item == &&s {
                return (true, i);
            };
        };
        return (false, 0);
    }

    pub(crate) fn sum(vec: Vec<u32>) -> f32 {
        let mut sum: u32 = 0;
        for each in vec.iter() {
            sum += each;
        };
        return sum as f32;
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
}

//endregion

pub fn train(map: &mapping::map<String, String>) -> Vec<f32> {
    let dec = map.deconstruct();
    let keys = mapping::translate(&dec.keys);
    let values = mapping::translate(&dec.values);

    let mut mega: Vec<f32> = Vec::new();

    let mut temporal: f32;
    let memory: i32 = CONFIG.memory;


    for (i, aphrase) in keys.iter().enumerate() {
        // Then we guess the next word.
        if i < memory as usize { temporal = math::sum(aphrase[i - memory as usize..i].to_vec()); }
        else { temporal = math::sum(aphrase[(memory as usize)..i].to_vec()); };
        
        println!("{}", temporal);
        for x in 0..mega.len() {
            if (temporal / mega[x] - 1.0).abs() > CONFIG.threshold {
                mega[x] += 1.0;
            } else {
                mega.push(temporal);
            };
        };
    };
    return math::sort(mega);
}

//region

pub fn run(RawInput: String, map: &mapping::map<String, String>, TrainedData: Vec<f32>) {
    let mut input: Vec<f32> = Vec::new();
    let mut sum: u32 = 0;
    // &**********************************
    // ^ Translating the input to numbers.

    for (i, word) in RawInput.split_whitespace().enumerate() {
        for c in word.chars() {
            sum += CONFIG.multiplier * c as u32;
        };
        input.push(sum as f32);
        sum = 0;
    };

    // ^ Calculating the result

    let mut result: String = String::new();
    for (i, input_word) in input.iter().enumerate() {

    };
}

//endregion
