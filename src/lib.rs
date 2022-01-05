pub mod Data {
    pub struct Data {
        pub keys: Vec<String>,
        pub values: Vec<String>
    }

    impl Data {
        pub fn from(keys: Vec<String>, values: Vec<String>) -> Data {
            Data {
                keys: keys,
                values: values
            }
        }

        pub fn from_hashmap(map: std::collections::HashMap<String, String>) -> Data {
            let mut keys = Vec::new();
            let mut values = Vec::new();

            for (key, value) in map {
                keys.push(key);
                values.push(value);
            }

            Data {
                keys: keys,
                values: values
            }
        }

        fn keys(&self) -> Vec<String> {
            return self.keys.clone();
        }

        pub fn values(&self) -> Vec<String> {
            return self.values.clone();
        }

        fn translate(&mut self) -> Vec<f32> {
            let mut sum: f32 = 0.0;
            let mut vec: Vec<f32> = Vec::new();
            for phrase in &self.keys() {
                for word in phrase.split_whitespace() {
                    for character in word.chars() {
                    sum += (3 * (character as u32)) as f32;
                    }
                }
                sum = 0.0;
            }
            return vec;
        }

        fn enumerate(&self) -> Vec<(usize, String)> {
            let mut vec: Vec<(usize, String)> = Vec::new();
            for (index, value) in self.values().iter().enumerate() {
                vec.push((index, value.clone()));
            }
            return vec;
        }
    }
}

pub fn run (rawinput: &str, rawdata: Data::Data, no_similarity_found: impl Fn()){

}