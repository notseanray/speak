// * /////////////////////////////
// ^ CONFIG //////////////////////
// * /////////////////////////////

pub struct Config {
    multiplier: u32
    ,threshold: f32
    ,memory:    i32
}

pub static CONFIG: Config = Config {
    multiplier: 1
    ,threshold: 0.1
    ,memory:    1
};

// * /////////////////////////////
// ^ Maps ////////////////////////
// * /////////////////////////////

// A map only allows these types: String, &str

pub(crate) trait LiteralTrait {
    fn literal(&self) -> String;
}

// Using the .literal() method on a string or &str returns the String.

impl LiteralTrait for String { fn literal(&self) -> String { return *self; } }
impl LiteralTrait for &str { fn literal(&self) -> String { return String::from(*self); } }

pub struct Map<T> { entries: Vec<(T, T)> }
pub(crate) struct Deconstructed<T: LiteralTrait> { keys: Vec<T>, values: Vec<T> }

// Creates a new map

pub(self) fn __new__<T: LiteralTrait>() -> Map<T> { return Map { entries: Vec::new() } }

// Creates a new map with the given entries

pub(self) fn __from__<T: LiteralTrait>(vec1: Vec<T>, vec2: Vec<T>) -> Map<T> {
    let mut entries: Vec<(T, T)> = Vec::new();
    for (key, value) in vec1.iter().zip(vec2.iter()) { entries.push((*key, *value)); }
    return Map { entries }; }

pub(crate) fn __deconstruct__<T: LiteralTrait>(map: Map<T>) -> Deconstructed<T> {
    let mut keys: Vec<T> = Vec::new();
    let mut values: Vec<T> = Vec::new();

    for (key, value) in map.entries.iter() {
        keys.push(*key);
        values.push(*value);
    }

    return Deconstructed { keys, values };
}

macro_rules! impl_map {
    ($($T: path),*) => {
        $(
            impl Map<$T> {
                pub fn new() -> Map<$T> { return __new__::<$T>(); }
                pub fn from(vec1: Vec<$T>, vec2: Vec<$T>) -> Map<$T> { return __from__::<$T>(vec1, vec2); }
                pub fn deconstruct(self) -> Deconstructed<$T> { return __deconstruct__::<$T>(self); }
            }
        )*
    };
}

type T = String;
type VT = Vec<String>;

type U = &'static str; 
type VU = Vec<&'static str>;

impl_map!(T, VT, U, VU);

// I'm so proud of this thing.

/*
impl<K, V> Map<K, V> {
    // ^ UX:

    pub fn new() -> Self { Map { entries: Vec::new() } }

    pub fn from(entries: Vec<(K, V)>) -> Map<K, V> {
        return Map { entries };
    }

    // ^ Auxiliar

    pub(crate) fn deconstruct<T>(&self) -> Deconstructed<&K, &V> {
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for (key, value) in self.entries.iter() {
            keys.push(key);
            values.push(value);
        }

        Deconstructed { keys, values }
    }
}*/

// * /////////////////////////////
// ^ MISC. ///////////////////////
// * ////////////////////////////

pub(crate) fn translate<Literal: LiteralTrait>(vec: Vec<Literal>) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();

    for word in vec {
        let word = word.literal();
        for (i, word) in word.split_whitespace().enumerate() {
            let mut sum: u32 = 0;

            for c in word.chars() {
                sum += CONFIG.multiplier * c as u32;
            };
            
            ram.push(sum);
            sum = 0;
        };
        result.push(ram);
        ram.clear();
    };
        return result;
}

pub(crate) fn sum(vec: Vec<u32>) -> u32{
    let mut sum: u32 = 0;
    for each in vec.iter() {
        sum += each;
    };
    return sum as u32;
}

pub(crate) fn sortf32(vec: Vec<f32>) -> Vec<f32>{
    let mut v = vec;
    for i in 0..vec.len() {
        if v[i] > v[i+1] {
            v.swap(i, i+1);
        };
    };
    return v;
}