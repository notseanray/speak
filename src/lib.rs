// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.

#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

pub(crate) fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) {
    let data: Deconstructed<String> = deconstruct(rawdata);
    let keys = translate(data.keys);
    let values = translate(data.values);

    println!("{:?}", keys);
}