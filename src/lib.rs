#[path = "lib/utils.rs"]
pub mod utils;

// Speak crate made by Alex G. C. aka Blyxyas. Visit github.com/blyxyas/speak-rust for more information.

// &* MACROS
#[macro_export]
macro_rules! contains {
    ($mega: expr, $temporal: expr, $length: expr, $threshold: expr) => {
        // $mega: Vec<f32>
        // $temporal: Vec<f32>
        for x in 0..$length {
            if ($temporal / $mega[x] - 1.0).abs() > $threshold {
                $mega[x] += 1.0;
            } else {
                $mega.push($temporal);
            };
        };
    };
}

// &* TRAIN & RUN!

pub(crate) fn train<T>(
    data: utils::mapping::map<T, T>
) -> String {
    // Train function.
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
