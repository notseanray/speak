use std::collections::HashMap;
use speak::{self, learn};

fn main() {
	let demand: &str = &std::env::args().nth(1).unwrap();
	match demand {
		"1" => example1(),
		_ => {println!("\033[1;94mPlease, enter the number of the example you want to run. Currently there are 1 example\n\n\033[1;93mExample: Run the first example:\033[0m\ncargo run 1");}
	}
}

// * ====================
// ^ Example1:
// * ====================

fn example1() {
	let map: HashMap<&str, &str> = HashMap::from([
		("a", "b"),
		("c", "d"),
	]);

	let learnt = learn(&map, None);
	println!("{:#?}", learnt.0);
}