// THIS FILE IS OBSOLETE!! THIS FILE IS JUST FOR ARCHIVING THE OLD ENGINE. BUT THIS ENGINE ISN'T EVEN COMPLETE! IT IS MID DEBUGGING!

#![allow(unused_variables)]
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
                    for character in word.to_lowercase().chars() {
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

        pub fn clone(&self) -> Data {
            Data {
                keys: self.keys.clone(),
                values: self.values.clone(),
            }
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

pub fn train(rawdata: data::Data) -> (Vec<Vec<f32>>, Vec<f32>) {
    let data: data::TranslatedData = rawdata.translate();
    let mut trained: Vec<f32> = Vec::new();
    for (i, phrase) in data.keys.iter().enumerate() {
        // A phrase is each list of numbers
        for (j, word) in phrase.iter().enumerate() {
            // each number
            for (y, bphrase) in data.values.iter().enumerate() {
                for (z, bword) in bphrase.iter().enumerate() {
                    if i != y {
                        let tmp = *word / *bword;
                        println!("{}: {} / {} ({} :: {})", bword,word, bword, tmp, tmp * ((2 + j + z) as f32));
                        if tmp < 0.5 {
                            trained.push(tmp * ((2 + j + z) as f32));
                        };
                    }
                };
            };
        };
    };
    return (data.keys, trained);
    }

pub fn run(rawinput: String, trained: Vec<f32>, keys: Vec<Vec<f32>>, values: Vec<String>, callback: impl Fn()) {
        let mut sum: f32 = 0.0;
        let mut input: Vec<f32> = Vec::new();
        let mut returnal: Vec<Option<String>> = Vec::new();

        // Translate input

        for word in rawinput.split_whitespace() {
            for character in word.chars() {
                sum += (7 * character as u32) as f32;
            }
            input.push(sum);
            sum = 0.0;
        }

        for (i, number) in input.iter().enumerate() {
            for (j, bnumber) in trained.iter().enumerate() {
                for list in keys.iter().enumerate() {
                    for (z, ext) in list.1.iter().enumerate() {
                        //if difference((number / ext) * (2 + i + z) as f32, *bnumber) < 0.2 {
                        //    returnal.push(Some(values[j].clone()));
                        //};
                    }
                }
            }
        }

        if input.len() == 0 {
            callback();
        };
    }