// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.

#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

mod c {
    extern "C" {
        pub(crate) fn train(keys: Vec<Vec<u32>>, values: Vec<Vec<u32>>, threshold: f32, memory: i32);
    }
}

// The function train() is just a wrapper / interface for the __train__c() function, writen in C.

pub(crate) fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) {
    let data: Deconstructed<String> = deconstruct(rawdata);
    let keys = translate(data.keys);
    let values = translate(data.values);

    unsafe {
        c::train(keys, values, config.threshold, config.memory);
    }
}