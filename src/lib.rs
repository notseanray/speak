// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.

#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

pub fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) -> Vec<f32> {
    let data: Deconstructed<String> = deconstruct(rawdata);
    let keys = translate(data.keys);
    let values = translate(data.values);

    /*
    Ok, now we have all we need to train our model.
    keys: a list of all the TRANSLATED keys in the data
    values: a list of all the TRANSLATED values in the data
    */

    let mut mega: Vec<f32> = Vec::new();
    // println!("{:?}\t-\t{}", keys, keys.len());

    let mut from_i: usize;
    let mut from_x: usize;

    for key in keys {
        for i in 0..key.len(){
            from_i = if i < config.memory { 0 } else { i - config.memory };
            for value in &values {
                for x in 0..value.len() {
                    from_x = if x < config.memory { 0 } else { x - config.memory };
                    // println!("――――――\n{:?} = {}", key[from_i .. i + 1].to_vec(), sum(&key[from_i .. i + 1].to_vec()));

                    mega.push(sum(&key[from_i .. i + 1].to_vec()) / sum(&value[from_x .. x + 1].to_vec()));
                };
            };
        };
    };
    return mega;
}

pub fn run(
    trained: Vec<f32>,
    config: &Config
) -> String {
    let mut result: String = String::new();
    return result;
}