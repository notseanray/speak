//! Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.

// ^ ##################
// &* ## Crate Config ##
//^ ###################

#![crate_type = "lib"]
#![crate_name = "speak"]
#![allow(dead_code)]

// ^ ##################
// &* ###### PUBLIC ######
//^ ###################

pub struct Config {
    pub multiplier: u32,
    pub threshold: f32,
    pub memory: usize,
}

pub static CONFIG: Config = Config {
    multiplier: 1,
    threshold: 0.3,
    memory: 1,
};

// ^ ##################
// & ###### MAPS ######
//^ ###################

// A map only allows these types: String, &str

pub trait Literal {
    fn literal(&self) -> String;
}

// Using the .literal() method on a string or &str returns the String.

impl Literal for String {
    fn literal(&self) -> String {
        return String::from(self);
    }
}

impl Literal for &str {
    fn literal(&self) -> String {
        return String::from(*self);
    }
}

pub struct Map<T: Literal> {
    pub entries: Vec<(T, T)>,
}

// ^ ##################
// &* ###### Train & Run ######
//^ ###################

#[path = "lib/train.rs"]
pub(crate) mod train;

// Train wrapper:
pub fn train<T: Literal>(rawdata: Map<T>, config: Option::<Config>) {
    match config {
        Some(x) => {
            train::__train__::<T>(rawdata, &x);
        },
        None => {
            train::__train__::<T>(rawdata, &crate::CONFIG);
        }
    };
}

#[path = "lib/run.rs"]
pub(crate) mod run;

pub fn run(input: String, trainData: Vec<Vec<u32>>, config: Option<Config>) {
    match config {
        Some(x) => {
            run::__run__(input, trainData, &x);
        },
        None => {
            run::__run__(input, trainData, &crate::CONFIG);
        }
    };
}

// ^ ##################
// & ###### UTILS ######
//^ ###################

// Creates a new map

pub(self) fn __new__<T: Literal>() -> Map<T> {
    return Map {
        entries: Vec::new(),
    };
}

// Creates a new map with the given entries

pub(self) fn __from__<T: Literal>(vec: Vec<(T, T)>) -> Map<String> {
    let mut entries: Vec<(String, String)> = Vec::new();
    for (key, value) in vec {
        entries.push((key.literal(), value.literal()));
    }
    return Map { entries };
}

// Deconstructs a map into a Deconstructed struct (two vectors of strings, keys & values)

pub(crate) fn deconstruct<T: Literal>(map: Map<T>) -> Deconstructed<String> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (key, value) in map.entries.iter() {
        keys.push(key.literal());
        values.push(value.literal());
    }

    return Deconstructed { keys, values };
}

macro_rules! impl_map {
    ($($T: path),*) => {
        $(
            impl Map<$T> {
                pub fn new() -> Map<$T> { return __new__::<$T>(); }
                pub fn from(vec: Vec<($T, $T)>) -> Map<String> { return __from__::<$T>(vec); }

                pub fn push(mut self, to_push: ($T, $T)) {
                    self.entries.push(to_push);
                }

                pub fn insert(mut self, index: usize, to_insert: ($T, $T)) {
                    self.entries.insert(index, to_insert);
                }

                pub fn remove(mut self, index: usize) {
                    self.entries.remove(index);
                }

                pub fn clear(mut self) {
                    self.entries.clear();
                }
            }
        )*
    };
}

type T = String;
type U = &'static str;

impl_map!(T, U);

pub(crate) struct Deconstructed<T> {
    pub keys: Vec<T>,
    pub values: Vec<T>
}

impl<T> Deconstructed<T> {
    pub fn new() -> Deconstructed<T> {
        Deconstructed { keys: Vec::new(), values: Vec::new() }
    }
}

pub(crate) fn translate<L: crate::Literal>(vec: Vec<L>) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;
    for word in vec {
        let word = word.literal();
        for word in word.split_whitespace() {
            for c in word.chars() {
                sum += crate::CONFIG.multiplier * c as u32;
            };
            ram.push(sum);
            sum = 0;
        };
        result.push(ram.clone());
        ram.clear();
    };
        return result;
}

pub(crate) fn sum(vec: Vec<u32>) -> f32 {
    let mut sum: u32 = 0;
    for i in vec {
        sum += i;
    };
    return sum as f32;
}