//! This little example translates a Spanish "Hola mundo" to English "Hello world" in the variable EN and viceversa in ES.

use colored::Colorize;
use speak::{learn, run, Map};

#[path = "utils.rs"]
mod utils;
use crate::test_result_ as test_result;

fn main() {
	let map: Map<&str> = Map::from(vec![
		("Hello world", "Hola mundo"),
		("Hola mundo", "Hello world"),
		("a", "b"),
	]);

	let learnt = learn(&map, None);

	// * This output should be "Hola mundo"
	let es: String = run("Hello world", &learnt, None, None, None, Some(0));

	// * This output should be "Hello world"
	let en: String = run("Hola mundo", &learnt, None, None, None, Some(0));


	test_result!("ES", es, "Hola mundo.");
	test_result!("EN", en, "Hello world.");
}
