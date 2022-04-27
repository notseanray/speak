use speak::{learn, Map};

fn main() {
	let map = Map::<&str>::from(vec![
		("Hello world", "Hola mundo"),
		("Hola mundo", "Hello world"),
	]);

	let learnt = learn(&map, None);
	println!("!{:?}", learnt.0);
}