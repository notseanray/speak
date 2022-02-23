//! Speak crate made by Alex G. C. aka Blyxyas. visit github.com/blyxyas/speak for more information.

// Thanks to the Rust community, compiler and creators for making Rust a great language.

//
// ─── CRATE CONFIG ───────────────────────────────────────────────────────────────
//

#![allow(unused_attributes)]
#![crate_type = "lib"]
#![crate_name = "speak"]
#![allow(dead_code)]

//
// ─── PUBLIC ─────────────────────────────────────────────────────────────────────
//

/// # Config struct
/// This (public) struct is used to configure the algorithm, it has 2 public fields:
/// * threshold: f32,
/// * memory: usize.
/// 
/// * The multiplier is used for the `translate(...)` function, this function **is private**, so you can't use it. Please, **keep this function at the default value:** ***2***.
/// 
/// * The threshold is used for the `run(...)` function, it's the way to demand more or less overlapping between words. If the threshold is low, it will demand more, if the threshold is high, it will demand less.
/// 
/// * The memory is used both for the `learn(...)` & `run(...)` function, because of the way this NLP works, instead of linking between words (that would be lame), it links between phrases, so the memory is used to define how many words are in each phrase.
/// 
/// # WARNING
/// There also another public struct, `CONFIG`, this config is the default configuration, it is strongly encouraged to use this one, also, it's the configuration used by both functions in the case of `None` as one of the final arguments.
/// 
pub struct Config {
    
    pub multiplier: u32,
    pub threshold: f32,
    pub memory: usize,
}


/// # CONFIG
/// This struct is the **default configuration** in the form of a `Config` struct, it's strongly encouraged to use this one, also, it's the configuration used by both functions in the case of `None` as one of the final arguments.
/// 
/// ## Also
/// If you want to change something about this configuration, but mantaining the default values (because you're only experimenting, for example) you can create another struct with these values:
/// 
/// ```rust
/// let my_config = Config {
///     multiplier: 3,
///     threshold: 0.5,
///     memory: 2
/// }
/// ```
/// And experiment with it.
/// 
pub static CONFIG: Config = Config {
    multiplier: 3,
    threshold: 0.3,
    memory: 2,
};

//
// ─── MAPS ───────────────────────────────────────────────────────────────────────
//

// A map only allows these types: String, &str

/// # Literal
/// This is the trait made for the polymorphism required for the Map, the methods in this trait are only available for:
/// * String,
/// * &'static str
/// 
/// Please take in mind, you're not suposed to use this trait, it's just a helper for 90% of functions.
/// 
pub trait Literal {
    fn literal(self) -> String;
}

// Using the .literal() method on a String or &str returns the String.

impl Literal for String {
    fn literal(self) -> String {
        self
    }
}
impl Literal for &str {
    fn literal(self) -> String {
        return self.to_string();
    }
}

/// # Map<T>
/// The `Map<T>` is the most important struct in the whole crate, it's used to store the expected inputs and outputs of the NLP algorithm. If you used a HashMap before, it's almost the same.
/// 
/// # Example
/// ```rust
/// let map = Map::<&'static str>::from(vec![
///     ("Hi", "Hello"),
///     ("How are you?", "I'm fine, thank you!")
/// ]);
/// ```
/// **OR**
/// 
/// ```rust
/// let map = Map::<String>::from(vec![
///    ("Hi".to_string(), "Hello".to_string()),
///    ("How are you?".to_string(), "I'm fine, thank you!".to_string())
/// 
/// # Types
/// The Map struct only accepts types with the `Literal` trait, those are:
/// * String
/// * &'static str
/// 
/// # Methods
/// The Map struct has the following methods:
/// * `from(Vec<(T, T)>)`: This method is used to create a Map from a Vec<(T, T)>, where T is the type of the Map.
/// * `new()`: This method is used to create a Map from nothing, it creates a new Map with an empty Vec<(T, T)> as the main field.`
/// * `push(mut self, to_push: (T, T))`: This method is used to push a new element to the Map, **This new element will be in the last position of the Map**.
/// * `insert(mut self, index: usize, to_insert: (T, T))`: This method is used to insert a new element to the Map, **This new element will be in the position of the index**.
/// * remove(mut self, index: usize): This method is used to remove an element with the given from the Map.
/// * clear(): This method is used to clear the Map, it will remove all the elements.
/// # WARNING
/// Take in count that internally the Map is just a Vector, and this vector (named internally `entries`) uses all the methods of a Vector struct.

pub struct Map<T: Literal> {
    pub entries: Vec<(T, T)>,
}

