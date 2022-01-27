// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak-rust for more information.

#[path = "lib/utils.rs"]
pub mod utils;
use utils::*;

pub(crate) fn train<T: Literal>(
    rawdata: Map<T>,
    config: Config // I recommend using the default config: utils::CONFIG
) where T: Literal {
    let data: Deconstructed<String> = deconstruct(rawdata);
    let keys = translate(data.keys);
    let values = translate(data.values);

    for phrase in keys { // Number vectors (Ej: [[a, b, c, d] <- This is phrase, ···])
        
    }
}

//region
/*
pub fn run(RawInput: String, map: &utils::mapping::map<String, String>, TrainedData: Vec<f32>) {
    let mut input: Vec<f32> = Vec::new();
    let mut sum: u32 = 0;
    // &**********************************
    // ^ Translating the input to numbers.

    for (i, word) in RawInput.split_whitespace().enumerate() {
        for c in word.chars() {
            sum += utils::CONFIG.multiplier * c as u32;
        };
        input.push(sum as f32);
        sum = 0;
    };

    // ^ Calculating the result

    let mut result: String = String::new();
    for (i, input_word) in input.iter().enumerate() {

    };
}

//endregion */
