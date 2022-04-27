//! This little example translates a Spanish "Hola mundo" to English "Hello world" in the variable EN and viceversa in ES.

use colored::Colorize;
use speak::{learn, run, Map};

macro_rules! test_result {
	($name: expr, $x: ident, $supposed: expr) => {
		if $x == $supposed {
			println!("{} {}", $name.bold(), "OK".black().italic().on_bright_green());
		} else {
			println!(
				"{} {}{} \"{}\"{}{}",
				$name.bold(),
				"FAIL".black().italic().on_bright_red(),
				" ; Result was:".red().bold(),
				$x.italic(),
				" ; Supposed to be: ".red().bold(),
				$supposed.italic()
			);
		}
	};
}

fn main() {
	let map = Map::<&str>::from(vec![
		("Hello world", "Hola mundo"),
		("Hola mundo", "Hello world"),
		("a", "b")
	]);

	let learnt = learn(&map, None);
	println!("!{:?}", learnt.0);

	// * This output should be "Hola mundo"
	let es: String = run("Hello world", &learnt, None, None);

	// * This output should be "Hello world"
	let en: String = run("Hola mundo", &learnt, None, None);

	test_result!("ES", es, "Hola mundo");
	test_result!("EN", en, "Hello world");
}
