use std::collections::HashMap;
use speak::learn;

fn main() {
	let map = HashMap::from([
		("a", "1"),
		("b", "2"),
		("c", "3"),
	]);

	let learnt = learn(&map, None);
	println!("!{:?}", learnt.0);
}