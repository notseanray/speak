//! Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

pub(crate) fn __run__(
    input: String, // The input string
    trainingData: Vec<Vec<f32>>, // The training data (returned fro m the train function)
    threshold: f32, // The threshold (default: 0.4)
    memory: usize
) -> String {
    let mut result: String = String::new();

    // First, we translate the input into a vector
    
    let mut inputvec: Vec<u32> = Vec::new();
    {    
        let mut sum: u32 = 0;
        for word in input.split_whitespace() {
            for char in word.chars() {
                sum += char as u32;
            };
            inputvec.push(sum);
            sum = 0;
        };
    };

    // Then, we calculate the distance between the input and the training data.

    for x in memory..trainingData.len() {

    }

    return result;
}
