use crate::*;
pub fn train<T: Literal>(rawdata: Map<T>) {
    // * ////////////////////////////////////
    // ^ // Deconstructing & Translating ///
    // * ////////////////////////////////////Minor changes
    
    let mut data: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    {
        let _data: Deconstructed<String> = deconstruct(rawdata);
        let decdata: utils::Deconstructed<Vec<u32>> = Deconstructed::<Vec<u32>> {
            keys: translate(_data.keys),
            values: translate(_data.values)
        };
        
        for x in 0..decdata.values.len() {
            data.push((decdata.keys[x].clone(), decdata.values[x].clone()));
        }
    }

    // Data.0 - Vec<(Keys, Values)>
    // Data.1 - Length

    // * ////////////////////////////////////
    // ^ // Training! ///////////////////////f
    // * ////////////////////////////////////
    println!("{:?}", data);
    let mut kl: usize = 0; // Current Key length
    let mut kv: usize = 0; // Current Value length
    for (key, value) in data {
        kv = key.len();
    }

}