macro_rules! impl_map {
    ($($T: path),*) => {
        $(
            impl Map<$T> {
                pub fn new() -> Map<$T> { return __new__::<$T>(); }
                pub fn from(vec: Vec<($T, $T)>) -> Map<String> { return __from__::<$T>(vec); }

                pub fn push(mut self, to_push: ($T, $T)) { self.entries.push(to_push); }
                pub fn insert(mut self, index: usize, to_insert: ($T, $T)) { self.entries.insert(index, to_insert); }
                pub fn remove(mut self, index: usize) { self.entries.remove(index); }
                pub fn clear(mut self) { self.entries.clear(); }
            }
        )*
    };
}

type T = String;
type U = &'static str;

impl_map!(T, U);

//
// ─── ALGORITHM ──────────────────────────────────────────────────────────────────
//

#[path = "libs/algorithm.rs"]
pub(crate) mod algo;

// learn wrapper:

///
/// # learn
/// This function is part of the main algorithm, that means two things:
/// 
/// * If you're training a very big map, I strongly recommend to make this function asynchroneous, because it will be a long process.
/// 
/// * Second, this function haves the option to use the default configuration, it's strongly recommended to use this option, you can use `None` as the final argument to use the default configuration. If you don't want the recommended configuration, use your own `usize` as memory.
/// 
/// This function is used to learn the NLP algorithm. Its parameters are:
/// * `Map<T>` (being T String or &'static str)
/// * `Option<usize>`: The memory of the algortihm, if you don't want to change the default value, use `None`, if you don't know what this is, chech the docs for the `Config` struct.
/// It will return a `Learnt` struct, which contains all the necessary info about the results. Feed with that struct to the `run(...)` function.
/// # Example
/// ```rust
/// let map = Map::<&'static str>::from(vec![
///    ("Hi", "Hello"),
///   ("How are you?", "I'm fine, thank you!")
/// ]);
/// let learned = learn(map, None);
/// ```
/// 
/// # Warning
/// This function takes some time, so don't use it too much. Because of that, it's recommended to use it in a thread. But that's on your own. Because I want to keep the code the lightest as possible.
/// 
pub fn learn<T: Literal>(map: Map<T>, memory: Option<usize>) -> algo::Learnt {
    if let Some(x) = memory {
        return algo::__learn__::<T>(map, x);
    } else {
        return algo::__learn__::<T>(map, crate::CONFIG.memory);
    }
}

// run wrapper

/// # Run
/// This function is one of the main function of the NLP algorithm, this means two things:
/// 
/// * If you trained over a very big, I recommend to make this function asynchroneous, because it will be a long process.
/// 
/// * Second, this function has option to use the recommended configuration with `None`, or with your own `f32` as threshold and `usize` as memory.
/// 
/// # Arguments
/// * `input: String` (being the input of the user)
/// * `Learnt` (being the struct returned by the `learn(...)` function)
/// * `Option<f32>` is the threshold, it's strongly recommended to use the default configuration, use `None` to use the default configuration.
/// * `Option<usize>` is the memory, it's strongly recommended to use the default configuration, use `None` to use the default configuration.
pub fn run(
    input: String,
    learnt: algo::Learnt,
    threshold: Option<f32>,
    memory: Option<usize>,
) -> String {
    match (threshold, memory) {
        (Some(x), Some(m)) => return algo::__run__(input, learnt, x, m),

        (Some(x), None) => return algo::__run__(input, learnt, x, crate::CONFIG.memory),

        (None, Some(m)) => return algo::__run__(input, learnt, crate::CONFIG.threshold, m),

        (None, None) => {
            return algo::__run__(
                input,
                learnt,
                crate::CONFIG.threshold,
                crate::CONFIG.memory,
            )
        }
    }
}

//
// ─── UTILS ──────────────────────────────────────────────────────────────────────
//

// Deconstructs a map into a Deconstructed struct (two vectors of strings, keys & values)

pub(crate) fn deconstruct<T: Literal>(map: Map<T>) -> Deconstructed<String> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (key, value) in map.entries {
        keys.push(key.literal());
        values.push(value.literal());
    }

    return Deconstructed { keys, values };
}

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

/// # ⚠️⚠️⚠️⚠️⚠️⚠️ NOT MEANT FOR PUBLIC USE, PLEASE STOP USING IT! ⚠️⚠️⚠️⚠️⚠️⚠️
/// Please, this struct is just public
pub struct Deconstructed<T> {
    pub keys: Vec<T>,
    pub values: Vec<T>,
}

impl<T> Deconstructed<T> {
    pub(crate) fn new() -> Deconstructed<T> {
        Deconstructed {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
}

pub(crate) fn translate(vec: &Vec<String>) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;
    for word in vec {
        let word = word;
        for word in word.split_whitespace() {
            for c in word.chars() {
                sum += crate::CONFIG.multiplier * c as u32;
            }
            ram.push(sum);
            sum = 0;
        }
        result.push(ram.clone());
        ram.clear();
    }
    return result;
}
