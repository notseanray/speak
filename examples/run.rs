use std::collections::HashMap;
use speak::{learn, run};

fn main() {
	let map = HashMap::from([
		("a", "1"),
		("b", "2"),
		("c", "3"),
	]);

	let learnt = learn(&map, None);
	let ran = run("a", learnt, None, None);
	println!("-> {}", ran);
}