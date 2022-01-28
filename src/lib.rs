// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.

#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

pub(crate) fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) {
    //let keys = translate(data.keys);
    //let values = translate(data.values);

    for tuple in rawdata.entries {
        println!("{}", tuple.0.literal());
        println!("{}", tuple.1.literal());
    }
}