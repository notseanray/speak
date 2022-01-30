// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak for more information.
#[path = "lib/utils.rs"]
pub mod utils;
pub use utils::*;

pub fn train<T: Literal>(
    rawdata: Map<T>,
    config: &Config // I recommend using the default config: utils::CONFIG
) -> (
    Vec<Vec<u32>>, // Translated Keys
    Vec<Vec<u32>>, // Translated Values
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
    return (keys, values, data.values, mega);
}

pub fn run(
    rawinput: String,
    trained: (
        Vec<Vec<u32>>, // Translated Keys
        Vec<Vec<u32>>, // Translated Values
        Vec<String>, // Raw Values
        Vec<f32> // Mega
    ),
    config: &Config
) -> String {
    let mut result: String = String::new();
    let keys = trained.0;
    let values = trained.1;
    let rawvalues = trained.2;
    let mega = trained.3;

// * ////////////////////////////
// ^ Translate input ////////////
// * ////////////////////////////
    let mut input: Vec<u32> = Vec::new();
    {
        let mut sum: u32;
        for word in rawinput.split_whitespace() {
            sum = 0;
            for c in word.chars() {
                sum += config.multiplier * c as u32;
            };
            input.push(sum);
        };
    };

    println!("{:#?}", mega);
    let mut from_i: usize;
    let mut from_x: usize;
    for i in 0..input.len() {
        from_i = fromindex!(i, config.memory);
        for key in &keys {
            for x in 0..key.len() {
                for megavalue in &mega {
                    from_x = fromindex!(x, config.memory);
                    if ((sum(&input[from_i .. i].to_vec()) / sum(&key[from_x .. x].to_vec())) / megavalue - 1.0).abs() < config.threshold {
                        result = format!("{} {}", result, rawvalues[x]);
                    }
                }
            }
        }
    }
    //println!("++{:?}", input);

    result.pop();
    println!("result = {}", result);
    return result;
}