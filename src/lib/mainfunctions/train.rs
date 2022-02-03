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
    // ^ // Spliting into chunks ////////////
    // * ////////////////////////////////////

    let mut chunks: Vec<(Vec<Vec<u32>>, Vec<Vec<u32>>)> = Vec::new();
    let mut i: usize = 0;

    let mut ram: Vec<u32> = Vec::new();
    for (key, value) in data {
    }
}