#[path = "libs/literal.rs"]
mod lit;
use lit::*;

#[path = "libs/mapping.rs"]
mod map;
use map::*;

#[path = "libs/chunks.rs"]
mod chk;
use chk::*;

//
// ────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────
//

const DEFAULT_MEMORY: usize = 2;
const DEFAULT_THRESHOLD: f32 = 0.1;
const DEFAULT_MULTIPLIER: u8 = 7;

//
// ────────────────────────────────────────────────── I ──────────
//   :::::: L E A R N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

// __learn__(...) wrapper
pub fn learn<T>(data: std::collections::HashMap<T, T>, multiplier: Option<u8>) where T: Literal<Vec<String>> + Clone {
	if let Some(x) = multiplier {
		__learn__(data.to_map(), x);
	} else {
		__learn__(data.to_map(), DEFAULT_MULTIPLIER);
	};
}

fn __learn__<T>(data: Map<T>, multiplier: u8) {
	// Learn algorithm here!
}

//
// ────────────────────────────────────────────── I ──────────
//   :::::: R U N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────
//
