use std::collections::HashMap;
use speak::{learn, run};

fn main() {
	let map = HashMap::from([
		("Hello world", "Hola mundo"),
		("Hola mundo", "Hello world"),
	]);

	let learnt = learn(&map, Some(1));
	let ran = run("Hello world", learnt, Some(1), None);
	println!("-> {}", ran);
}