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

pub fn train(map: &utils::mapping::map<String, String>) -> Vec<f32> {
    let dec = map.deconstruct();
    let keys = utils::mapping::translate(&dec.keys);
    //let values = mapping::translate(&dec.values);

    let mut temporal: f32;
    let memory: usize = utils::CONFIG.memory as usize;

    let mut mega: Vec<f32> = Vec::new();

    for (i, aphrase) in keys.iter().enumerate() {
        // Then we guess the next word.
        if i < (memory as usize) { temporal = utils::sum(aphrase[0..i].to_vec()); }
        else { temporal = utils::sum(aphrase[(memory as usize)..i].to_vec()); };

        println!("{}", temporal);

        if i != 0 {
                contains!(mega, temporal, mega.len(), utils::CONFIG.threshold);
        } else {
            mega.push(temporal);
        }

    };
    return utils::sort(mega);
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

//endregion
*/