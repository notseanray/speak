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
// ^ MapS ////////////////////////
// * /////////////////////////////

// A map only allows these types: String, &str, u32

pub(crate) trait Allowed {}
impl Allowed for String {}
impl Allowed for Vec<String> {}

impl Allowed for &str {}
impl Allowed for Vec<&str> {}

impl Allowed for u32 {}
impl Allowed for Vec<u32> {}

pub struct Map<K: Allowed, V: Allowed> { entries: Vec<(K, V)> }
pub(crate) struct Deconstructed<K: Allowed, V: Allowed> { keys: Vec<K>, values: Vec<V> }

// Creates a new map

pub(self) fn __new__<T: Allowed>() -> Map<T, T> { return Map { entries: Vec::new() } }

// Creates a new map with the given entries

pub(self) fn __from__<T: Allowed>(vec1: Vec<T>, vec2: Vec<T>) -> Map<T, T> {
    let mut entries: Vec<(T, T)> = Vec::new();
    for (key, value) in vec1.iter().zip(vec2.iter()) { entries.push((*key, *value)); }
    return Map { entries }; }

pub(crate) fn __deconstruct__<T: Allowed>(map: Map<T, T>) -> Deconstructed<T, T> {
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
            impl Map<$T, $T> {
                pub fn new() -> Map<$T, $T> { return __new__::<$T>(); }
                pub fn from(vec1: Vec<$T>, vec2: Vec<$T>) -> Map<$T, $T> { return __from__::<$T>(vec1, vec2); }
                pub fn deconstruct(self) -> Deconstructed<$T, $T> { return __deconstruct__::<$T>(self); }
            }
        )*
    };
}


type T = String;
type VT = Vec<String>;

type U = &'static str; 
type VU = Vec<&'static str>;

type V = u32;
type VV = Vec<u32>;

impl_map!(T, VT, U, VU, V, VV);

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

pub(crate) fn translate(vec: Vec<&String>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();
    for word in vec {
        for (i, word) in word.split_whitespace().enumerate() {
            let mut sum: u32 = 0;
            for c in word.chars() {
                sum += CONFIG.multiplier * c as u32;
            };
            result.push(vec![sum as f32]);
        };
    };
        result
}

pub(crate) fn sum(vec: Vec<u32>) -> u32{
    let mut sum: u32 = 0;
    for each in vec.iter() {
        sum += each;
    };
    return sum as u32;
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