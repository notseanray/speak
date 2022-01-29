// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.

#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

pub fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) {
    let data: Deconstructed<String> = deconstruct(rawdata);
    let keys = translate(data.keys);
    //let values = translate(data.values);

    /*
    Ok, now we have all we need to train our model.
    keys: a list of all the TRANSLATED keys in the data
    values: a list of all the TRANSLATED values in the data
    */

    let mut mega: Vec<u32> = Vec::new();
    println!("{:?}\t-\t{}", keys, keys.len());

    let mut from: usize;
    for key in keys {
        for i in 0..key.len(){
            from = if i < config.memory { 0 } else { i - config.memory };
                println!("{:?} = {}", key[from .. i + 1].to_vec(), sum(&key[from .. i + 1].to_vec()));
                mega.push(sum(&key[from .. i + 1].to_vec()));
        };
    }
    println!("{:?}", mega);
}
