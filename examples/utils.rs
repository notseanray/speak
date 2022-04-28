use std::process::exit;

#[macro_export]
macro_rules! test_result_ {
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
	println!("This file was not intended to be used directly. It's just a file containing util functions for other examples.");
	exit(0);
}