use speak::*;
use std::collections::HashMap;

fn main() {
	let map: HashMap<&str, &str> = HashMap::from([
		("a", "b"),
		("c", "d"),
	]);

	let learnt = learn(&map, None);
	println!("{:#?}", learnt.0);
}
