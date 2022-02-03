use crate::*;
pub fn train<T: Literal>(rawdata: Map<T>) {
    // * ////////////////////////////////////
    // ^ // Deconstructing & Translating ///
    // * ////////////////////////////////////Minor changes
    
    let _data: Deconstructed<String> = deconstruct(rawdata);
    let decdata: utils::Deconstructed<Vec<u32>> = Deconstructed::<Vec<u32>> {
        keys: translate(_data.keys),
        values: translate(_data.values)
    };
    
    let mut data: (Vec<(Vec<u32>, Vec<u32>)>, usize) = (Vec::new(), 0);
    for x in 0..decdata.values.len() {
        data.0.push((decdata.keys[x].clone(), decdata.values[x].clone()));
    }
    data.1 = decdata.values.len();

    // * ////////////////////////////////////
    // ^ // Training! ///////////////////////f
    // * ////////////////////////////////////
    println!("{:?}", data.0);
}