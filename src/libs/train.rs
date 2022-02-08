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

    let mut generalmega: Vec<(Vec<f32>, Vec<f32>)> = Vec::new();
    let mut ram: Vec<f32> = Vec::new();

	let mut key_length: usize;
	let mut value_length: usize;

    for (key, value) in data {

        key_length = key.len();
        value_length = value.len();

//
// ^ ─── KEYS ───────────────────────────────────────────────────────────────────────
//

        let mut keysmega: Vec<&Vec<f32>> = Vec::new();

        if memory < key_length {
            println!("NoK")
        } else if memory < key_length {

            // I thought a lot of these 5 lines 
            for i in (memory..key_length).step_by(memory) {
                ram.push(sum(key[i - memory..i].to_vec()));
            };

            if memory % key.len() != 0 {
                ram.push(sum(key[(key_length - memory ..)].to_vec()))
            }
                keysmega.push(&ram);
                ram.clear();
                };

// ─── VALUES ─────────────────────────────────────────────────────────────────────

            let mut valuesmega: Vec<Vec<f32>> = Vec::new();
            if memory > value_length {
                println!("NoV");
        } else {
            for i in (memory..value_length).step_by(memory) {
                ram.push(sum(value[i - memory .. i].to_vec()));
            }
            if memory % value_length != 0 {
                ram.push(sum(value[(value_length - memory ..)].to_vec()));
            }
        };
    }
}