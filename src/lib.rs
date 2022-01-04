/*
 & Want to contribute to the project? Go to github.com/blyxyas/speakrust for more info!

 ^ Dependencies used:
    * Colored

*/

fn translate(whitespace: std::str::SplitWhitespace) -> Vec<f32> {
    let mut sum: f32 = 0.0;
    let mut vec: Vec<f32> = Vec::new();
    for word in whitespace {
        for c in word.chars() {
            sum += (c as u32) as f32;
        }
        vec.push(sum);
        sum = 0.0;
    }
    return vec;
}

use std::collections::HashMap;

// ^ I know, I know what you're thinking... What is this? I know, I have no idea what I'm doing, so this section will be a mess, but I'm just trying to get it working.

pub fn run (rawinput: &str, data: HashMap<&str, &str>, no_similarity_found: impl Fn()){
    
    let mut result: Vec<&str> = Vec::new();
    let input = translate(rawinput.split_whitespace());
    let mut ram: Vec<(usize, usize)> = Vec::new(); // This vector will store the y (the index in the global 'data' hashmap) and i (the index in data[y])

    for number in input { // ^ Currently input is just an array of numbers
        for (y, rawkey) in data.keys().enumerate() {
            let key = translate(rawkey.split_whitespace());
            for (i, keynumber) in key.iter().enumerate() {
                if (keynumber / number - 1.0).abs() < 0.2 {
                    ram.push((y, i))
                }
            }
        }
    }

    // ^ Now, postprocessing!

    if ram.len() != 0 {
        // TODO posprocessing
    }
        else { // If RAM has no elements
        no_similarity_found();
        std::process::exit(0);
    }
}