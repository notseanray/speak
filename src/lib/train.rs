
//
// ──────────────────────────────────────────────────────────────────── I ──────────
//   :::::: T R A I N   F U N C T I O N : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────────
//

pub(self) use crate::{
    Deconstructed,
    deconstruct,
    Literal,

    Map,

    Config,
    CONFIG,

    translate,
    sum
};

pub(crate) fn __train__<T: Literal>(rawdata: Map<T>, memory: usize) {

//
// ─── DECONSTRUCTING AND TRANSLATING ─────────────────────────────────────────────
//

    
    
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

//
// ─── KEYS ───────────────────────────────────────────────────────────────────────
//

        let mut generalmega: Vec<Vec<f32>> = Vec::new();
        let mut keysmega: Vec<Vec<f32>> = Vec::new();
        let mut ram: Vec<f32> = Vec::new();

        if memory > key.len() {
            println!("No")
        } else if memory < key.len() {
            // I thought a lot of these 5 lines 
            for i in (memory..key.len()).step_by(memory) {
                ram.push(sum(key[i - memory..i].to_vec()));
            };
            if memory % key.len() != 0 {
                // key[(key.len() - memory)..]
                        ram.push(sum(key[(key.len() - memory ..)].to_vec()))
                    }
                    keysmega.push(ram);
                };
    };
}
