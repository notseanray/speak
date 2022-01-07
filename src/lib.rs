pub mod data {
    pub struct Data {
        pub keys: Vec<String>,
        pub values: Vec<String>
    }

    macro_rules! translate {
        // ↓ For the whole rawdata
        ($var: expr, $vec: expr) => { // var: Vec<String> vec: Vec<Vec<f32>>           
            let mut sum: f32 = 0.0;
            let mut ram: Vec<f32> = Vec::new();
            for phrase in $var.iter() {
                for word in phrase.split_whitespace() {
                    for character in word.chars() {
                        sum += (7 * character as u32) as f32;
                    };
                    ram.push(sum);
                    sum = 0.0;
                };
                $vec.push(ram.clone());
                ram.clear();
            };
        };

        ($var: expr) => {
            let mut sum: f32 = 0.0;
            let mut ram: Vec<f32> = Vec::new();
            for word in $var.iter() {
                for character in word.chars() {
                    sum += (7 * character as u32) as f32;
                };
                ram.push(sum);
                sum = 0.0;
            };
        }
    }

    impl Data {
        pub fn from(keys: Vec<String>, values: Vec<String>) -> Data {
            Data {keys, values}
        }

        pub fn from_hashmap(hashmap: std::collections::HashMap<String, String>) -> Data {
            let mut keys: Vec<String> = Vec::new();
            let mut values: Vec<String> = Vec::new();
            for (key, value) in hashmap {
                keys.push(key);
                values.push(value);
            };
            Data {keys, values}
        }

        pub fn keys(&self) -> Vec<String> {
            return self.keys.clone();
        }
    
        pub fn values(&self) -> Vec<String> {
            return self.values.clone();
        }
    
        pub fn translate(self: Data) -> TranslatedData {
            let mut keys: Vec<Vec<f32>> = Vec::new();
            let mut values: Vec<Vec<f32>> = Vec::new();
    
            translate!(self.keys(), keys);
            translate!(self.values(), values);
    
            return TranslatedData {
                keys,
                values
            };
        }
    }

    pub struct TranslatedData {
        pub keys: Vec<Vec<f32>>,
        pub values: Vec<Vec<f32>>
    }
}

use data::*;
pub fn run(input: &str, rawdata: Data, no_similarity_found: impl Fn()){
    let data: data::TranslatedData = rawdata.translate();
    let mut temp: Vec<(f32, usize)> = Vec::new();
    for (i, phrase) in data.keys.iter().enumerate() { // A phrase is each list of numbers
        for (j, word) in phrase.iter().enumerate() { // each number
            for bphrase in data.values.iter() {
                for (z, bword) in bphrase.iter().enumerate() {
                let tmp = (*word / *bword - 1.0).abs();
                if tmp < 0.2 {
                temp.push((tmp * ((1 + j + z) as f32), i));
                }}
            };
        };
    };
    // Usually we would sort the temp vector here, but we're going to use the position of the elements in the vector to calc ulate the origin. 
    
    translate!(input);
    for number in input {
        for each in temp.iter() {
           // CURRENTLY WORKING HERE 
        }
    }
}