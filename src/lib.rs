pub struct Data {
    pub keys: Vec<String>,
    pub values: Vec<String>
}

macro_rules! translate {
    ($var: expr, $vec: expr) => {
        let mut sum: f32 = 0.0;
        for phrase in $var.iter() {
            for word in phrase.split_whitespace() {
                for character in word.chars() {
                    sum += (3 * (character as u32)) as f32;
                }
                $vec.push(sum);
                sum = 0.0;
            }
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
        let mut vtkey: Vec<f32> = Vec::new();
        let mut vtval: Vec<f32> = Vec::new();

        translate!(self.keys(), vtkey);
        translate!(self.values(), vtval);

        return TranslatedData {
            keys: vtkey,
            values: vtval
        };
    }
}

struct TranslatedData {
    keys: Vec<f32>,
    values: Vec<f32>
}

#[allow(unused_variables)]
pub fn run(rawinput: &str, rawdata: Data, no_similarity_found: impl Fn()){
    let data: TranslatedData = rawdata.translate();
    println!("{:?}", data.keys);
    println!("{:?}", data.values);
}