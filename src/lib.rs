// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.
#[path = "lib/utils.rs"]
pub(crate) mod utils;
pub(crate) use utils::*;

pub struct Config {
    pub multiplier: u32,
    pub threshold: f32,
    pub memory: usize
}

pub static CONFIG: Config = Config {
    multiplier: 1
    ,threshold: 0.3
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

macro_rules! fromindex {
    ($index: expr, $memory: expr) => {
        if $index < $memory { 0 } else {$index - $memory}
    };
}

pub(crate) use fromindex;

// * /////////////////////////////
// ^ For the algorithm. //////////
// * ////////////////////////////

pub fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) -> (
    Deconstructed<Vec<u32>>, // Keys & Values
    Vec<String>, // Raw Values
    Vec<f32> // Mega
) {
    let data: Deconstructed<String> = deconstruct(rawdata);
    let keys = translate(&data.keys);
    let values = translate(&data.values);

    /*
    Ok, now we have all we need to train our model.
    keys: a list of all the TRANSLATED keys in the data
    values: a list of all the TRANSLATED values in the data
    */

    let mut mega: Vec<f32> = Vec::new();
    // println!("{:?}\t-\t{}", keys, keys.len());

    let mut from_i: usize;
    let mut from_x: usize;

    for key in &keys {
        for i in 0..key.len(){
            from_i = fromindex!(i, config.memory);
            
            for value in &values {
                for x in 0..value.len() {
                    from_x = fromindex!(x, config.memory);

                    //println!("――――――\n{:?} = {}", key[from_i .. i + 1].to_vec(), sum(&key[from_i .. i + 1].to_vec()));

                    mega.push(
                        sum(&key[from_i .. i].to_vec()) / sum(&value[from_x .. x].to_vec())
                    );
                };
            };
        };
    };
    return (Deconstructed::<Vec<u32>> {
        keys,
        values
    }, data.values, mega);
}