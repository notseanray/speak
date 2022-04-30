//! This little example translates a Spanish "Hola mundo" to English "Hello world" in the variable EN and viceversa in ES.

use colored::Colorize;
use speak::{learn, run, Map};

#[path = "utils.rs"]
mod utils;
use crate::test_result_ as test_result;

fn main() {
	let map = Map::<&str>::from(vec![
		("Como estás", "Estoy bien"),
		("hoy es un buen dia", "Si, si lo es")
		]);

	let learnt = learn(&map, None);
	let ran = run("Como estas, hoy es un dia muy bonito", &learnt, None, None, None, None);
	println!("!{:?}", ran);
}