#![allow(dead_code, unused_variables)]

// * /////////////////////////////
// ^ CONFIG //////////////////////
// * /////////////////////////////

pub struct Config {
    pub multiplier: u32,
    pub threshold: f32,
    pub memory: usize
}

pub static CONFIG: Config = Config {
    multiplier: 1
    ,threshold: 0.1
    ,memory: 1
};

// * /////////////////////////////
// ^ Maps ////////////////////////
// * /////////////////////////////

// A map only allows these types: String, &str

pub trait Literal {
    fn literal(&self) -> String;
}

// Using the .literal() method on a string or &str returns the String.

impl Literal for String { fn literal(&self) -> String { return String::from(self)} }
impl Literal for &str { fn literal(&self) -> String { return String::from(*self); } }

pub struct Map<T: Literal> { pub entries: Vec<(T, T)> }
pub(crate) struct Deconstructed<T: Literal> { pub keys: Vec<T>, pub values: Vec<T> }
pub struct TrainedData<T: Literal> { pub values: Vec<T>, pub mega: Vec<f32> } // Please don't use this struct.

// Creates a new map

pub(self) fn __new__<T: Literal>() -> Map<T> { return Map { entries: Vec::new() } }

// Creates a new map with the given entries

pub(self) fn __from__<T: Literal>(vec: Vec<(T, T)>) -> Map<String> {
    let mut entries: Vec<(String, String)> = Vec::new();
    for (key, value) in vec {
        entries.push((key.literal(), value.literal()));
    };
    return Map { entries }; }

// Deconstructs a map into a Deconstructed struct (two vectors of strings, keys & values)

pub(crate) fn deconstruct<T: Literal>(map: Map<T>) -> Deconstructed<String> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (key, value) in map.entries.iter() {
        keys.push(key.literal());
        values.push(value.literal());
    };

    return Deconstructed { keys, values };
}

macro_rules! impl_map {
    ($($T: path),*) => {
        $(
            impl Map<$T> {
                pub fn new() -> Map<$T> { return __new__::<$T>(); }
                pub fn from(vec: Vec<($T, $T)>) -> Map<String> { return __from__::<$T>(vec); }

                // TODO NOT TESTED!! I'LL TEST IT LATER
                pub fn push(mut self, to_push: ($T, $T)) -> Map<$T> {
                    self.entries.push(to_push);
                    return self;
                }

                pub fn insert(mut self, index: usize, to_insert: ($T, $T)) -> Map<$T> {
                    self.entries.insert(index, to_insert);
                    return self;
                }
            }
        )*
    };
}

type T = String;
type U = &'static str; 

impl_map!(T, U);

// I'm so proud of this thing.

// * /////////////////////////////
// ^ For the algorithm. //////////
// * ////////////////////////////

pub(crate) fn translate<L: Literal>(vec: &Vec<L>) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;
    for word in vec {
        let word = word.literal();
        for (i, word) in word.split_whitespace().enumerate() {
            for c in word.chars() {
                sum += CONFIG.multiplier * c as u32;
            };
            ram.push(sum);
            sum = 0;
        };
        result.push(ram.clone());
        ram.clear();
    };
        return result;
}

pub(crate) fn sum(vec: &Vec<u32>) -> f32 {
    let mut sum: u32 = 0;
    for i in vec {
        sum += i;
    };
    return sum as f32;
}

pub(crate) fn addword(str: String, part: String) -> String {
    return format!("{} {}", str, part);
}