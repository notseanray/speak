#![allow(dead_code)]
pub mod data {
    pub struct Data {
        pub keys: Vec<String>,
        pub values: Vec<String>,
    }

    macro_rules! translate {
        // ↓ For the whole rawdata
        ($var: expr, $vec: expr) => {
            // var: Vec<String> vec: Vec<Vec<f32>> vec is "just for storage"
            let mut ram: Vec<f32> = Vec::new();
            let mut sum: f32 = 0.0;
            for phrase in $var.iter() {
                for word in phrase.split_whitespace() {
                    for character in word.chars() {
                        sum += (7 * character as u32) as f32;
                    }

                    ram.push(sum);
                    sum = 0.0;
                }
                $vec.push(ram.clone());
                ram.clear();
            }
        };
    }

    impl Data {
        pub fn from(keys: Vec<String>, values: Vec<String>) -> Data {
            Data { keys, values }
        }

        pub fn from_hashmap(hashmap: std::collections::HashMap<String, String>) -> Data {
            let mut keys: Vec<String> = Vec::new();
            let mut values: Vec<String> = Vec::new();
            for (key, value) in hashmap {
                keys.push(key);
                values.push(value);
            }
            Data { keys, values }
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

            return TranslatedData { keys, values };
        }
    }

    pub struct TranslatedData {
        pub keys: Vec<Vec<f32>>,
        pub values: Vec<Vec<f32>>,
    }
}

fn difference(a: f32, other: f32) -> f32 {
    return (a / other - 1.0).abs();
}

#[macro_export]
macro_rules! train {
    (rawdata: data::Data, while_not: ident) => {
        train!($data::Data, $while_not());
    };

    (rawdata: data::Data, while_not: block) => {
        let data: data::TranslatedData = rawdata.translate();
        let mut trained: Vec<f32> = Vec::new();
        for phrase in data.keys.iter() { // A phrase is each list of numbers
            for (j, word) in phrase.iter().enumerate() { // each number
                for bphrase in data.values.iter() {
                    for (z, bword) in bphrase.iter().enumerate() {
                        let tmp = (*word / *bword - 1.0).abs();
                        if tmp < 0.2 {
                            trained.push(tmp * ((2 + j + z) as f32));
                        };
                    }; //por cada frase -> cada palabra en esa frase -> cada frase b -> cada palabra en esa frase b
                };
            };
        };
    };
}

#[macro_export]
macro_rules! run {
    (rawinput: expr, not_found: block) => {{
        let mut sum: f32 = 0.0;
        let mut input: Vec<f32> = Vec::new();

        // Translate input

        for word in $rawinput.split_whitespace() {
            for character in word.chars() {
                sum += (7 * character as u32) as f32;
            }
            input.push(sum);
            sum = 0.0;
        }

        for (i, number) in input.iter().enumerate() {
            for (j, bnumber) in trained.iter().enumerate() {
                for (z, ext) in data.keys.iter().enumerate() {
                    if difference((number / ext) * (2 + i + z), bnumber) < 0.1 {
                        returnal.push()
                    };
                }
            }
        }

        if input.len() == 0 {
            $not_found;
        };
    };};
}
