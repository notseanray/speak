// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.
#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

pub fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) -> TrainedData<String> {
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

    for key in keys {
        for i in 0..key.len(){
            from_i =
            
            if i < config.memory { 0 }
            else { i - config.memory };
            
            for value in &values {
                for x in 0..value.len() {
                    from_x =

                    if x < config.memory { 0 }
                    else { x - config.memory };

                    //println!("――――――\n{:?} = {}", key[from_i .. i + 1].to_vec(), sum(&key[from_i .. i + 1].to_vec()));

                    mega.push(
                        sum(&key[from_i .. i + 1].to_vec()) / sum(&value[from_x .. x + 1].to_vec())
                    );
                };
            };
        };
    };
    return TrainedData { values: data.values, mega: mega};
}

pub fn run(
    rawinput: String,
    trained: TrainedData<String>,
    config: &Config
) -> String {
    let mut result: String = String::new();
// * /////////////////////////////
// ^ Translate input ////////////
// * ////////////////////////////
    let mut input: Vec<f32> = Vec::new();
    {
        let mut sum: f32;
        for word in rawinput.split_whitespace() {
            sum = 0.0;
            for c in word.chars() {
                sum += (config.multiplier * c as u32) as f32;
            };
            input.push(sum);
        };
    };

    println!("{:#?}", trained.mega);
    for word in input {
        for i in 0..trained.mega.len() {
        }
    }
    //println!("++{:?}", input);

    result.pop();
    println!("result = {}", result);
    return result;
}