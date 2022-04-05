use std::collections::HashMap;
use speak::learn;

fn main() {
	let map = HashMap::from([
		("Hello world", "Hola mundo"),
		("Hola mundo", "Hello world"),
	]);

	let learnt = learn(&map, None);
	println!("!{:?}", learnt.0);
}