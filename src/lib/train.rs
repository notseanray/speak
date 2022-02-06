use crate::*;

pub(crate) fn __train__<T: Literal>(rawdata: Map<T>, config: &Config) {
    let memory: usize = CONFIG.memory;
    let mut mega: Vec<Vec<f32>> = Vec::new();
    // * ////////////////////////////////////
    // ^ // Deconstructing & Translating ////
    // * ////////////////////////////////////

    let mut data: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    {
        let _data: Deconstructed<String> = deconstruct(rawdata);
        let decdata: Deconstructed<Vec<u32>> = Deconstructed::<Vec<u32>> {
            keys: translate(_data.keys),
            values: translate(_data.values)
        };

        for x in 0..decdata.values.len() {
            data.push((decdata.keys[x].clone(), decdata.values[x].clone()));
        }
    }

    // Data.0 - Vec<(Keys, Values)>
    // Data.1 - Length
    for (key, value) in data {
        println!("-{:?}", key);

// ^ ##################
// & ###### Keys ######
//^ ###################


        let mut ram: Vec<f32> = Vec::new();

        if memory > key.len() {
            println!("No")
        } else {
            for ki in 0..key.len() {
                for vi in 0..value.len() {
                    //ram.push(utils::sum(key[]));
                }
            }
        }

// ^ ##################
// & ###### Values ######
//^ ###################

        println!("{:?}", mega);

    };
}
