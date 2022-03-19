pub(crate) fn translate(vec: &Vec<String>, multiplier: u32) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;
    for phrase in vec {

		for word in phrase.split_whitespace() {
            for c in word.chars() {
                sum += multiplier * c as u32;
            }
            ram.push(sum);
            sum = 0;
        }
        result.push(ram.clone());
        ram.clear();
    }
    return result;
}
