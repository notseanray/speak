use crate::{Literal, Map, Deconstructed, deconstruct, translate, utils};
pub fn train<T: Literal>(rawdata: Map<T>) {
    let memory: usize = crate::CONFIG.memory;
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
    for (key, value) in data {
        
        // * ////////////////////////////////////
        // ^ // Keys ////////////////////////////
        // * ////////////////////////////////////
        
        let mut ram: Vec<f32> = Vec::new();
{
        for i in 0..key.len() {
            if i % memory == 0 || i == key.len() {
                ram.push(utils::sum(key[i - memory .. i].to_vec()));
            }
        };
}
// Clearing ownerships.

        // * ////////////////////////////////////
        // ^ // Values //////////////////////////
        // * ////////////////////////////////////

{

}

    };
}