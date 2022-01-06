pub struct Data {
    pub keys: Vec<String>,
    pub values: Vec<String>
}

struct TranslatedData {
    keys: Vec<Vec<f32>>,
    values: Vec<Vec<f32>>
}

macro_rules! translate {
    ($var: expr, $vec: expr) => {
        let mut sum: f32 = 0.0;
        let mut ram: Vec<f32> = Vec::new();
        for phrase in $var.iter() {
            for word in phrase.split_whitespace() {
                for character in word.chars() {
                    sum += (7 * (character as u32)) as f32;
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
        Data {keys, values}
    }

    fn keys(&self) -> Vec<String> {
        return self.keys.clone();
    }

    pub fn values(&self) -> Vec<String> {
        return self.values.clone();
    }

    fn translate(self: Data) -> TranslatedData {
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

#[allow(unused_variables)]
pub fn run(rawinput: &str, rawdata: Data, no_similarity_found: impl Fn()){
    let data: TranslatedData = rawdata.translate();
    let mut temp: Vec<(f32, usize)> = Vec::new();
    for (i, phrase) in data.keys.iter().enumerate() { // A phrase is each list of numbers
        for (j, word) in phrase.iter().enumerate() { // each number
            for bphrase in data.values.iter() {
                for (z, bword) in bphrase.iter().enumerate() {
                temp.push(((*word / *bword) * (j + z) as f32, i))
                }
            }
        }
    }
    // Usually we would sort the temp vector here, but we're going to use the position of the elements in the vector to calculate the origin. 

    for number in temp.iter() {
        // WORKING ON HERE RIGHT NOW
    }
}