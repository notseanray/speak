use crate::*;

pub(crate) fn __train__<T: Literal>(rawdata: Map<T>, memory: usize) -> Vec<Vec<f32>> {

    let dec: Deconstructed<String> = deconstruct(rawdata);
    
    let decdata: Deconstructed<Vec<u32>> = Deconstructed {
        keys: translate::<String>(dec.keys),
        values: translate::<String>(dec.values)
    };

    let mut data: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    for x in 0..decdata.values.len() {
        data.push((decdata.keys[x].clone(), decdata.values[x].clone()));
    }

    let mut mega: Vec<Vec<f32>> = Vec::new();
    let mut ram: Vec<f32> = Vec::new();

    // Now, we can start learning the data relations between the keys and values.

    let mut key_length: usize;
    let mut value_length: usize;

    let mut key_chunk: &[u32]; // ⇐ Slice of the key
    let mut value_chunk: &[u32]; // ⇐ Slice of the value

    let mut mem: usize;

    for (key, value) in data {

        key_length = key.len();
        value_length = value.len();

        mem = if memory >= key_length { key_length } else { memory };

        for x in (mem..key_length).step_by(mem) {
            key_chunk = &key[x - mem .. x];
            
            for y in (mem .. value_length).step_by(mem) {
            value_chunk = &value[y - mem .. y];
            
                // We can now learn the relation between the key and value.
                ram.push(
                    key_chunk.iter().sum::<u32>() as f32 /
                    value_chunk.iter().sum::<u32>() as f32
                );
            };
        };

        mega.push(ram.clone());
        ram.clear();

    };
    return mega;
}