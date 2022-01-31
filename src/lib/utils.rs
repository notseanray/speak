#![allow(dead_code, unused_variables)]
#[path = "../lib.rs"]
mod lib;
use lib::*;

// * /////////////////////////////
// ^ CONFIG //////////////////////
// * /////////////////////////////

pub struct Deconstructed<T> { pub keys: Vec<T>, pub values: Vec<T> }

// I'm so proud of this thing.

// * /////////////////////////////
// ^ For the algorithm. //////////
// * ////////////////////////////

pub(crate) fn translate<L: lib::Literal>(vec: &Vec<L>) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;
    for word in vec {
        let word = word.literal();
        for (i, word) in word.split_whitespace().enumerate() {
            for c in word.chars() {
                sum += CONFIG.multiplier * c as u32;
            };
            ram.push(sum);
            sum = 0;
        };
        result.push(ram.clone());
        ram.clear();
    };
        return result;
}

pub(crate) fn sum(vec: &Vec<u32>) -> f32 {
    let mut sum: u32 = 0;
    for i in vec {
        sum += i;
    };
    return sum as f32;
}