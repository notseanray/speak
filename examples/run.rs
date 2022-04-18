use std::collections::HashMap;
use speak::*;
fn main() {
	let map = HashMap::from([
		("hello", "world"),
		("hola", "mundo"),
	]);

	let learned = learn(&map, None);
	let ran: String = run(String::from("hello"), learned, None, None);
	
	println!("{}", ran.len());
}